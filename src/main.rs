mod schema;
mod generator;
mod sorter; // NEW MODULE

use clap::{Parser, Subcommand};
use sqlx::postgres::PgPoolOptions;
use crate::generator::Generator;
use std::time::Instant;

#[derive(Parser)]
#[command(name = "synthdb")]
#[command(about = "Production-Ready Synthetic Data Engine")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Clone a database structure and data patterns
    Clone {
        #[arg(short, long)]
        url: String,

        #[arg(short, long, default_value = "dump.sql")]
        output: String,

        #[arg(short, long, default_value = "100")]
        rows: usize,
    },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Clone { url, output, rows } => {
            let start = Instant::now();
            println!("🚀 Connecting to database...");

            let pool = PgPoolOptions::new()
                .max_connections(5)
                .connect(&url)
                .await?;

            println!("🔍 Analyzing schema & sampling data...");
            // 1. Extract Schema + Samples
            let raw_schema = schema::extract_schema(&pool).await?;
            println!("✅ Found {} tables. Calculating dependencies...", raw_schema.len());

            // 2. Topological Sort
            let sorted_schema = sorter::sort_tables(raw_schema)?;
            println!("✅ Dependencies resolved. Insertion order determined.");

            println!("🔨 Generating synthetic data...");
            // 3. Generate
            let mut generator = Generator::new(sorted_schema);
            generator.generate_sql_dump(&output, rows)?;

            println!("✨ Done in {:.2?}! Saved to {}", start.elapsed(), output);
        }
    }

    Ok(())
}