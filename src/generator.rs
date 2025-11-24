use crate::schema::{Table, Column};
use rand::Rng;
use rand::seq::SliceRandom;
use anyhow::Result;
use std::fs::File;
use std::io::Write;
use std::collections::HashMap;

use fake::Fake;
use fake::faker::name::en::*;
use fake::faker::internet::en::*;
use fake::faker::address::en::*;
use fake::faker::lorem::en::*;
use fake::faker::company::en::*;
use fake::faker::phone_number::en::*; // Used for CellNumber
use fake::faker::currency::en::*; 
use fake::faker::filesystem::en::*;   // Used for FilePath
use fake::faker::number::en::*;
use fake::faker::color::en::*;
use chrono::{Utc, Duration, Datelike};
use uuid::Uuid;

pub struct Generator {
    tables: Vec<Table>,
    primary_keys: HashMap<String, Vec<String>>, 
}

impl Generator {
    pub fn new(tables: Vec<Table>) -> Self {
        Self { 
            tables,
            primary_keys: HashMap::new(),
        }
    }

    pub fn generate_sql_dump(&mut self, output_file: &str, row_count: usize) -> Result<()> {
        let mut file = File::create(output_file)?;
        
        writeln!(file, "-- SynthDB Generated Dump (Bugfix v1.7)")?;
        writeln!(file, "BEGIN;")?;
        writeln!(file, "SET CONSTRAINTS ALL DEFERRED;")?; 

        let tables_list = self.tables.clone();

        for table in &tables_list {
            println!("Generating data for table: {}", table.table_name);
            
            writeln!(file, "\n-- Data for {}", table.table_name)?;
            writeln!(file, "INSERT INTO {} ({}) VALUES", 
                table.table_name,
                table.columns.iter().map(|c| c.name.as_str()).collect::<Vec<_>>().join(", ")
            )?;

            let mut table_ids = Vec::new();

            for i in 0..row_count {
                let mut row_values = Vec::new();
                let mut row_context: HashMap<String, String> = HashMap::new();
                
                // SEQUENCE COUNTER: Used for Integer PKs
                let current_seq_id = (i + 1).to_string();

                for col in &table.columns {
                    let fk = table.foreign_keys.iter().find(|f| f.column == col.name);
                    
                    let value = if let Some(foreign_key) = fk {
                        self.get_random_id(&foreign_key.ref_table)
                    } else {
                        self.generate_value(col, &row_context, &current_seq_id)
                    };

                    let clean_val = value.trim_matches('\'').to_string();
                    let name = col.name.to_lowercase();
                    
                    // Context Capture
                    if name.contains("user") || name.contains("login") {
                        row_context.insert("username".to_string(), clean_val.clone());
                    }
                    
                    if (name.contains("name") || name.contains("full")) && !name.contains("company") && !name.contains("user") {
                        if !name.contains("_name") || name.contains("first") || name.contains("last") {
                             row_context.insert("person_name".to_string(), clean_val.clone());
                        }
                    }
                    
                    if (name.contains("company") || name.contains("business") || name.ends_with("_name")) && !row_context.contains_key("person_name") {
                        row_context.insert("entity_name".to_string(), clean_val.clone());
                        // Also map generic org
                        if name.contains("company") {
                            row_context.insert("org_name".to_string(), clean_val.clone());
                        }
                    }

                    // ID Storage
                    if col.name == "id" || col.name == "user_id" || col.name.ends_with("_id") {
                        if value != "NULL" {
                            table_ids.push(clean_val.clone());
                        }
                    }

                    row_values.push(value);
                }

                let separator = if i == row_count - 1 { ";" } else { "," };
                writeln!(file, "({}){}", row_values.join(", "), separator)?;
            }

            self.primary_keys.insert(table.table_name.clone(), table_ids);
        }
        
        writeln!(file, "COMMIT;")?;
        Ok(())
    }

