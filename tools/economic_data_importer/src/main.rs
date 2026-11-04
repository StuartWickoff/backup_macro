//! Economic Data Importer for Macro Journal
//!
//! This tool imports economic data CSV files into the Macro Journal SQLite database.
//! 
//! Usage:
//!   economic-data-importer import --data-dir /path/to/economic_data --db-path /path/to/macro_journal.db
//!   economic-data-importer validate --data-dir /path/to/economic_data
//!   economic-data-importer generate-summary --data-dir /path/to/economic_data --output /path/to/CONTROL_SUMMARY.md

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use csv::Reader;
use serde::Deserialize;
use sqlx::sqlite::SqlitePoolOptions;
use sqlx::SqlitePool;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::path::{Path, PathBuf};

/// Economic data record from CSV
#[derive(Debug, Deserialize, Clone)]
pub struct EconomicRecord {
    pub series_id: String,
    pub date: String,
    pub value: Option<f64>,
    pub value_secondary: Option<f64>,
}

/// Supported currencies
const CURRENCIES: &[&str] = &[
    "EUR", "GBP", "JPY", "CHF", "CAD", "AUD", "NZD", "CNY",
];

/// Series IDs by currency
const SERIES_IDS: &[&str] = &[
    "policy_rate",
    "ppi_yoy",
    "unemployment_rate",
    "real_gdp_growth",
    "hicp_headline_yoy",
    "hicp_core_yoy",
    "cpi_headline_yoy",
    "cpi_core_yoy",
    "cpi_common",
    "cpi_trimmed",
    "cpi_median",
    "cpi_trimmed_mean",
    "cpi_weighted_median",
    "sp_global_manufacturing_pmi",
    "sp_global_services_pmi",
    "caixin_manufacturing_pmi",
    "caixin_services_pmi",
];

#[derive(Parser)]
#[command(name = "economic-data-importer")]
#[command(author = "Macro Journal")]
#[command(version = "0.1.0")]
#[command(about = "Import economic data into Macro Journal database")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Import CSV files into SQLite database
    Import {
        /// Directory containing economic_data subdirectories
        #[arg(short, long)]
        data_dir: PathBuf,
        /// Path to SQLite database (default: XDG location)
        #[arg(short, long)]
        db_path: Option<PathBuf>,
        /// Overwrite existing data
        #[arg(short, long)]
        overwrite: bool,
    },
    /// Validate CSV files
    Validate {
        /// Directory containing economic_data subdirectories
        #[arg(short, long)]
        data_dir: PathBuf,
    },
    /// Generate control summary report
    GenerateSummary {
        /// Directory containing economic_data subdirectories
        #[arg(short, long)]
        data_dir: PathBuf,
        /// Output file path
        #[arg(short, long)]
        output: PathBuf,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Import {
            data_dir,
            db_path,
            overwrite,
        } => {
            let pool = open_database(db_path).await?;
            import_data(&pool, &data_dir, overwrite).await?;
        }
        Commands::Validate { data_dir } => {
            validate_data(&data_dir)?;
        }
        Commands::GenerateSummary { data_dir, output } => {
            generate_summary(&data_dir, &output)?;
        }
    }

    Ok(())
}

/// Open or create SQLite database
async fn open_database(db_path: Option<PathBuf>) -> Result<SqlitePool> {
    let path = db_path.unwrap_or_else(|| {
        // Default to XDG path
        let mut path = dirs::data_dir().expect("Cannot determine XDG data directory");
        path.push("macro-journal");
        path.push("macro_journal.db");
        path
    });

    // Ensure parent directory exists
    if let Some(parent) = path.parent() {
        tokio::fs::create_dir_all(parent).await?;
    }

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(sqlx::sqlite::SqliteConnectOptions::new()
            .filename(path)
            .create_if_missing(true)
            .foreign_keys(true))
        .await?;

    // Create economic_data table if not exists
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS economic_data (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            currency_code TEXT NOT NULL,
            series_id TEXT NOT NULL,
            date TEXT NOT NULL,           -- YYYY-MM-DD
            value REAL,
            value_secondary REAL,
            source TEXT NOT NULL,
            last_updated TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now')),
            UNIQUE(currency_code, series_id, date)
        )
        "#,
    )
    .execute(&pool)
    .await?;

    Ok(pool)
}

/// Import economic data from CSV files
async fn import_data(pool: &SqlitePool, data_dir: &Path, overwrite: bool) -> Result<()> {
    println!("📥 Importing economic data from: {}", data_dir.display());

    for currency in CURRENCIES {
        let csv_path = data_dir.join(currency).join("history.csv");
        
        if !csv_path.exists() {
            println!("⚠️  No data found for {}", currency);
            continue;
        }

        println!("📄 Processing {}...", currency);
        let records = load_csv(&csv_path)?;
        let count = import_records(pool, currency, &records, overwrite).await?;
        println!("✅ Imported {} records for {}", count, currency);
    }

    Ok(())
}

