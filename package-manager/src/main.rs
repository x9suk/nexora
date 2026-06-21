use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

use clap::{Parser, Subcommand};
use colored::*;
use serde::{Deserialize, Serialize};

#[derive(Parser)]
#[command(name = "npxr")]
#[command(about = "Nexora Package Manager")]
#[command(version = "0.1.0")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize a new package
    Init,
    /// Install a package
    Install {
        /// Package name
        package: Option<String>,
    },
    /// Install all dependencies
    InstallAll,
    /// Add a dependency
    Add {
        /// Package name
        package: String,
        /// Version (optional)
        version: Option<String>,
        /// Save as dev dependency
        #[arg(short, long)]
        dev: bool,
    },
    /// Remove a dependency
    Remove {
        /// Package name
        package: String,
    },
    /// Update a package
    Update {
        /// Package name (all if omitted)
        package: Option<String>,
    },
    /// List installed packages
    List,
    /// Search for packages
    Search {
        /// Search query
        query: String,
    },
    /// Publish a package
    Publish,
    /// Show package info
    Info {
        /// Package name
        package: String,
    },
    /// Clean cache
    Clean,
}

#[derive(Debug, Serialize, Deserialize)]
struct PackageConfig {
    name: String,
    version: String,
    description: Option<String>,
    main: Option<String>,
    author: Option<String>,
    license: Option<String>,
    dependencies: HashMap<String, String>,
    #[serde(default)]
    dev_dependencies: HashMap<String, String>,
    repository: Option<String>,
    keywords: Option<Vec<String>>,
}

impl Default for PackageConfig {
    fn default() -> Self {
        PackageConfig {
            name: String::new(),
            version: "0.1.0".to_string(),
            description: None,
            main: Some("src/main.nx".to_string()),
            author: None,
            license: Some("MIT".to_string()),
            dependencies: HashMap::new(),
            dev_dependencies: HashMap::new(),
            repository: None,
            keywords: None,
        }
    }
}

struct PackageManager {
    config_path: PathBuf,
    packages_dir: PathBuf,
    cache_dir: PathBuf,
}

impl PackageManager {
    fn new() -> Self {
        let config_path = PathBuf::from("nexora.json");
        let packages_dir = PathBuf::from("node_modules");
        let cache_dir = dirs::home_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join(".nexora")
            .join("cache");

        PackageManager {
            config_path,
            packages_dir,
            cache_dir,
        }
    }

    fn load_config(&self) -> Result<PackageConfig, String> {
        if !self.config_path.exists() {
            return Err("No nexora.json found. Run 'npxr init' first.".to_string());
        }

        let content = fs::read_to_string(&self.config_path)
            .map_err(|e| format!("Failed to read nexora.json: {}", e))?;

        serde_json::from_str(&content)
            .map_err(|e| format!("Failed to parse nexora.json: {}", e))
    }

    fn save_config(&self, config: &PackageConfig) -> Result<(), String> {
        let content = serde_json::to_string_pretty(config)
            .map_err(|e| format!("Failed to serialize config: {}", e))?;

        fs::write(&self.config_path, content)
            .map_err(|e| format!("Failed to write nexora.json: {}", e))
    }

