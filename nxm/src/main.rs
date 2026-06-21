use clap::{Parser, Subcommand};
use colored::*;
use std::process;

mod commands;
mod config;
mod package;
mod registry;
mod resolver;

#[derive(Parser)]
#[command(
    name = "nxm",
    about = "Nexora Package Manager - A modern package manager for Nexora projects",
    version = "1.0.0",
    author = "Nexora Team"
)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize a new Nexora project
    Init {
        /// Package name
        #[arg(short, long)]
        name: Option<String>,
    },
    /// Install a package
    Install {
        /// Package name
        package: Option<String>,
        /// Save as dev dependency
        #[arg(short, long)]
        save_dev: bool,
    },
    /// Publish package to registry
    Publish {
        /// Access level (public or restricted)
        #[arg(short, long, default_value = "public")]
        access: String,
    },
    /// Search packages in registry
    Search {
        /// Search query
        query: String,
    },
    /// Run a script from nexora.json
    Run {
        /// Script name
        script: String,
    },
    /// Run tests
    Test {
        /// Test pattern
        #[arg(short, long)]
        pattern: Option<String>,
    },
    /// Update packages
    Update {
        /// Specific package to update
        package: Option<String>,
    },
    /// List installed packages
    List {
        /// Show global packages
        #[arg(short, long)]
        global: bool,
    },
    /// Remove a package
    Remove {
        /// Package name
        package: String,
    },
    /// List available modules and installed packages
    Modules,
    /// Show version information
    Version,
}

fn main() {
    let cli = Cli::parse();

    let result = match cli.command {
        Some(Commands::Init { name }) => commands::init::execute(name),
        Some(Commands::Install { package, save_dev }) => {
            commands::install::execute(package, save_dev)
        }
        Some(Commands::Publish { access }) => commands::publish::execute(&access),
        Some(Commands::Search { query }) => commands::search::execute(&query),
        Some(Commands::Run { script }) => commands::run::execute(&script),
        Some(Commands::Test { pattern }) => commands::test::execute(pattern.as_deref()),
        Some(Commands::Update { package }) => commands::update::execute(package),
        Some(Commands::List { global }) => commands::list::execute(global),
        Some(Commands::Remove { package }) => commands::remove::execute(&package),
        Some(Commands::Modules) => commands::modules::execute(),
        Some(Commands::Version) => {
            println!("{}", "nxm v1.0.0".green().bold());
            println!("{}", "Nexora Package Manager".dimmed());
            Ok(())
        }
        None => {
            print_welcome();
            Ok(())
        }
    };

    if let Err(e) = result {
        eprintln!("{} {}", "Error:".red().bold(), e);
        process::exit(1);
    }
}

fn print_welcome() {
    println!(
        "{}",
        "╔══════════════════════════════════════════╗".cyan()
    );
    println!(
        "{}",
        "║     Nexora Package Manager (nxm)         ║".cyan()
    );
    println!(
        "{}",
        "║     Version 1.0.0                        ║".cyan()
    );
    println!(
        "{}",
        "╚══════════════════════════════════════════╝".cyan()
    );
    println!();
    println!("Usage: nxm <command> [options]");
    println!();
    println!("Commands:");
    println!("  init                  Initialize a new project");
    println!("  install <pkg>         Install a package");
    println!("  publish               Publish to registry");
    println!("  search <query>        Search packages");
    println!("  run <script>          Run a script");
    println!("  test                  Run tests");
    println!("  update                Update packages");
    println!("  list                  List packages");
    println!("  remove <pkg>          Remove a package");
    println!("  version               Show version");
    println!("  help                  Show this help");
}