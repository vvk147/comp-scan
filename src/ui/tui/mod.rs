pub mod actions;
pub mod app;
pub mod dashboard;
pub mod insights;
pub mod widgets;

use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::prelude::*;
use std::io;
use std::time::Duration;

use crate::storage::Database;
use app::{App, Tab};

pub async fn run_tui(db: &Database) -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new(db)?;
    app.refresh(db)?;

    let result = run_app(&mut terminal, &mut app, db).await;

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    result
}

async fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    app: &mut App,
    db: &Database,
) -> Result<()> {
    let tick_rate = Duration::from_millis(250);
    let mut last_refresh = std::time::Instant::now();
    let refresh_interval = Duration::from_secs(5);

    loop {
        terminal.draw(|f| render(f, app))?;

        if event::poll(tick_rate)? {
            if let Event::Key(key) = event::read()? {
                if key.kind != KeyEventKind::Press {
                    continue;
                }
                match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => return Ok(()),
                    KeyCode::Tab => app.next_tab(),
                    KeyCode::BackTab => app.prev_tab(),
                    KeyCode::Char('1') => app.current_tab = Tab::Overview,
                    KeyCode::Char('2') => app.current_tab = Tab::Insights,
                    KeyCode::Char('3') => app.current_tab = Tab::Actions,
                    KeyCode::Char('4') => app.current_tab = Tab::History,
                    KeyCode::Char('r') => app.refresh(db)?,
                    KeyCode::Up | KeyCode::Char('k') => app.scroll_up(),
                    KeyCode::Down | KeyCode::Char('j') => app.scroll_down(),
                    KeyCode::Enter => app.select_current(),
                    _ => {}
                }
            }
        }

        if last_refresh.elapsed() >= refresh_interval {
            app.refresh(db)?;
            last_refresh = std::time::Instant::now();
        }
    }
}

fn render(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // header
            Constraint::Min(0),    // content
            Constraint::Length(1), // footer
        ])
        .split(f.area());

    widgets::render_header(f, chunks[0], app);

    match app.current_tab {
        Tab::Overview => dashboard::render(f, chunks[1], app),
        Tab::Insights => insights::render(f, chunks[1], app),
        Tab::Actions => actions::render(f, chunks[1], app),
        Tab::History => actions::render_history(f, chunks[1], app),
    }

    widgets::render_footer(f, chunks[2]);
}
