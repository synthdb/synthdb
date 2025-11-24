use crate::schema::{Table, Column};
use rand::Rng;
use rand::seq::SliceRandom;
use anyhow::Result;
use std::fs::File;
use std::io::{Write, BufWriter};
use std::collections::{HashMap, HashSet, VecDeque};

use fake::Fake;
use fake::faker::name::en::*;
use fake::faker::address::en::*;
use fake::faker::company::en::*;
use chrono::{Utc, Duration, NaiveDate};
use uuid::Uuid;

// ====================================================================================
// DEEP LEARNING SEMANTIC ENGINE
// ====================================================================================

#[derive(Debug, Clone, PartialEq)]
enum SemanticType {
    // Identity
    PrimaryKey,
    ForeignKey(String),
    UUID,
    
    // Personal Identity
    FirstName,
    LastName,
    FullName,
    Username,
    Email,
    Gender,
    Age,
    BirthDate,
    
    // Organization
    CompanyName,
    MerchantName,
    OrganizationName,
    
    // Location (Geographic)
    Country,
    State,
    City,
    StreetAddress,
    PostalCode,
    Latitude,
    Longitude,
    GalacticCoordinate,
    
    // Contact
    PhoneNumber,
    MobileNumber,
    
    // Web/Network
    DomainName,
    URL,
    EmailAddress,
    IPv4Address,
    IPv6Address,
    MacAddress,
    NetworkPort,
    
    // Temporal
    DateStart,
    DateEnd,
    DateCreated,
    DateUpdated,
    DateRegistered,
    DateLaunched,
    DateSigned,
    DateEstablished,
    Timestamp,
    
    // Financial
    MoneyAmount,
    CurrencyCode,
    CreditValue,
    PriceValue,
    BalanceValue,
    
    // Cryptographic
    WalletAddress,
    HashValue,
    EncryptionKey,
    TokenValue,
    
    // Classification/Status
    StatusActive,
    OperationalStatus,
    TypeCategory,
    ClassLevel,
    SkillLevel,
    SecurityLevel,
    ClearanceLevel,
    RankTitle,
    PriorityLevel,
    
    // Identifiers/Codes
    TrackingCode,
    SerialNumber,
    BadgeID,
    SKUCode,
    BarcodeValue,
    ReferenceID,
    
    // Sci-Fi/Gaming Domain
    SectorName,
    OutpostName,
    PlanetName,
    StationName,
    SpecimenName,
    CharacterName,
    GuildName,
    ItemName,
    JurisdictionZone,
    HazardClassification,
    
    // Content/Text
    TitleText,
    DescriptionText,
    BodyContent,
    CommentText,
    SummaryText,
    NotesText,
    
    // File/Path
    FilePath,
    FileName,
    DirectoryPath,
    
    // Measurements
    WeightMetric,
    TemperatureCelsius,
    FrequencyHz,
    DurationSeconds,
    DurationHours,
    ByteSize,
    
    // Technical
    FirmwareVersion,
    SoftwareVersion,
    
    // Generic Types
    BooleanValue,
    IntegerValue,
    DecimalValue,
    TextValue,
    JSONValue,
}

struct DeepAnalyzer;

impl DeepAnalyzer {
    fn analyze_field_intelligence(
        field_name: &str,
        data_type: &str,
        is_foreign_key: bool,
        foreign_table: Option<&str>,
        sample_values: &[String],
        table_name: &str,
    ) -> SemanticType {
        let field_lower = field_name.to_lowercase();
        let type_lower = data_type.to_lowercase();
        let table_lower = table_name.to_lowercase();
        
        // === FOREIGN KEY DETECTION ===
        if is_foreign_key {
            return SemanticType::ForeignKey(foreign_table.unwrap_or("unknown").to_string());
        }
        
        // === PRIMARY KEY DETECTION ===
        if Self::is_primary_key(&field_lower, &table_lower) {
            return SemanticType::PrimaryKey;
        }
        
        // === LEARN FROM SAMPLE DATA (if available) ===
        if !sample_values.is_empty() {
            if let Some(inferred_type) = Self::infer_from_samples(&field_lower, sample_values) {
                return inferred_type;
            }
        }
        
        // === DATA TYPE BASED DETECTION ===
        if type_lower.contains("uuid") {
            return SemanticType::UUID;
        }
        if type_lower.contains("bool") {
            return SemanticType::BooleanValue;
        }
        
        // === DEEP SEMANTIC ANALYSIS ===
        Self::deep_semantic_inference(&field_lower, &type_lower, &table_lower)
    }
    
    fn is_primary_key(field: &str, _table: &str) -> bool {
        field == "id" || 
        field.ends_with("_id") && !field.contains("user") && !field.contains("account") && !field.contains("badge") && !field.contains("ref")
    }
    
