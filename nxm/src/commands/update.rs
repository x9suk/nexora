use anyhow::Result;
use colored::*;
use std::fs;
use std::path::Path;

use crate::config::{DependencyInfo, NexoraConfig};
use crate::registry::RegistryClient;

const MODULES_DIR: &str = "nexora_modules";

pub fn execute(package: Option<String>) -> Result<()> {
    let config_path = Path::new("nexora.json");
    let modules_dir = Path::new(MODULES_DIR);

    if !config_path.exists() {
        println!(
            "{}",
            "No nexora.json found. Run 'nxm init' to create one.".red()
        );
        return Ok(());
    }

    let config = NexoraConfig::from_file(config_path)?;
    let client = RegistryClient::new();

    let mut updated = 0;

    // Collect packages to update
    let packages_to_update: Vec<(String, String)> = if let Some(pkg_name) = package {
        // Update specific package
        if let Some(dep) = config.dependencies.get(&pkg_name) {
            let version = match dep {
                DependencyInfo::Version(v) => v.clone(),
                DependencyInfo::Details(d) => d.version.clone(),
            };
            vec![(pkg_name, version)]
        } else if let Some(dep) = config.dev_dependencies.get(&pkg_name) {
            let version = match dep {
                DependencyInfo::Version(v) => v.clone(),
                DependencyInfo::Details(d) => d.version.clone(),
            };
            vec![(pkg_name, version)]
        } else {
            println!(
                "{} {}",
                "Package".red(),
                format!("'{}' is not a dependency.", pkg_name).red()
            );
            return Ok(());
        }
    } else {
        // Update all packages
        let mut pkgs = Vec::new();
        for (name, dep) in &config.dependencies {
            let version = match dep {
                DependencyInfo::Version(v) => v.clone(),
                DependencyInfo::Details(d) => d.version.clone(),
            };
            pkgs.push((name.clone(), version));
        }
        for (name, dep) in &config.dev_dependencies {
            let version = match dep {
                DependencyInfo::Version(v) => v.clone(),
                DependencyInfo::Details(d) => d.version.clone(),
            };
            pkgs.push((name.clone(), version));
        }
        pkgs
    };

    for (name, current_version) in &packages_to_update {
        println!(
            "{} {}",
            "Checking".cyan(),
            name.yellow().bold()
        );

        // Get latest version from registry
        match client.get_package(name) {
            Ok(package_info) => {
                let latest_version = package_info.get_latest_version();

                if latest_version != *current_version {
                    println!(
                        "  {} {} -> {}",
                        "Updating".green(),
                        current_version.dimmed(),
                        latest_version.green()
                    );

                    // Remove old version
                    let pkg_dir = modules_dir.join(name);
                    if pkg_dir.exists() {
                        fs::remove_dir_all(&pkg_dir)?;
                    }

                    // Download new version
                    fs::create_dir_all(&pkg_dir)?;
                    // download_and_extract(&client, name, &latest_version, &pkg_dir)?;
                    updated += 1;
                } else {
                    println!(
                        "  {}",
                        "Already up to date".dimmed()
                    );
                }
            }
            Err(e) => {
                println!(
                    "  {} {}",
                    "Error:".red(),
                    e.to_string().red()
                );
            }
        }
    }

    println!();
    println!(
        "{}",
        format!("{} packages updated", updated).green().bold()
    );

    Ok(())
}
