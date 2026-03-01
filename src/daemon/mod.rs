pub mod scheduler;

use anyhow::Result;
use crate::storage::Database;

pub async fn show_status(db: &Database) -> Result<()> {
    let config = db.get_config()?;
    let snapshot = db.get_latest_snapshot()?;
    let activity_count = db.activity_count()?;
    let insight_count = db.insight_count()?;

    println!("  Status");
    println!("  ======");
    println!("  Database:    {}", db.path.display());
    println!("  Activities:  {activity_count} recorded");
    println!("  Insights:    {insight_count} generated");
    println!("  Ollama:      {}", config.ollama_endpoint);

    if let Some(snap) = snapshot {
        println!("\n  Last scan:   {}", snap.timestamp.format("%Y-%m-%d %H:%M:%S UTC"));
        println!("  Host:        {}", snap.hostname);
        println!("  OS:          {} {}", snap.os_name, snap.os_version);
        let mem_pct = snap.used_memory_bytes as f64 / snap.total_memory_bytes.max(1) as f64 * 100.0;
        println!("  Memory:      {:.0}%", mem_pct);
        println!("  Processes:   {}", snap.process_count);
    } else {
        println!("\n  No scan data. Run `compscan scan` to get started.");
    }

    Ok(())
}
