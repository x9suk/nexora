use anyhow::Result;
use clap::Parser;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

use crate::formatter::Formatter;

mod formatter;

#[derive(Parser)]
#[command(name = "nexora-fmt", about = "Code formatter for Nexora (.nx) files")]
struct Cli {
    /// File or directory to format
    #[arg(default_value = ".")]
    path: PathBuf,

    /// Check mode - only check if formatting is needed
    #[arg(short, long)]
    check: bool,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let path = &cli.path;

    if path.is_file() {
        format_file(path, cli.check)?;
    } else if path.is_dir() {
        format_directory(path, cli.check)?;
    } else {
        eprintln!("Error: Path does not exist: {}", path.display());
        std::process::exit(1);
    }

    Ok(())
}

fn format_file(path: &Path, check: bool) -> Result<()> {
    let content = std::fs::read_to_string(path)?;
    let formatter = Formatter::new();
    let formatted = formatter.format(&content);

    if formatted == content {
        if !check {
            println!("Already formatted: {}", path.display());
        }
        return Ok(());
    }

    if check {
        println!("Needs formatting: {}", path.display());
        std::process::exit(1);
    }

    std::fs::write(path, &formatted)?;
    println!("Formatted: {}", path.display());
    Ok(())
}

fn format_directory(dir: &Path, check: bool) -> Result<()> {
    let mut has_errors = false;

    for entry in WalkDir::new(dir).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) == Some("nx") {
            match format_file(path, check) {
                Ok(_) => {}
                Err(e) => {
                    eprintln!("Error formatting {}: {}", path.display(), e);
                    has_errors = true;
                }
            }
        }
    }

    if has_errors {
        std::process::exit(1);
    }

    Ok(())
}
