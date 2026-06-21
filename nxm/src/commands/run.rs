use anyhow::{Context, Result};
use colored::*;
use std::path::Path;
use std::process::Command;

use crate::config::NexoraConfig;

pub fn execute(script: &str) -> Result<()> {
    let config_path = Path::new("nexora.json");

    if !config_path.exists() {
        println!(
            "{}",
            "No nexora.json found. Run 'nxm init' first.".red()
        );
        return Ok(());
    }

    let config = NexoraConfig::from_file(config_path)?;

    let script_command = config
        .scripts
        .get(script)
        .ok_or_else(|| anyhow::anyhow!("Script '{}' not found in nexora.json", script))?;

    println!(
        "{} {}",
        "Running".cyan().bold(),
        script.yellow().bold()
    );
    println!(
        "{}",
        format!("  Command: {}", script_command).dimmed()
    );
    println!();

    // Execute the script
    let status = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(["/C", script_command])
            .status()
            .context("Failed to execute script")?
    } else {
        Command::new("sh")
            .args(["-c", script_command])
            .status()
            .context("Failed to execute script")?
    };

    if !status.success() {
        return Err(anyhow::anyhow!(
            "Script '{}' failed with exit code: {:?}",
            script,
            status.code()
        ));
    }

    println!();
    println!(
        "{} {}",
        "✓ Script".green().bold(),
        script.green().bold()
    );

    Ok(())
}