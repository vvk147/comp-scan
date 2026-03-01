use ratatui::prelude::*;
use ratatui::widgets::*;

use super::app::App;
use crate::storage::models::*;

pub fn render(f: &mut Frame, area: Rect, app: &App) {
    let block = Block::default()
        .title(" Insights ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Magenta));

    if app.insights.is_empty() {
        let paragraph = Paragraph::new("No insights yet. Run `compscan scan` or `compscan report` to generate insights.")
            .block(block)
            .style(Style::default().fg(Color::DarkGray));
        f.render_widget(paragraph, area);
        return;
    }

    let rows: Vec<Row> = app
        .insights
        .iter()
        .map(|i| {
            let severity_style = match i.severity {
                InsightSeverity::Critical => Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
                InsightSeverity::Warning => Style::default().fg(Color::Yellow),
                InsightSeverity::Suggestion => Style::default().fg(Color::Cyan),
                InsightSeverity::Info => Style::default().fg(Color::Blue),
            };

            let source = match i.source {
                InsightSource::RuleEngine => "rule",
                InsightSource::Statistical => "stat",
                InsightSource::Ollama => "ai",
            };

            Row::new(vec![
                Cell::from(format!("{}", i.severity)).style(severity_style),
                Cell::from(format!("{}", i.category)),
                Cell::from(source),
                Cell::from(truncate(&i.title, 40)),
                Cell::from(if i.action_id.is_some() { "Y" } else { "-" }),
            ])
        })
        .collect();

    let header = Row::new(vec![
        Cell::from("Sev").style(Style::default().fg(Color::Cyan)),
        Cell::from("Category").style(Style::default().fg(Color::Cyan)),
        Cell::from("Src").style(Style::default().fg(Color::Cyan)),
        Cell::from("Title").style(Style::default().fg(Color::Cyan)),
        Cell::from("Fix").style(Style::default().fg(Color::Cyan)),
    ]);

    let widths = [
        Constraint::Length(12),
        Constraint::Length(14),
        Constraint::Length(5),
        Constraint::Min(20),
        Constraint::Length(4),
    ];

    let table = Table::new(rows, widths)
        .header(header)
        .block(block)
        .row_highlight_style(Style::default().bg(Color::DarkGray));

    let mut state = TableState::default();
    state.select(Some(app.selected_index));
    f.render_stateful_widget(table, area, &mut state);
}

fn truncate(s: &str, max: usize) -> String {
    if s.len() > max {
        format!("{}...", &s[..max.saturating_sub(3)])
    } else {
        s.to_string()
    }
}
