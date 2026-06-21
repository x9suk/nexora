use anyhow::Result;
use colored::*;
use std::fs;
use std::path::Path;

use crate::config::NexoraConfig;

pub fn execute(name: Option<String>) -> Result<()> {
    let config_path = Path::new("nexora.json");

    if config_path.exists() {
        println!(
            "{}",
            "nexora.json already exists in this directory".yellow()
        );
        return Ok(());
    }

    let project_name = name.unwrap_or_else(|| {
        std::env::current_dir()
            .ok()
            .and_then(|p| p.file_name().map(|n| n.to_string_lossy().to_string()))
            .unwrap_or_else(|| "my-nexora-project".to_string())
    });

    // Create nexora.json
    let config = NexoraConfig::new(&project_name);
    let json = serde_json::to_string_pretty(&config)?;
    fs::write(config_path, &json)?;

    // Create .gitignore
    let gitignore_content = r#"# Dependencies
nexora_modules/

# Build output
dist/
build/

# OS files
.DS_Store
Thumbs.db

# IDE
.vscode/
.idea/

# Logs
*.log
"#;

    let gitignore_path = Path::new(".gitignore");
    if !gitignore_path.exists() {
        fs::write(gitignore_path, gitignore_content)?;
    }

    // Create index.nx with hello world
    let index_content = r#"// Nexora - Hello World
print "Hello, World!"
print "Welcome to ${"Nexora"}!"
"#;

    let index_path = Path::new("index.nx");
    if !index_path.exists() {
        fs::write(index_path, index_content)?;
    }

    // Create test.nx
    let test_content = r#"// Nexora - Tests
test "basic test" {
    assert(1 + 1 == 2, "Math is broken!")
    assert("hello" == "hello", "String equality broken!")
    print "All tests passed!"
}
"#;

    let test_path = Path::new("test.nx");
    if !test_path.exists() {
        fs::write(test_path, test_content)?;
    }

    println!(
        "{}",
        "Created nexora.json".green().bold()
    );
    println!(
        "{}",
        "Created .gitignore".green().bold()
    );
    println!(
        "{}",
        "Created index.nx".green().bold()
    );
    println!(
        "{}",
        "Created test.nx".green().bold()
    );
    println!();
    println!("Project: {}", project_name.cyan());
    println!();
    println!("Next steps:");
    println!("  1. {} - Install dependencies", "nxm install".cyan());
    println!("  2. {} - Run your project", "nexora run index.nx".cyan());
    println!("  3. {} - Run tests", "nxm test".cyan());
    println!();
    println!("Folder structure:");
    println!("  {}", "my-project/".dimmed());
    println!("  ├── {}", "nexora.json".dimmed());
    println!("  ├── {}", ".gitignore".dimmed());
    println!("  ├── {}", "index.nx".dimmed());
    println!("  ├── {}", "test.nx".dimmed());
    println!("  └── {}", "nexora_modules/  (after nxm install)".dimmed());

    Ok(())
}
