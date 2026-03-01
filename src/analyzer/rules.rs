use chrono::Utc;
use uuid::Uuid;

use crate::storage::models::*;

pub fn evaluate_snapshot(snapshot: &SystemSnapshot) -> Vec<Insight> {
    let mut insights = Vec::new();
    let now = Utc::now();

    // Memory pressure rules
    let mem_pct = snapshot.used_memory_bytes as f64 / snapshot.total_memory_bytes.max(1) as f64 * 100.0;
    if mem_pct > 90.0 {
        insights.push(Insight {
            id: Uuid::new_v4().to_string(),
            timestamp: now,
            category: InsightCategory::Memory,
            severity: InsightSeverity::Critical,
            title: format!("Critical memory pressure: {mem_pct:.0}% used"),
            description: format!(
                "System is using {:.1} GB of {:.1} GB RAM. Performance degradation is likely.",
                snapshot.used_memory_bytes as f64 / 1e9,
                snapshot.total_memory_bytes as f64 / 1e9
            ),
            suggestion: "Close memory-heavy applications or consider upgrading RAM.".into(),
            action_id: Some("kill-top-memory".into()),
            source: InsightSource::RuleEngine,
        });
    } else if mem_pct > 75.0 {
        insights.push(Insight {
            id: Uuid::new_v4().to_string(),
            timestamp: now,
            category: InsightCategory::Memory,
            severity: InsightSeverity::Warning,
            title: format!("High memory usage: {mem_pct:.0}%"),
            description: "Memory usage is elevated. Monitor for further increases.".into(),
            suggestion: "Review running applications and close those not in active use.".into(),
            action_id: None,
            source: InsightSource::RuleEngine,
        });
    }

    // Swap usage
    if snapshot.total_swap_bytes > 0 {
        let swap_pct = snapshot.used_swap_bytes as f64 / snapshot.total_swap_bytes as f64 * 100.0;
        if swap_pct > 50.0 {
            insights.push(Insight {
                id: Uuid::new_v4().to_string(),
                timestamp: now,
                category: InsightCategory::Performance,
                severity: InsightSeverity::Warning,
                title: format!("Heavy swap usage: {swap_pct:.0}%"),
                description: "System is heavily swapping to disk, causing slowdowns.".into(),
                suggestion: "Free up memory by closing unused applications.".into(),
                action_id: None,
                source: InsightSource::RuleEngine,
            });
        }
    }

    // Disk space rules
    for disk in &snapshot.disks {
        if disk.total_bytes == 0 {
            continue;
        }
        let used = disk.total_bytes.saturating_sub(disk.available_bytes);
        let used_pct = used as f64 / disk.total_bytes as f64 * 100.0;

        if used_pct > 95.0 {
            insights.push(Insight {
                id: Uuid::new_v4().to_string(),
                timestamp: now,
                category: InsightCategory::DiskSpace,
                severity: InsightSeverity::Critical,
                title: format!("Disk {} is almost full: {used_pct:.0}%", disk.mount_point),
                description: format!(
                    "Only {:.1} GB free on {}",
                    disk.available_bytes as f64 / 1e9,
                    disk.mount_point
                ),
                suggestion: "Run `compscan scan --full` to find large files to clean up.".into(),
                action_id: Some("cleanup-disk".into()),
                source: InsightSource::RuleEngine,
            });
        } else if used_pct > 85.0 {
            insights.push(Insight {
                id: Uuid::new_v4().to_string(),
                timestamp: now,
                category: InsightCategory::DiskSpace,
                severity: InsightSeverity::Warning,
                title: format!("Disk {} running low: {used_pct:.0}%", disk.mount_point),
                description: format!("{:.1} GB free remaining", disk.available_bytes as f64 / 1e9),
                suggestion: "Consider cleaning caches, temp files, or old downloads.".into(),
                action_id: Some("cleanup-caches".into()),
                source: InsightSource::RuleEngine,
            });
        }
    }

    // Uptime check
    let days = snapshot.uptime_secs / 86400;
    if days > 14 {
        insights.push(Insight {
            id: Uuid::new_v4().to_string(),
            timestamp: now,
            category: InsightCategory::Performance,
            severity: InsightSeverity::Suggestion,
            title: format!("System has been running for {days} days"),
            description: "Extended uptime can lead to memory leaks and degraded performance.".into(),
            suggestion: "Consider restarting your system to clear accumulated state.".into(),
            action_id: None,
            source: InsightSource::RuleEngine,
        });
    }

    // Process count
    if snapshot.process_count > 300 {
        insights.push(Insight {
            id: Uuid::new_v4().to_string(),
            timestamp: now,
            category: InsightCategory::Performance,
            severity: InsightSeverity::Suggestion,
            title: format!("{} processes running", snapshot.process_count),
            description: "High process count may indicate background bloat.".into(),
            suggestion: "Review startup items and close unnecessary background services.".into(),
            action_id: None,
            source: InsightSource::RuleEngine,
        });
    }

    insights
}

