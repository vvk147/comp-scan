use anyhow::Result;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct StartupItem {
    pub name: String,
    pub path: String,
    pub enabled: bool,
    pub source: StartupSource,
}

#[derive(Debug, Clone)]
pub enum StartupSource {
    LaunchAgent,
    LaunchDaemon,
    LoginItem,
    SystemdUser,
    SystemdSystem,
    WindowsRegistry,
    WindowsStartup,
    Crontab,
    Unknown,
}

impl std::fmt::Display for StartupSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

pub fn list_startup_items() -> Result<Vec<StartupItem>> {
    let mut items = Vec::new();

    #[cfg(target_os = "macos")]
    {
        items.extend(scan_macos_launch_agents()?);
    }

    #[cfg(target_os = "linux")]
    {
        items.extend(scan_linux_systemd()?);
    }

    #[cfg(target_os = "windows")]
    {
        items.extend(scan_windows_startup()?);
    }

    items.extend(scan_crontab()?);

    Ok(items)
}

#[cfg(target_os = "macos")]
fn scan_macos_launch_agents() -> Result<Vec<StartupItem>> {
    let mut items = Vec::new();
    let home = dirs::home_dir().unwrap_or_default();

    let agent_dirs = [
        home.join("Library/LaunchAgents"),
        PathBuf::from("/Library/LaunchAgents"),
        PathBuf::from("/Library/LaunchDaemons"),
    ];

    for dir in &agent_dirs {
        if !dir.exists() {
            continue;
        }
        if let Ok(entries) = std::fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().map(|e| e == "plist").unwrap_or(false) {
                    let name = path
                        .file_stem()
                        .map(|s| s.to_string_lossy().to_string())
                        .unwrap_or_default();
                    let source = if dir.to_string_lossy().contains("LaunchDaemons") {
                        StartupSource::LaunchDaemon
                    } else {
                        StartupSource::LaunchAgent
                    };
                    items.push(StartupItem {
                        name,
                        path: path.to_string_lossy().to_string(),
                        enabled: true,
                        source,
                    });
                }
            }
        }
    }

    Ok(items)
}

#[cfg(target_os = "linux")]
fn scan_linux_systemd() -> Result<Vec<StartupItem>> {
    let mut items = Vec::new();
    let home = dirs::home_dir().unwrap_or_default();

    let user_dir = home.join(".config/systemd/user");
    if user_dir.exists() {
        if let Ok(entries) = std::fs::read_dir(&user_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().map(|e| e == "service").unwrap_or(false) {
                    items.push(StartupItem {
                        name: path.file_stem().unwrap_or_default().to_string_lossy().to_string(),
                        path: path.to_string_lossy().to_string(),
                        enabled: true,
                        source: StartupSource::SystemdUser,
                    });
                }
            }
        }
    }

    let output = std::process::Command::new("systemctl")
        .args(["--user", "list-unit-files", "--type=service", "--state=enabled", "--no-pager", "--no-legend"])
        .output();

    if let Ok(output) = output {
        let stdout = String::from_utf8_lossy(&output.stdout);
        for line in stdout.lines() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if let Some(name) = parts.first() {
                items.push(StartupItem {
                    name: name.trim_end_matches(".service").to_string(),
                    path: name.to_string(),
                    enabled: true,
                    source: StartupSource::SystemdUser,
                });
            }
        }
    }

    Ok(items)
}

#[cfg(target_os = "windows")]
fn scan_windows_startup() -> Result<Vec<StartupItem>> {
    let mut items = Vec::new();

    let output = std::process::Command::new("reg")
        .args(["query", r"HKCU\Software\Microsoft\Windows\CurrentVersion\Run"])
        .output();

    if let Ok(output) = output {
        let stdout = String::from_utf8_lossy(&output.stdout);
        for line in stdout.lines().skip(2) {
            let parts: Vec<&str> = line.trim().splitn(3, "    ").collect();
            if parts.len() >= 3 {
                items.push(StartupItem {
                    name: parts[0].to_string(),
                    path: parts[2].to_string(),
                    enabled: true,
                    source: StartupSource::WindowsRegistry,
                });
            }
        }
    }

    Ok(items)
}

fn scan_crontab() -> Result<Vec<StartupItem>> {
    let mut items = Vec::new();

    let output = std::process::Command::new("crontab")
        .arg("-l")
        .output();

    if let Ok(output) = output {
        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            for (i, line) in stdout.lines().enumerate() {
                let line = line.trim();
                if !line.is_empty() && !line.starts_with('#') {
                    items.push(StartupItem {
                        name: format!("cron-job-{}", i + 1),
                        path: line.to_string(),
                        enabled: true,
                        source: StartupSource::Crontab,
                    });
                }
            }
        }
    }

    Ok(items)
}

pub fn print_summary(items: &[StartupItem]) {
    println!("  Found {} startup items:", items.len());
    for item in items.iter().take(15) {
        let status = if item.enabled { "ON " } else { "OFF" };
        println!(
            "    [{status}] {:<35} ({})",
            truncate(&item.name, 35),
            item.source
        );
    }
    if items.len() > 15 {
        println!("    ... and {} more", items.len() - 15);
    }
}

fn truncate(s: &str, max: usize) -> String {
    if s.len() > max {
        format!("{}...", &s[..max.saturating_sub(3)])
    } else {
        s.to_string()
    }
}