    fn infer_from_samples(field: &str, samples: &[String]) -> Option<SemanticType> {
        let sample = samples[0].to_lowercase();
        
        // MAC Address pattern (xx:xx:xx:xx:xx:xx)
        if sample.matches(':').count() == 5 && sample.len() == 17 {
            return Some(SemanticType::MacAddress);
        }
        
        // IPv4 pattern (xxx.xxx.xxx.xxx)
        if sample.matches('.').count() == 3 && sample.split('.').all(|p| p.parse::<u8>().is_ok()) {
            return Some(SemanticType::IPv4Address);
        }
        
        // Email pattern
        if sample.contains('@') && sample.contains('.') {
            return Some(SemanticType::EmailAddress);
        }
        
        // Status keywords
        if ["active", "inactive", "pending", "completed", "cancelled", "processing"].contains(&sample.as_str()) {
            return Some(SemanticType::StatusActive);
        }
        
        // Skill level keywords
        if ["beginner", "intermediate", "advanced", "expert", "master"].contains(&sample.as_str()) {
            return Some(SemanticType::SkillLevel);
        }
        
        // Priority keywords
        if ["low", "medium", "high", "critical", "urgent"].contains(&sample.as_str()) {
            return Some(SemanticType::PriorityLevel);
        }
        
        None
    }
    
