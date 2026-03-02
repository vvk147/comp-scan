use crate::storage::models::*;
use crate::storage::Database;
use anyhow::Result;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tab {
    Overview,
    Insights,
    Actions,
    History,
}

impl Tab {
    pub const ALL: [Tab; 4] = [Tab::Overview, Tab::Insights, Tab::Actions, Tab::History];

    pub fn title(&self) -> &str {
        match self {
            Tab::Overview => "Overview",
            Tab::Insights => "Insights",
            Tab::Actions => "Actions",
            Tab::History => "History",
        }
    }

    pub fn index(&self) -> usize {
        match self {
            Tab::Overview => 0,
            Tab::Insights => 1,
            Tab::Actions => 2,
            Tab::History => 3,
        }
    }
}

pub struct App {
    pub current_tab: Tab,
    pub snapshot: Option<SystemSnapshot>,
    pub activities: Vec<ActivityRecord>,
    pub insights: Vec<Insight>,
    pub actions: Vec<Action>,
    pub action_logs: Vec<ActionLog>,
    pub scroll_offset: usize,
    pub selected_index: usize,
    pub activity_count: usize,
    pub insight_count: usize,
}

impl App {
    pub fn new(_db: &Database) -> Result<Self> {
        Ok(Self {
            current_tab: Tab::Overview,
            snapshot: None,
            activities: Vec::new(),
            insights: Vec::new(),
            actions: Vec::new(),
            action_logs: Vec::new(),
            scroll_offset: 0,
            selected_index: 0,
            activity_count: 0,
            insight_count: 0,
        })
    }

    pub fn refresh(&mut self, db: &Database) -> Result<()> {
        self.snapshot = db.get_latest_snapshot()?;
        self.activities = db.get_recent_activities(50)?;
        self.insights = db.get_all_insights()?;
        self.actions = db.get_all_actions()?;
        self.action_logs = db.get_action_logs()?;
        self.activity_count = db.activity_count()?;
        self.insight_count = db.insight_count()?;
        Ok(())
    }

    pub fn next_tab(&mut self) {
        let idx = self.current_tab.index();
        self.current_tab = Tab::ALL[(idx + 1) % Tab::ALL.len()];
        self.scroll_offset = 0;
        self.selected_index = 0;
    }

    pub fn prev_tab(&mut self) {
        let idx = self.current_tab.index();
        self.current_tab = Tab::ALL[(idx + Tab::ALL.len() - 1) % Tab::ALL.len()];
        self.scroll_offset = 0;
        self.selected_index = 0;
    }

    pub fn scroll_up(&mut self) {
        if self.selected_index > 0 {
            self.selected_index -= 1;
        }
    }

    pub fn scroll_down(&mut self) {
        let max = match self.current_tab {
            Tab::Insights => self.insights.len().saturating_sub(1),
            Tab::Actions => self.actions.len().saturating_sub(1),
            Tab::History => self.action_logs.len().saturating_sub(1),
            Tab::Overview => self.activities.len().saturating_sub(1),
        };
        if self.selected_index < max {
            self.selected_index += 1;
        }
    }

    pub fn select_current(&mut self) {
        // placeholder for action execution from TUI
    }
}