    fn init(&self) -> Result<(), String> {
        if self.config_path.exists() {
            return Err("nexora.json already exists.".to_string());
        }

        let config = PackageConfig {
            name: "my-nexora-project".to_string(),
            description: Some("A Nexora project".to_string()),
            ..Default::default()
        };

        self.save_config(&config)?;

        // Create directory structure
        fs::create_dir_all("src").map_err(|e| e.to_string())?;
        fs::create_dir_all("tests").map_err(|e| e.to_string())?;

        // Create main.nx
        fs::write("src/main.nx", r#"print "Hello from Nexora!""#)
            .map_err(|e| e.to_string())?;

        println!(
            "{} Initialized new Nexora project",
            "Success:".green().bold()
        );
        Ok(())
    }

    fn install(&self, package: Option<String>) -> Result<(), String> {
        let mut config = self.load_config()?;

        match package {
            Some(pkg) => {
                println!(
                    "{} Installing {}...",
                    "Info:".cyan().bold(),
                    pkg
                );
                // Simulate package installation
                config.dependencies.insert(pkg.clone(), "latest".to_string());
                self.save_config(&config)?;
                println!(
                    "{} Installed {}",
                    "Success:".green().bold(),
                    pkg
                );
            }
            None => {
                println!("{} Installing all dependencies...", "Info:".cyan().bold());
                for (pkg, version) in &config.dependencies {
                    println!("  Installing {}@{}", pkg, version);
                }
                println!("{} All dependencies installed", "Success:".green().bold());
            }
        }

        Ok(())
    }

    fn add(&self, package: String, version: Option<String>, dev: bool) -> Result<(), String> {
        let mut config = self.load_config()?;
        let version = version.unwrap_or_else(|| "latest".to_string());

        if dev {
            config
                .dev_dependencies
                .insert(package.clone(), version.clone());
        } else {
            config.dependencies.insert(package.clone(), version.clone());
        }

        self.save_config(&config)?;

        let dep_type = if dev { "dev dependency" } else { "dependency" };
        println!(
            "{} Added {} as {} ({})",
            "Success:".green().bold(),
            package,
            dep_type,
            version
        );

        Ok(())
    }

    fn remove(&self, package: String) -> Result<(), String> {
        let mut config = self.load_config()?;

        if config.dependencies.remove(&package).is_some() {
            self.save_config(&config)?;
            println!(
                "{} Removed {}",
                "Success:".green().bold(),
                package
            );
        } else if config.dev_dependencies.remove(&package).is_some() {
            self.save_config(&config)?;
            println!(
                "{} Removed {} (dev dependency)",
                "Success:".green().bold(),
                package
            );
        } else {
            return Err(format!("Package '{}' not found", package));
        }

        Ok(())
    }

    fn list(&self) -> Result<(), String> {
        let config = self.load_config()?;

        println!("\n{}\n", "Dependencies:".cyan().bold());
        if config.dependencies.is_empty() {
            println!("  No dependencies installed.");
        } else {
            for (pkg, version) in &config.dependencies {
                println!("  {} @ {}", pkg, version);
            }
        }

        println!("\n{}\n", "Dev Dependencies:".cyan().bold());
        if config.dev_dependencies.is_empty() {
            println!("  No dev dependencies installed.");
        } else {
            for (pkg, version) in &config.dev_dependencies {
                println!("  {} @ {}", pkg, version);
            }
        }

        Ok(())
    }

    fn search(&self, query: String) -> Result<(), String> {
        println!(
            "{} Searching for '{}'...\n",
            "Info:".cyan().bold(),
            query
        );

        // Simulated search results
        let packages = vec![
            ("nexora-std", "0.1.0", "Nexora standard library"),
            ("nexora-http", "0.1.0", "HTTP client for Nexora"),
            ("nexora-json", "0.1.0", "JSON parser for Nexora"),
            ("nexora-test", "0.1.0", "Testing framework for Nexora"),
            ("nexora-ai", "0.1.0", "AI integration for Nexora"),
        ];

        let results: Vec<_> = packages
            .iter()
            .filter(|(name, _, desc)| {
                name.contains(&query) || desc.to_lowercase().contains(&query.to_lowercase())
            })
            .collect();

        if results.is_empty() {
            println!("  No packages found matching '{}'", query);
        } else {
            for (name, version, desc) in results {
                println!(
                    "  {} v{} - {}",
                    name.green().bold(),
                    version.cyan(),
                    desc
                );
            }
        }

        Ok(())
    }

    fn info(&self, package: String) -> Result<(), String> {
        println!(
            "{} Package: {}\n",
            "Info:".cyan().bold(),
            package.green().bold()
        );

        // Simulated package info
        println!("  Name: {}", package);
        println!("  Version: 0.1.0");
        println!("  Description: A Nexora package");
        println!("  Author: Nexora Team");
        println!("  License: MIT");
        println!("  Repository: https://github.com/nexora/{}", package);

        Ok(())
    }

    fn publish(&self) -> Result<(), String> {
        let config = self.load_config()?;

        println!(
            "{} Publishing {} v{}...",
            "Info:".cyan().bold(),
            config.name,
            config.version
        );

        // Simulate publish
        println!("{} Package published successfully!", "Success:".green().bold());
        println!("  https://registry.nexora.dev/{}", config.name);

        Ok(())
    }

    fn clean(&self) -> Result<(), String> {
        println!("{} Cleaning cache...", "Info:".cyan().bold());

        if self.cache_dir.exists() {
            fs::remove_dir_all(&self.cache_dir).map_err(|e| e.to_string())?;
        }

        println!("{} Cache cleaned", "Success:".green().bold());
        Ok(())
    }

    fn update(&self, package: Option<String>) -> Result<(), String> {
        let mut config = self.load_config()?;

        match package {
            Some(pkg) => {
                println!(
                    "{} Updating {}...",
                    "Info:".cyan().bold(),
                    pkg
                );
                if let Some(version) = config.dependencies.get_mut(&pkg) {
                    *version = "latest".to_string();
                }
                self.save_config(&config)?;
                println!(
                    "{} Updated {}",
                    "Success:".green().bold(),
                    pkg
                );
            }
            None => {
                println!("{} Updating all dependencies...", "Info:".cyan().bold());
                for version in config.dependencies.values_mut() {
                    *version = "latest".to_string();
                }
                for version in config.dev_dependencies.values_mut() {
                    *version = "latest".to_string();
                }
                self.save_config(&config)?;
                println!("{} All dependencies updated", "Success:".green().bold());
            }
        }

        Ok(())
    }
}

fn main() {
    let cli = Cli::parse();
    let pm = PackageManager::new();

    let result = match cli.command {
        Commands::Init => pm.init(),
        Commands::Install { package } => pm.install(package),
        Commands::InstallAll => pm.install(None),
        Commands::Add {
            package,
            version,
            dev,
        } => pm.add(package, version, dev),
        Commands::Remove { package } => pm.remove(package),
        Commands::List => pm.list(),
        Commands::Search { query } => pm.search(query),
        Commands::Info { package } => pm.info(package),
        Commands::Publish => pm.publish(),
        Commands::Clean => pm.clean(),
        Commands::Update { package } => pm.update(package),
    };

    if let Err(e) = result {
        eprintln!("{} {}", "Error:".red().bold(), e);
        std::process::exit(1);
    }
}
