use ratatui::prelude::*;
use ratatui::widgets::*;

use super::app::{App, Tab};

pub fn render_header(f: &mut Frame, area: Rect, app: &App) {
    let titles: Vec<Line> = Tab::ALL
        .iter()
        .map(|t| {
            let style = if *t == app.current_tab {
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD)
                    .add_modifier(Modifier::UNDERLINED)
            } else {
                Style::default().fg(Color::DarkGray)
            };
            Line::from(Span::styled(
                format!(" {} {} ", t.index() + 1, t.title()),
                style,
            ))
        })
        .collect();

    let tabs = Tabs::new(titles)
        .block(
            Block::default()
                .title(format!(
                    " CompScan v{} ",
                    env!("CARGO_PKG_VERSION")
                ))
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::White)),
        )
        .select(app.current_tab.index())
        .style(Style::default())
        .highlight_style(Style::default().fg(Color::Cyan));

    f.render_widget(tabs, area);
}

pub fn render_footer(f: &mut Frame, area: Rect) {
    let footer = Paragraph::new(Line::from(vec![
        Span::styled(" q", Style::default().fg(Color::Red)),
        Span::raw(":quit "),
        Span::styled("Tab", Style::default().fg(Color::Cyan)),
        Span::raw(":switch "),
        Span::styled("j/k", Style::default().fg(Color::Cyan)),
        Span::raw(":navigate "),
        Span::styled("Enter", Style::default().fg(Color::Green)),
        Span::raw(":select "),
        Span::styled("r", Style::default().fg(Color::Yellow)),
        Span::raw(":refresh"),
    ]))
    .style(Style::default().bg(Color::DarkGray));

    f.render_widget(footer, area);
}
