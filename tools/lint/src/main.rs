use anyhow::Result;
use clap::Parser;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

mod linter;
mod rules;

use crate::linter::Linter;

#[derive(Parser)]
#[command(name = "nexora-lint", about = "Linter for Nexora (.nx) files")]
struct Cli {
    /// File or directory to lint
    #[arg(default_value = ".")]
    path: PathBuf,

    /// Output format (text, json)
    #[arg(short, long, default_value = "text")]
    format: String,
}

#[derive(Debug, serde::Serialize)]
pub struct LintDiagnostic {
    pub file: String,
    pub line: usize,
    pub column: usize,
    pub severity: Severity,
    pub rule: String,
    pub message: String,
}

#[derive(Debug, PartialEq, serde::Serialize)]
pub enum Severity {
    Error,
    Warning,
    Info,
}

impl std::fmt::Display for Severity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Severity::Error => write!(f, "error"),
            Severity::Warning => write!(f, "warning"),
            Severity::Info => write!(f, "info"),
        }
    }
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let path = &cli.path;
    let linter = Linter::new();
    let mut all_diagnostics = Vec::new();

    if path.is_file() {
        if let Some(diags) = lint_file(path, &linter) {
            all_diagnostics.extend(diags);
        }
    } else if path.is_dir() {
        for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
            let entry_path = entry.path();
            if entry_path.extension().and_then(|e| e.to_str()) == Some("nx") {
                if let Some(diags) = lint_file(entry_path, &linter) {
                    all_diagnostics.extend(diags);
                }
            }
        }
    } else {
        eprintln!("Error: Path does not exist: {}", path.display());
        std::process::exit(1);
    }

    if cli.format == "json" {
        println!("{}", serde_json::to_string_pretty(&all_diagnostics)?);
    } else {
        for diag in &all_diagnostics {
            let color = match diag.severity {
                Severity::Error => "\x1b[31m",
                Severity::Warning => "\x1b[33m",
                Severity::Info => "\x1b[36m",
            };
            println!(
                "{}:{}:{}: {}{}{}\t{} [{}]",
                diag.file,
                diag.line,
                diag.column,
                color,
                diag.severity,
                "\x1b[0m",
                diag.message,
                diag.rule
            );
        }

        if all_diagnostics.is_empty() {
            println!("\x1b[32mNo issues found!\x1b[0m");
        } else {
            let errors = all_diagnostics.iter().filter(|d| d.severity == Severity::Error).count();
            let warnings = all_diagnostics.iter().filter(|d| d.severity == Severity::Warning).count();
            println!("\n\x1b[1m{} error(s), {} warning(s)\x1b[0m", errors, warnings);
            if errors > 0 {
                std::process::exit(1);
            }
        }
    }

    Ok(())
}

fn lint_file(path: &Path, linter: &Linter) -> Option<Vec<LintDiagnostic>> {
    match std::fs::read_to_string(path) {
        Ok(content) => {
            let diagnostics = linter.lint(&content, path.to_str().unwrap_or("unknown"));
            Some(diagnostics)
        }
        Err(e) => {
            eprintln!("Error reading {}: {}", path.display(), e);
            None
        }
    }
}
