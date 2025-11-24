use crate::schema::{Table, Column};
use rand::Rng;
use rand::seq::SliceRandom;
use anyhow::Result;
use std::fs::File;
use std::io::Write;
use std::collections::HashMap;

// Smart Seeding Imports
use fake::Fake;
use fake::faker::name::en::*;
use fake::faker::internet::en::*;
use fake::faker::address::en::*;
use fake::faker::lorem::en::*;
use fake::faker::company::en::*;
use fake::faker::phone_number::en::*;
use fake::faker::currency::en::*; 
use fake::faker::filesystem::en::*;
use fake::faker::number::en::*;
use chrono::{Utc, Duration};
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
        
        writeln!(file, "-- SynthDB Generated Dump")?;
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
                
                // CONTEXT MEMORY: Remembers values for this specific row
                let mut row_context: HashMap<String, String> = HashMap::new();

                for col in &table.columns {
                    // 1. Foreign Key Check
                    let fk = table.foreign_keys.iter().find(|f| f.column == col.name);
                    
                    let value = if let Some(foreign_key) = fk {
                        self.get_random_id(&foreign_key.ref_table)
                    } else {
                        // Pass the memory to the generator
                        self.generate_value(col, &row_context)
                    };

                    // Store value in context (strip single quotes for clean storage)
                    let clean_val = value.trim_matches('\'').to_string();
                    row_context.insert(col.name.to_lowercase(), clean_val.clone());
                    
                    // Also store generic keys for loose matching (e.g., "first_name" -> "name")
                    if col.name.contains("name") { row_context.insert("generic_name".to_string(), clean_val.clone()); }
                    if col.name.contains("company") { row_context.insert("generic_company".to_string(), clean_val.clone()); }

                    if col.name == "id" || col.name == "user_id" || col.name.ends_with("_id") {
                        if value != "NULL" {
                            table_ids.push(value.clone());
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
                return ids.choose(&mut rng).unwrap().clone();
            }
        }
        "NULL".to_string() 
    }

    fn generate_value(&self, col: &Column, context: &HashMap<String, String>) -> String {
        let mut rng = rand::thread_rng();
        let name = col.name.to_lowercase();

        // --- 0. SAMPLED DATA ---
        if !col.distinct_values.is_empty() {
            let val = col.distinct_values.choose(&mut rng).unwrap();
            return format!("'{}'", val.replace("'", "''"));
        }

        // --- 1. CONTEXT AWARENESS (The Fix) ---
        
        // Email Matching: If we generated a name/username/company, use it!
        if name.contains("email") {
            let base = if let Some(u) = context.get("username") { u.clone() }
                       else if let Some(n) = context.get("generic_name") { n.replace(" ", ".").to_lowercase() }
                       else if let Some(c) = context.get("generic_company") { "info".to_string() }
                       else { "".to_string() };

            let domain = if let Some(c) = context.get("generic_company") { 
                format!("{}.com", c.replace(" ", "").to_lowercase()) 
            } else { 
                "example.com".to_string() 
            };

            if !base.is_empty() {
                return format!("'{}@{}'", base, domain);
            }
            return format!("'{}'", SafeEmail().fake::<String>());
        }

        // Website Matching: Match company name
        if name.contains("url") || name.contains("website") {
            if let Some(c) = context.get("generic_company") {
                return format!("'https://www.{}.com'", c.replace(" ", "").replace(",", "").to_lowercase());
            }
            return format!("'https://{}'", DomainName().fake::<String>());
        }

        // --- 2. NUMERIC HANDLING ---
        if col.data_type == "numeric" || col.data_type == "decimal" {
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
        }

        // --- 3. HEURISTICS ---
        if name == "username" || name.contains("login") {
            return format!("'{}'", Username().fake::<String>().replace(" ", "").to_lowercase());
        }
        if name.contains("phone") || name.contains("mobile") {
            return format!("'{}'", CellNumber().fake::<String>());
        }
        if name == "city" || name.ends_with("_city") { 
            return format!("'{}'", CityName().fake::<String>().replace("'", "''")); 
        }
        if name.contains("ip_address") || name == "last_login_ip" {
             return format!("'192.168.{}.{}'", rng.gen_range(0..255), rng.gen_range(0..255));
        }
        if name.contains("sku") { return format!("'PROD-{}'", rng.gen_range(1000..9999)); }
        if name.contains("tracking") { return format!("'1Z{}'", Digit(10..10).fake::<String>()); }
        if name.contains("address") || name.contains("shipping") { return format!("'{}'", StreetName().fake::<String>().replace("'", "''")); }
        if name.contains("zip") || name.contains("postal") { return format!("'{}'", ZipCode().fake::<String>()); }
        if name.contains("country") { return format!("'{}'", CountryName().fake::<String>().replace("'", "''")); }
        if name.contains("company") { return format!("'{}'", CompanyName().fake::<String>().replace("'", "''")); }

        if name.contains("created") || name.contains("updated") || name.contains("at") {
            let days_ago = rng.gen_range(0..730);
            let date = Utc::now() - Duration::days(days_ago);
            return format!("'{}'", date.format("%Y-%m-%d %H:%M:%S"));
        }

        // --- 4. DATA TYPE FALLBACKS (The "Any Format" Fix) ---

        match col.data_type.as_str() {
            "integer" | "bigint" | "smallint" => {
                 if name.contains("capacity") { return rng.gen_range(1000..50000).to_string(); }
                 if name.contains("price") || name.contains("amount") { return rng.gen_range(10..10000).to_string(); }
                 rng.gen_range(1..1000).to_string()
            },
            "boolean" => rng.gen_bool(0.5).to_string(),
            "text" | "character varying" | "varchar" => {
                if name.contains("name") {
                     format!("'{}'", Name().fake::<String>().replace("'", "''"))
                } else if name.contains("desc") || name.contains("bio") {
                     format!("'{}'", Sentence(5..15).fake::<String>().replace("'", "''"))
                } else {
                     // SMART FALLBACK: Look at length limit if possible, otherwise generic word
                     format!("'{}'", Word().fake::<String>())
                }
            },
            "json" | "jsonb" => format!("'{{\"generated\": true, \"tag\": \"{}\"}}'", Word().fake::<String>()),
            "uuid" => format!("'{}'", Uuid::new_v4()),
            "ARRAY" => format!("'{{ \"{}\", \"{}\" }}'", Word().fake::<String>(), Word().fake::<String>()),
            "inet" => format!("'10.0.0.{}'", rng.gen_range(1..255)),
            _ => "NULL".to_string()
        }
    }
}