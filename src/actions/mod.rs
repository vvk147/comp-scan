pub mod cleanup;
pub mod executor;
pub mod optimization;
pub mod permissions;
pub mod workflow;

use anyhow::{bail, Result};
use chrono::Utc;
use uuid::Uuid;

use crate::storage::models::*;
use crate::storage::Database;

pub async fn execute_action(db: &Database, action_id: &str, force: bool) -> Result<()> {
    let action = match db.get_action(action_id)? {
        Some(a) => a,
        None => {
            if let Some(a) = get_builtin_action(action_id) {
                a
            } else {
                bail!(
                    "Unknown action: {action_id}. Run `compscan report` to see available actions."
                );
            }
        }
    };

    println!("  Action: {}", action.title);
    println!("  Risk:   {}", action.risk_level);
    println!("  Detail: {}", action.description);
    println!();

    let approved = permissions::check_permission(&action, force)?;

    if !approved {
        println!("  Action cancelled.");
        db.log_action_execution(&ActionLog {
            id: Uuid::new_v4().to_string(),
            action_id: action_id.to_string(),
            timestamp: Utc::now(),
            approved: false,
            executed: false,
            success: false,
            output: "User declined".into(),
        })?;
        return Ok(());
    }

    println!("  Executing...");
    let result = executor::run_action(&action).await;

    match &result {
        Ok(output) => {
            println!("  Done: {output}");
            db.log_action_execution(&ActionLog {
                id: Uuid::new_v4().to_string(),
                action_id: action_id.to_string(),
                timestamp: Utc::now(),
                approved: true,
                executed: true,
                success: true,
                output: output.clone(),
            })?;
        }
        Err(e) => {
            println!("  Failed: {e}");
            db.log_action_execution(&ActionLog {
                id: Uuid::new_v4().to_string(),
                action_id: action_id.to_string(),
                timestamp: Utc::now(),
                approved: true,
                executed: true,
                success: false,
                output: e.to_string(),
            })?;
        }
    }

    result.map(|_| ())
}

fn get_builtin_action(id: &str) -> Option<Action> {
    match id {
        "cleanup-caches" => Some(cleanup::cache_cleanup_action()),
        "cleanup-temp" => Some(cleanup::temp_cleanup_action()),
        "cleanup-logs" => Some(cleanup::log_cleanup_action()),
        "cleanup-disk" => Some(cleanup::disk_cleanup_action()),
        "kill-top-memory" => Some(optimization::kill_top_memory_action()),
        "optimize-startup" => Some(optimization::optimize_startup_action()),
        _ => None,
    }
}
