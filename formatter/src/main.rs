use std::fs;
use std::path::PathBuf;

use clap::{Parser, Subcommand};
use colored::*;

#[derive(Parser)]
#[command(name = "nxfmt")]
#[command(about = "Nexora Code Formatter")]
#[command(version = "0.1.0")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Format a file
    Format {
        /// Path to the .nx file
        file: PathBuf,
        /// Check mode (don't modify files)
        #[arg(short, long)]
        check: bool,
    },
    /// Format all .nx files in a directory
    FormatAll {
        /// Path to directory
        path: PathBuf,
        /// Check mode
        #[arg(short, long)]
        check: bool,
    },
}

struct Formatter {
    indent_size: usize,
    indent_char: char,
}

impl Formatter {
    fn new() -> Self {
        Formatter {
            indent_size: 4,
            indent_char: ' ',
        }
    }

    fn format(&self, source: &str) -> String {
        let mut output = String::new();
        let mut indent_level = 0;
        let mut in_string = false;
        let mut string_char = ' ';
        let mut chars = source.chars().peekable();
        let mut prev_char = '\n';

        while let Some(ch) = chars.next() {
            // Handle strings
            if in_string {
                output.push(ch);
                if ch == string_char && prev_char != '\\' {
                    in_string = false;
                }
                prev_char = ch;
                continue;
            }

            if ch == '"' || ch == '\'' {
                in_string = true;
                string_char = ch;
                output.push(ch);
                prev_char = ch;
                continue;
            }

            match ch {
                '{' => {
                    output.push(ch);
                    indent_level += 1;
                    output.push('\n');
                    output.push_str(&self.indent(indent_level));
                }
                '}' => {
                    indent_level = indent_level.saturating_sub(1);
                    output.push('\n');
                    output.push_str(&self.indent(indent_level));
                    output.push(ch);
                }
                ';' => {
                    output.push(ch);
                    output.push('\n');
                    output.push_str(&self.indent(indent_level));
                }
                '\n' => {
                    // Skip multiple newlines
                    if prev_char != '\n' {
                        output.push('\n');
                        output.push_str(&self.indent(indent_level));
                    }
                }
                ' ' => {
                    // Skip multiple spaces
                    if prev_char != ' ' && prev_char != '\n' {
                        output.push(ch);
                    }
                }
                _ => {
                    output.push(ch);
                }
            }

            prev_char = ch;
        }

        // Clean up trailing whitespace
        let lines: Vec<&str> = output.lines().collect();
        let formatted: Vec<String> = lines
            .iter()
            .map(|line| line.trim_end().to_string())
            .collect();

        formatted.join("\n")
    }

    fn indent(&self, level: usize) -> String {
        std::iter::repeat(self.indent_char)
            .take(level * self.indent_size)
            .collect()
    }

    fn format_file(&self, path: &PathBuf, check: bool) -> Result<bool, String> {
        let source = fs::read_to_string(path).map_err(|e| e.to_string())?;
        let formatted = self.format(&source);

        if source == formatted {
            return Ok(true); // Already formatted
        }

        if !check {
            fs::write(path, &formatted).map_err(|e| e.to_string())?;
        }

        Ok(false) // Was not formatted
    }

    fn format_directory(&self, path: &PathBuf, check: bool) -> Result<(usize, usize), String> {
        let mut total = 0;
        let mut formatted = 0;

        if path.is_dir() {
            for entry in fs::read_dir(path).map_err(|e| e.to_string())? {
                let entry = entry.map_err(|e| e.to_string())?;
                let path = entry.path();

                if path.is_dir() {
                    let (t, f) = self.format_directory(&path, check)?;
                    total += t;
                    formatted += f;
                } else if path.extension().and_then(|s| s.to_str()) == Some("nx") {
                    total += 1;
                    if !self.format_file(&path, check)? {
                        formatted += 1;
                    }
                }
            }
        } else if path.extension().and_then(|s| s.to_str()) == Some("nx") {
            total = 1;
            if !self.format_file(path, check)? {
                formatted = 1;
            }
        }

        Ok((total, formatted))
    }
}

fn main() {
    let cli = Cli::parse();
    let formatter = Formatter::new();

    match cli.command {
        Commands::Format { file, check } => {
            match formatter.format_file(&file, check) {
                Ok(true) => {
                    println!(
                        "{} {} is already formatted",
                        "OK:".green().bold(),
                        file.display()
                    );
                }
                Ok(false) => {
                    if check {
                        println!(
                            "{} {} needs formatting",
                            "Check:".yellow().bold(),
                            file.display()
                        );
                        std::process::exit(1);
                    } else {
                        println!(
                            "{} Formatted {}",
                            "Done:".green().bold(),
                            file.display()
                        );
                    }
                }
                Err(e) => {
                    eprintln!(
                        "{} Failed to format {}: {}",
                        "Error:".red().bold(),
                        file.display(),
                        e
                    );
                    std::process::exit(1);
                }
            }
        }
        Commands::FormatAll { path, check } => {
            match formatter.format_directory(&path, check) {
                Ok((total, formatted)) => {
                    if check {
                        if formatted > 0 {
                            println!(
                                "{} {} file(s) need formatting",
                                "Check:".yellow().bold(),
                                formatted
                            );
                            std::process::exit(1);
                        } else {
                            println!(
                                "{} All {} file(s) are formatted",
                                "OK:".green().bold(),
                                total
                            );
                        }
                    } else {
                        println!(
                            "{} Formatted {} file(s)",
                            "Done:".green().bold(),
                            formatted
                        );
                    }
                }
                Err(e) => {
                    eprintln!(
                        "{} Failed to format: {}",
                        "Error:".red().bold(),
                        e
                    );
                    std::process::exit(1);
                }
            }
        }
    }
}