    fn deep_semantic_inference(field: &str, dtype: &str, _table: &str) -> SemanticType {
        // === NAMES & IDENTITY ===
        if field.contains("first") && field.contains("name") { return SemanticType::FirstName; }
        if field.contains("last") && field.contains("name") { return SemanticType::LastName; }
        if (field == "full_name" || field == "name") && !field.contains("user") && !field.contains("file") && !field.contains("domain") && !field.contains("host") {
            return SemanticType::FullName;
        }
        if field.contains("username") || field == "login" { return SemanticType::Username; }
        if field.contains("email") || field == "mail" { return SemanticType::EmailAddress; }
        if field.contains("gender") || field == "sex" { return SemanticType::Gender; }
        if field.contains("age") && dtype.contains("int") { return SemanticType::Age; }
        
        // === DATES & TIMES ===
        if field.contains("birth") || field == "dob" { return SemanticType::BirthDate; }
        if field.contains("expir") || field.contains("deadline") || (field.contains("end") && field.contains("date")) {
            return SemanticType::DateEnd;
        }
        if field.contains("signed") { return SemanticType::DateSigned; }
        if field.contains("established") { return SemanticType::DateEstablished; }
        if field.contains("launch") { return SemanticType::DateLaunched; }
        if field.contains("created") { return SemanticType::DateCreated; }
        if field.contains("updated") || field.contains("modified") { return SemanticType::DateUpdated; }
        if field.contains("registered") { return SemanticType::DateRegistered; }
        if field.contains("start") { return SemanticType::DateStart; }
        if dtype.contains("timestamp") { return SemanticType::Timestamp; }
        
        // === ORGANIZATION ===
        if (field.contains("company") || field.contains("mining")) && field.contains("name") {
            return SemanticType::CompanyName;
        }
        if field.contains("merchant") && field.contains("name") { return SemanticType::MerchantName; }
        if field.contains("organization") || field.contains("organisation") {
            return SemanticType::OrganizationName;
        }
        
        // === LOCATION (EARTH) ===
        if field.contains("country") { return SemanticType::Country; }
        if field.contains("state") && !field.contains("status") { return SemanticType::State; }
        if field.contains("city") { return SemanticType::City; }
        if field.contains("address") && !field.contains("email") && !field.contains("ip") && !field.contains("wallet") && !field.contains("mac") {
            return SemanticType::StreetAddress;
        }
        if field.contains("postal") || field.contains("zip") { return SemanticType::PostalCode; }
        
        // === COORDINATES ===
        if field.contains("lat") && !field.contains("relation") {
            return SemanticType::Latitude;
        }
        if field.contains("long") || field == "lng" || field == "lon" {
            return SemanticType::Longitude;
        }
        if field.contains("galactic") && field.contains("coordinate") {
            return SemanticType::GalacticCoordinate;
        }
        
        // === CONTACT ===
        if field.contains("phone") || field.contains("telephone") || field.contains("tel") {
            return SemanticType::PhoneNumber;
        }
        if field.contains("mobile") || field.contains("cell") {
            return SemanticType::MobileNumber;
        }
        
        // === WEB & NETWORK ===
        if field.contains("domain") && !field.contains("_id") {
            return SemanticType::DomainName;
        }
        if field.contains("url") || field.contains("website") {
            return SemanticType::URL;
        }
        if dtype.contains("inet") || field.contains("ipv4") || (field.contains("ip") && field.contains("address") && !field.contains("ipv6")) {
            return SemanticType::IPv4Address;
        }
        if field.contains("ipv6") {
            return SemanticType::IPv6Address;
        }
        if dtype.contains("macaddr") || field.contains("mac_address") || field == "mac" {
            return SemanticType::MacAddress;
        }
        if field.contains("port") && dtype.contains("int") {
            return SemanticType::NetworkPort;
        }
        
        // === FINANCIAL ===
        if field.contains("price") || field.contains("cost") {
            return SemanticType::PriceValue;
        }
        if field.contains("balance") {
            return SemanticType::BalanceValue;
        }
        if field.contains("value") && field.contains("credit") {
            return SemanticType::CreditValue;
        }
        if field.contains("currency") {
            return SemanticType::CurrencyCode;
        }
        if dtype.contains("money") || field.contains("amount") {
            return SemanticType::MoneyAmount;
        }
        
        // === CRYPTOGRAPHIC ===
        if field.contains("wallet") {
            return SemanticType::WalletAddress;
        }
        if field.contains("hash") {
            return SemanticType::HashValue;
        }
        if field.contains("encryption") && field.contains("key") {
            return SemanticType::EncryptionKey;
        }
        if field.contains("token") {
            return SemanticType::TokenValue;
        }
        
        // === STATUS & CLASSIFICATION ===
        if field.contains("status") && !field.contains("_id") {
            return SemanticType::StatusActive;
        }
        if field.contains("operational") {
            return SemanticType::OperationalStatus;
        }
        if field.contains("type") || field.contains("category") {
            return SemanticType::TypeCategory;
        }
        if field.contains("class") && !field.contains("_id") {
            return SemanticType::ClassLevel;
        }
        if field.contains("level") && field.contains("security") {
            return SemanticType::SecurityLevel;
        }
        if field.contains("level") && field.contains("clearance") {
            return SemanticType::ClearanceLevel;
        }
        if field.contains("level") && !field.contains("_id") {
            return SemanticType::SkillLevel;
        }
        if field.contains("rank") || field.contains("title") {
            return SemanticType::RankTitle;
        }
        if field.contains("priority") {
            return SemanticType::PriorityLevel;
        }
        
        // === CODES & IDENTIFIERS ===
        if field.contains("tracking") && field.contains("code") {
            return SemanticType::TrackingCode;
        }
        if field.contains("serial") || field.contains("unit_serial") {
            return SemanticType::SerialNumber;
        }
        if field.contains("badge") && field.contains("id") {
            return SemanticType::BadgeID;
        }
        if field.contains("sku") {
            return SemanticType::SKUCode;
        }
        if field.contains("code") || field.contains("ref") && !field.contains("_id") {
            return SemanticType::ReferenceID;
        }
        
        // === SCI-FI/GAMING DOMAIN ===
        if field.contains("sector") && field.contains("name") {
            return SemanticType::SectorName;
        }
        if field.contains("outpost") && field.contains("name") {
            return SemanticType::OutpostName;
        }
        if field.contains("planet") && field.contains("name") {
            return SemanticType::PlanetName;
        }
        if field.contains("station") && field.contains("name") {
            return SemanticType::StationName;
        }
        if field.contains("specimen") && field.contains("name") {
            return SemanticType::SpecimenName;
        }
        if field.contains("character") && field.contains("name") {
            return SemanticType::CharacterName;
        }
        if field.contains("guild") && field.contains("name") {
            return SemanticType::GuildName;
        }
        if field.contains("item") && field.contains("name") {
            return SemanticType::ItemName;
        }
        if field.contains("jurisdiction") || (field.contains("zone") && field.contains("legal")) {
            return SemanticType::JurisdictionZone;
        }
        if field.contains("hazard") && field.contains("class") {
            return SemanticType::HazardClassification;
        }
        
        // === CONTENT ===
        if field.contains("title") || field.contains("subject") {
            return SemanticType::TitleText;
        }
        if field.contains("description") || field.contains("summary") {
            return SemanticType::DescriptionText;
        }
        if field.contains("body") || field.contains("content") {
            return SemanticType::BodyContent;
        }
        if field.contains("comment") || field.contains("note") {
            return SemanticType::CommentText;
        }
        
        // === FILE & PATH ===
        if field.contains("path") || field.contains("file") {
            return SemanticType::FilePath;
        }
        
        // === MEASUREMENTS ===
        if field.contains("weight") || field.contains("ton") {
            return SemanticType::WeightMetric;
        }
        if field.contains("temperature") || field.contains("celsius") {
            return SemanticType::TemperatureCelsius;
        }
        if field.contains("frequency") || field.contains("hz") {
            return SemanticType::FrequencyHz;
        }
        if field.contains("duration") && field.contains("hour") {
            return SemanticType::DurationHours;
        }
        if field.contains("duration") || field.contains("seconds") {
            return SemanticType::DurationSeconds;
        }
        if field.contains("size") && field.contains("byte") {
            return SemanticType::ByteSize;
        }
        
        // === VERSION ===
        if field.contains("firmware") || field.contains("version") {
            return SemanticType::FirmwareVersion;
        }
        
        // === JSON ===
        if dtype.contains("json") {
            return SemanticType::JSONValue;
        }
        
        // === TYPE-BASED FALLBACK ===
        if dtype.contains("int") || dtype.contains("serial") {
            return SemanticType::IntegerValue;
        }
        if dtype.contains("numeric") || dtype.contains("decimal") || dtype.contains("float") || dtype.contains("real") || dtype.contains("double") {
            return SemanticType::DecimalValue;
        }
        
        SemanticType::TextValue
    }
    