pub fn evaluate_activities(activities: &[ActivityRecord]) -> Vec<Insight> {
    let mut insights = Vec::new();
    let now = Utc::now();

    if activities.is_empty() {
        return insights;
    }

    // High average CPU
    let avg_cpu: f32 = activities.iter().map(|a| a.cpu_usage_percent).sum::<f32>() / activities.len() as f32;
    if avg_cpu > 70.0 {
        insights.push(Insight {
            id: Uuid::new_v4().to_string(),
            timestamp: now,
            category: InsightCategory::Performance,
            severity: InsightSeverity::Warning,
            title: format!("Sustained high CPU usage: {avg_cpu:.0}% average"),
            description: "CPU has been consistently high across recent observations.".into(),
            suggestion: "Identify and address the CPU-intensive process.".into(),
            action_id: None,
            source: InsightSource::RuleEngine,
        });
    }

    // Memory trend (increasing)
    if activities.len() >= 5 {
        let recent = &activities[..5];
        let older = if activities.len() >= 10 {
            &activities[5..10]
        } else {
            &activities[activities.len() / 2..]
        };

        let recent_avg: f32 = recent.iter().map(|a| a.memory_usage_percent).sum::<f32>() / recent.len() as f32;
        let older_avg: f32 = older.iter().map(|a| a.memory_usage_percent).sum::<f32>() / older.len().max(1) as f32;

        if recent_avg > older_avg + 10.0 {
            insights.push(Insight {
                id: Uuid::new_v4().to_string(),
                timestamp: now,
                category: InsightCategory::Memory,
                severity: InsightSeverity::Warning,
                title: "Memory usage trending upward".into(),
                description: format!(
                    "Memory usage increased from {older_avg:.0}% to {recent_avg:.0}% recently."
                ),
                suggestion: "Possible memory leak. Check for processes with growing memory footprint.".into(),
                action_id: None,
                source: InsightSource::RuleEngine,
            });
        }
    }

    // Dominant process
    let mut process_freq: std::collections::HashMap<String, usize> = std::collections::HashMap::new();
    for activity in activities {
        *process_freq.entry(activity.top_cpu_process.clone()).or_default() += 1;
    }
    if let Some((proc_name, count)) = process_freq.iter().max_by_key(|(_, c)| *c) {
        let pct = *count as f32 / activities.len() as f32 * 100.0;
        if pct > 60.0 && !proc_name.is_empty() {
            insights.push(Insight {
                id: Uuid::new_v4().to_string(),
                timestamp: now,
                category: InsightCategory::Productivity,
                severity: InsightSeverity::Info,
                title: format!("{proc_name} dominates CPU {pct:.0}% of the time"),
                description: format!("{proc_name} has been the top CPU consumer in {count}/{} samples.", activities.len()),
                suggestion: "If this is expected (e.g., compilation), no action needed. Otherwise, investigate.".into(),
                action_id: None,
                source: InsightSource::RuleEngine,
            });
        }
    }

    insights
}
