use anyhow::{Context, Result};
use colored::*;
use std::fs;
use std::io::Write;
use std::path::Path;

use crate::config::NexoraConfig;
use crate::registry::RegistryClient;

pub fn execute(access: &str) -> Result<()> {
    let config_path = Path::new("nexora.json");

    if !config_path.exists() {
        println!(
            "{}",
            "No nexora.json found. Run 'nxm init' first.".red()
        );
        return Ok(());
    }

    let config = NexoraConfig::from_file(config_path)?;

    println!(
        "{}",
        "Publishing package...".cyan().bold()
    );
    println!();

    // Validate package name
    if config.name.is_empty() {
        return Err(anyhow::anyhow!("Package name is required"));
    }

    // Validate version
    if config.version.is_empty() {
        return Err(anyhow::anyhow!("Package version is required"));
    }

    println!("  Name: {}", config.name.cyan());
    println!("  Version: {}", config.version.green());
    println!("  Access: {}", access.yellow());

    // Create package tarball
    let tarball_path = create_tarball(&config)?;

    // Upload to registry
    let client = RegistryClient::new();
    client.publish_package(&config, &tarball_path, access)?;

    println!();
    println!(
        "{} {} v{}",
        "✓ Published".green().bold(),
        config.name.green().bold(),
        config.version.green()
    );
    println!();
    println!(
        "Registry: {}",
        "https://registry.nexora.dev".dimmed()
    );

    // Cleanup
    fs::remove_file(&tarball_path)?;

    Ok(())
}

fn create_tarball(config: &NexoraConfig) -> Result<std::path::PathBuf> {
    let tarball_name = format!("{}-{}.zip", config.name, config.version);
    let tarball_path = std::env::temp_dir().join(&tarball_name);

    let file = fs::File::create(&tarball_path)
        .context("Failed to create tarball")?;
    let mut zip = zip::ZipWriter::new(file);

    // Add nexora.json
    let config_json = serde_json::to_string_pretty(config)?;
    zip.start_file("nexora.json", zip::write::FileOptions::default())?;
    zip.write_all(config_json.as_bytes())?;

    // Add source files (simplified - in real implementation, you'd follow .nexignore)
    let current_dir = std::env::current_dir()?;
    add_files_to_zip(&mut zip, &current_dir, &current_dir)?;

    zip.finish()?;

    Ok(tarball_path)
}

fn add_files_to_zip(
    zip: &mut zip::ZipWriter<fs::File>,
    base_dir: &Path,
    current_dir: &Path,
) -> Result<()> {
    for entry in fs::read_dir(current_dir)? {
        let entry = entry?;
        let path = entry.path();
        let relative_path = path.strip_prefix(base_dir)?;

        // Skip common directories that shouldn't be published
        let name = relative_path.to_string_lossy();
        if name.starts_with('.') || name == "node_modules" || name == ".nexora" {
            continue;
        }

        if path.is_dir() {
            add_files_to_zip(zip, base_dir, &path)?;
        } else {
            let options = zip::write::FileOptions::default()
                .compression_method(zip::CompressionMethod::Deflated);

            zip.start_file(relative_path.to_string_lossy(), options)?;
            let content = fs::read(&path)?;
            zip.write_all(&content)?;
        }
    }

    Ok(())
}