use crate::storage::models::*;

pub fn suggest_workflow_actions(insights: &[Insight]) -> Vec<Action> {
    let mut actions = Vec::new();

    for insight in insights {
        match insight.category {
            InsightCategory::Coding => {
                if insight.title.contains("uncommitted") {
                    actions.push(Action {
                        id: format!("workflow-git-{}", &insight.id[..8]),
                        title: "Commit or stash pending changes".into(),
                        description: insight.description.clone(),
                        risk_level: RiskLevel::Low,
                        category: InsightCategory::Coding,
                        command: ActionCommand::Custom(
                            "Review uncommitted changes and commit or stash them.".into(),
                        ),
                        reversible: true,
                        estimated_impact: "Prevent data loss from uncommitted work".into(),
                    });
                }
            }
            InsightCategory::Habits => {
                if insight.title.contains("break") || insight.title.contains("session") {
                    actions.push(Action {
                        id: format!("workflow-break-{}", &insight.id[..8]),
                        title: "Take a break".into(),
                        description: "Step away from the screen for 5 minutes.".into(),
                        risk_level: RiskLevel::Low,
                        category: InsightCategory::Habits,
                        command: ActionCommand::Custom("Take a 5-minute break.".into()),
                        reversible: true,
                        estimated_impact: "Improved focus and reduced eye strain".into(),
                    });
                }
            }
            _ => {}
        }
    }

    actions
}
