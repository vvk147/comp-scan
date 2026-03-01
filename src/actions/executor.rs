use anyhow::{Context, Result};
use std::time::Duration;

use crate::storage::models::{Action, ActionCommand};

const ACTION_TIMEOUT: Duration = Duration::from_secs(60);

pub async fn run_action(action: &Action) -> Result<String> {
    match &action.command {
        ActionCommand::ShellCommand(cmd) => run_shell_command(cmd).await,
        ActionCommand::KillProcess(pid) => kill_process(*pid),
        ActionCommand::DeleteFiles(paths) => delete_files(paths),
        ActionCommand::DisableStartupItem(name) => disable_startup(name),
        ActionCommand::Custom(description) => {
            Ok(format!("Custom action noted: {description}. Manual execution required."))
        }
    }
}

async fn run_shell_command(cmd: &str) -> Result<String> {
    let output = tokio::time::timeout(ACTION_TIMEOUT, async {
        tokio::process::Command::new("sh")
            .args(["-c", cmd])
            .output()
            .await
    })
    .await
    .context("Action timed out")?
    .context("Failed to execute command")?;

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();

    if output.status.success() {
        Ok(if stdout.is_empty() {
            "Completed successfully".into()
        } else {
            stdout.trim().to_string()
        })
    } else {
        anyhow::bail!("Command failed: {stderr}")
    }
}

fn kill_process(pid: u32) -> Result<String> {
    let sys = sysinfo::System::new_all();
    let spid = sysinfo::Pid::from_u32(pid);
    if let Some(process) = sys.process(spid) {
        let name = process.name().to_string_lossy().to_string();
        if process.kill() {
            Ok(format!("Killed process {name} (PID {pid})"))
        } else {
            anyhow::bail!("Failed to kill process {name} (PID {pid})")
        }
    } else {
        anyhow::bail!("Process with PID {pid} not found")
    }
}

fn delete_files(paths: &[std::path::PathBuf]) -> Result<String> {
    let mut deleted = 0usize;
    let mut total_freed = 0u64;

    for path in paths {
        if path.exists() {
            if let Ok(meta) = std::fs::metadata(path) {
                total_freed += meta.len();
            }
            if path.is_dir() {
                std::fs::remove_dir_all(path)?;
            } else {
                std::fs::remove_file(path)?;
            }
            deleted += 1;
        }
    }

    Ok(format!(
        "Deleted {deleted} items, freed {:.1} MB",
        total_freed as f64 / 1_048_576.0
    ))
}

fn disable_startup(name: &str) -> Result<String> {
    #[cfg(target_os = "macos")]
    {
        let output = std::process::Command::new("launchctl")
            .args(["unload", "-w", name])
            .output()
            .context("Failed to disable startup item")?;

        if output.status.success() {
            return Ok(format!("Disabled startup item: {name}"));
        }
    }

    #[cfg(target_os = "linux")]
    {
        let output = std::process::Command::new("systemctl")
            .args(["--user", "disable", name])
            .output()
            .context("Failed to disable startup item")?;

        if output.status.success() {
            return Ok(format!("Disabled startup item: {name}"));
        }
    }

    Ok(format!(
        "Could not auto-disable {name}. Manual intervention required."
    ))
}
