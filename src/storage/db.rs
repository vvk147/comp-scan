use anyhow::{Context, Result};
use redb::{Database as RedbDatabase, ReadableTable, ReadableTableMetadata, TableDefinition};
use std::path::PathBuf;
use std::sync::Arc;

use super::models::*;

const CONFIG_TABLE: TableDefinition<&str, &[u8]> = TableDefinition::new("config");
const SNAPSHOTS_TABLE: TableDefinition<&str, &[u8]> = TableDefinition::new("snapshots");
const ACTIVITIES_TABLE: TableDefinition<&str, &[u8]> = TableDefinition::new("activities");
const INSIGHTS_TABLE: TableDefinition<&str, &[u8]> = TableDefinition::new("insights");
const ACTIONS_TABLE: TableDefinition<&str, &[u8]> = TableDefinition::new("actions");
const ACTION_LOGS_TABLE: TableDefinition<&str, &[u8]> = TableDefinition::new("action_logs");

#[derive(Clone)]
pub struct Database {
    inner: Arc<RedbDatabase>,
    pub path: PathBuf,
}

impl Database {
    pub fn open() -> Result<Self> {
        let data_dir = dirs::data_local_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("compscan");
        std::fs::create_dir_all(&data_dir).context("Failed to create data directory")?;

        let db_path = data_dir.join("compscan.redb");
        let db = RedbDatabase::create(&db_path).context("Failed to open database")?;

        let instance = Self {
            inner: Arc::new(db),
            path: db_path,
        };
        instance.init_tables()?;
        Ok(instance)
    }

    fn init_tables(&self) -> Result<()> {
        let tx = self.inner.begin_write()?;
        {
            let _ = tx.open_table(CONFIG_TABLE)?;
            let _ = tx.open_table(SNAPSHOTS_TABLE)?;
            let _ = tx.open_table(ACTIVITIES_TABLE)?;
            let _ = tx.open_table(INSIGHTS_TABLE)?;
            let _ = tx.open_table(ACTIONS_TABLE)?;
            let _ = tx.open_table(ACTION_LOGS_TABLE)?;
        }
        tx.commit()?;
        Ok(())
    }

    pub fn get_config(&self) -> Result<AppConfig> {
        let tx = self.inner.begin_read()?;
        let table = tx.open_table(CONFIG_TABLE)?;
        match table.get("app_config")? {
            Some(data) => Ok(serde_json::from_slice(data.value())?),
            None => Ok(AppConfig::default()),
        }
    }

    pub fn save_config(&self, config: &AppConfig) -> Result<()> {
        let tx = self.inner.begin_write()?;
        {
            let mut table = tx.open_table(CONFIG_TABLE)?;
            let data = serde_json::to_vec(config)?;
            table.insert("app_config", data.as_slice())?;
        }
        tx.commit()?;
        Ok(())
    }

    pub fn clear_config(&self) -> Result<()> {
        self.save_config(&AppConfig::default())
    }

    pub fn save_snapshot(&self, snapshot: &SystemSnapshot) -> Result<()> {
        let tx = self.inner.begin_write()?;
        {
            let mut table = tx.open_table(SNAPSHOTS_TABLE)?;
            let data = serde_json::to_vec(snapshot)?;
            table.insert(snapshot.id.as_str(), data.as_slice())?;
        }
        tx.commit()?;
        Ok(())
    }

    pub fn get_latest_snapshot(&self) -> Result<Option<SystemSnapshot>> {
        let tx = self.inner.begin_read()?;
        let table = tx.open_table(SNAPSHOTS_TABLE)?;
        let mut latest: Option<SystemSnapshot> = None;
        let iter = table.iter()?;
        for entry in iter {
            let entry = entry?;
            let snapshot: SystemSnapshot = serde_json::from_slice(entry.1.value())?;
            match &latest {
                None => latest = Some(snapshot),
                Some(current) if snapshot.timestamp > current.timestamp => {
                    latest = Some(snapshot);
                }
                _ => {}
            }
        }
        Ok(latest)
    }