    fn get_random_id(&self, table_name: &str) -> String {
        if let Some(ids) = self.primary_keys.get(table_name) {
            if !ids.is_empty() {
                let mut rng = rand::thread_rng();
                let id = ids.choose(&mut rng).unwrap();
                if id.chars().all(|c| c.is_numeric()) {
                    return id.clone();
                } else {
                    return format!("'{}'", id);
                }
            }
        }
        "NULL".to_string() 
    }

    // Dynamic Entity Namer
    fn dynamic_entity_name(&self, col_name: &str) -> String {
        let mut rng = rand::thread_rng();
        let raw = col_name.replace("_name", "").replace("name", "").replace("_id", "").replace("id", "");
        let suffix: String = raw.split('_')
            .map(|s| {
                let mut c = s.chars();
                match c.next() {
                    None => String::new(),
                    Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
                }
            })
            .collect::<Vec<String>>()
            .join(" ");

        if rng.gen_bool(0.6) {
            format!("{} {}", CityName().fake::<String>(), suffix.trim())
        } else {
            format!("{} {}", LastName().fake::<String>(), suffix.trim())
        }
    }

    fn generate_value(&self, col: &Column, context: &HashMap<String, String>, seq_id: &str) -> String {
        let mut rng = rand::thread_rng();
        let name = col.name.to_lowercase();

        // --- 0. PRIMARY KEY SEQUENCE ---
        if (name == "id" || name == "user_id" || name.ends_with("_id")) 
           && (col.data_type.contains("int") || col.data_type.contains("serial")) {
            return seq_id.to_string();
        }

        // --- 1. SAMPLED DATA ---
        if !col.distinct_values.is_empty() {
            let val = col.distinct_values.choose(&mut rng).unwrap();
            return format!("'{}'", val.replace("'", "''"));
        }

        // =========================================================
        //  LINGUISTIC HEURISTIC ENGINE
        // =========================================================

        let kw_email = ["email", "mail", "sender"];
        let kw_phone = ["phone", "mobile", "fax", "cell"];
        let kw_internet = ["url", "web", "site", "link", "uri", "domain"];
        let kw_file = ["file", "image", "photo", "avatar", "path", "logo"];
        let kw_video = ["video", "movie", "clip", "footage"];
        let kw_audio = ["audio", "sound", "voice", "mp3"];
        let kw_doc = ["pdf", "doc", "invoice", "report", "contract"]; 
        let kw_address = ["address", "street", "residence", "hq"];
        
        let kw_company = ["company", "business", "corp", "supplier", "agency"];
        let kw_education = ["university", "college", "school", "institution", "campus"];
        let kw_person = ["passenger", "customer", "employee", "user", "citizen", "officer", "doctor", "pilot", "crew", "staff"];
        let kw_content = ["desc", "bio", "comment", "review", "body", "content", "note", "abstract", "summary", "message"];
        let kw_title = ["title", "subject", "label", "headline", "slug"];
        let kw_code = ["sku", "code", "isbn", "uuid", "token", "key", "tracking", "serial", "vin", "ssn", "badge", "license"];
        let kw_color = ["color", "hex", "rgb"];
        let kw_status = ["status", "state", "type", "mode", "phase", "category"];

        // 1. PEOPLE
        if kw_person.iter().any(|k| name.contains(k)) {
             return format!("'{}'", Name().fake::<String>().replace("'", "''"));
        }
        if name.contains("first") { return format!("'{}'", FirstName().fake::<String>().replace("'", "''")); }
        if name.contains("last") { return format!("'{}'", LastName().fake::<String>().replace("'", "''")); }
        if (name == "name" || name == "full_name") && !name.contains("company") {
             return format!("'{}'", Name().fake::<String>().replace("'", "''"));
        }

        // 2. EDUCATION
        if kw_education.iter().any(|k| name.contains(k)) && !name.contains("email") {
            let suffix = ["University", "College", "Institute", "Academy"].choose(&mut rng).unwrap();
            let prefix = LastName().fake::<String>();
            return format!("'{} {}'", prefix, suffix);
        }

        // 3. COMMUNICATIONS
        if kw_email.iter().any(|k| name.contains(k)) {
            let base = if let Some(u) = context.get("username") { u.clone() }
                       else if let Some(n) = context.get("person_name") { n.replace(" ", ".").to_lowercase() }
                       else if let Some(_) = context.get("entity_name") { "contact".to_string() }
                       else if let Some(_) = context.get("org_name") { "info".to_string() }
                       else { "".to_string() };

            let domain = if let Some(o) = context.get("entity_name") { 
                let clean = o.replace(" ", "").replace(",", "").to_lowercase();
                if o.contains("University") { format!("{}.edu", clean) }
                else { format!("{}.com", clean) }
            } else { 
                "example.com".to_string() 
            };

            if !base.is_empty() { return format!("'{}@{}'", base, domain); }
            return format!("'{}'", SafeEmail().fake::<String>());
        }

        if kw_phone.iter().any(|k| name.contains(k)) {
            return format!("'+1-{}-{}-{}'", 
                NumberWithFormat("###").fake::<String>(), 
                NumberWithFormat("###").fake::<String>(), 
                NumberWithFormat("####").fake::<String>()
            );
        }

        // 4. INTERNET & MEDIA & NETWORK
        if kw_internet.iter().any(|k| name.contains(k)) {
            if let Some(o) = context.get("entity_name") {
                return format!("'https://www.{}.com'", o.replace(" ", "").replace(",", "").to_lowercase());
            }
            return format!("'https://www.{}.{}'", Word().fake::<String>(), DomainSuffix().fake::<String>());
        }
        
        if kw_video.iter().any(|k| name.contains(k)) {
            let ext = ["mp4", "mov", "avi"].choose(&mut rng).unwrap();
            return format!("'/uploads/video/{}.{}'", Uuid::new_v4(), ext);
        }
        if kw_audio.iter().any(|k| name.contains(k)) {
            let ext = ["mp3", "wav"].choose(&mut rng).unwrap();
            return format!("'/uploads/audio/{}.{}'", Uuid::new_v4(), ext);
        }
        if kw_doc.iter().any(|k| name.contains(k)) {
            let ext = ["pdf", "docx"].choose(&mut rng).unwrap();
            return format!("'/uploads/docs/{}.{}'", Uuid::new_v4(), ext);
        }
        if kw_file.iter().any(|k| name.contains(k)) {
            let ext = ["jpg", "png", "webp"].choose(&mut rng).unwrap();
            return format!("'/uploads/img/{}.{}'", Uuid::new_v4(), ext);
        }

        // --- NETWORK FIX ---
        // Explicitly handle IP and MAC before the generic "Address" block catches them
        if name.contains("ip") && (name.contains("addr") || name == "ip" || name.contains("v4")) {
             return format!("'192.168.{}.{}'", rng.gen_range(0..255), rng.gen_range(0..255));
        }
        if (name.contains("mac") && name.contains("addr")) || name.contains("hardware") {
             return format!("'00:0a:95:9d:68:{}'", NumberWithFormat("##").fake::<String>());
        }
        if name.contains("subnet") || name.contains("cidr") {
             return format!("'10.0.{}.0/24'", rng.gen_range(0..255));
        }

        // 5. ADDRESSES (With Exclusion)
        // Ignore if it's a MAC address or IP address that slipped through
        if kw_address.iter().any(|k| name.contains(k)) && !name.contains("mac") && !name.contains("ip") && !name.contains("email") {
            let addr = format!("{} {}, {}, {} {}", 
                NumberWithFormat("####").fake::<String>(), 
                StreetName().fake::<String>(), 
                CityName().fake::<String>(), 
                StateAbbr().fake::<String>(), 
                ZipCode().fake::<String>()
            );
            return format!("'{}'", addr.replace("'", "''"));
        }
        if name.contains("zip") || name.contains("postal") { return format!("'{}'", ZipCode().fake::<String>()); }
        if name.contains("city") { return format!("'{}'", CityName().fake::<String>().replace("'", "''")); }
        if name.contains("country") { return format!("'{}'", CountryName().fake::<String>().replace("'", "''")); }
        if name.contains("region") { 
            let r = ["North America", "Europe", "Asia Pacific", "Latin America"].choose(&mut rng).unwrap();
            return format!("'{}'", r);
        }

        // 6. COMMERCE
        if kw_company.iter().any(|k| name.contains(k)) { 
            return format!("'{}'", CompanyName().fake::<String>().replace("'", "''")); 
        }
        if name.contains("currency") { return format!("'{}'", CurrencyCode().fake::<String>()); }
        
        // 7. CODES
        if kw_code.iter().any(|k| name.contains(k)) {
            return format!("'{}-{}'", Word().fake::<String>().to_uppercase().chars().take(3).collect::<String>(), NumberWithFormat("####").fake::<String>());
        }

        // 8. MISC
        if kw_color.iter().any(|k| name.contains(k)) { return format!("'{}'", HexColor().fake::<String>()); }
        if kw_status.iter().any(|k| name.contains(k)) {
            let st = ["Active", "Inactive", "Pending", "Completed", "Verified"];
            return format!("'{}'", st.choose(&mut rng).unwrap());
        }

        // 9. DATES
        if name.contains("created") || name.contains("updated") || name.contains("at") {
            let days_ago = rng.gen_range(0..365);
            let date = Utc::now() - Duration::days(days_ago);
            return format!("'{}'", date.format("%Y-%m-%d %H:%M:%S"));
        }
        if name.contains("date") || name.contains("dob") {
            let years_ago = rng.gen_range(10..80);
            let date = Utc::now() - Duration::days(years_ago * 365);
            return format!("'{}'", date.format("%Y-%m-%d"));
        }

        // =========================================================
        //  DATA TYPE FALLBACKS
        // =========================================================

        match col.data_type.as_str() {
            "integer" | "bigint" | "smallint" => {
                 if name.contains("year") { return rng.gen_range(1990..2025).to_string(); }
                 rng.gen_range(1..1000).to_string()
            },
            "numeric" | "decimal" | "money" | "real" | "double precision" => {
                if let Some(scale) = col.numeric_scale {
                    if scale > 0 {
                        let precision = col.numeric_precision.unwrap_or(5);
                        let max_power = (precision - scale) as u32;
                        let max = 10_i32.pow(max_power.min(9)) - 1;
                        let whole = rng.gen_range(0..=max);
                        let decimal = rng.gen_range(0..100);
                        return format!("{}.{:02}", whole, decimal);
                    }
                }
                format!("{}.99", rng.gen_range(10..1000))
            },
            "boolean" => rng.gen_bool(0.5).to_string(),
            "text" | "character varying" | "varchar" => {
                if kw_content.iter().any(|k| name.contains(k)) {
                     format!("'{}'", Sentence(5..15).fake::<String>().replace("'", "''"))
                } else if kw_title.iter().any(|k| name.contains(k)) {
                     format!("'{}'", Sentence(2..6).fake::<String>().replace("'", "''").trim_end_matches('.'))
                } else {
                     if name.ends_with("name") {
                         format!("'{}'", self.dynamic_entity_name(&col.name).replace("'", "''"))
                     } else {
                         format!("'{}'", Word().fake::<String>())
                     }
                }
            },
            "json" | "jsonb" => format!("'{{\"generated\": true, \"type\": \"{}\"}}'", Word().fake::<String>()),
            "uuid" => format!("'{}'", Uuid::new_v4()),
            "ARRAY" => format!("'{{ \"{}\", \"{}\" }}'", Word().fake::<String>(), Word().fake::<String>()),
            "inet" | "cidr" => format!("'192.168.0.{}'", rng.gen_range(1..255)),
            "macaddr" => format!("'00:0a:95:9d:68:{}'", NumberWithFormat("##").fake::<String>()),
            _ => "NULL".to_string()
        }
    }
}