    fn get_generation_priority(semantic_type: &SemanticType) -> u8 {
        match semantic_type {
            SemanticType::PrimaryKey => 100,
            SemanticType::FirstName | SemanticType::LastName | SemanticType::FullName => 95,
            SemanticType::Gender => 94,
            SemanticType::CompanyName | SemanticType::OrganizationName => 93,
            SemanticType::DateSigned | SemanticType::DateEstablished | SemanticType::DateCreated | SemanticType::DateStart => 90,
            SemanticType::DateEnd => 85,
            SemanticType::Username => 80,
            SemanticType::DomainName => 78,
            SemanticType::EmailAddress => 75,
            _ => 50,
        }
    }
}

// ====================================================================================
// CONTEXT ENGINE
// ====================================================================================

#[derive(Debug, Clone, Default)]
struct ContextEngine {
    data: HashMap<String, String>,
    dates: HashMap<String, NaiveDate>,
}

impl ContextEngine {
    fn new() -> Self {
        Self::default()
    }
    
    fn set(&mut self, key: &str, value: &str) {
        if value != "NULL" && !value.is_empty() {
            self.data.insert(key.to_lowercase(), value.to_string());
        }
    }
    
    fn get(&self, key: &str) -> Option<&String> {
        self.data.get(&key.to_lowercase())
    }
    
    fn set_date(&mut self, key: &str, date: NaiveDate) {
        self.dates.insert(key.to_lowercase(), date);
    }
    
    fn get_any_start_date(&self) -> Option<NaiveDate> {
        for (key, date) in &self.dates {
            if key.contains("signed") || key.contains("created") || key.contains("established") || key.contains("start") || key.contains("launch") {
                return Some(*date);
            }
        }
        None
    }
}

// ====================================================================================
// TOPOLOGICAL SORTER
// ====================================================================================

struct TopologicalSorter;

impl TopologicalSorter {
    fn sort(tables: &[Table]) -> Vec<Table> {
        let mut in_degree: HashMap<String, usize> = HashMap::new();
        let mut adjacency: HashMap<String, Vec<String>> = HashMap::new();
        
        for table in tables {
            in_degree.insert(table.table_name.clone(), 0);
            adjacency.insert(table.table_name.clone(), Vec::new());
        }
        
        for table in tables {
            for fk in &table.foreign_keys {
                if fk.ref_table == table.table_name || !adjacency.contains_key(&fk.ref_table) {
                    continue;
                }
                adjacency.get_mut(&fk.ref_table).unwrap().push(table.table_name.clone());
                *in_degree.get_mut(&table.table_name).unwrap() += 1;
            }
        }
        
        let mut queue: VecDeque<String> = VecDeque::new();
        let mut sorted_names = Vec::new();
        
        for (name, &degree) in &in_degree {
            if degree == 0 {
                queue.push_back(name.clone());
            }
        }
        
        while let Some(current) = queue.pop_front() {
            sorted_names.push(current.clone());
            
            if let Some(neighbors) = adjacency.get(&current) {
                for neighbor in neighbors {
                    let degree = in_degree.get_mut(neighbor).unwrap();
                    *degree -= 1;
                    if *degree == 0 {
                        queue.push_back(neighbor.clone());
                    }
                }
            }
        }
        
        let sorted_set: HashSet<_> = sorted_names.iter().cloned().collect();
        for table in tables {
            if !sorted_set.contains(&table.table_name) {
                sorted_names.push(table.table_name.clone());
            }
        }
        
        let table_map: HashMap<_, _> = tables.iter()
            .map(|t| (t.table_name.clone(), t.clone()))
            .collect();
        
        sorted_names.iter()
            .filter_map(|name| table_map.get(name).cloned())
            .collect()
    }
}

// ====================================================================================
// AI GENERATOR
// ====================================================================================

pub struct Generator {
    tables: Vec<Table>,
    pk_storage: HashMap<String, Vec<String>>,
}

impl Generator {
    pub fn new(tables: Vec<Table>) -> Self {
        Self { 
            tables,
            pk_storage: HashMap::new(),
        }
    }

