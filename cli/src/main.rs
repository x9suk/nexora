use std::fs;
use std::path::PathBuf;
use std::process;

use clap::{Parser, Subcommand};
use colored::*;
use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;

use nexora_compiler::parse;
use nexora_runtime::{run, Interpreter};

#[derive(Parser)]
#[command(name = "nx")]
#[command(about = "Nexora - AI-Native Programming Language")]
#[command(version = "0.1.0")]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Run a Nexora file
    Run {
        /// Path to the .nx file
        file: PathBuf,
    },
    /// Start the Nexora REPL (interactive shell)
    Repl,
    /// Format a Nexora file
    Fmt {
        /// Path to the .nx file or directory
        path: PathBuf,
    },
    /// Lint a Nexora file
    Lint {
        /// Path to the .nx file or directory
        path: PathBuf,
    },
    /// Run tests
    Test {
        /// Path to test file or directory
        path: Option<PathBuf>,
    },
    /// Create a new Nexora project
    New {
        /// Project name
        name: String,
    },
    /// Show Nexora version
    Version,
    /// Explain code in plain language
    Explain {
        /// Path to the .nx file
        file: PathBuf,
    },
    /// AI generate code from prompt
    Ai {
        /// The prompt for code generation
        prompt: String,
    },
    /// Check project for issues
    Doctor {
        /// Path to project directory
        path: PathBuf,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Run { file } => run_file(file),
        Commands::Repl => start_repl(),
        Commands::Fmt { path } => format_code(path),
        Commands::Lint { path } => lint_code(path),
        Commands::Test { path } => run_tests(path),
        Commands::New { name } => create_project(name),
        Commands::Version => show_version(),
        Commands::Explain { file } => explain_code(file),
        Commands::Ai { prompt } => ai_generate(prompt),
        Commands::Doctor { path } => doctor_project(path),
    }
}

fn run_file(file: PathBuf) {
    let source = match fs::read_to_string(&file) {
        Ok(s) => s,
        Err(e) => {
            eprintln!(
                "{} Failed to read file: {}",
                "Error:".red().bold(),
                e
            );
            process::exit(1);
        }
    };

    match run(&source) {
        Ok(_) => {}
        Err(e) => {
            eprintln!("{} {}", "Runtime Error:".red().bold(), e);
            process::exit(1);
        }
    }
}

