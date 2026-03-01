use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "compscan",
    about = "A fully local AI agent that observes, learns, and optimizes your digital life",
    version,
    long_about = None,
    after_help = "All data stays on your machine. Zero network egress. Full privacy."
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    /// Enable verbose debug output
    #[arg(short, long, global = true)]
    pub verbose: bool,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Run a full system scan and generate insights
    Scan {
        /// Include deep filesystem analysis (slower but more thorough)
        #[arg(short, long)]
        full: bool,
    },

    /// Start the background observer daemon
    Observe {
        /// Sampling interval in seconds
        #[arg(short, long, default_value = "30")]
        interval: u64,
    },

    /// Launch the interactive TUI dashboard
    Dashboard,

    /// Start the web dashboard on localhost
    Web {
        /// Port to bind the web server to
        #[arg(short, long, default_value = "7890")]
        port: u16,
    },

    /// Generate an insights report
    Report {
        /// Output as JSON instead of human-readable
        #[arg(short, long)]
        json: bool,
    },

    /// Execute a suggested action by ID
    Act {
        /// The action ID to execute
        action_id: String,

        /// Skip confirmation for non-critical actions
        #[arg(short, long)]
        force: bool,
    },

    /// Show daemon status and system summary
    Status,

    /// Manage configuration
    Config {
        /// Reset configuration to defaults
        #[arg(long)]
        reset: bool,
    },
}