/// Load CSV file into EconomicRecord structs
fn load_csv(path: &Path) -> Result<Vec<EconomicRecord>> {
    let file = File::open(path)
        .with_context(|| format!("Cannot open CSV file: {}", path.display()))?;
    
    let mut rdr = Reader::from_reader(file);
    let mut records = Vec::new();
    
    for result in rdr.deserialize() {
        let record: EconomicRecord = result
            .map_err(|e| anyhow::anyhow!("CSV parsing error: {}", e))?;
        records.push(record);
    }

    Ok(records)
}

/// Import records into database
async fn import_records(
    pool: &SqlitePool,
    currency: &str,
    records: &[EconomicRecord],
    overwrite: bool,
) -> Result<usize> {
    let mut count = 0;
    
    for record in records {
        // Validate date format
        if !is_valid_date(&record.date) {
            println!("⚠️  Invalid date format: {}", record.date);
            continue;
        }

        // Validate series_id
        if !SERIES_IDS.contains(&record.series_id.as_str()) {
            println!("⚠️  Unknown series_id: {}", record.series_id);
            continue;
        }

        let result = sqlx::query(
            r#"
            INSERT OR REPLACE INTO economic_data
            (currency_code, series_id, date, value, value_secondary, source, last_updated)
            VALUES (?, ?, ?, ?, ?, ?, datetime('now'))
            "#,
        )
        .bind(currency)
        .bind(&record.series_id)
        .bind(&record.date)
        .bind(record.value)
        .bind(record.value_secondary)
        .bind("manual_import")  // Source: manual import from official data
        .execute(pool)
        .await;

        match result {
            Ok(_) => count += 1,
            Err(e) => {
                if !overwrite {
                    println!("⚠️  Duplicate or error for {} {} {}: {}", 
                        currency, record.series_id, record.date, e);
                }
            }
        }
    }

    Ok(count)
}

/// Validate CSV files
fn validate_data(data_dir: &Path) -> Result<()> {
    println!("🔍 Validating economic data in: {}", data_dir.display());
    
    let mut total_records = 0;
    let mut errors = Vec::new();
    
    for currency in CURRENCIES {
        let csv_path = data_dir.join(currency).join("history.csv");
        
        if !csv_path.exists() {
            println!("⚠️  No CSV file found for {}", currency);
            continue;
        }

        let records = match load_csv(&csv_path) {
            Ok(recs) => recs,
            Err(e) => {
                errors.push(format!("{}: CSV load error - {}", currency, e));
                continue;
            }
        };

        let currency_errors = validate_records(&records, currency);
        errors.extend(currency_errors);
        total_records += records.len();
    }

    println!("\n📊 Validation Results:");
    println!("   Total records: {}", total_records);
    println!("   Errors: {}", errors.len());

    if !errors.is_empty() {
        println!("\n❌ Validation Errors:");
        for error in &errors {
            println!("   - {}", error);
        }
        anyhow::bail!("Validation failed with {} errors", errors.len());
    }

    println!("✅ All data validated successfully!");
    Ok(())
}

