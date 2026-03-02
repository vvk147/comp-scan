use chrono::Utc;
use uuid::Uuid;

use crate::storage::models::*;

pub fn analyze_trends(activities: &[ActivityRecord]) -> Vec<Insight> {
    let mut insights = Vec::new();

    if activities.len() < 3 {
        return insights;
    }

    let now = Utc::now();

    let cpu_values: Vec<f32> = activities.iter().map(|a| a.cpu_usage_percent).collect();
    let mem_values: Vec<f32> = activities.iter().map(|a| a.memory_usage_percent).collect();

    // CPU anomaly detection (simple z-score)
    if let Some(anomaly) = detect_anomaly(&cpu_values) {
        if anomaly.z_score > 2.0 {
            insights.push(Insight {
                id: Uuid::new_v4().to_string(),
                timestamp: now,
                category: InsightCategory::Performance,
                severity: InsightSeverity::Warning,
                title: format!("CPU usage spike detected: {:.0}%", anomaly.value),
                description: format!(
                    "Current CPU usage ({:.0}%) is {:.1} standard deviations above the mean ({:.0}%).",
                    anomaly.value, anomaly.z_score, anomaly.mean
                ),
                suggestion: "Check for runaway processes or unexpected background tasks.".into(),
                action_id: None,
                source: InsightSource::Statistical,
            });
        }
    }

    // Memory anomaly detection
    if let Some(anomaly) = detect_anomaly(&mem_values) {
        if anomaly.z_score > 2.0 {
            insights.push(Insight {
                id: Uuid::new_v4().to_string(),
                timestamp: now,
                category: InsightCategory::Memory,
                severity: InsightSeverity::Warning,
                title: format!("Memory usage spike: {:.0}%", anomaly.value),
                description: format!(
                    "Memory at {:.0}% vs average {:.0}% ({:.1} sigma).",
                    anomaly.value, anomaly.mean, anomaly.z_score
                ),
                suggestion: "Investigate recently opened applications for memory leaks.".into(),
                action_id: None,
                source: InsightSource::Statistical,
            });
        }
    }

    // Process count trend
    let proc_counts: Vec<f32> = activities.iter().map(|a| a.process_count as f32).collect();
    let proc_mean = mean(&proc_counts);
    let latest_proc = proc_counts.first().copied().unwrap_or(0.0);
    if latest_proc > proc_mean * 1.5 && latest_proc > 200.0 {
        insights.push(Insight {
            id: Uuid::new_v4().to_string(),
            timestamp: now,
            category: InsightCategory::Performance,
            severity: InsightSeverity::Suggestion,
            title: format!("Process count above normal: {latest_proc:.0} vs avg {proc_mean:.0}"),
            description: "More processes than usual are running.".into(),
            suggestion: "Check for spawned child processes or stuck background jobs.".into(),
            action_id: None,
            source: InsightSource::Statistical,
        });
    }

    insights
}

struct Anomaly {
    value: f32,
    mean: f32,
    z_score: f32,
}

fn detect_anomaly(values: &[f32]) -> Option<Anomaly> {
    if values.len() < 3 {
        return None;
    }

    let m = mean(values);
    let sd = std_dev(values, m);

    if sd < 0.001 {
        return None;
    }

    let latest = values[0];
    let z = (latest - m).abs() / sd;

    Some(Anomaly {
        value: latest,
        mean: m,
        z_score: z,
    })
}

fn mean(values: &[f32]) -> f32 {
    if values.is_empty() {
        return 0.0;
    }
    values.iter().sum::<f32>() / values.len() as f32
}

fn std_dev(values: &[f32], mean: f32) -> f32 {
    if values.len() < 2 {
        return 0.0;
    }
    let variance: f32 =
        values.iter().map(|v| (v - mean).powi(2)).sum::<f32>() / (values.len() - 1) as f32;
    variance.sqrt()
}
