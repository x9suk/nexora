use anyhow::Result;
use colored::*;
use std::fs;
use std::path::Path;

use crate::config::NexoraConfig;

const MODULES_DIR: &str = "nexora_modules";

pub fn execute(name: &str) -> Result<()> {
    let config_path = Path::new("nexora.json");
    let modules_dir = Path::new(MODULES_DIR);

    if !config_path.exists() {
        println!(
            "{}",
            "No nexora.json found.".red()
        );
        return Ok(());
    }

    let mut config = NexoraConfig::from_file(config_path)?;

    // Check if package exists
    let is_dep = config.dependencies.contains_key(name);
    let is_dev_dep = config.dev_dependencies.contains_key(name);

    if !is_dep && !is_dev_dep {
        println!(
            "{} {}",
            "Package".red(),
            format!("'{}' is not a dependency.", name).red()
        );
        return Ok(());
    }

    // Remove from config
    if is_dep {
        config.dependencies.remove(name);
        println!(
            "  {} from dependencies",
            "Removed".green()
        );
    }
    if is_dev_dep {
        config.dev_dependencies.remove(name);
        println!(
            "  {} from devDependencies",
            "Removed".green()
        );
    }

    // Save config
    let json = serde_json::to_string_pretty(&config)?;
    fs::write(config_path, &json)?;

    // Remove from nexora_modules/
    let pkg_dir = modules_dir.join(name);
    if pkg_dir.exists() {
        fs::remove_dir_all(&pkg_dir)?;
        println!(
            "  {} from nexora_modules/",
            "Removed".green()
        );
    }

    println!();
    println!(
        "{} {}",
        "Removed".green().bold(),
        name.green().bold()
    );

    Ok(())
}
