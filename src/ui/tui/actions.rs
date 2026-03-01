use ratatui::prelude::*;
use ratatui::widgets::*;

use super::app::App;

pub fn render(f: &mut Frame, area: Rect, app: &App) {
    let block = Block::default()
        .title(" Available Actions ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Green));

    if app.actions.is_empty() {
        let lines = vec![
            Line::from("No pending actions."),
            Line::from(""),
            Line::from("Built-in actions available via CLI:"),
            Line::from(Span::styled("  compscan act cleanup-caches", Style::default().fg(Color::Cyan))),
            Line::from(Span::styled("  compscan act cleanup-temp", Style::default().fg(Color::Cyan))),
            Line::from(Span::styled("  compscan act cleanup-logs", Style::default().fg(Color::Cyan))),
            Line::from(Span::styled("  compscan act cleanup-disk", Style::default().fg(Color::Cyan))),
        ];
        let paragraph = Paragraph::new(lines).block(block);
        f.render_widget(paragraph, area);
        return;
    }

    let rows: Vec<Row> = app
        .actions
        .iter()
        .map(|a| {
            let risk_style = match a.risk_level {
                crate::storage::models::RiskLevel::Low => Style::default().fg(Color::Green),
                crate::storage::models::RiskLevel::Medium => Style::default().fg(Color::Yellow),
                crate::storage::models::RiskLevel::High => Style::default().fg(Color::Red),
                crate::storage::models::RiskLevel::Critical => Style::default().fg(Color::Magenta),
            };

            Row::new(vec![
                Cell::from(a.id.clone()),
                Cell::from(format!("{}", a.risk_level)).style(risk_style),
                Cell::from(truncate(&a.title, 35)),
                Cell::from(&*a.estimated_impact),
            ])
        })
        .collect();

    let header = Row::new(vec![
        Cell::from("ID").style(Style::default().fg(Color::Cyan)),
        Cell::from("Risk").style(Style::default().fg(Color::Cyan)),
        Cell::from("Action").style(Style::default().fg(Color::Cyan)),
        Cell::from("Impact").style(Style::default().fg(Color::Cyan)),
    ]);

    let widths = [
        Constraint::Length(20),
        Constraint::Length(10),
        Constraint::Length(35),
        Constraint::Min(15),
    ];

    let table = Table::new(rows, widths)
        .header(header)
        .block(block)
        .row_highlight_style(Style::default().bg(Color::DarkGray));

    let mut state = TableState::default();
    state.select(Some(app.selected_index));
    f.render_stateful_widget(table, area, &mut state);
}

pub fn render_history(f: &mut Frame, area: Rect, app: &App) {
    let block = Block::default()
        .title(" Action History ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::DarkGray));

    if app.action_logs.is_empty() {
        let paragraph = Paragraph::new("No actions executed yet.")
            .block(block)
            .style(Style::default().fg(Color::DarkGray));
        f.render_widget(paragraph, area);
        return;
    }

    let rows: Vec<Row> = app
        .action_logs
        .iter()
        .map(|log| {
            let status_style = if log.success {
                Style::default().fg(Color::Green)
            } else {
                Style::default().fg(Color::Red)
            };

            Row::new(vec![
                Cell::from(log.timestamp.format("%Y-%m-%d %H:%M").to_string()),
                Cell::from(&*log.action_id),
                Cell::from(if log.success { "OK" } else { "FAIL" }).style(status_style),
                Cell::from(truncate(&log.output, 40)),
            ])
        })
        .collect();

    let header = Row::new(vec![
        Cell::from("Time").style(Style::default().fg(Color::Cyan)),
        Cell::from("Action").style(Style::default().fg(Color::Cyan)),
        Cell::from("Status").style(Style::default().fg(Color::Cyan)),
        Cell::from("Output").style(Style::default().fg(Color::Cyan)),
    ]);

    let widths = [
        Constraint::Length(18),
        Constraint::Length(20),
        Constraint::Length(6),
        Constraint::Min(15),
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
