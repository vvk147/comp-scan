use anyhow::Result;
use bytesize::ByteSize;
use chrono::Utc;
use sysinfo::System;
use uuid::Uuid;

use crate::storage::models::{DiskInfo, SystemSnapshot};

pub fn collect_system_snapshot() -> Result<SystemSnapshot> {
    let mut sys = System::new_all();
    sys.refresh_all();

    let disks = sysinfo::Disks::new_with_refreshed_list();
    let disk_info: Vec<DiskInfo> = disks
        .iter()
        .map(|d| DiskInfo {
            name: d.name().to_string_lossy().to_string(),
            mount_point: d.mount_point().to_string_lossy().to_string(),
            total_bytes: d.total_space(),
            available_bytes: d.available_space(),
            filesystem: d.file_system().to_string_lossy().to_string(),
        })
        .collect();

    let cpus = sys.cpus();
    let cpu_brand = cpus
        .first()
        .map(|c| c.brand().to_string())
        .unwrap_or_else(|| "Unknown".into());

    Ok(SystemSnapshot {
        id: Uuid::new_v4().to_string(),
        timestamp: Utc::now(),
        hostname: hostname::get()
            .map(|h| h.to_string_lossy().to_string())
            .unwrap_or_else(|_| "unknown".into()),
        os_name: System::name().unwrap_or_else(|| "Unknown".into()),
        os_version: System::os_version().unwrap_or_else(|| "Unknown".into()),
        cpu_count: cpus.len(),
        cpu_brand,
        total_memory_bytes: sys.total_memory(),
        used_memory_bytes: sys.used_memory(),
        total_swap_bytes: sys.total_swap(),
        used_swap_bytes: sys.used_swap(),
        disks: disk_info,
        process_count: sys.processes().len(),
        uptime_secs: System::uptime(),
    })
}

pub fn print_summary(snapshot: &SystemSnapshot) {
    println!("  ╭─────────────────────────────────────────╮");
    println!("  │          System Overview                 │");
    println!("  ├─────────────────────────────────────────┤");
    println!("  │ Host:     {:<30}│", snapshot.hostname);
    println!(
        "  │ OS:       {:<30}│",
        format!("{} {}", snapshot.os_name, snapshot.os_version)
    );
    println!(
        "  │ CPU:      {:<30}│",
        format!(
            "{} ({} cores)",
            truncate(&snapshot.cpu_brand, 20),
            snapshot.cpu_count
        )
    );
    println!(
        "  │ Memory:   {:<30}│",
        format!(
            "{} / {}",
            ByteSize(snapshot.used_memory_bytes),
            ByteSize(snapshot.total_memory_bytes)
        )
    );
    println!(
        "  │ Swap:     {:<30}│",
        format!(
            "{} / {}",
            ByteSize(snapshot.used_swap_bytes),
            ByteSize(snapshot.total_swap_bytes)
        )
    );
    println!(
        "  │ Processes:{:<30}│",
        format!(" {}", snapshot.process_count)
    );
    println!("  │ Uptime:   {:<30}│", format_uptime(snapshot.uptime_secs));
    println!("  ├─────────────────────────────────────────┤");
    println!("  │ Disks:                                  │");
    for disk in &snapshot.disks {
        let used = disk.total_bytes.saturating_sub(disk.available_bytes);
        let pct = if disk.total_bytes > 0 {
            (used as f64 / disk.total_bytes as f64 * 100.0) as u8
        } else {
            0
        };
        let bar = progress_bar(pct as f32 / 100.0, 15);
        println!(
            "  │  {:<8} {bar} {:>3}% ({:>8}/{:>8}) │",
            truncate(&disk.mount_point, 8),
            pct,
            ByteSize(used),
            ByteSize(disk.total_bytes)
        );
    }
    println!("  ╰─────────────────────────────────────────╯");
}

fn format_uptime(secs: u64) -> String {
    let days = secs / 86400;
    let hours = (secs % 86400) / 3600;
    let mins = (secs % 3600) / 60;
    if days > 0 {
        format!("{days}d {hours}h {mins}m")
    } else if hours > 0 {
        format!("{hours}h {mins}m")
    } else {
        format!("{mins}m")
    }
}

fn truncate(s: &str, max: usize) -> String {
    if s.len() > max {
        format!("{}...", &s[..max.saturating_sub(3)])
    } else {
        s.to_string()
    }
}

fn progress_bar(fraction: f32, width: usize) -> String {
    let filled = (fraction * width as f32).round() as usize;
    let empty = width.saturating_sub(filled);
    format!("[{}{}]", "█".repeat(filled), "░".repeat(empty))
}