    pub fn generate_sql_dump(&mut self, output_file: &str, row_count: usize) -> Result<()> {
        let file = File::create(output_file)?;
        let mut writer = BufWriter::new(file);
        
        writeln!(writer, "-- SynthDB Deep Learning AI Generator v14.0")?;
        writeln!(writer, "-- Generated: {} (UTC)", Utc::now().format("%Y-%m-%d %H:%M:%S"))?;
        writeln!(writer, "-- Rows per table: {}", row_count)?;
        writeln!(writer, "-- AI: Deep Semantic Analysis, Pattern Recognition, Context Learning")?;
        writeln!(writer, "BEGIN;")?;
        writeln!(writer, "SET CONSTRAINTS ALL DEFERRED;\n")?;

        let sorted_tables = TopologicalSorter::sort(&self.tables);
        
        println!("🧠 Deep Learning Analysis Complete:");
        for (idx, table) in sorted_tables.iter().enumerate() {
            println!("   {}. {} ({} columns analyzed)", idx + 1, table.table_name, table.columns.len());
        }
        println!();

        for table in sorted_tables {
            println!("📊 Generating semantic data for: {}", table.table_name);
            
            writeln!(writer, "-- Data for {}", table.table_name)?;
            
            let col_names: Vec<String> = table.columns.iter().map(|c| c.name.clone()).collect();
            writeln!(writer, "INSERT INTO {} ({}) VALUES", table.table_name, col_names.join(", "))?;

            let mut table_pks = Vec::new();

            for row_idx in 0..row_count {
                let row_values = self.generate_intelligent_row(&table, row_idx);

                for (idx, col) in table.columns.iter().enumerate() {
                    let semantic_type = self.analyze_column(col, &table);
                    if semantic_type == SemanticType::PrimaryKey {
                        let pk = row_values[idx].trim_matches('\'').to_string();
                        table_pks.push(pk);
                        break;
                    }
                }

                let sep = if row_idx == row_count - 1 { ";" } else { "," };
                writeln!(writer, "({}){}", row_values.join(", "), sep)?;
            }

            self.pk_storage.insert(table.table_name.clone(), table_pks);
            writeln!(writer)?;
        }
        
        writeln!(writer, "COMMIT;")?;
        writer.flush()?;
        
        println!("\n✅ AI-Generated SQL: {}", output_file);
        println!("📊 Tables: {} | Total rows: {}", 
            self.pk_storage.len(), 
            self.pk_storage.values().map(|v| v.len()).sum::<usize>()
        );
        
        Ok(())
    }
    
    fn analyze_column(&self, col: &Column, table: &Table) -> SemanticType {
        let fk = table.foreign_keys.iter().find(|f| f.column == col.name);
        DeepAnalyzer::analyze_field_intelligence(
            &col.name,
            &col.data_type,
            fk.is_some(),
            fk.map(|f| f.ref_table.as_str()),
            &col.distinct_values,
            &table.table_name
        )
    }
    
    fn generate_intelligent_row(&self, table: &Table, row_idx: usize) -> Vec<String> {
        let mut context = ContextEngine::new();
        let mut temp_values: HashMap<String, String> = HashMap::new();
        
        let mut column_semantics: Vec<(Column, SemanticType)> = table.columns.iter()
            .map(|col| {
                let semantic = self.analyze_column(col, table);
                (col.clone(), semantic)
            })
            .collect();
        
        column_semantics.sort_by_key(|(_, sem)| std::cmp::Reverse(DeepAnalyzer::get_generation_priority(sem)));
        
        for (col, semantic) in &column_semantics {
            let value = self.generate_by_semantic(semantic, col, &context, row_idx);
            self.update_context(&col.name, &value, semantic, &mut context);
            temp_values.insert(col.name.clone(), value);
        }
        
        table.columns.iter()
            .map(|col| temp_values.get(&col.name).unwrap().clone())
            .collect()
    }
    
