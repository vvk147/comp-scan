use crate::storage::models::ActivityRecord;

#[derive(Debug)]
pub struct UsagePattern {
    pub peak_hour: u8,
    pub avg_cpu_usage: f32,
    pub avg_memory_usage: f32,
    pub most_used_app: String,
    pub context_switches_per_hour: f32,
}

pub fn detect_patterns(activities: &[ActivityRecord]) -> Option<UsagePattern> {
    if activities.len() < 10 {
        return None;
    }

    let mut hour_counts = [0u32; 24];
    let mut total_cpu = 0.0f32;
    let mut total_mem = 0.0f32;
    let mut app_frequency: std::collections::HashMap<String, usize> = std::collections::HashMap::new();

    for activity in activities {
        let hour = activity.timestamp.hour() as usize;
        if hour < 24 {
            hour_counts[hour] += 1;
        }
        total_cpu += activity.cpu_usage_percent;
        total_mem += activity.memory_usage_percent;

        *app_frequency.entry(activity.top_cpu_process.clone()).or_default() += 1;
    }

    let peak_hour = hour_counts
        .iter()
        .enumerate()
        .max_by_key(|(_, &count)| count)
        .map(|(hour, _)| hour as u8)
        .unwrap_or(0);

    let count = activities.len() as f32;
    let most_used_app = app_frequency
        .into_iter()
        .max_by_key(|(_, count)| *count)
        .map(|(name, _)| name)
        .unwrap_or_default();

    Some(UsagePattern {
        peak_hour,
        avg_cpu_usage: total_cpu / count,
        avg_memory_usage: total_mem / count,
        most_used_app,
        context_switches_per_hour: 0.0, // needs window tracking for accurate measurement
    })
}

use chrono::Timelike;
