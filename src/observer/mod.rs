pub mod activity;
pub mod coding;
pub mod habits;
pub mod patterns;

use crate::storage::Database;
use anyhow::Result;
use tokio::signal;

pub async fn run_observer(db: &Database, interval_secs: u64) -> Result<()> {
    let db = db.clone();
    let interval = std::time::Duration::from_secs(interval_secs);

    println!("  Observer daemon started. Press Ctrl+C to stop.\n");

    let observe_handle = tokio::spawn({
        let db = db.clone();
        async move {
            let mut ticker = tokio::time::interval(interval);
            loop {
                ticker.tick().await;
                match activity::collect_activity_snapshot() {
                    Ok(record) => {
                        tracing::debug!(
                            "Activity: {} processes, CPU top: {}, Mem top: {}",
                            record.process_count,
                            record.top_cpu_process,
                            record.top_memory_process,
                        );
                        if let Err(e) = db.save_activity(&record) {
                            tracing::error!("Failed to save activity: {e}");
                        }
                    }
                    Err(e) => {
                        tracing::error!("Failed to collect activity: {e}");
                    }
                }
            }
        }
    });

    let coding_handle = tokio::spawn({
        let db = db.clone();
        async move {
            let mut ticker =
                tokio::time::interval(std::time::Duration::from_secs(interval_secs * 4));
            loop {
                ticker.tick().await;
                if let Ok(Some(insight)) = coding::check_coding_activity() {
                    tracing::info!("Coding insight: {}", insight.title);
                    let _ = db.save_insight(&insight);
                }
            }
        }
    });

    let habits_handle = tokio::spawn({
        let db = db.clone();
        async move {
            let mut ticker = tokio::time::interval(std::time::Duration::from_secs(300));
            loop {
                ticker.tick().await;
                let insights = habits::check_habits(&db);
                for insight in insights {
                    tracing::info!("Habit insight: {}", insight.title);
                    let _ = db.save_insight(&insight);
                }
            }
        }
    });

    signal::ctrl_c().await?;
    println!("\n  Observer daemon stopping...");
    observe_handle.abort();
    coding_handle.abort();
    habits_handle.abort();

    println!("  Stopped. Data saved to: {}", db.path.display());
    Ok(())
}
