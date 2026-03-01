use ratatui::prelude::*;
use ratatui::widgets::*;

use super::app::App;

pub fn render(f: &mut Frame, area: Rect, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(8),  // system info
            Constraint::Length(8),  // resource bars
            Constraint::Min(5),    // recent activity
        ])
        .split(area);

    render_system_info(f, chunks[0], app);
    render_resources(f, chunks[1], app);
    render_recent_activity(f, chunks[2], app);
}

fn render_system_info(f: &mut Frame, area: Rect, app: &App) {
    let info = if let Some(ref snap) = app.snapshot {
        vec![
            Line::from(vec![
                Span::styled("Host:     ", Style::default().fg(Color::Cyan)),
                Span::raw(&snap.hostname),
            ]),
            Line::from(vec![
                Span::styled("OS:       ", Style::default().fg(Color::Cyan)),
                Span::raw(format!("{} {}", snap.os_name, snap.os_version)),
            ]),
            Line::from(vec![
                Span::styled("CPU:      ", Style::default().fg(Color::Cyan)),
                Span::raw(format!("{} ({} cores)", snap.cpu_brand, snap.cpu_count)),
            ]),
            Line::from(vec![
                Span::styled("Uptime:   ", Style::default().fg(Color::Cyan)),
                Span::raw(format_uptime(snap.uptime_secs)),
            ]),
            Line::from(vec![
                Span::styled("Processes:", Style::default().fg(Color::Cyan)),
                Span::raw(format!(" {}", snap.process_count)),
            ]),
            Line::from(vec![
                Span::styled("Tracked:  ", Style::default().fg(Color::Cyan)),
                Span::raw(format!("{} activities, {} insights", app.activity_count, app.insight_count)),
            ]),
        ]
    } else {
        vec![
            Line::from(Span::styled(
                "No system snapshot available. Run `compscan scan` first.",
                Style::default().fg(Color::Yellow),
            )),
        ]
    };

    let block = Block::default()
        .title(" System ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Cyan));

    let paragraph = Paragraph::new(info).block(block);
    f.render_widget(paragraph, area);
}

fn render_resources(f: &mut Frame, area: Rect, app: &App) {
    let block = Block::default()
        .title(" Resources ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Green));

    if let Some(ref snap) = app.snapshot {
        let mem_pct = snap.used_memory_bytes as f64 / snap.total_memory_bytes.max(1) as f64;
        let swap_pct = if snap.total_swap_bytes > 0 {
            snap.used_swap_bytes as f64 / snap.total_swap_bytes as f64
        } else {
            0.0
        };

        let mut gauges = vec![
            Line::from(vec![
                Span::styled("Memory: ", Style::default().fg(Color::Yellow)),
                Span::raw(format!("{:.0}% ", mem_pct * 100.0)),
                Span::raw(make_bar(mem_pct, 30)),
                Span::raw(format!(
                    " {:.1}GB / {:.1}GB",
                    snap.used_memory_bytes as f64 / 1e9,
                    snap.total_memory_bytes as f64 / 1e9
                )),
            ]),
            Line::from(vec![
                Span::styled("Swap:   ", Style::default().fg(Color::Magenta)),
                Span::raw(format!("{:.0}% ", swap_pct * 100.0)),
                Span::raw(make_bar(swap_pct, 30)),
            ]),
        ];

        for disk in snap.disks.iter().take(3) {
            let used = disk.total_bytes.saturating_sub(disk.available_bytes);
            let pct = used as f64 / disk.total_bytes.max(1) as f64;
            gauges.push(Line::from(vec![
                Span::styled(
                    format!("{:<8}", truncate(&disk.mount_point, 8)),
                    Style::default().fg(Color::Blue),
                ),
                Span::raw(format!("{:.0}% ", pct * 100.0)),
                Span::raw(make_bar(pct, 30)),
                Span::raw(format!(
                    " {:.1}GB free",
                    disk.available_bytes as f64 / 1e9
                )),
            ]));
        }

        let paragraph = Paragraph::new(gauges).block(block);
        f.render_widget(paragraph, area);
    } else {
        let paragraph = Paragraph::new("No data")
            .block(block)
            .style(Style::default().fg(Color::DarkGray));
        f.render_widget(paragraph, area);
    }
}

fn render_recent_activity(f: &mut Frame, area: Rect, app: &App) {
    let block = Block::default()
        .title(" Recent Activity ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Yellow));

    if app.activities.is_empty() {
        let paragraph = Paragraph::new("No activity recorded yet. Run `compscan observe` to start tracking.")
            .block(block)
            .style(Style::default().fg(Color::DarkGray));
        f.render_widget(paragraph, area);
        return;
    }

    let rows: Vec<Row> = app
        .activities
        .iter()
        .take(20)
        .map(|a| {
            Row::new(vec![
                Cell::from(a.timestamp.format("%H:%M:%S").to_string()),
                Cell::from(format!("{:.0}%", a.cpu_usage_percent)),
                Cell::from(format!("{:.0}%", a.memory_usage_percent)),
                Cell::from(format!("{}", a.process_count)),
                Cell::from(truncate(&a.top_cpu_process, 20)),
            ])
        })
        .collect();

    let header = Row::new(vec![
        Cell::from("Time").style(Style::default().fg(Color::Cyan)),
        Cell::from("CPU").style(Style::default().fg(Color::Cyan)),
        Cell::from("Mem").style(Style::default().fg(Color::Cyan)),
        Cell::from("Procs").style(Style::default().fg(Color::Cyan)),
        Cell::from("Top CPU").style(Style::default().fg(Color::Cyan)),
    ]);

    let widths = [
        Constraint::Length(10),
        Constraint::Length(6),
        Constraint::Length(6),
        Constraint::Length(7),
        Constraint::Min(10),
    ];

    let table = Table::new(rows, widths)
        .header(header)
        .block(block)
        .row_highlight_style(Style::default().bg(Color::DarkGray));

    let mut state = TableState::default();
    state.select(Some(app.selected_index));
    f.render_stateful_widget(table, area, &mut state);
}

fn make_bar(fraction: f64, width: usize) -> String {
    let filled = (fraction * width as f64).round() as usize;
    let empty = width.saturating_sub(filled);
    format!("[{}{}]", "█".repeat(filled), "░".repeat(empty))
}

fn format_uptime(secs: u64) -> String {
    let days = secs / 86400;
    let hours = (secs % 86400) / 3600;
    let mins = (secs % 3600) / 60;
    if days > 0 {
        format!("{days}d {hours}h {mins}m")
    } else {
        format!("{hours}h {mins}m")
    }
}

fn truncate(s: &str, max: usize) -> String {
    if s.len() > max {
        format!("{}...", &s[..max.saturating_sub(3)])
    } else {
        s.to_string()
    }
}
