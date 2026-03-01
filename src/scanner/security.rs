use anyhow::Result;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct SecurityReport {
    pub findings: Vec<SecurityFinding>,
    pub score: u8, // 0-100
}

#[derive(Debug, Clone)]
pub struct SecurityFinding {
    pub severity: FindingSeverity,
    pub category: String,
    pub title: String,
    pub detail: String,
    pub remediation: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FindingSeverity {
    Info,
    Low,
    Medium,
    High,
    Critical,
}

impl std::fmt::Display for FindingSeverity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Info => write!(f, "INFO"),
            Self::Low => write!(f, "LOW "),
            Self::Medium => write!(f, "MED "),
            Self::High => write!(f, "HIGH"),
            Self::Critical => write!(f, "CRIT"),
        }
    }
}

pub fn audit() -> Result<SecurityReport> {
    let mut findings = Vec::new();

    check_ssh_keys(&mut findings);
    check_sensitive_files(&mut findings);
    check_world_writable(&mut findings);
    check_firewall(&mut findings);

    let deductions: u8 = findings
        .iter()
        .map(|f| match f.severity {
            FindingSeverity::Info => 0u8,
            FindingSeverity::Low => 5,
            FindingSeverity::Medium => 10,
            FindingSeverity::High => 20,
            FindingSeverity::Critical => 30,
        })
        .sum::<u8>()
        .min(100);

    let score = 100u8.saturating_sub(deductions);

    Ok(SecurityReport { findings, score })
}

fn check_ssh_keys(findings: &mut Vec<SecurityFinding>) {
    let ssh_dir = dirs::home_dir()
        .map(|h| h.join(".ssh"))
        .unwrap_or_else(|| PathBuf::from("~/.ssh"));

    if !ssh_dir.exists() {
        return;
    }

    if let Ok(entries) = std::fs::read_dir(&ssh_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() {
                let name = path.file_name().unwrap_or_default().to_string_lossy();
                if name.starts_with("id_") && !name.ends_with(".pub") {
                    #[cfg(unix)]
                    {
                        use std::os::unix::fs::PermissionsExt;
                        if let Ok(meta) = std::fs::metadata(&path) {
                            let mode = meta.permissions().mode();
                            if mode & 0o077 != 0 {
                                findings.push(SecurityFinding {
                                    severity: FindingSeverity::High,
                                    category: "SSH".into(),
                                    title: format!("Insecure permissions on {name}"),
                                    detail: format!("Key file has mode {:o}, should be 600", mode & 0o777),
                                    remediation: format!("chmod 600 {}", path.display()),
                                });
                            }
                        }
                    }
                }
            }
        }
    }

    let auth_keys = ssh_dir.join("authorized_keys");
    if auth_keys.exists() {
        if let Ok(content) = std::fs::read_to_string(&auth_keys) {
            let key_count = content.lines().filter(|l| !l.trim().is_empty() && !l.starts_with('#')).count();
            if key_count > 10 {
                findings.push(SecurityFinding {
                    severity: FindingSeverity::Medium,
                    category: "SSH".into(),
                    title: format!("{key_count} authorized SSH keys found"),
                    detail: "Large number of authorized keys increases attack surface".into(),
                    remediation: "Review and remove unused authorized keys".into(),
                });
            }
        }
    }
}

fn check_sensitive_files(findings: &mut Vec<SecurityFinding>) {
    let home = dirs::home_dir().unwrap_or_default();

    let sensitive = [
        (".env", "Environment file may contain secrets"),
        (".aws/credentials", "AWS credentials file"),
        (".netrc", "Network credentials file"),
        (".npmrc", "NPM config may contain tokens"),
        (".docker/config.json", "Docker config may contain registry creds"),
    ];

    for (path, description) in &sensitive {
        let full = home.join(path);
        if full.exists() {
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                if let Ok(meta) = std::fs::metadata(&full) {
                    let mode = meta.permissions().mode();
                    if mode & 0o077 != 0 {
                        findings.push(SecurityFinding {
                            severity: FindingSeverity::Medium,
                            category: "Credentials".into(),
                            title: format!("{path} has loose permissions"),
                            detail: format!("{description} — mode {:o}", mode & 0o777),
                            remediation: format!("chmod 600 {}", full.display()),
                        });
                    }
                }
            }
        }
    }
}

fn check_world_writable(findings: &mut Vec<SecurityFinding>) {
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let home = dirs::home_dir().unwrap_or_default();
        if let Ok(meta) = std::fs::metadata(&home) {
            let mode = meta.permissions().mode();
            if mode & 0o002 != 0 {
                findings.push(SecurityFinding {
                    severity: FindingSeverity::Critical,
                    category: "Filesystem".into(),
                    title: "Home directory is world-writable".into(),
                    detail: format!("Mode {:o}", mode & 0o777),
                    remediation: format!("chmod 755 {}", home.display()),
                });
            }
        }
    }
}

fn check_firewall(findings: &mut Vec<SecurityFinding>) {
    #[cfg(target_os = "macos")]
    {
        let output = std::process::Command::new("defaults")
            .args(["read", "/Library/Preferences/com.apple.alf", "globalstate"])
            .output();

        if let Ok(output) = output {
            let stdout = String::from_utf8_lossy(&output.stdout);
            if stdout.trim() == "0" {
                findings.push(SecurityFinding {
                    severity: FindingSeverity::Medium,
                    category: "Firewall".into(),
                    title: "macOS firewall is disabled".into(),
                    detail: "System firewall is not active".into(),
                    remediation: "Enable firewall in System Preferences > Security & Privacy".into(),
                });
            }
        }
    }

    #[cfg(target_os = "linux")]
    {
        let output = std::process::Command::new("ufw")
            .arg("status")
            .output();

        if let Ok(output) = output {
            let stdout = String::from_utf8_lossy(&output.stdout);
            if stdout.contains("inactive") {
                findings.push(SecurityFinding {
                    severity: FindingSeverity::Medium,
                    category: "Firewall".into(),
                    title: "UFW firewall is inactive".into(),
                    detail: "System firewall is not active".into(),
                    remediation: "sudo ufw enable".into(),
                });
            }
        }
    }
}

pub fn print_summary(report: &SecurityReport) {
    let score_color = match report.score {
        80..=100 => "\x1b[32m", // green
        50..=79 => "\x1b[33m",  // yellow
        _ => "\x1b[31m",        // red
    };
    println!("  Security Score: {score_color}{}/100\x1b[0m", report.score);
    println!("  Findings: {}", report.findings.len());

    for finding in &report.findings {
        let sev_color = match finding.severity {
            FindingSeverity::Info => "\x1b[36m",
            FindingSeverity::Low => "\x1b[34m",
            FindingSeverity::Medium => "\x1b[33m",
            FindingSeverity::High => "\x1b[31m",
            FindingSeverity::Critical => "\x1b[35m",
        };
        println!(
            "    {sev_color}[{}]\x1b[0m {} — {}",
            finding.severity, finding.title, finding.detail
        );
    }
}
