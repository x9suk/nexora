use anyhow::Result;
use colored::*;

use crate::registry::RegistryClient;

pub fn execute(query: &str) -> Result<()> {
    println!(
        "{} {}",
        "Searching for".cyan().bold(),
        query.yellow().bold()
    );
    println!();

    let client = RegistryClient::new();
    let results = client.search_packages(query)?;

    if results.is_empty() {
        println!(
            "{}",
            "No packages found matching your query".yellow()
        );
        return Ok(());
    }

    println!(
        "{}",
        format!("Found {} packages:", results.len()).cyan()
    );
    println!();

    for package in &results {
        println!(
            "  {} {} {}",
            package.name.green().bold(),
            package.version.dimmed(),
            package.description.as_deref().unwrap_or_default().dimmed()
        );
        println!(
            "    {}",
            format!(
                "Downloads: {} | Stars: {}",
                package.downloads, package.stars
            )
            .dimmed()
        );
        println!();
    }

    Ok(())
}