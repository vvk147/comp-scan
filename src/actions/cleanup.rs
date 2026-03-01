use crate::storage::models::*;

pub fn cache_cleanup_action() -> Action {
    let home = dirs::home_dir().unwrap_or_default();
    let cache_dir = home.join(".cache");

    Action {
        id: "cleanup-caches".into(),
        title: "Clean application caches".into(),
        description: "Remove cached data from ~/.cache and platform-specific cache directories.".into(),
        risk_level: RiskLevel::Low,
        category: InsightCategory::DiskSpace,
        command: ActionCommand::ShellCommand(format!(
            "du -sh {} 2>/dev/null && find {} -type f -atime +30 -delete 2>/dev/null && echo 'Old cache files cleaned'",
            cache_dir.display(),
            cache_dir.display()
        )),
        reversible: false,
        estimated_impact: "Free several hundred MB to several GB of disk space".into(),
    }
}

pub fn temp_cleanup_action() -> Action {
    let tmp = std::env::temp_dir();

    Action {
        id: "cleanup-temp".into(),
        title: "Clean temporary files".into(),
        description: format!("Remove old temporary files from {}", tmp.display()),
        risk_level: RiskLevel::Low,
        category: InsightCategory::DiskSpace,
        command: ActionCommand::ShellCommand(format!(
            "find {} -type f -atime +7 -delete 2>/dev/null && echo 'Temp files older than 7 days cleaned'",
            tmp.display()
        )),
        reversible: false,
        estimated_impact: "Free up temporary disk space".into(),
    }
}

pub fn log_cleanup_action() -> Action {
    let home = dirs::home_dir().unwrap_or_default();

    Action {
        id: "cleanup-logs".into(),
        title: "Clean old log files".into(),
        description: "Remove log files older than 30 days from common log directories.".into(),
        risk_level: RiskLevel::Low,
        category: InsightCategory::DiskSpace,
        command: ActionCommand::ShellCommand(format!(
            "find {} -name '*.log' -type f -mtime +30 -delete 2>/dev/null && echo 'Old log files cleaned'",
            home.display()
        )),
        reversible: false,
        estimated_impact: "Free disk space from accumulated log files".into(),
    }
}

pub fn disk_cleanup_action() -> Action {
    Action {
        id: "cleanup-disk".into(),
        title: "Full disk cleanup".into(),
        description: "Comprehensive cleanup: caches, temp files, logs, and package manager caches.".into(),
        risk_level: RiskLevel::Medium,
        category: InsightCategory::DiskSpace,
        command: ActionCommand::ShellCommand(
            "echo 'Cleaning npm cache...' && npm cache clean --force 2>/dev/null; \
             echo 'Cleaning cargo cache...' && cargo cache -a 2>/dev/null; \
             echo 'Cleaning pip cache...' && pip cache purge 2>/dev/null; \
             echo 'Disk cleanup complete'"
                .into(),
        ),
        reversible: false,
        estimated_impact: "Can free 1-10+ GB depending on development tool caches".into(),
    }
}
