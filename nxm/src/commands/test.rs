use anyhow::{Context, Result};
use colored::*;
use std::path::Path;
use std::process::Command;

use crate::config::NexoraConfig;

pub fn execute(pattern: Option<&str>) -> Result<()> {
    let config_path = Path::new("nexora.json");

    if !config_path.exists() {
        println!(
            "{}",
            "No nexora.json found. Run 'nxm init' first.".red()
        );
        return Ok(());
    }

    let config = NexoraConfig::from_file(config_path)?;

    // Check if test script exists
    let test_script = config
        .scripts
        .get("test")
        .ok_or_else(|| anyhow::anyhow!("No 'test' script found in nexora.json"))?;

    println!(
        "{}",
        "Running tests...".cyan().bold()
    );
    println!(
        "{}",
        format!("  Command: {}", test_script).dimmed()
    );
    if let Some(p) = pattern {
        println!(
            "{}",
            format!("  Pattern: {}", p).dimmed()
        );
    }
    println!();

    // Execute the test script
    let status = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(["/C", test_script])
            .status()
            .context("Failed to execute test script")?
    } else {
        Command::new("sh")
            .args(["-c", test_script])
            .status()
            .context("Failed to execute test script")?
    };

    println!();

    if status.success() {
        println!(
            "{}",
            "✓ All tests passed!".green().bold()
        );
    } else {
        println!(
            "{}",
            "✗ Tests failed!".red().bold()
        );
        return Err(anyhow::anyhow!(
            "Tests failed with exit code: {:?}",
            status.code()
        ));
    }

    Ok(())
}