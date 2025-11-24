use serde::{Deserialize, Serialize};
use sqlx::postgres::PgPool;
use sqlx::Row;
use anyhow::Result;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Column {
    pub name: String,
    pub data_type: String, // "integer", "text", "numeric", "ARRAY"
    pub is_nullable: bool,
    pub numeric_precision: Option<i32>, // Total digits
    pub numeric_scale: Option<i32>,     // Decimal places
    pub distinct_values: Vec<String>,   // Sampled data
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ForeignKey {
    pub column: String,
    pub ref_table: String,
    pub ref_column: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Table {
    pub table_name: String,
    pub columns: Vec<Column>,
    pub foreign_keys: Vec<ForeignKey>,
}

pub async fn extract_schema(pool: &PgPool) -> Result<Vec<Table>> {
    let tables = sqlx::query!(
        "SELECT table_name FROM information_schema.tables 
         WHERE table_schema = 'public' AND table_type = 'BASE TABLE'"
    )
    .fetch_all(pool)
    .await?;

    let mut schema = Vec::new();

    for t in tables {
        let t_name = t.table_name.unwrap();
        println!("   ...analyzing table: {}", t_name);

        // We now fetch numeric_precision and numeric_scale
        let cols_raw = sqlx::query!(
            "SELECT column_name, data_type, is_nullable, numeric_precision, numeric_scale, udt_name
             FROM information_schema.columns 
             WHERE table_name = $1 AND table_schema = 'public'
             ORDER BY ordinal_position",
            t_name
        )
        .fetch_all(pool)
        .await?;

        let mut columns = Vec::new();

        for c in cols_raw {
            let col_name = c.column_name.unwrap();
            let mut data_type = c.data_type.unwrap();
            let udt_name = c.udt_name.unwrap_or_default(); // Detect arrays via udt_name
            
            // Detect Array Types (Postgres specific)
            if udt_name.starts_with('_') {
                data_type = "ARRAY".to_string();
            }

            let is_nullable = c.is_nullable.unwrap() == "YES";
            let numeric_precision = c.numeric_precision;
            let numeric_scale = c.numeric_scale;

            // SAMPLER: Only sample if it makes sense
            let mut distinct_values = Vec::new();
            if (data_type == "text" || data_type.contains("char")) 
                && !col_name.contains("id") 
                && !col_name.contains("email") 
                && !col_name.contains("name") 
                && !col_name.contains("url") {
                
                let query = format!("SELECT DISTINCT {} FROM {} LIMIT 20", col_name, t_name);
                if let Ok(rows) = sqlx::query(&query).fetch_all(pool).await {
                    for row in rows {
                        if let Ok(val) = row.try_get::<String, _>(0) {
                            if !val.trim().is_empty() {
                                distinct_values.push(val);
                            }
                        }
                    }
                }
            }

            columns.push(Column {
                name: col_name,
                data_type,
                is_nullable,
                numeric_precision,
                numeric_scale,
                distinct_values,
            });
        }

        let fks = sqlx::query!(
            r#"
            SELECT
                kcu.column_name,
                ccu.table_name AS foreign_table_name,
                ccu.column_name AS foreign_column_name
            FROM information_schema.key_column_usage AS kcu
            JOIN information_schema.constraint_column_usage AS ccu
            ON kcu.constraint_name = ccu.constraint_name
            JOIN information_schema.table_constraints AS tc
            ON kcu.constraint_name = tc.constraint_name
            WHERE kcu.table_name = $1 AND tc.constraint_type = 'FOREIGN KEY'
            "#,
            t_name
        )
        .fetch_all(pool)
        .await?;

        let foreign_keys: Vec<ForeignKey> = fks.into_iter().map(|f| ForeignKey {
            column: f.column_name.unwrap(),
            ref_table: f.foreign_table_name.unwrap(),
            ref_column: f.foreign_column_name.unwrap(),
        }).collect();

        schema.push(Table {
            table_name: t_name,
            columns,
            foreign_keys,
        });
    }

    Ok(schema)
}