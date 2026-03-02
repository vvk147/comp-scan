use crate::storage::models::*;

pub fn print_report(insights: &[Insight], snapshot: Option<&SystemSnapshot>) {
    println!("\n  ╔═══════════════════════════════════════════╗");
    println!("  ║         CompScan Insights Report          ║");
    println!("  ╚═══════════════════════════════════════════╝\n");

    if let Some(snap) = snapshot {
        println!(
            "  System: {} ({} {})",
            snap.hostname, snap.os_name, snap.os_version
        );
        println!(
            "  Scan time: {}\n",
            snap.timestamp.format("%Y-%m-%d %H:%M:%S UTC")
        );
    }

    if insights.is_empty() {
        println!("  No insights generated yet.");
        println!("  Run `compscan scan` to analyze your system,");
        println!("  or `compscan observe` to start tracking activity.\n");
        return;
    }

    let critical: Vec<&Insight> = insights
        .iter()
        .filter(|i| i.severity == InsightSeverity::Critical)
        .collect();
    let warnings: Vec<&Insight> = insights
        .iter()
        .filter(|i| i.severity == InsightSeverity::Warning)
        .collect();
    let suggestions: Vec<&Insight> = insights
        .iter()
        .filter(|i| i.severity == InsightSeverity::Suggestion)
        .collect();
    let info: Vec<&Insight> = insights
        .iter()
        .filter(|i| i.severity == InsightSeverity::Info)
        .collect();

    println!(
        "  Summary: {} critical, {} warnings, {} suggestions, {} info\n",
        critical.len(),
        warnings.len(),
        suggestions.len(),
        info.len()
    );

    if !critical.is_empty() {
        println!("  \x1b[31m--- CRITICAL ---\x1b[0m");
        for insight in &critical {
            print_insight(insight);
        }
    }

    if !warnings.is_empty() {
        println!("  \x1b[33m--- WARNINGS ---\x1b[0m");
        for insight in &warnings {
            print_insight(insight);
        }
    }

    if !suggestions.is_empty() {
        println!("  \x1b[36m--- SUGGESTIONS ---\x1b[0m");
        for insight in &suggestions {
            print_insight(insight);
        }
    }

    if !info.is_empty() {
        println!("  \x1b[34m--- INFO ---\x1b[0m");
        for insight in &info {
            print_insight(insight);
        }
    }

    let actionable: Vec<&Insight> = insights.iter().filter(|i| i.action_id.is_some()).collect();
    if !actionable.is_empty() {
        println!("\n  Actionable items ({}):", actionable.len());
        for insight in &actionable {
            if let Some(ref action_id) = insight.action_id {
                println!("    compscan act {action_id}  →  {}", insight.suggestion);
            }
        }
    }

    println!();
}

fn print_insight(insight: &Insight) {
    let icon = match insight.severity {
        InsightSeverity::Critical => "\x1b[31m!!\x1b[0m",
        InsightSeverity::Warning => "\x1b[33m! \x1b[0m",
        InsightSeverity::Suggestion => "\x1b[36m* \x1b[0m",
        InsightSeverity::Info => "\x1b[34mi \x1b[0m",
    };

    let source = match insight.source {
        InsightSource::RuleEngine => "rule",
        InsightSource::Statistical => "stat",
        InsightSource::Ollama => "ai",
    };

    println!(
        "  {icon} [{:>11}] [{}] {}",
        insight.category, source, insight.title
    );
    println!("     {}", insight.description);
    println!("     → {}", insight.suggestion);

    if let Some(ref action_id) = insight.action_id {
        println!("     ⚡ Fix: compscan act {action_id}");
    }
    println!();
}
