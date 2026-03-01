use crate::storage::models::*;
use sysinfo::System;

pub fn kill_top_memory_action() -> Action {
    let mut sys = System::new_all();
    sys.refresh_all();

    let top_proc = sys
        .processes()
        .values()
        .max_by_key(|p| p.memory());

    let (pid, name) = top_proc
        .map(|p| (p.pid().as_u32(), p.name().to_string_lossy().to_string()))
        .unwrap_or((0, "unknown".into()));

    Action {
        id: "kill-top-memory".into(),
        title: format!("Kill highest memory process: {name}"),
        description: format!("Terminate {name} (PID {pid}) which is consuming the most memory."),
        risk_level: RiskLevel::High,
        category: InsightCategory::Memory,
        command: ActionCommand::KillProcess(pid),
        reversible: false,
        estimated_impact: "Immediately free significant memory, but may cause data loss in the killed app".into(),
    }
}

pub fn optimize_startup_action() -> Action {
    Action {
        id: "optimize-startup".into(),
        title: "Review and optimize startup items".into(),
        description: "List all startup items and suggest which ones to disable for faster boot.".into(),
        risk_level: RiskLevel::Medium,
        category: InsightCategory::Performance,
        command: ActionCommand::Custom(
            "Review startup items in `compscan scan` output and disable unnecessary ones.".into(),
        ),
        reversible: true,
        estimated_impact: "Faster boot time and lower baseline resource usage".into(),
    }
}
