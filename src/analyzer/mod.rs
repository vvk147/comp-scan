pub mod insights;
pub mod rules;
pub mod statistics;

use crate::storage::Database;
use anyhow::Result;

pub async fn generate_report(db: &Database, json_output: bool) -> Result<()> {
    let activities = db.get_recent_activities(200)?;
    let snapshot = db.get_latest_snapshot()?;

    let mut all_insights = Vec::new();

    if let Some(ref snap) = snapshot {
        let rule_insights = rules::evaluate_snapshot(snap);
        all_insights.extend(rule_insights);
    }

    if !activities.is_empty() {
        let stat_insights = statistics::analyze_trends(&activities);
        all_insights.extend(stat_insights);
    }

    let pattern_insights = rules::evaluate_activities(&activities);
    all_insights.extend(pattern_insights);

    let existing = db.get_all_insights()?;
    all_insights.extend(existing);

    all_insights.sort_by(|a, b| b.severity.cmp(&a.severity));
    all_insights.dedup_by(|a, b| a.title == b.title);

    for insight in &all_insights {
        db.save_insight(insight)?;
    }

    if json_output {
        println!("{}", serde_json::to_string_pretty(&all_insights)?);
    } else {
        insights::print_report(&all_insights, snapshot.as_ref());
    }

    Ok(())
}