/// Validate individual records
fn validate_records(records: &[EconomicRecord], currency: &str) -> Vec<String> {
    let mut errors = Vec::new();
    let mut seen_series = HashSet::new();
    let mut monthly_dates = HashSet::new();
    let mut quarterly_dates = HashSet::new();
    
    // Series that should be monthly
    let monthly_series: HashSet<&str> = [
        "policy_rate", "ppi_yoy", "unemployment_rate",
        "hicp_headline_yoy", "hicp_core_yoy", "cpi_headline_yoy", 
        "cpi_core_yoy", "cpi_common", "cpi_trimmed", "cpi_median",
        "cpi_trimmed_mean", "cpi_weighted_median",
        "sp_global_manufacturing_pmi", "sp_global_services_pmi",
        "caixin_manufacturing_pmi", "caixin_services_pmi",
    ].iter().cloned().collect();
    
    // Series that should be quarterly
    let quarterly_series: HashSet<&str> = ["real_gdp_growth"].iter().cloned().collect();

    for record in records {
        // Check series_id
        if !SERIES_IDS.contains(&record.series_id.as_str()) {
            errors.push(format!(
                "{}: Unknown series_id '{}'", currency, record.series_id
            ));
            continue;
        }

        // Check date format
        if !is_valid_date(&record.date) {
            errors.push(format!(
                "{}: Invalid date '{}' for series {}", 
                currency, record.date, record.series_id
            ));
            continue;
        }

        // Check date type based on series
        let date_parts: Vec<&str> = record.date.split('-').collect();
        if date_parts.len() != 3 {
            errors.push(format!(
                "{}: Invalid date format '{}'", currency, record.date
            ));
            continue;
        }

        let day = date_parts[2];
        
        if monthly_series.contains(record.series_id.as_str()) {
            if day != "01" {
                errors.push(format!(
                    "{}: Monthly series '{}' has date '{}' (expected YYYY-MM-01)",
                    currency, record.series_id, record.date
                ));
            }
            monthly_dates.insert(&record.date);
        } else if quarterly_series.contains(record.series_id.as_str()) {
            if !["01", "04", "07", "10"].contains(&day) {
                errors.push(format!(
                    "{}: Quarterly series '{}' has date '{}' (expected YYYY-01-01, YYYY-04-01, etc.)",
                    currency, record.series_id, record.date
                ));
            }
            quarterly_dates.insert(&record.date);
        }

        // Track seen series
        seen_series.insert(record.series_id.as_str());
    }

    // Check for missing series
    let expected_series = get_expected_series(currency);
    for series in &expected_series {
        if !seen_series.contains(*series) {
            errors.push(format!(
                "{}: Missing expected series '{}'", currency, series
            ));
        }
    }

    // Check monthly coverage (36 months)
    if !monthly_dates.is_empty() {
        let expected_months = 36;
        let unique_months: HashSet<&str> = monthly_dates.iter()
            .map(|d| &d[..7])  // Extract YYYY-MM
            .collect();
        
        if unique_months.len() < expected_months {
            errors.push(format!(
                "{}: Only {} monthly observations (expected {})",
                currency, unique_months.len(), expected_months
            ));
        }
    }

    // Check quarterly coverage (12 quarters)
    if !quarterly_dates.is_empty() {
        let expected_quarters = 12;
        let unique_quarters: HashSet<&str> = quarterly_dates.iter()
            .map(|d| &d[..7])  // Extract YYYY-MM
            .collect();
        
        if unique_quarters.len() < expected_quarters {
            errors.push(format!(
                "{}: Only {} quarterly observations (expected {})",
                currency, unique_quarters.len(), expected_quarters
            ));
        }
    }

    errors
}

/// Get expected series for each currency
fn get_expected_series(currency: &str) -> Vec<&'static str> {
    match currency {
        "EUR" => vec![
            "policy_rate",
            "hicp_headline_yoy",
            "hicp_core_yoy",
            "ppi_yoy",
            "unemployment_rate",
            "real_gdp_growth",
            "sp_global_manufacturing_pmi",
            "sp_global_services_pmi",
        ],
        "CAD" => vec![
            "policy_rate",
            "cpi_common",
            "cpi_trimmed",
            "cpi_median",
            "ppi_yoy",
            "unemployment_rate",
            "real_gdp_growth",
            "sp_global_manufacturing_pmi",
            "sp_global_services_pmi",
        ],
        "AUD" => vec![
            "policy_rate",
            "cpi_trimmed_mean",
            "cpi_weighted_median",
            "ppi_yoy",
            "unemployment_rate",
            "real_gdp_growth",
            "sp_global_manufacturing_pmi",
            "sp_global_services_pmi",
        ],
        "CNY" => vec![
            "policy_rate",
            "cpi_headline_yoy",
            "cpi_core_yoy",
            "ppi_yoy",
            "unemployment_rate",
            "real_gdp_growth",
            "caixin_manufacturing_pmi",
            "caixin_services_pmi",
        ],
        _ => vec![
            "policy_rate",
            "cpi_headline_yoy",
            "cpi_core_yoy",
            "ppi_yoy",
            "unemployment_rate",
            "real_gdp_growth",
            "sp_global_manufacturing_pmi",
            "sp_global_services_pmi",
        ],
    }
}

/// Validate date format (YYYY-MM-DD)
fn is_valid_date(date: &str) -> bool {
    if date.len() != 10 {
        return false;
    }
    
    let parts: Vec<&str> = date.split('-').collect();
    if parts.len() != 3 {
        return false;
    }
    
    // Check YYYY
    if parts[0].len() != 4 || !parts[0].chars().all(|c| c.is_ascii_digit()) {
        return false;
    }
    
    // Check MM (01-12)
    if parts[1].len() != 2 || !parts[1].chars().all(|c| c.is_ascii_digit()) {
        return false;
    }
    let month: u32 = parts[1].parse().unwrap_or(0);
    if month < 1 || month > 12 {
        return false;
    }
    
    // Check DD (01-31)
    if parts[2].len() != 2 || !parts[2].chars().all(|c| c.is_ascii_digit()) {
        return false;
    }
    let day: u32 = parts[2].parse().unwrap_or(0);
    if day < 1 || day > 31 {
        return false;
    }
    
    true
}