    pub fn save_activity(&self, activity: &ActivityRecord) -> Result<()> {
        let tx = self.inner.begin_write()?;
        {
            let mut table = tx.open_table(ACTIVITIES_TABLE)?;
            let data = serde_json::to_vec(activity)?;
            table.insert(activity.id.as_str(), data.as_slice())?;
        }
        tx.commit()?;
        Ok(())
    }

    pub fn get_recent_activities(&self, limit: usize) -> Result<Vec<ActivityRecord>> {
        let tx = self.inner.begin_read()?;
        let table = tx.open_table(ACTIVITIES_TABLE)?;
        let mut activities: Vec<ActivityRecord> = Vec::new();
        let iter = table.iter()?;
        for entry in iter {
            let entry = entry?;
            let record: ActivityRecord = serde_json::from_slice(entry.1.value())?;
            activities.push(record);
        }
        activities.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
        activities.truncate(limit);
        Ok(activities)
    }

    pub fn save_insight(&self, insight: &Insight) -> Result<()> {
        let tx = self.inner.begin_write()?;
        {
            let mut table = tx.open_table(INSIGHTS_TABLE)?;
            let data = serde_json::to_vec(insight)?;
            table.insert(insight.id.as_str(), data.as_slice())?;
        }
        tx.commit()?;
        Ok(())
    }

    pub fn get_all_insights(&self) -> Result<Vec<Insight>> {
        let tx = self.inner.begin_read()?;
        let table = tx.open_table(INSIGHTS_TABLE)?;
        let mut insights: Vec<Insight> = Vec::new();
        let iter = table.iter()?;
        for entry in iter {
            let entry = entry?;
            let insight: Insight = serde_json::from_slice(entry.1.value())?;
            insights.push(insight);
        }
        insights.sort_by(|a, b| b.severity.cmp(&a.severity));
        Ok(insights)
    }

    pub fn save_action(&self, action: &Action) -> Result<()> {
        let tx = self.inner.begin_write()?;
        {
            let mut table = tx.open_table(ACTIONS_TABLE)?;
            let data = serde_json::to_vec(action)?;
            table.insert(action.id.as_str(), data.as_slice())?;
        }
        tx.commit()?;
        Ok(())
    }

    pub fn get_action(&self, id: &str) -> Result<Option<Action>> {
        let tx = self.inner.begin_read()?;
        let table = tx.open_table(ACTIONS_TABLE)?;
        match table.get(id)? {
            Some(data) => Ok(Some(serde_json::from_slice(data.value())?)),
            None => Ok(None),
        }
    }

    pub fn get_all_actions(&self) -> Result<Vec<Action>> {
        let tx = self.inner.begin_read()?;
        let table = tx.open_table(ACTIONS_TABLE)?;
        let mut actions: Vec<Action> = Vec::new();
        let iter = table.iter()?;
        for entry in iter {
            let entry = entry?;
            let action: Action = serde_json::from_slice(entry.1.value())?;
            actions.push(action);
        }
        Ok(actions)
    }

    pub fn log_action_execution(&self, log: &ActionLog) -> Result<()> {
        let tx = self.inner.begin_write()?;
        {
            let mut table = tx.open_table(ACTION_LOGS_TABLE)?;
            let data = serde_json::to_vec(log)?;
            table.insert(log.id.as_str(), data.as_slice())?;
        }
        tx.commit()?;
        Ok(())
    }

    pub fn get_action_logs(&self) -> Result<Vec<ActionLog>> {
        let tx = self.inner.begin_read()?;
        let table = tx.open_table(ACTION_LOGS_TABLE)?;
        let mut logs: Vec<ActionLog> = Vec::new();
        let iter = table.iter()?;
        for entry in iter {
            let entry = entry?;
            let log: ActionLog = serde_json::from_slice(entry.1.value())?;
            logs.push(log);
        }
        logs.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
        Ok(logs)
    }

    pub fn activity_count(&self) -> Result<usize> {
        let tx = self.inner.begin_read()?;
        let table = tx.open_table(ACTIVITIES_TABLE)?;
        Ok(table.len()? as usize)
    }

    pub fn insight_count(&self) -> Result<usize> {
        let tx = self.inner.begin_read()?;
        let table = tx.open_table(INSIGHTS_TABLE)?;
        Ok(table.len()? as usize)
    }
}
