mod cli;
mod scanner;
mod observer;
mod analyzer;
mod ai;
mod actions;
mod storage;
mod ui;
mod daemon;

use anyhow::Result;
use clap::Parser;
use tracing_subscriber::{fmt, EnvFilter};

use cli::{Cli, Commands};
use storage::Database;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const BANNER: &str = r#"
   ___                      ____
  / __\___  _ __ ___  _ __ / ___|  ___ __ _ _ __
 / /  / _ \| '_ ` _ \| '_ \___ \ / __/ _` | '_ \
/ /__| (_) | | | | | | |_) |__) | (_| (_| | | | |
\____/\___/|_| |_| |_| .__/____/ \___\__,_|_| |_|
                      |_|
"#;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| EnvFilter::new(if cli.verbose { "debug" } else { "info" })),
        )
        .with_target(false)
        .init();

    let db = Database::open()?;

    let is_first_run = db.activity_count().unwrap_or(0) == 0
        && db.insight_count().unwrap_or(0) == 0;

    match cli.command {
        Commands::Scan { full } => {
            println!("{BANNER}");
            println!("  CompScan v{VERSION} — Local AI Agent\n");
            if is_first_run {
                print_welcome();
            }
            scanner::run_scan(&db, full).await?;
            print_next_steps("scan");
        }
        Commands::Observe { interval } => {
            println!("{BANNER}");
            println!("  Starting observer daemon (interval: {interval}s)...\n");
            observer::run_observer(&db, interval).await?;
        }
        Commands::Dashboard => {
            ui::tui::run_tui(&db).await?;
        }
        Commands::Web { port } => {
            println!("{BANNER}");
            println!("  Web dashboard: http://localhost:{port}\n");
            ui::web::run_web_server(&db, port).await?;
        }
        Commands::Report { json } => {
            analyzer::generate_report(&db, json).await?;
            if !json {
                print_next_steps("report");
            }
        }
        Commands::Act { action_id, force } => {
            actions::execute_action(&db, &action_id, force).await?;
            print_next_steps("act");
        }
        Commands::Status => {
            println!("{BANNER}");
            println!("  CompScan v{VERSION}\n");
            daemon::show_status(&db).await?;
            print_next_steps("status");
        }
        Commands::Config { reset } => {
            if reset {
                storage::reset_config(&db)?;
                println!("Configuration reset to defaults.");
            } else {
                storage::show_config(&db)?;
            }
        }
    }

    Ok(())
}

fn print_welcome() {
    println!("  \x1b[36m┌─────────────────────────────────────────────────┐\x1b[0m");
    println!("  \x1b[36m│\x1b[0m  \x1b[1mWelcome to CompScan!\x1b[0m                            \x1b[36m│\x1b[0m");
    println!("  \x1b[36m│\x1b[0m                                                 \x1b[36m│\x1b[0m");
    println!("  \x1b[36m│\x1b[0m  This is your first scan. CompScan will:        \x1b[36m│\x1b[0m");
    println!("  \x1b[36m│\x1b[0m    1. Check your hardware & system health       \x1b[36m│\x1b[0m");
    println!("  \x1b[36m│\x1b[0m    2. Find resource-hungry processes            \x1b[36m│\x1b[0m");
    println!("  \x1b[36m│\x1b[0m    3. Discover reclaimable disk space           \x1b[36m│\x1b[0m");
    println!("  \x1b[36m│\x1b[0m    4. List startup items slowing your boot      \x1b[36m│\x1b[0m");
    println!("  \x1b[36m│\x1b[0m    5. Run a security audit                      \x1b[36m│\x1b[0m");
    println!("  \x1b[36m│\x1b[0m                                                 \x1b[36m│\x1b[0m");
    println!("  \x1b[36m│\x1b[0m  \x1b[2mAll data stays 100% on your machine.\x1b[0m           \x1b[36m│\x1b[0m");
    println!("  \x1b[36m└─────────────────────────────────────────────────┘\x1b[0m\n");
}

fn print_next_steps(after_command: &str) {
    println!();
    println!("  \x1b[1mWhat next?\x1b[0m");
    match after_command {
        "scan" => {
            println!("  \x1b[36m  compscan report\x1b[0m        Get AI-powered insights from this scan");
            println!("  \x1b[36m  compscan observe\x1b[0m       Start background tracking (leave running)");
            println!("  \x1b[36m  compscan act cleanup-caches\x1b[0m  Free up disk space now");
            println!("  \x1b[36m  compscan dashboard\x1b[0m     Interactive terminal dashboard");
        }
        "report" => {
            println!("  \x1b[36m  compscan act <action>\x1b[0m  Fix an issue from the report above");
            println!("  \x1b[36m  compscan observe\x1b[0m       Start tracking for richer future reports");
            println!("  \x1b[36m  compscan dashboard\x1b[0m     See everything in an interactive view");
            println!("  \x1b[36m  compscan web\x1b[0m           Open browser dashboard at localhost:7890");
        }
        "act" => {
            println!("  \x1b[36m  compscan report\x1b[0m        Check for more actionable insights");
            println!("  \x1b[36m  compscan scan\x1b[0m          Re-scan to verify the improvement");
            println!("  \x1b[36m  compscan status\x1b[0m        Quick system overview");
        }
        "status" => {
            println!("  \x1b[36m  compscan scan\x1b[0m          Full system health check");
            println!("  \x1b[36m  compscan report\x1b[0m        Generate insights report");
            println!("  \x1b[36m  compscan dashboard\x1b[0m     Interactive terminal dashboard");
        }
        _ => {}
    }
    println!();
}
