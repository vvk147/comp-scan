pub mod filesystem;
pub mod processes;
pub mod security;
pub mod startup;
pub mod system;

use crate::storage::Database;
use anyhow::Result;
use chrono::Utc;
use uuid::Uuid;

pub async fn run_scan(db: &Database, full: bool) -> Result<()> {
    println!("  [1/5] Scanning hardware & system info...");
    let snapshot = system::collect_system_snapshot()?;
    db.save_snapshot(&snapshot)?;
    system::print_summary(&snapshot);

    println!("\n  [2/5] Analyzing running processes...");
    let procs = processes::analyze_processes()?;
    processes::print_summary(&procs);

    println!("\n  [3/5] Auditing disk usage...");
    let disk_report = if full {
        filesystem::deep_scan()?
    } else {
        filesystem::quick_scan()?
    };
    filesystem::print_summary(&disk_report);

    println!("\n  [4/5] Checking startup items...");
    let startup_items = startup::list_startup_items()?;
    startup::print_summary(&startup_items);

    println!("\n  [5/5] Running security audit...");
    let sec_report = security::audit()?;
    security::print_summary(&sec_report);

    let activity = crate::storage::models::ActivityRecord {
        id: Uuid::new_v4().to_string(),
        timestamp: Utc::now(),
        active_processes: procs.top_by_cpu.clone(),
        cpu_usage_percent: snapshot.used_memory_bytes as f32 / snapshot.total_memory_bytes as f32
            * 100.0,
        memory_usage_percent: snapshot.used_memory_bytes as f32
            / snapshot.total_memory_bytes as f32
            * 100.0,
        top_cpu_process: procs
            .top_by_cpu
            .first()
            .map(|p| p.name.clone())
            .unwrap_or_default(),
        top_memory_process: procs
            .top_by_memory
            .first()
            .map(|p| p.name.clone())
            .unwrap_or_default(),
        process_count: procs.total_count,
    };
    db.save_activity(&activity)?;

    println!("\n  Scan complete. Run `compscan report` for AI-powered insights.");
    println!("  Data saved to: {}", db.path.display());
    Ok(())
}