fn start_repl() {
    println!(
        "{}",
        "Nexora REPL v0.1.0".cyan().bold()
    );
    println!("Type 'exit' or 'quit' to exit, 'help' for help\n");

    let mut rl = DefaultEditor::new().expect("Failed to create readline editor");
    let history_path = dirs::home_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join(".nexora_history");
    let _ = rl.load_history(&history_path);

    let mut interpreter = Interpreter::new();

    loop {
        let readline = rl.readline(&format!("{} ", "nexora>".green().bold()));
        match readline {
            Ok(line) => {
                let line = line.trim();
                if line.is_empty() {
                    continue;
                }

                let _ = rl.add_history_entry(line);

                match line {
                    "exit" | "quit" => {
                        println!("{}", "Goodbye!".cyan());
                        break;
                    }
                    "help" => {
                        println!("\n{}", "Nexora REPL Help:".cyan().bold());
                        println!("  exit/quit  - Exit the REPL");
                        println!("  help       - Show this help message");
                        println!("  clear      - Clear the screen");
                        println!("  history    - Show command history");
                        println!();
                        println!("You can type any Nexora code to evaluate it.\n");
                        continue;
                    }
                    "clear" => {
                        print!("{}[2J{}[1;1H", 27 as char, 27 as char);
                        continue;
                    }
                    "history" => {
                        let history = rl.history();
                        for (i, entry) in history.iter().enumerate() {
                            println!("  {}: {}", i + 1, entry);
                        }
                        continue;
                    }
                    _ => {}
                }

                // Try to parse and run the expression
                match parse(line) {
                    Ok(program) => {
                        match interpreter.interpret(&program) {
                            Ok(value) => {
                                if !matches!(value, nexora_runtime::Value::Null) {
                                    println!("{}", value);
                                }
                            }
                            Err(e) => {
                                eprintln!("{} {}", "Error:".red().bold(), e);
                            }
                        }
                    }
                    Err(e) => {
                        // Try as expression
                        let wrapped = format!("print {}", line);
                        if let Ok(program) = parse(&wrapped) {
                            match interpreter.interpret(&program) {
                                Ok(_) => {}
                                Err(e) => {
                                    eprintln!("{} {}", "Error:".red().bold(), e);
                                }
                            }
                        } else {
                            eprintln!("{} {}", "Parse Error:".red().bold(), e);
                        }
                    }
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("{}", "Ctrl-C".yellow());
                continue;
            }
            Err(ReadlineError::Eof) => {
                println!("{}", "Goodbye!".cyan());
                break;
            }
            Err(e) => {
                eprintln!("{} {}", "Error:".red().bold(), e);
                break;
            }
        }
    }

    let _ = rl.save_history(&history_path);
}

fn format_code(path: PathBuf) {
    if path.is_dir() {
        println!(
            "{} Formatting all .nx files in {}...",
            "Info:".cyan().bold(),
            path.display()
        );
        // TODO: Implement recursive formatting
    } else {
        println!(
            "{} Formatting {}...",
            "Info:".cyan().bold(),
            path.display()
        );
        // TODO: Implement file formatting
        println!("{} Formatter is not yet implemented", "Warning:".yellow().bold());
    }
}

fn lint_code(path: PathBuf) {
    if path.is_dir() {
        println!(
            "{} Linting all .nx files in {}...",
            "Info:".cyan().bold(),
            path.display()
        );
    } else {
        println!(
            "{} Linting {}...",
            "Info:".cyan().bold(),
            path.display()
        );
        // TODO: Implement linting
        println!("{} Linter is not yet implemented", "Warning:".yellow().bold());
    }
}

fn run_tests(path: Option<PathBuf>) {
    let test_path = path.unwrap_or_else(|| PathBuf::from("."));
    println!(
        "{} Running tests in {}...",
        "Info:".cyan().bold(),
        test_path.display()
    );
    // TODO: Implement test runner
    println!("{} Test runner is not yet implemented", "Warning:".yellow().bold());
}

fn create_project(name: String) {
    let project_dir = PathBuf::from(&name);

    if project_dir.exists() {
        eprintln!(
            "{} Directory '{}' already exists",
            "Error:".red().bold(),
            name
        );
        process::exit(1);
    }

    fs::create_dir_all(&project_dir).expect("Failed to create project directory");
    fs::create_dir_all(project_dir.join("src")).expect("Failed to create src directory");
    fs::create_dir_all(project_dir.join("tests")).expect("Failed to create tests directory");

    // Create main.nx
    let main_content = r#"print "Hello from Nexora!"
print "Welcome to " + "#;
    fs::write(project_dir.join("src/main.nx"), main_content)
        .expect("Failed to create main.nx");

    // Create nexora.json config
    let config = serde_json::json!({
        "name": name,
        "version": "0.1.0",
        "description": "",
        "main": "src/main.nx",
        "ai": {
            "autocomplete": true,
            "autofix": true,
            "explain": true,
            "optimize": true
        }
    });
    fs::write(
        project_dir.join("nexora.json"),
        serde_json::to_string_pretty(&config).unwrap(),
    )
    .expect("Failed to create nexora.json");

    // Create README
    let readme = format!(
        r#"# {}

A new Nexora project.

## Getting Started

Run your project:
```bash
nx run src/main.nx
```

Start the REPL:
```bash
nx repl
```

## AI Features

Ask AI to help you:
```bash
nx ai "create a hello world function"
```

Explain code:
```bash
nx explain src/main.nx
```
"#,
        name
    );
    fs::write(project_dir.join("README.md"), readme).expect("Failed to create README");

    println!(
        "{} Created project '{}' successfully!",
        "Success:".green().bold(),
        name
    );
    println!("\nNext steps:");
    println!("  cd {}", name);
    println!("  nx run src/main.nx");
}

fn show_version() {
    println!(
        "{} {}",
        "Nexora".cyan().bold(),
        "v0.1.0".green()
    );
    println!("AI-Native Programming Language");
    println!("Copyright (c) 2026 Nexora Team");
}

fn explain_code(file: PathBuf) {
    let source = match fs::read_to_string(&file) {
        Ok(s) => s,
        Err(e) => {
            eprintln!(
                "{} Failed to read file: {}",
                "Error:".red().bold(),
                e
            );
            process::exit(1);
        }
    };

    println!(
        "\n{} Explaining {}...\n",
        "AI:".cyan().bold(),
        file.display()
    );

    // Parse and explain the code structure
    match parse(&source) {
        Ok(program) => {
            println!("This file contains {} statement(s):\n", program.stmts.len());
            for (i, stmt) in program.stmts.iter().enumerate() {
                explain_statement(stmt, i + 1);
            }
        }
        Err(e) => {
            eprintln!("{} {}", "Parse Error:".red().bold(), e);
        }
    }
}

fn explain_statement(stmt: &nexora_compiler::Stmt, num: usize) {
    use nexora_compiler::Stmt;
    match stmt {
        Stmt::VarDecl { name, value, is_const, .. } => {
            let keyword = if *is_const { "Constant" } else { "Variable" };
            println!("  {}. {} declaration: '{}'", num, keyword, name);
        }
        Stmt::FuncDecl { name, params, .. } => {
            let param_names: Vec<String> = params.iter().map(|p| p.name.clone()).collect();
            println!(
                "  {}. Function '{}' with parameters: [{}]",
                num,
                name,
                param_names.join(", ")
            );
        }
        Stmt::If { .. } => {
            println!("  {}. Conditional (if) statement", num);
        }
        Stmt::While { .. } => {
            println!("  {}. While loop", num);
        }
        Stmt::For { variable, .. } => {
            println!("  {}. For loop with variable '{}'", num, variable);
        }
        Stmt::ClassDecl { name, .. } => {
            println!("  {}. Class definition: '{}'", num, name);
        }
        Stmt::Import { module, .. } => {
            println!("  {}. Import statement for module: '{}'", num, module);
        }
        Stmt::Expr(expr) => {
            println!("  {}. Expression statement", num);
        }
        _ => {
            println!("  {}. Statement", num);
        }
    }
}

fn ai_generate(prompt: String) {
    println!(
        "\n{} Generating code for: \"{}\"\n",
        "AI:".cyan().bold(),
        prompt
    );

    // Placeholder for AI generation
    println!("{}\n", "AI code generation will be available soon with the AI engine.".yellow());
    println!("For now, you can use Nexora's syntax to write code manually.");
    println!("\nExample:");
    println!("  func {}() {{", prompt.replace(' ', "_").to_lowercase());
    println!("      print \"Implementation here\"");
    println!("  }}");
}

fn doctor_project(path: PathBuf) {
    println!(
        "\n{} Scanning project at {}...\n",
        "Doctor:".cyan().bold(),
        path.display()
    );

    let mut issues = 0;
    let mut warnings = 0;

    // Check for nexora.json
    if !path.join("nexora.json").exists() {
        println!(
            "  {} Missing nexora.json configuration file",
            "Warning:".yellow().bold()
        );
        warnings += 1;
    } else {
        println!("  {} Found nexora.json", "OK:".green().bold());
    }

    // Check for src directory
    if !path.join("src").exists() {
        println!(
            "  {} Missing src/ directory",
            "Warning:".yellow().bold()
        );
        warnings += 1;
    } else {
        println!("  {} Found src/ directory", "OK:".green().bold());
    }

    // Scan for .nx files
    let mut nx_files = Vec::new();
    if path.join("src").exists() {
        for entry in fs::read_dir(path.join("src")).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("nx") {
                nx_files.push(path);
            }
        }
    }

    if nx_files.is_empty() {
        println!(
            "  {} No .nx files found in src/",
            "Info:".cyan().bold()
        );
    } else {
        println!(
            "  {} Found {} .nx file(s)",
            "OK:".green().bold(),
            nx_files.len()
        );

        // Try to parse each file
        for file in &nx_files {
            let source = fs::read_to_string(file).unwrap();
            match parse(&source) {
                Ok(_) => {
                    println!(
                        "    {} {} - OK",
                        "OK:".green().bold(),
                        file.file_name().unwrap().to_str().unwrap()
                    );
                }
                Err(e) => {
                    println!(
                        "    {} {} - {}",
                        "Error:".red().bold(),
                        file.file_name().unwrap().to_str().unwrap(),
                        e
                    );
                    issues += 1;
                }
            }
        }
    }

    println!("\n{}", "Summary:".cyan().bold());
    println!("  Issues: {}", issues);
    println!("  Warnings: {}", warnings);

    if issues == 0 && warnings == 0 {
        println!(
            "\n{} Your project looks healthy!",
            "Success:".green().bold()
        );
    } else if issues == 0 {
        println!(
            "\n{} No critical issues found. {} warning(s).",
            "OK:".green().bold(),
            warnings
        );
    } else {
        println!(
            "\n{} Found {} issue(s) that need attention.",
            "Warning:".yellow().bold(),
            issues
        );
    }
}
