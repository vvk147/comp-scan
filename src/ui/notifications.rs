use crate::storage::models::Insight;

pub fn notify(insight: &Insight) {
    #[cfg(target_os = "macos")]
    {
        let _ = std::process::Command::new("osascript")
            .args([
                "-e",
                &format!(
                    "display notification \"{}\" with title \"CompScan: {}\"",
                    insight.suggestion.replace('"', "'"),
                    insight.title.replace('"', "'")
                ),
            ])
            .output();
    }

    #[cfg(target_os = "linux")]
    {
        let _ = std::process::Command::new("notify-send")
            .args([
                &format!("CompScan: {}", insight.title),
                &insight.suggestion,
            ])
            .output();
    }

    #[cfg(target_os = "windows")]
    {
        let _ = std::process::Command::new("powershell")
            .args([
                "-Command",
                &format!(
                    "New-BurntToastNotification -Text 'CompScan: {}', '{}'",
                    insight.title.replace("'", "''"),
                    insight.suggestion.replace("'", "''")
                ),
            ])
            .output();
    }
}
