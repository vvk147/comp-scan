use anyhow::Result;
use crate::storage::Database;
use crate::storage::models::*;
use super::ollama::OllamaClient;
use super::prompts;

pub struct HybridAI {
    ollama: OllamaClient,
    ollama_available: bool,
}

impl HybridAI {
    pub async fn new(endpoint: &str, model: &str) -> Self {
        let ollama = OllamaClient::new(endpoint, model);
        let ollama_available = ollama.is_available().await;

        if ollama_available {
            tracing::info!("Ollama connected at {endpoint} (model: {model})");
        } else {
            tracing::info!("Ollama not available — using rule engine only");
        }

        Self {
            ollama,
            ollama_available,
        }
    }

    pub async fn analyze(&self, db: &Database) -> Result<Vec<Insight>> {
        let mut insights = Vec::new();

        let snapshot = db.get_latest_snapshot()?;
        let activities = db.get_recent_activities(50)?;

        if let Some(ref snap) = snapshot {
            let rule_insights = crate::analyzer::rules::evaluate_snapshot(snap);
            insights.extend(rule_insights);
        }

        let activity_insights = crate::analyzer::rules::evaluate_activities(&activities);
        insights.extend(activity_insights);

        let stat_insights = crate::analyzer::statistics::analyze_trends(&activities);
        insights.extend(stat_insights);

        if self.ollama_available && should_use_llm(&insights) {
            tracing::info!("Routing to Ollama for deeper analysis...");
            if let Ok(ai_insights) = self.get_llm_insights(snapshot.as_ref(), &activities).await {
                insights.extend(ai_insights);
            }
        }

        Ok(insights)
    }

    async fn get_llm_insights(
        &self,
        snapshot: Option<&SystemSnapshot>,
        activities: &[ActivityRecord],
    ) -> Result<Vec<Insight>> {
        let mut context = String::new();

        if let Some(snap) = snapshot {
            context.push_str(&format!(
                "System: {} {} | CPU: {} cores | RAM: {:.1}GB/{:.1}GB ({:.0}%) | Processes: {}\n",
                snap.os_name,
                snap.os_version,
                snap.cpu_count,
                snap.used_memory_bytes as f64 / 1e9,
                snap.total_memory_bytes as f64 / 1e9,
                snap.used_memory_bytes as f64 / snap.total_memory_bytes.max(1) as f64 * 100.0,
                snap.process_count,
            ));

            for disk in &snap.disks {
                let used_pct = (disk.total_bytes.saturating_sub(disk.available_bytes)) as f64
                    / disk.total_bytes.max(1) as f64
                    * 100.0;
                context.push_str(&format!(
                    "Disk {}: {:.0}% used ({:.1}GB free)\n",
                    disk.mount_point,
                    used_pct,
                    disk.available_bytes as f64 / 1e9,
                ));
            }
        }

        if !activities.is_empty() {
            let avg_cpu: f32 =
                activities.iter().map(|a| a.cpu_usage_percent).sum::<f32>() / activities.len() as f32;
            let avg_mem: f32 =
                activities.iter().map(|a| a.memory_usage_percent).sum::<f32>() / activities.len() as f32;

            context.push_str(&format!(
                "\nActivity ({} samples): Avg CPU {:.0}%, Avg Memory {:.0}%\n",
                activities.len(),
                avg_cpu,
                avg_mem,
            ));

            let mut proc_freq: std::collections::HashMap<String, usize> =
                std::collections::HashMap::new();
            for a in activities {
                *proc_freq.entry(a.top_cpu_process.clone()).or_default() += 1;
            }
            let mut top_procs: Vec<(String, usize)> = proc_freq.into_iter().collect();
            top_procs.sort_by(|a, b| b.1.cmp(&a.1));
            context.push_str("Top processes: ");
            for (name, count) in top_procs.iter().take(5) {
                context.push_str(&format!("{name}({count}) "));
            }
            context.push('\n');
        }

        let prompt = prompts::build_analysis_prompt(&context, "system");
        let response = self.ollama.generate(&prompt).await?;

        let insights = parse_llm_response(&response);
        Ok(insights)
    }
}

fn should_use_llm(rule_insights: &[Insight]) -> bool {
    let has_complex_issues = rule_insights
        .iter()
        .any(|i| matches!(i.severity, InsightSeverity::Critical | InsightSeverity::Warning));

    let enough_data = rule_insights.len() >= 3;

    has_complex_issues || enough_data
}

fn parse_llm_response(response: &str) -> Vec<Insight> {
    let mut insights = Vec::new();

    for line in response.lines() {
        let line = line.trim();
        if line.is_empty() || line.len() < 10 {
            continue;
        }

        let trimmed = line.trim_start_matches(|c: char| c.is_numeric() || c == '.' || c == ')' || c == ' ');
        if trimmed.len() < 10 {
            continue;
        }

        let category = if trimmed.to_lowercase().contains("security") {
            InsightCategory::Security
        } else if trimmed.to_lowercase().contains("memory") {
            InsightCategory::Memory
        } else if trimmed.to_lowercase().contains("disk") {
            InsightCategory::DiskSpace
        } else if trimmed.to_lowercase().contains("productiv") {
            InsightCategory::Productivity
        } else if trimmed.to_lowercase().contains("habit") {
            InsightCategory::Habits
        } else {
            InsightCategory::Performance
        };

        let severity = if trimmed.to_lowercase().contains("critical") {
            InsightSeverity::Critical
        } else if trimmed.to_lowercase().contains("warning") {
            InsightSeverity::Warning
        } else {
            InsightSeverity::Suggestion
        };

        insights.push(Insight {
            id: uuid::Uuid::new_v4().to_string(),
            timestamp: chrono::Utc::now(),
            category,
            severity,
            title: truncate(trimmed, 80),
            description: trimmed.to_string(),
            suggestion: "AI-generated insight — review and apply as appropriate.".into(),
            action_id: None,
            source: InsightSource::Ollama,
        });
    }

    insights.truncate(5);
    insights
}

fn truncate(s: &str, max: usize) -> String {
    if s.len() > max {
        format!("{}...", &s[..max.saturating_sub(3)])
    } else {
        s.to_string()
    }
}
