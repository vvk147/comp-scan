pub mod db;
pub mod models;
pub mod encryption;

pub use db::Database;

use anyhow::Result;

pub fn reset_config(db: &Database) -> Result<()> {
    db.clear_config()?;
    tracing::info!("Configuration reset to defaults");
    Ok(())
}

pub fn show_config(db: &Database) -> Result<()> {
    let config = db.get_config()?;
    println!("CompScan Configuration");
    println!("======================");
    println!("Observer interval:  {}s", config.observer_interval_secs);
    println!("Ollama endpoint:    {}", config.ollama_endpoint);
    println!("Ollama model:       {}", config.ollama_model);
    println!("Web dashboard port: {}", config.web_port);
    println!("Encryption:         {}", if config.encryption_enabled { "enabled" } else { "disabled" });
    println!("Data directory:     {}", config.data_dir.display());
    println!("\nTrust Levels:");
    println!("  Low risk:    auto-approve");
    println!("  Medium risk: notify + 1-click approve");
    println!("  High risk:   explicit confirmation");
    println!("  Critical:    blocked");
    Ok(())
}
