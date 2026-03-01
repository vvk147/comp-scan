use anyhow::Result;
use bytesize::ByteSize;
use sysinfo::System;

use crate::storage::models::ProcessSnapshot;

pub struct ProcessReport {
    pub total_count: usize,
    pub top_by_cpu: Vec<ProcessSnapshot>,
    pub top_by_memory: Vec<ProcessSnapshot>,
    pub resource_hogs: Vec<ProcessSnapshot>,
    pub zombie_count: usize,
}

pub fn analyze_processes() -> Result<ProcessReport> {
    let mut sys = System::new_all();
    sys.refresh_all();
    std::thread::sleep(std::time::Duration::from_millis(500));
    sys.refresh_all();

    let mut all: Vec<ProcessSnapshot> = sys
        .processes()
        .values()
        .map(|p| ProcessSnapshot {
            name: p.name().to_string_lossy().to_string(),
            pid: p.pid().as_u32(),
            cpu_usage: p.cpu_usage(),
            memory_bytes: p.memory(),
            disk_read_bytes: p.disk_usage().read_bytes,
            disk_write_bytes: p.disk_usage().written_bytes,
            start_time: p.start_time(),
            command: p.cmd().iter().map(|s| s.to_string_lossy().to_string()).collect::<Vec<_>>().join(" "),
        })
        .collect();

    let total_count = all.len();

    let zombie_count = sys
        .processes()
        .values()
        .filter(|p| matches!(p.status(), sysinfo::ProcessStatus::Dead))
        .count();

    all.sort_by(|a, b| b.cpu_usage.partial_cmp(&a.cpu_usage).unwrap_or(std::cmp::Ordering::Equal));
    let top_by_cpu: Vec<ProcessSnapshot> = all.iter().take(10).cloned().collect();

    all.sort_by(|a, b| b.memory_bytes.cmp(&a.memory_bytes));
    let top_by_memory: Vec<ProcessSnapshot> = all.iter().take(10).cloned().collect();

    let resource_hogs: Vec<ProcessSnapshot> = all
        .iter()
        .filter(|p| p.cpu_usage > 50.0 || p.memory_bytes > 1_000_000_000)
        .cloned()
        .collect();

    Ok(ProcessReport {
        total_count,
        top_by_cpu,
        top_by_memory,
        resource_hogs,
        zombie_count,
    })
}

pub fn print_summary(report: &ProcessReport) {
    println!("  Total processes: {}", report.total_count);
    if report.zombie_count > 0 {
        println!("  Zombie processes: {} (potential cleanup)", report.zombie_count);
    }

    if !report.resource_hogs.is_empty() {
        println!("  Resource hogs detected:");
        for p in &report.resource_hogs {
            println!(
                "    - {} (PID {}) | CPU: {:.1}% | Mem: {}",
                p.name, p.pid, p.cpu_usage, ByteSize(p.memory_bytes)
            );
        }
    }

    println!("\n  Top by CPU:");
    for (i, p) in report.top_by_cpu.iter().take(5).enumerate() {
        println!(
            "    {}. {:<25} CPU: {:>6.1}%  Mem: {:>10}",
            i + 1,
            truncate_name(&p.name, 25),
            p.cpu_usage,
            ByteSize(p.memory_bytes)
        );
    }

    println!("\n  Top by Memory:");
    for (i, p) in report.top_by_memory.iter().take(5).enumerate() {
        println!(
            "    {}. {:<25} Mem: {:>10}  CPU: {:>6.1}%",
            i + 1,
            truncate_name(&p.name, 25),
            ByteSize(p.memory_bytes),
            p.cpu_usage,
        );
    }
}

fn truncate_name(s: &str, max: usize) -> String {
    if s.len() > max {
        format!("{}...", &s[..max.saturating_sub(3)])
    } else {
        s.to_string()
    }
}
