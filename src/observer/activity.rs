use anyhow::Result;
use chrono::Utc;
use sysinfo::System;
use uuid::Uuid;

use crate::storage::models::{ActivityRecord, ProcessSnapshot};

pub fn collect_activity_snapshot() -> Result<ActivityRecord> {
    let mut sys = System::new();
    sys.refresh_all();
    std::thread::sleep(std::time::Duration::from_millis(200));
    sys.refresh_all();

    let mut processes: Vec<ProcessSnapshot> = sys
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

    let total_cpu: f32 = processes.iter().map(|p| p.cpu_usage).sum();
    let total_memory: u64 = sys.total_memory();
    let used_memory: u64 = sys.used_memory();

    processes.sort_by(|a, b| {
        b.cpu_usage
            .partial_cmp(&a.cpu_usage)
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    let top_cpu = processes.first().map(|p| p.name.clone()).unwrap_or_default();

    processes.sort_by(|a, b| b.memory_bytes.cmp(&a.memory_bytes));
    let top_mem = processes.first().map(|p| p.name.clone()).unwrap_or_default();

    let top_processes: Vec<ProcessSnapshot> = processes.into_iter().take(20).collect();
    let process_count = sys.processes().len();
    let cpu_count = sys.cpus().len().max(1) as f32;

    Ok(ActivityRecord {
        id: Uuid::new_v4().to_string(),
        timestamp: Utc::now(),
        active_processes: top_processes,
        cpu_usage_percent: total_cpu / cpu_count,
        memory_usage_percent: if total_memory > 0 {
            used_memory as f32 / total_memory as f32 * 100.0
        } else {
            0.0
        },
        top_cpu_process: top_cpu,
        top_memory_process: top_mem,
        process_count,
    })
}
