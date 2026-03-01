use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub observer_interval_secs: u64,
    pub ollama_endpoint: String,
    pub ollama_model: String,
    pub web_port: u16,
    pub encryption_enabled: bool,
    pub data_dir: PathBuf,
}

impl Default for AppConfig {
    fn default() -> Self {
        let data_dir = dirs::data_local_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("compscan");
        Self {
            observer_interval_secs: 30,
            ollama_endpoint: "http://127.0.0.1:11434".into(),
            ollama_model: "llama3.2".into(),
            web_port: 7890,
            encryption_enabled: false,
            data_dir,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RiskLevel {
    Low,
    Medium,
    High,
    Critical,
}

impl std::fmt::Display for RiskLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Low => write!(f, "Low"),
            Self::Medium => write!(f, "Medium"),
            Self::High => write!(f, "High"),
            Self::Critical => write!(f, "Critical"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemSnapshot {
    pub id: String,
    pub timestamp: DateTime<Utc>,
    pub hostname: String,
    pub os_name: String,
    pub os_version: String,
    pub cpu_count: usize,
    pub cpu_brand: String,
    pub total_memory_bytes: u64,
    pub used_memory_bytes: u64,
    pub total_swap_bytes: u64,
    pub used_swap_bytes: u64,
    pub disks: Vec<DiskInfo>,
    pub process_count: usize,
    pub uptime_secs: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiskInfo {
    pub name: String,
    pub mount_point: String,
    pub total_bytes: u64,
    pub available_bytes: u64,
    pub filesystem: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessSnapshot {
    pub name: String,
    pub pid: u32,
    pub cpu_usage: f32,
    pub memory_bytes: u64,
    pub disk_read_bytes: u64,
    pub disk_write_bytes: u64,
    pub start_time: u64,
    pub command: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivityRecord {
    pub id: String,
    pub timestamp: DateTime<Utc>,
    pub active_processes: Vec<ProcessSnapshot>,
    pub cpu_usage_percent: f32,
    pub memory_usage_percent: f32,
    pub top_cpu_process: String,
    pub top_memory_process: String,
    pub process_count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Insight {
    pub id: String,
    pub timestamp: DateTime<Utc>,
    pub category: InsightCategory,
    pub severity: InsightSeverity,
    pub title: String,
    pub description: String,
    pub suggestion: String,
    pub action_id: Option<String>,
    pub source: InsightSource,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InsightCategory {
    Performance,
    Security,
    Productivity,
    DiskSpace,
    Memory,
    Habits,
    Coding,
    Network,
}

impl std::fmt::Display for InsightCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum InsightSeverity {
    Info,
    Suggestion,
    Warning,
    Critical,
}

impl std::fmt::Display for InsightSeverity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum InsightSource {
    RuleEngine,
    Statistical,
    Ollama,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Action {
    pub id: String,
    pub title: String,
    pub description: String,
    pub risk_level: RiskLevel,
    pub category: InsightCategory,
    pub command: ActionCommand,
    pub reversible: bool,
    pub estimated_impact: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ActionCommand {
    ShellCommand(String),
    KillProcess(u32),
    DeleteFiles(Vec<PathBuf>),
    DisableStartupItem(String),
    Custom(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionLog {
    pub id: String,
    pub action_id: String,
    pub timestamp: DateTime<Utc>,
    pub approved: bool,
    pub executed: bool,
    pub success: bool,
    pub output: String,
}
