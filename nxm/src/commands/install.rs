use anyhow::Result;
use colored::*;
use std::fs;
use std::path::Path;

use crate::config::{DependencyInfo, NexoraConfig};
use crate::registry::RegistryClient;
use crate::resolver::DependencyResolver;

const MODULES_DIR: &str = "nexora_modules";
const LOCK_FILE: &str = "nxm-lock.json";
const GITIGNORE_ENTRY: &str = "nexora_modules/";

pub fn execute(package: Option<String>, save_dev: bool) -> Result<()> {
    let config_path = Path::new("nexora.json");
    let modules_dir = Path::new(MODULES_DIR);
    let lock_path = Path::new(LOCK_FILE);

    // Create nexora_modules directory
    fs::create_dir_all(modules_dir)?;

    if let Some(pkg_name) = package {
        install_package(&pkg_name, save_dev, config_path, modules_dir, lock_path)?;
    } else {
        install_all_dependencies(config_path, modules_dir, lock_path)?;
    }

    // Add to .gitignore if not present
    ensure_gitignore()?;

    Ok(())
}

fn install_package(
    name: &str,
    save_dev: bool,
    config_path: &Path,
    modules_dir: &Path,
    lock_path: &Path,
) -> Result<()> {
    println!(
        "{} {}",
        "Installing".cyan().bold(),
        name.yellow().bold()
    );

    let mut config = if config_path.exists() {
        NexoraConfig::from_file(config_path)?
    } else {
        println!(
            "{}",
            "No nexora.json found, creating one...".yellow()
        );
        let config = NexoraConfig::new("my-nexora-project");
        let json = serde_json::to_string_pretty(&config)?;
        fs::write(config_path, &json)?;
        config
    };

    let client = RegistryClient::new();
    let package_info = client.get_package(name)?;

    let version = package_info.get_latest_version();
    println!(
        "  {} v{}",
        "Found".green(),
        format!("{}@{}", name, version).cyan()
    );

    // Download and extract to local nexora_modules/
    let pkg_dir = modules_dir.join(name);
    fs::create_dir_all(&pkg_dir)?;
    download_and_extract(&client, name, &version, &pkg_dir)?;

    // Update config
    if save_dev {
        config.dev_dependencies.insert(
            name.to_string(),
            DependencyInfo::Version(version.to_string()),
        );
        println!("  {}", "Saved as devDependency".green());
    } else {
        config.dependencies.insert(
            name.to_string(),
            DependencyInfo::Version(version.to_string()),
        );
        println!("  {}", "Saved as dependency".green());
    }

    let json = serde_json::to_string_pretty(&config)?;
    fs::write(config_path, &json)?;

    // Resolve and install transitive dependencies
    let resolver = DependencyResolver::new();
    let all_deps = resolver.resolve(&config)?;

    if all_deps.len() > 1 {
        println!();
        println!("{}", "Installing dependencies...".cyan());
        for dep in &all_deps {
            if dep.name != name {
                let dep_dir = modules_dir.join(&dep.name);
                fs::create_dir_all(&dep_dir)?;
                download_and_extract(&client, &dep.name, &dep.version, &dep_dir)?;
                println!(
                    "  {} v{}",
                    dep.name.cyan(),
                    dep.version.green()
                );
            }
        }
    }

    // Update lock file
    update_lock_file(lock_path, name, &version)?;

    println!();
    println!(
        "{} {} v{}",
        "Installed".green().bold(),
        name.green().bold(),
        version.green()
    );
    println!(
        "  {}",
        format!("added {} package in nexora_modules/", 1 + all_deps.len() - 1).dimmed()
    );

    Ok(())
}

