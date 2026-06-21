use anyhow::Result;
use colored::*;
use std::fs;
use std::path::Path;

const MODULES_DIR: &str = "nexora_modules";

pub fn execute(global: bool) -> Result<()> {
    if global {
        list_global()?;
    } else {
        list_local()?;
    }
    Ok(())
}

fn list_local() -> Result<()> {
    let modules_dir = Path::new(MODULES_DIR);

    if !modules_dir.exists() {
        println!(
            "{}",
            "No nexora_modules/ found. Run 'nxm install' first.".yellow()
        );
        return Ok(());
    }

    let mut packages = Vec::new();

    for entry in fs::read_dir(modules_dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            let name = path.file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string();

            let config_path = path.join("nexora.json");
            let version = if config_path.exists() {
                let content = fs::read_to_string(&config_path)?;
                let config: serde_json::Value = serde_json::from_str(&content)?;
                config["version"].as_str().unwrap_or("0.0.0").to_string()
            } else {
                "0.0.0".to_string()
            };

            packages.push((name, version));
        }
    }

    if packages.is_empty() {
        println!(
            "{}",
            "No packages installed.".yellow()
        );
        return Ok(());
    }

    println!();
    println!(
        "{}",
        "nexora_modules/".cyan().bold()
    );
    println!();

    for (name, version) in &packages {
        println!(
            "  {} v{}",
            name.green(),
            version.dimmed()
        );
    }

    println!();
    println!(
        "{}",
        format!("{} packages", packages.len()).dimmed()
    );

    Ok(())
}

fn list_global() -> Result<()> {
    let home = dirs::home_dir()
        .unwrap_or_else(|| Path::new(".").to_path_buf());
    let global_dir = home.join(".nexora").join("packages");

    if !global_dir.exists() {
        println!(
            "{}",
            "No global packages found.".yellow()
        );
        return Ok(());
    }

    let mut packages = Vec::new();

    for entry in fs::read_dir(&global_dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            let name = path.file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string();

            let config_path = path.join("nexora.json");
            let version = if config_path.exists() {
                let content = fs::read_to_string(&config_path)?;
                let config: serde_json::Value = serde_json::from_str(&content)?;
                config["version"].as_str().unwrap_or("0.0.0").to_string()
            } else {
                "0.0.0".to_string()
            };

            packages.push((name, version));
        }
    }

    if packages.is_empty() {
        println!(
            "{}",
            "No global packages installed.".yellow()
        );
        return Ok(());
    }

    println!();
    println!(
        "{}",
        "Global packages:".cyan().bold()
    );
    println!();

    for (name, version) in &packages {
        println!(
            "  {} v{}",
            name.green(),
            version.dimmed()
        );
    }

    println!();
    println!(
        "{}",
        format!("{} packages", packages.len()).dimmed()
    );

    Ok(())
}