/// Generate control summary report
fn generate_summary(data_dir: &Path, output: &Path) -> Result<()> {
    println!("📊 Generating control summary...");
    
    let mut summary = String::new();
    let mut total_complete = 0;
    let mut total_series = 0;
    
    // Header
    summary.push_str(&format!(
        "# Résumé de Contrôle - Données Économiques\n\n{}\n\n",
        chrono::Local::now().format("%Y-%m-%d %H:%M:%S")
    ));
    
    summary.push_str("## 📊 Statut par Devise\n\n");
    
    for currency in CURRENCIES {
        let csv_path = data_dir.join(currency).join("history.csv");
        
        if !csv_path.exists() {
            summary.push_str(&format!(
                "### {} (❌ Non disponible)\n\n",
                currency
            ));
            continue;
        }

        let records = match load_csv(&csv_path) {
            Ok(recs) => recs,
            Err(e) => {
                summary.push_str(&format!(
                    "### {} (❌ Erreur: {})\n\n",
                    currency, e
                ));
                continue;
            }
        };

        let expected_series = get_expected_series(currency);
        let mut series_stats: HashMap<&str, (Vec<String>, Vec<String>)> = HashMap::new();
        let mut monthly_dates = HashSet::new();
        let mut quarterly_dates = HashSet::new();
        
        // Monthly series
        let monthly_series: HashSet<&str> = [
            "policy_rate", "ppi_yoy", "unemployment_rate",
            "hicp_headline_yoy", "hicp_core_yoy", "cpi_headline_yoy", 
            "cpi_core_yoy", "cpi_common", "cpi_trimmed", "cpi_median",
            "cpi_trimmed_mean", "cpi_weighted_median",
            "sp_global_manufacturing_pmi", "sp_global_services_pmi",
            "caixin_manufacturing_pmi", "caixin_services_pmi",
        ].iter().cloned().collect();
        
        // Quarterly series
        let quarterly_series: HashSet<&str> = ["real_gdp_growth"].iter().cloned().collect();

        for record in &records {
            if monthly_series.contains(record.series_id.as_str()) {
                monthly_dates.insert(&record.date);
            } else if quarterly_series.contains(record.series_id.as_str()) {
                quarterly_dates.insert(&record.date);
            }
            
            let entry = series_stats.entry(record.series_id.as_str()).or_insert_with(|| {
                (Vec::new(), Vec::new())
            });
            entry.0.push(record.date.clone());
        }

        // Generate table for this currency
        summary.push_str(&format!("### {} (", currency));
        
        // Check if all expected series are present
        let mut all_present = true;
        for series in &expected_series {
            if !series_stats.contains_key(*series) {
                all_present = false;
                break;
            }
        }
        
        if all_present {
            summary.push_str("✅");
        } else {
            summary.push_str("⚠️");
        }
        summary.push_str(")\n\n");
        
        summary.push_str("| Série | Observations | Première Date | Dernière Date | Manquants | Source |\n");
        summary.push_str("|-------|--------------|---------------|---------------|-----------|--------|\n");
        
        for series in &expected_series {
            if let Some((dates, _)) = series_stats.get(*series) {
                let count = dates.len();
                let first = dates.first().cloned().unwrap_or_default();
                let last = dates.last().cloned().unwrap_or_default();
                let expected_count = if monthly_series.contains(series) { 36 } else { 12 };
                let missing = if count >= expected_count { 0 } else { expected_count - count };
                
                summary.push_str(&format!(
                    "| `{}` | {} | {} | {} | {} | Official |\n",
                    series, count, first, last, missing
                ));
                total_series += 1;
                if missing == 0 {
                    total_complete += 1;
                }
            } else {
                summary.push_str(&format!(
                    "| `{}` | 0 | - | - | 36 | Official |\n",
                    series
                ));
                total_series += 1;
            }
        }
        
        summary.push_str("\n");
    }

    // Global summary
    summary.push_str("## 📈 Résumé Global\n\n");
    summary.push_str("| Métrique | Valeur |\n");
    summary.push_str("|----------|--------|\n");
    summary.push_str(&format!("| Séries complètes | {} / {} |\n", total_complete, total_series));
    
    // Calculate coverage percentage
    let coverage_pct = if total_series > 0 {
        (total_complete as f64 / total_series as f64) * 100.0
    } else {
        0.0
    };
    summary.push_str(&format!("| Couverture | {:.1}% |\n", coverage_pct));
    
    // Write to output file
    std::fs::write(output, summary)?;
    println!("✅ Control summary written to: {}", output.display());
    
    Ok(())
}