    fn generate_by_semantic(&self, semantic: &SemanticType, col: &Column, ctx: &ContextEngine, row_idx: usize) -> String {
        let mut rng = rand::thread_rng();
        
        match semantic {
            SemanticType::ForeignKey(ref_table) => {
                let value = self.get_fk_value(ref_table, &col.data_type);
                if value == "NULL" {
                    return self.generate_default(&col.data_type, row_idx);
                }
                value
            },
            
            SemanticType::PrimaryKey => {
                if col.data_type.contains("uuid") {
                    format!("'{}'", Uuid::new_v4())
                } else {
                    (row_idx + 1).to_string()
                }
            },
            
            SemanticType::UUID => format!("'{}'", Uuid::new_v4()),
            SemanticType::BooleanValue => rng.gen_bool(0.75).to_string(),
            
            SemanticType::FirstName => format!("'{}'", FirstName().fake::<String>().replace("'", "''")),
            SemanticType::LastName => format!("'{}'", LastName().fake::<String>().replace("'", "''")),
            SemanticType::FullName => {
                if let (Some(f), Some(l)) = (ctx.get("first_name"), ctx.get("last_name")) {
                    format!("'{} {}'", f, l)
                } else {
                    format!("'{}'", Name().fake::<String>().replace("'", "''"))
                }
            },
            
            SemanticType::Username => {
                if let (Some(f), Some(l)) = (ctx.get("first_name"), ctx.get("last_name")) {
                    format!("'{}.{}'", f.to_lowercase(), l.to_lowercase())
                } else {
                    format!("'user{}'", row_idx + 100000)
                }
            },
            
            SemanticType::EmailAddress => {
                let local = if let (Some(f), Some(l)) = (ctx.get("first_name"), ctx.get("last_name")) {
                    format!("{}.{}", f.to_lowercase(), l.to_lowercase())
                } else {
                    format!("user{}", row_idx + 100000)
                };
                let providers = ["gmail.com", "yahoo.com", "outlook.com", "hotmail.com", "icloud.com"];
                format!("'{}@{}'", local, providers.choose(&mut rng).unwrap())
            },
            
            SemanticType::Gender => {
                let genders = ["male", "female", "other"];
                format!("'{}'", genders.choose(&mut rng).unwrap())
            },
            
            SemanticType::Age => rng.gen_range(18..75).to_string(),
            SemanticType::BirthDate => {
                let years = rng.gen_range(18..70);
                format!("'{}'", (Utc::now() - Duration::days(years * 365)).format("%Y-%m-%d"))
            },
            
            SemanticType::CompanyName | SemanticType::OrganizationName => {
                format!("'{}'", CompanyName().fake::<String>().replace("'", "''"))
            },
            
            SemanticType::MerchantName => {
                if let Some(company) = ctx.get("company_name") {
                    let variants = vec![
                        format!("{} Store", company),
                        format!("{} Market", company),
                        CompanyName().fake::<String>(),
                    ];
                    format!("'{}'", variants.choose(&mut rng).unwrap().replace("'", "''"))
                } else {
                    format!("'{}'", CompanyName().fake::<String>().replace("'", "''"))
                }
            },
            
            SemanticType::Country => format!("'{}'", CountryName().fake::<String>().replace("'", "''")),
            SemanticType::State => format!("'{}'", StateName().fake::<String>().replace("'", "''")),
            SemanticType::City => format!("'{}'", CityName().fake::<String>().replace("'", "''")),
            SemanticType::StreetAddress => {
                let streets = ["Main St", "Oak Ave", "Maple Dr", "Pine Rd", "Elm St", "Park Blvd", "Broadway", "Market St"];
                format!("'{} {}'", rng.gen_range(100..9999), streets.choose(&mut rng).unwrap())
            },
            SemanticType::PostalCode => format!("'{}'", ZipCode().fake::<String>()),
            
            SemanticType::Latitude | SemanticType::GalacticCoordinate => {
                format!("{:.6}", rng.gen_range(-90.0..90.0))
            },
            SemanticType::Longitude => {
                format!("{:.6}", rng.gen_range(-180.0..180.0))
            },
            
            SemanticType::PhoneNumber | SemanticType::MobileNumber => {
                let codes = ["+1", "+44", "+61", "+91"];
                format!("'{}-{}-{}-{}'", 
                    codes.choose(&mut rng).unwrap(),
                    rng.gen_range(200..999),
                    rng.gen_range(200..999),
                    rng.gen_range(1000..9999)
                )
            },
            
            SemanticType::DomainName => {
                if let Some(company) = ctx.get("company_name") {
                    let clean: String = company.chars().filter(|c| c.is_alphanumeric()).collect();
                    format!("'{}.com'", clean.to_lowercase())
                } else {
                    format!("'example{}.com'", rng.gen_range(1000..9999))
                }
            },
            
            SemanticType::URL => {
                if let Some(domain) = ctx.get("domain") {
                    format!("'https://www.{}'", domain)
                } else {
                    format!("'https://www.example{}.com'", rng.gen_range(1000..9999))
                }
            },
            
            SemanticType::DateSigned | SemanticType::DateEstablished | SemanticType::DateLaunched | 
            SemanticType::DateCreated | SemanticType::DateRegistered | SemanticType::DateStart => {
                let days_ago = rng.gen_range(365..1825);
                format!("'{}'", (Utc::now() - Duration::days(days_ago)).format("%Y-%m-%d"))
            },
            
            SemanticType::DateEnd => {
                let base = ctx.get_any_start_date()
                    .unwrap_or_else(|| Utc::now().naive_utc().date() - Duration::days(rng.gen_range(365..730)));
                let days_add = rng.gen_range(30..730);
                format!("'{}'", (base + Duration::days(days_add)).format("%Y-%m-%d"))
            },
            
            SemanticType::DateUpdated => {
                let days_ago = rng.gen_range(1..90);
                format!("'{}'", (Utc::now() - Duration::days(days_ago)).format("%Y-%m-%d"))
            },
            
            SemanticType::Timestamp => {
                let days_ago = rng.gen_range(0..365);
                format!("'{}'", (Utc::now() - Duration::days(days_ago)).format("%Y-%m-%d %H:%M:%S"))
            },
            
            SemanticType::IPv4Address => {
                let ranges = [
                    (10, rng.gen_range(0..256), rng.gen_range(0..256), rng.gen_range(1..255)),
                    (172, rng.gen_range(16..32), rng.gen_range(0..256), rng.gen_range(1..255)),
                    (192, 168, rng.gen_range(0..256), rng.gen_range(1..255)),
                ];
                let ip = ranges.choose(&mut rng).unwrap();
                format!("'{}.{}.{}.{}'", ip.0, ip.1, ip.2, ip.3)
            },
            
            SemanticType::IPv6Address => {
                let segs: Vec<String> = (0..8).map(|_| format!("{:04x}", rng.gen_range(0..65536))).collect();
                format!("'{}'", segs.join(":"))
            },
            
            SemanticType::MacAddress => {
                format!("'{:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}'",
                    rng.gen_range(0..256), rng.gen_range(0..256), rng.gen_range(0..256),
                    rng.gen_range(0..256), rng.gen_range(0..256), rng.gen_range(0..256))
            },
            
            SemanticType::NetworkPort => rng.gen_range(1024..65535).to_string(),
            
            SemanticType::MoneyAmount | SemanticType::PriceValue | SemanticType::BalanceValue | SemanticType::CreditValue => {
                format!("{:.2}", rng.gen_range(100.0..99999.99))
            },
            
            SemanticType::CurrencyCode => {
                let currencies = ["USD", "EUR", "GBP", "JPY", "AUD", "CAD"];
                format!("'{}'", currencies.choose(&mut rng).unwrap())
            },
            
            SemanticType::WalletAddress => {
                if rng.gen_bool(0.6) {
                    let hex: String = (0..40).map(|_| format!("{:x}", rng.gen_range(0..16))).collect();
                    format!("'0x{}'", hex)
                } else {
                    let chars = "123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";
                    let addr: String = (0..33)
                        .map(|_| chars.chars().nth(rng.gen_range(0..chars.len())).unwrap())
                        .collect();
                    format!("'1{}'", addr)
                }
            },
            
            SemanticType::HashValue | SemanticType::EncryptionKey | SemanticType::TokenValue => {
                let hash: String = (0..32).map(|_| format!("{:x}", rng.gen_range(0..16))).collect();
                format!("'{}'", hash)
            },
            
            SemanticType::StatusActive | SemanticType::OperationalStatus => {
                let statuses = ["active", "inactive", "pending", "completed", "cancelled", "processing"];
                format!("'{}'", statuses.choose(&mut rng).unwrap())
            },
            
            SemanticType::TypeCategory | SemanticType::ClassLevel => {
                let types = ["standard", "basic", "advanced", "premium", "professional"];
                format!("'{}'", types.choose(&mut rng).unwrap())
            },
            
            SemanticType::SkillLevel | SemanticType::SecurityLevel | SemanticType::ClearanceLevel | SemanticType::RankTitle => {
                let levels = ["beginner", "intermediate", "advanced", "expert", "master"];
                format!("'{}'", levels.choose(&mut rng).unwrap())
            },
            
            SemanticType::PriorityLevel => {
                let priorities = ["low", "medium", "high", "critical", "urgent"];
                format!("'{}'", priorities.choose(&mut rng).unwrap())
            },
            
            SemanticType::TrackingCode | SemanticType::SerialNumber | SemanticType::BadgeID | 
            SemanticType::SKUCode | SemanticType::ReferenceID => {
                let prefix: String = (b'A'..=b'Z').map(|c| c as char).collect::<Vec<_>>()
                    .choose_multiple(&mut rng, 3).collect();
                format!("'{}-{}-{}'", prefix, rng.gen_range(1000..9999), rng.gen_range(100..999))
            },
            
            SemanticType::SectorName | SemanticType::OutpostName | SemanticType::PlanetName | SemanticType::StationName => {
                let prefixes = ["Alpha", "Beta", "Gamma", "Delta", "Epsilon", "Zeta", "Theta", "Omega"];
                let suffixes = ["Prime", "Station", "Base", "Colony", "Outpost", "Hub"];
                format!("'{} {}'", prefixes.choose(&mut rng).unwrap(), suffixes.choose(&mut rng).unwrap())
            },
            
            SemanticType::SpecimenName => {
                let names = ["alpha", "beta", "gamma", "delta", "epsilon", "zeta", "theta", "omega"];
                format!("'{}'", names.choose(&mut rng).unwrap())
            },
            
            SemanticType::JurisdictionZone => {
                let zones = ["Alpha Sector", "Beta Quadrant", "Gamma Region", "Delta Zone", "Epsilon District"];
                format!("'{}'", zones.choose(&mut rng).unwrap())
            },
            
            SemanticType::HazardClassification => {
                let classes = ["Level-1", "Level-2", "Level-3", "Level-4", "Level-5", "Biohazard", "Chemical", "Radiation", "Toxic"];
                format!("'{}'", classes.choose(&mut rng).unwrap())
            },
            
            SemanticType::TitleText => {
                let text: String = (3..8).fake();
                format!("'{}'", text.replace("'", "''"))
            },
            
            SemanticType::DescriptionText | SemanticType::SummaryText => {
                let text: String = (10..30).fake();
                format!("'{}'", text.replace("'", "''"))
            },
            
            SemanticType::BodyContent | SemanticType::CommentText | SemanticType::NotesText => {
                let text: String = (20..60).fake();
                format!("'{}'", text.replace("'", "''"))
            },
            
            SemanticType::FilePath => {
                let exts = ["dat", "bin", "tmp", "log", "txt"];
                let folders = ["/uploads", "/media", "/files", "/storage", "/data"];
                format!("'{}/{}.{}'", 
                    folders.choose(&mut rng).unwrap(),
                    Uuid::new_v4(),
                    exts.choose(&mut rng).unwrap()
                )
            },
            
            SemanticType::FirmwareVersion | SemanticType::SoftwareVersion => {
                format!("'{}.{}.{}'", rng.gen_range(1..10), rng.gen_range(0..20), rng.gen_range(0..100))
            },
            
            SemanticType::WeightMetric => format!("{:.2}", rng.gen_range(50.0..10000.0)),
            SemanticType::TemperatureCelsius => format!("{:.2}", rng.gen_range(-273.0..1000.0)),
            SemanticType::FrequencyHz => format!("{:.2}", rng.gen_range(100.0..10000.0)),
            SemanticType::DurationSeconds => rng.gen_range(60..86400).to_string(),
            SemanticType::DurationHours => rng.gen_range(1..500).to_string(),
            SemanticType::ByteSize => rng.gen_range(100..10000).to_string(),
            
            SemanticType::IntegerValue => rng.gen_range(1..10000).to_string(),
            SemanticType::DecimalValue => format!("{:.2}", rng.gen_range(0.0..9999.99)),
            
            SemanticType::JSONValue => {
                format!("'{{\"id\": \"{}\", \"status\": \"active\"}}'", Uuid::new_v4())
            },
            
            SemanticType::TextValue => {
                let words = ["alpha", "beta", "gamma", "delta", "epsilon"];
                format!("'{}'", words.choose(&mut rng).unwrap())
            },
            
            _ => "'default'".to_string(),
        }
    }
    