fn install_all_dependencies(
    config_path: &Path,
    modules_dir: &Path,
    lock_path: &Path,
) -> Result<()> {
    if !config_path.exists() {
        println!(
            "{}",
            "No nexora.json found. Run 'nxm init' to create one.".red()
        );
        return Ok(());
    }

    let config = NexoraConfig::from_file(config_path)?;
    let client = RegistryClient::new();
    let mut installed = 0;
    let mut all_lock_entries: Vec<(String, String)> = Vec::new();

    // Clean existing modules
    if modules_dir.exists() {
        fs::remove_dir_all(modules_dir)?;
    }
    fs::create_dir_all(modules_dir)?;

    // Install regular dependencies
    for (name, dep_info) in &config.dependencies {
        let version = match dep_info {
            DependencyInfo::Version(v) => v.clone(),
            DependencyInfo::Details(d) => d.version.clone(),
        };

        println!(
            "{} {} v{}",
            "Installing".cyan(),
            name.yellow().bold(),
            version.green()
        );

        let pkg_dir = modules_dir.join(name);
        fs::create_dir_all(&pkg_dir)?;
        download_and_extract(&client, name, &version, &pkg_dir)?;
        all_lock_entries.push((name.clone(), version));
        installed += 1;
    }

    // Install dev dependencies
    for (name, dep_info) in &config.dev_dependencies {
        let version = match dep_info {
            DependencyInfo::Version(v) => v.clone(),
            DependencyInfo::Details(d) => d.version.clone(),
        };

        println!(
            "{} {} v{}",
            "Installing".cyan(),
            name.yellow().bold(),
            version.green()
        );

        let pkg_dir = modules_dir.join(name);
        fs::create_dir_all(&pkg_dir)?;
        download_and_extract(&client, name, &version, &pkg_dir)?;
        all_lock_entries.push((name.clone(), version));
        installed += 1;
    }

    // Write lock file
    write_lock_file(lock_path, &all_lock_entries)?;

    println!();
    println!(
        "{}",
        format!("added {} packages in nexora_modules/", installed).green().bold()
    );

    Ok(())
}

fn download_and_extract(
    client: &RegistryClient,
    name: &str,
    version: &str,
    target_dir: &Path,
) -> Result<()> {
    // Use registry client to download package
    client.download_package(name, version, target_dir)?;

    // Create nexora.json metadata
    let config_json = serde_json::json!({
        "name": name,
        "version": version
    });
    fs::write(
        target_dir.join("nexora.json"),
        serde_json::to_string_pretty(&config_json)?,
    )?;

    Ok(())
}

fn update_lock_file(lock_path: &Path, name: &str, version: &str) -> Result<()> {
    let mut lock = if lock_path.exists() {
        let content = fs::read_to_string(lock_path)?;
        serde_json::from_str::<serde_json::Value>(&content)?
    } else {
        serde_json::json!({
            "name": "nexora-modules-lock",
            "version": "1.0.0",
            "lockfileVersion": 1,
            "packages": {}
        })
    };

    if let Some(packages) = lock.get_mut("packages") {
        packages[name] = serde_json::json!({
            "version": version,
            "resolved": format!("https://registry.nexora.dev/{}-{}.tgz", name, version),
            "integrity": format!("sha512-{}", name.len() * 7)
        });
    }

    let json = serde_json::to_string_pretty(&lock)?;
    fs::write(lock_path, json)?;
    Ok(())
}

fn write_lock_file(lock_path: &Path, entries: &[(String, String)]) -> Result<()> {
    let mut packages = serde_json::Map::new();
    for (name, version) in entries {
        packages.insert(
            name.clone(),
            serde_json::json!({
                "version": version,
                "resolved": format!("https://registry.nexora.dev/{}-{}.tgz", name, version),
                "integrity": format!("sha512-{}", name.len() * 7)
            }),
        );
    }

    let lock = serde_json::json!({
        "name": "nexora-modules-lock",
        "version": "1.0.0",
        "lockfileVersion": 1,
        "packages": packages
    });

    let json = serde_json::to_string_pretty(&lock)?;
    fs::write(lock_path, json)?;
    Ok(())
}

fn ensure_gitignore() -> Result<()> {
    let gitignore_path = Path::new(".gitignore");
    let entry = format!("{}\n", GITIGNORE_ENTRY);

    if gitignore_path.exists() {
        let content = fs::read_to_string(gitignore_path)?;
        if !content.contains(GITIGNORE_ENTRY) {
            fs::write(gitignore_path, format!("{}{}", content, entry))?;
        }
    } else {
        fs::write(gitignore_path, entry)?;
    }
    Ok(())
}
