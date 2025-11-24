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
use fake::faker::currency::en::*; // FIXED: Added this
use fake::faker::filesystem::en::*;
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
        writeln!(file, "-- Order optimized by Topological Sort")?;
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
                
                for col in &table.columns {
                    // 1. Foreign Key Check
                    let fk = table.foreign_keys.iter().find(|f| f.column == col.name);
                    
                    let value = if let Some(foreign_key) = fk {
                        self.get_random_id(&foreign_key.ref_table)
                    } else {
                        self.generate_value(col)
                    };

                    if col.name == "id" {
                        table_ids.push(value.clone());
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

    fn generate_value(&self, col: &Column) -> String {
        let mut rng = rand::thread_rng();
        let name = col.name.to_lowercase();

        // --- 0. SAMPLED DATA (The "Reader" Feature) ---
        if !col.distinct_values.is_empty() {
            let val = col.distinct_values.choose(&mut rng).unwrap();
            return format!("'{}'", val.replace("'", "''"));
        }

        // --- 1. HEURISTICS ---
        if name == "username" || name.contains("login") {
            return format!("'{}'", Username().fake::<String>().replace(" ", "").to_lowercase());
        }
        if name.contains("email") {
            return format!("'{}'", SafeEmail().fake::<String>());
        }
        if name.contains("phone") || name.contains("mobile") {
            return format!("'{}'", CellNumber().fake::<String>());
        }
        if name.contains("url") || name.contains("website") {
            return format!("'{}'", DomainSuffix().fake::<String>());
        }

        // Names
        if name == "name" || name.contains("full_name") {
             return format!("'{}'", Name().fake::<String>().replace("'", "''"));
        }
        if name.contains("first_name") {
             return format!("'{}'", FirstName().fake::<String>().replace("'", "''"));
        }
        if name.contains("last_name") {
             return format!("'{}'", LastName().fake::<String>().replace("'", "''"));
        }

        // Location
        if name.contains("city") { return format!("'{}'", CityName().fake::<String>().replace("'", "''")); }
        if name.contains("country") { return format!("'{}'", CountryName().fake::<String>().replace("'", "''")); }
        if name.contains("zip") || name.contains("postal") { return format!("'{}'", ZipCode().fake::<String>()); }
        if name.contains("address") { return format!("'{}'", StreetName().fake::<String>().replace("'", "''")); }

        // Business
        if name.contains("company") { return format!("'{}'", CompanyName().fake::<String>().replace("'", "''")); }
        // FIXED: Used CurrencyCode() correctly
        if name.contains("currency") { return format!("'{}'", CurrencyCode().fake::<String>()); }
        if name.contains("sku") { return format!("'PROD-{}'", rng.gen_range(1000..9999)); }

        // Status/Enums
        if name.contains("status") || name.contains("state") {
            let statuses = ["active", "inactive", "pending", "archived"];
            return format!("'{}'", statuses.choose(&mut rng).unwrap());
        }
        
        // File Paths
        if name.contains("file") || name.contains("image") || name.contains("path") {
            return format!("'{}'", FilePath().fake::<String>());
        }

        // Dates
        if name.contains("created") || name.contains("updated") || name.contains("date") {
            let days_ago = rng.gen_range(0..730);
            let date = Utc::now() - Duration::days(days_ago);
            return format!("'{}'", date.format("%Y-%m-%d %H:%M:%S"));
        }

        // --- 2. TYPE FALLBACKS ---
        match col.data_type.as_str() {
            "integer" | "bigint" | "smallint" | "numeric" => {
                 if name.contains("price") || name.contains("amount") {
                     return rng.gen_range(10..10000).to_string();
                 }
                 rng.gen_range(1..1000).to_string()
            },
            "boolean" => rng.gen_bool(0.5).to_string(),
            "text" | "character varying" | "varchar" => {
                if name.contains("desc") || name.contains("bio") || name.contains("content") {
                     format!("'{}'", Sentence(5..15).fake::<String>().replace("'", "''"))
                } else {
                     format!("'{}'", Word().fake::<String>())
                }
            },
            "timestamp without time zone" | "timestamp with time zone" => {
                 let days_ago = rng.gen_range(0..365);
                 let date = Utc::now() - Duration::days(days_ago);
                 format!("'{}'", date.format("%Y-%m-%d %H:%M:%S"))
            },
            "date" => {
                 let days_ago = rng.gen_range(0..365);
                 let date = Utc::now() - Duration::days(days_ago);
                 format!("'{}'", date.format("%Y-%m-%d"))
            },
            "json" | "jsonb" => {
                 "'{\"generated\": true, \"value\": \"random\"}'".to_string()
            },
            "uuid" => {
                 format!("'{}'", Uuid::new_v4())
            }
            _ => "NULL".to_string()
        }
    }
}