use chrono::{Timelike, Utc};
use uuid::Uuid;

use crate::storage::Database;
use crate::storage::models::{Insight, InsightCategory, InsightSeverity, InsightSource};

pub fn check_habits(db: &Database) -> Vec<Insight> {
    let mut insights = Vec::new();

    let now = Utc::now();
    let hour = now.hour();

    if hour >= 23 || hour < 5 {
        insights.push(Insight {
            id: Uuid::new_v4().to_string(),
            timestamp: now,
            category: InsightCategory::Habits,
            severity: InsightSeverity::Suggestion,
            title: "Late night computer usage detected".into(),
            description: format!("It's {}:{}0 — extended screen time late at night can disrupt sleep patterns.", hour, now.minute() / 10),
            suggestion: "Consider winding down. Enable night mode or schedule a shutdown timer.".into(),
            action_id: None,
            source: InsightSource::RuleEngine,
        });
    }

    if let Ok(activities) = db.get_recent_activities(60) {
        if activities.len() >= 12 {
            let continuous_high_usage = activities
                .iter()
                .take(12)
                .all(|a| a.cpu_usage_percent > 5.0);

            if continuous_high_usage {
                insights.push(Insight {
                    id: Uuid::new_v4().to_string(),
                    timestamp: now,
                    category: InsightCategory::Habits,
                    severity: InsightSeverity::Suggestion,
                    title: "Extended work session detected".into(),
                    description: "You've been actively working for over 6 minutes without a break.".into(),
                    suggestion: "Take a 5-minute break. Look away from the screen, stretch, hydrate.".into(),
                    action_id: None,
                    source: InsightSource::RuleEngine,
                });
            }
        }

        if activities.len() >= 2 {
            let recent_two = &activities[..2];
            let rapid_app_switching = recent_two.iter().all(|a| a.process_count > 100);
            if rapid_app_switching {
                let avg_processes: f32 = recent_two.iter().map(|a| a.process_count as f32).sum::<f32>() / 2.0;
                insights.push(Insight {
                    id: Uuid::new_v4().to_string(),
                    timestamp: now,
                    category: InsightCategory::Productivity,
                    severity: InsightSeverity::Info,
                    title: format!("High process count: {avg_processes:.0} average"),
                    description: "Running many processes simultaneously may impact performance and focus.".into(),
                    suggestion: "Close unused applications to free resources and reduce distractions.".into(),
                    action_id: None,
                    source: InsightSource::RuleEngine,
                });
            }
        }
    }

    insights
}
