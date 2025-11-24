use serde::{Deserialize, Serialize};
use sqlx::postgres::PgPool;
use sqlx::Row;
use anyhow::Result;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Column {
    pub name: String,
    pub data_type: String,
    pub is_nullable: bool,
    // NEW: We store sampled values from the real DB
    pub distinct_values: Vec<String>, 
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
    // 1. Get all tables
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

        // 2. Get columns
        let cols_raw = sqlx::query!(
            "SELECT column_name, data_type, is_nullable 
             FROM information_schema.columns 
             WHERE table_name = $1 AND table_schema = 'public'
             ORDER BY ordinal_position",
            t_name
        )
        .fetch_all(pool)
        .await?;

        let mut columns = Vec::new();

        // 3. THE SAMPLER: Read real data from the table
        for c in cols_raw {
            let col_name = c.column_name.unwrap();
            let data_type = c.data_type.unwrap();
            let is_nullable = c.is_nullable.unwrap() == "YES";

            let mut distinct_values = Vec::new();

            // Only sample if it's text/keyword and NOT an ID/Primary Key
            // This detects things like "status", "category", "country"
            if (data_type == "text" || data_type.contains("char")) 
                && !col_name.contains("id") 
                && !col_name.contains("email") 
                && !col_name.contains("name") {
                
                // Query distinct values (limit 20)
                let query = format!("SELECT DISTINCT {} FROM {} LIMIT 20", col_name, t_name);
                if let Ok(rows) = sqlx::query(&query).fetch_all(pool).await {
                    for row in rows {
                        // Safely try to get string, ignore nulls
                        if let Ok(val) = row.try_get::<String, _>(0) {
                            distinct_values.push(val);
                        }
                    }
                }
            }

            columns.push(Column {
                name: col_name,
                data_type,
                is_nullable,
                distinct_values,
            });
        }

        // 4. Get Foreign Keys
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