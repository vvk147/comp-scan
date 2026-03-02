use anyhow::Result;
use std::io::{self, Write};

use crate::storage::models::{Action, RiskLevel};

pub fn check_permission(action: &Action, force: bool) -> Result<bool> {
    match action.risk_level {
        RiskLevel::Low => {
            if force {
                println!("  [AUTO-APPROVED] Low risk action");
                return Ok(true);
            }
            println!("  [LOW RISK] This action is safe to execute.");
            prompt_yn("  Proceed?")
        }
        RiskLevel::Medium => {
            println!("  [MEDIUM RISK] This action modifies system state.");
            println!("  Impact: {}", action.estimated_impact);
            if action.reversible {
                println!("  This action is reversible.");
            }
            prompt_yn("  Proceed?")
        }
        RiskLevel::High => {
            println!("  \x1b[33m[HIGH RISK] This action makes significant changes.\x1b[0m");
            println!("  Impact: {}", action.estimated_impact);
            println!(
                "  Reversible: {}",
                if action.reversible { "yes" } else { "NO" }
            );
            println!();
            prompt_confirm("  Type 'yes' to confirm: ")
        }
        RiskLevel::Critical => {
            println!("  \x1b[31m[BLOCKED] This action is classified as critical risk.\x1b[0m");
            println!("  CompScan will not execute critical-risk actions automatically.");
            println!(
                "  Manual execution required: review the suggested command and run it yourself."
            );
            Ok(false)
        }
    }
}

fn prompt_yn(message: &str) -> Result<bool> {
    print!("{message} [Y/n] ");
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let answer = input.trim().to_lowercase();

    Ok(answer.is_empty() || answer == "y" || answer == "yes")
}

fn prompt_confirm(message: &str) -> Result<bool> {
    print!("{message}");
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    Ok(input.trim().to_lowercase() == "yes")
}