    fn update_context(&self, field: &str, value: &str, semantic: &SemanticType, ctx: &mut ContextEngine) {
        let clean = value.trim_matches('\'').to_string();
        if clean == "NULL" || clean.is_empty() {
            return;
        }
        
        ctx.set(&field.to_lowercase(), &clean);
        
        match semantic {
            SemanticType::FirstName => ctx.set("first_name", &clean),
            SemanticType::LastName => ctx.set("last_name", &clean),
            SemanticType::FullName => {
                let parts: Vec<&str> = clean.split_whitespace().collect();
                if parts.len() >= 2 {
                    ctx.set("first_name", parts[0]);
                    ctx.set("last_name", parts.last().unwrap());
                }
            },
            SemanticType::CompanyName => ctx.set("company_name", &clean),
            SemanticType::DomainName => ctx.set("domain", &clean),
            SemanticType::DateSigned | SemanticType::DateEstablished | SemanticType::DateCreated | SemanticType::DateStart => {
                if let Ok(date) = NaiveDate::parse_from_str(&clean, "%Y-%m-%d") {
                    ctx.set_date(&field.to_lowercase(), date);
                }
            },
            _ => {}
        }
    }
    
    fn get_fk_value(&self, ref_table: &str, dtype: &str) -> String {
        let mut rng = rand::thread_rng();
        
        if let Some(ids) = self.pk_storage.get(ref_table) {
            if !ids.is_empty() {
                let id = ids.choose(&mut rng).unwrap();
                if dtype.contains("uuid") || dtype.contains("char") || dtype.contains("text") {
                    return format!("'{}'", id);
                }
                return id.clone();
            }
        }
        
        "NULL".to_string()
    }
    
    fn generate_default(&self, dtype: &str, row_idx: usize) -> String {
        if dtype.contains("uuid") {
            format!("'{}'", Uuid::new_v4())
        } else if dtype.contains("int") {
            (row_idx + 1).to_string()
        } else {
            "'default'".to_string()
        }
    }
}