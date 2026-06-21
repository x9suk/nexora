use std::fs;
use std::path::PathBuf;

use clap::{Parser, Subcommand};
use colored::*;
use serde::{Deserialize, Serialize};

#[derive(Parser)]
#[command(name = "nxlint")]
#[command(about = "Nexora Code Linter")]
#[command(version = "0.1.0")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Lint a file
    Lint {
        /// Path to the .nx file
        file: PathBuf,
        /// Output format (text, json)
        #[arg(short, long, default_value = "text")]
        format: String,
    },
    /// Lint all .nx files in a directory
    LintAll {
        /// Path to directory
        path: PathBuf,
        /// Output format
        #[arg(short, long, default_value = "text")]
        format: String,
    },
    /// Fix auto-fixable issues
    Fix {
        /// Path to the .nx file or directory
        path: PathBuf,
    },
}

#[derive(Debug, Serialize, Deserialize)]
enum Severity {
    Error,
    Warning,
    Info,
}

#[derive(Debug, Serialize, Deserialize)]
struct LintIssue {
    file: String,
    line: usize,
    column: usize,
    severity: Severity,
    message: String,
    rule: String,
}

impl LintIssue {
    fn new(
        file: &str,
        line: usize,
        column: usize,
        severity: Severity,
        message: &str,
        rule: &str,
    ) -> Self {
        LintIssue {
            file: file.to_string(),
            line,
            column,
            severity,
            message: message.to_string(),
            rule: rule.to_string(),
        }
    }
}

struct Linter {
    issues: Vec<LintIssue>,
}

impl Linter {
    fn new() -> Self {
        Linter {
            issues: Vec::new(),
        }
    }

    fn lint_source(&mut self, file: &str, source: &str) {
        let lines: Vec<&str> = source.lines().collect();

        for (line_num, line) in lines.iter().enumerate() {
            let line_num = line_num + 1;

            // Check for trailing whitespace
            if line.ends_with(' ') || line.ends_with('\t') {
                self.issues.push(LintIssue::new(
                    file,
                    line_num,
                    line.len(),
                    Severity::Warning,
                    "Trailing whitespace",
                    "trailing-whitespace",
                ));
            }

            // Check for very long lines
            if line.len() > 100 {
                self.issues.push(LintIssue::new(
                    file,
                    line_num,
                    100,
                    Severity::Warning,
                    "Line exceeds 100 characters",
                    "line-length",
                ));
            }

            // Check for TODO comments
            if line.contains("TODO") {
                self.issues.push(LintIssue::new(
                    file,
                    line_num,
                    line.find("TODO").unwrap_or(0),
                    Severity::Info,
                    "TODO comment found",
                    "todo-comment",
                ));
            }

            // Check for FIXME comments
            if line.contains("FIXME") {
                self.issues.push(LintIssue::new(
                    file,
                    line_num,
                    line.find("FIXME").unwrap_or(0),
                    Severity::Warning,
                    "FIXME comment found",
                    "fixme-comment",
                ));
            }

            // Check for console.log (JavaScript habit)
            if line.contains("console.log") {
                self.issues.push(LintIssue::new(
                    file,
                    line_num,
                    line.find("console.log").unwrap_or(0),
                    Severity::Warning,
                    "Use 'print' instead of 'console.log'",
                    "no-console-log",
                ));
            }

            // Check for var keyword (should use let/const)
            if line.trim_start().starts_with("var ") {
                self.issues.push(LintIssue::new(
                    file,
                    line_num,
                    line.find("var").unwrap_or(0),
                    Severity::Warning,
                    "Use 'let' or 'const' instead of 'var'",
                    "no-var",
                ));
            }
        }

        // Try to parse and check for syntax errors
        match nexora_compiler::parse(source) {
            Ok(program) => {
                // Check for unused variables (simplified)
                for stmt in &program.stmts {
                    if let nexora_compiler::Stmt::VarDecl { name, .. } = stmt {
                        // Simple check - in reality would need scope analysis
                        let count = source.matches(name.as_str()).count();
                        if count == 1 {
                            self.issues.push(LintIssue::new(
                                file,
                                1,
                                0,
                                Severity::Warning,
                                &format!("Variable '{}' might be unused", name),
                                "no-unused-vars",
                            ));
                        }
                    }
                }
            }
            Err(e) => {
                self.issues.push(LintIssue::new(
                    file,
                    1,
                    0,
                    Severity::Error,
                    &format!("Syntax error: {}", e),
                    "syntax-error",
                ));
            }
        }
    }

    fn lint_file(&mut self, path: &PathBuf) -> Result<(), String> {
        let source = fs::read_to_string(path).map_err(|e| e.to_string())?;
        let file = path.to_str().unwrap_or("unknown");
        self.lint_source(file, &source);
        Ok(())
    }

    fn lint_directory(&mut self, path: &PathBuf) -> Result<(), String> {
        if path.is_dir() {
            for entry in fs::read_dir(path).map_err(|e| e.to_string())? {
                let entry = entry.map_err(|e| e.to_string())?;
                let path = entry.path();

                if path.is_dir() {
                    self.lint_directory(&path)?;
                } else if path.extension().and_then(|s| s.to_str()) == Some("nx") {
                    self.lint_file(&path)?;
                }
            }
        } else if path.extension().and_then(|s| s.to_str()) == Some("nx") {
            self.lint_file(path)?;
        }
        Ok(())
    }

    fn print_text(&self) {
        if self.issues.is_empty() {
            println!("{} No issues found!", "OK:".green().bold());
            return;
        }

        for issue in &self.issues {
            let severity = match issue.severity {
                Severity::Error => "error".red().bold(),
                Severity::Warning => "warning".yellow().bold(),
                Severity::Info => "info".cyan().bold(),
            };

            println!(
                "{}:{}:{}: {} [{}]",
                issue.file, issue.line, issue.column, severity, issue.rule
            );
            println!("  {}\n", issue.message);
        }

        let errors = self.issues.iter().filter(|i| matches!(i.severity, Severity::Error)).count();
        let warnings = self.issues.iter().filter(|i| matches!(i.severity, Severity::Warning)).count();
        let infos = self.issues.iter().filter(|i| matches!(i.severity, Severity::Info)).count();

        println!(
            "\n{} {} errors, {} warnings, {} infos",
            "Summary:".cyan().bold(),
            errors,
            warnings,
            infos
        );
    }

    fn print_json(&self) {
        println!(
            "{}",
            serde_json::to_string_pretty(&self.issues).unwrap()
        );
    }
}

fn main() {
    let cli = Cli::parse();
    let mut linter = Linter::new();

    let (path, format) = match cli.command {
        Commands::Lint { file, format } => (file, format),
        Commands::LintAll { path, format } => (path, format),
        Commands::Fix { path } => {
            println!(
                "{} Auto-fix is not yet implemented",
                "Warning:".yellow().bold()
            );
            return;
        }
    };

    if let Err(e) = linter.lint_directory(&path) {
        eprintln!("{} {}", "Error:".red().bold(), e);
        std::process::exit(1);
    }

    match format.as_str() {
        "json" => linter.print_json(),
        _ => linter.print_text(),
    }

    if linter.issues.iter().any(|i| matches!(i.severity, Severity::Error)) {
        std::process::exit(1);
    }
}
