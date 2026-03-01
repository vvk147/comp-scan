use anyhow::Result;
use chrono::Utc;
use uuid::Uuid;

use crate::storage::models::{Insight, InsightCategory, InsightSeverity, InsightSource};

pub fn check_coding_activity() -> Result<Option<Insight>> {
    let git_status = check_git_repos()?;
    if let Some(insight) = git_status {
        return Ok(Some(insight));
    }
    Ok(None)
}

fn check_git_repos() -> Result<Option<Insight>> {
    let home = dirs::home_dir().unwrap_or_default();

    let common_code_dirs = [
        home.join("projects"),
        home.join("code"),
        home.join("dev"),
        home.join("src"),
        home.join("repos"),
        home.join("workspace"),
        home.join("Documents"),
    ];

    let mut uncommitted_repos = Vec::new();

    for dir in &common_code_dirs {
        if !dir.exists() {
            continue;
        }
        if let Ok(entries) = std::fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.join(".git").exists() {
                    let output = std::process::Command::new("git")
                        .args(["status", "--porcelain"])
                        .current_dir(&path)
                        .output();

                    if let Ok(output) = output {
                        let stdout = String::from_utf8_lossy(&output.stdout);
                        let changes = stdout.lines().count();
                        if changes > 0 {
                            uncommitted_repos.push((
                                path.file_name()
                                    .unwrap_or_default()
                                    .to_string_lossy()
                                    .to_string(),
                                changes,
                            ));
                        }
                    }
                }
            }
        }
    }

    if uncommitted_repos.is_empty() {
        return Ok(None);
    }

    let total_changes: usize = uncommitted_repos.iter().map(|(_, c)| c).sum();
    let repo_list: Vec<String> = uncommitted_repos
        .iter()
        .take(5)
        .map(|(name, count)| format!("{name} ({count} changes)"))
        .collect();

    Ok(Some(Insight {
        id: Uuid::new_v4().to_string(),
        timestamp: Utc::now(),
        category: InsightCategory::Coding,
        severity: if total_changes > 50 {
            InsightSeverity::Warning
        } else {
            InsightSeverity::Suggestion
        },
        title: format!(
            "{} repo(s) with uncommitted changes",
            uncommitted_repos.len()
        ),
        description: format!(
            "{total_changes} total uncommitted changes across: {}",
            repo_list.join(", ")
        ),
        suggestion: "Consider committing or stashing your work to avoid losing changes".into(),
        action_id: None,
        source: InsightSource::RuleEngine,
    }))
}
