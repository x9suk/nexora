use anyhow::Result;
use colored::*;
use std::fs;
use std::path::PathBuf;

pub fn execute() -> Result<()> {
    println!("{}", "📦 Nexora Modules".green().bold());
    println!();

    // 1. Standard library modules
    println!("{}", "Standard Library:".cyan().bold());
    let stdlib = vec![
        ("math", "Math functions (sqrt, pow, abs, min, max, etc.)"),
        ("string", "String operations (split, join, contains, upper, lower, etc.)"),
        ("collection", "Collection operations (map, filter, reduce, sort, etc.)"),
        ("http", "HTTP client (get, post, put, delete, async)"),
        ("fs", "File system operations (read, write, append, exists, mkdir, etc.)"),
        ("json", "JSON parsing (parse, stringify)"),
        ("os", "OS operations (env, args, platform, sleep, exec)"),
        ("time", "Time operations (now, sleep, timestamp)"),
        ("test", "Testing framework (describe, it, expect, assert)"),
    ];

    for (name, desc) in &stdlib {
        println!("  {} {}", name.green().bold(), desc.dimmed());
    }

    // 2. Game packages
    println!();
    println!("{}", "Game Packages:".magenta().bold());
    let games = vec![
        ("minecraft-nx", "Minecraft bot framework (connect, chat, build, fight, pathfind)"),
        ("steam-nx", "Steam API (users, games, achievements, inventory, trades)"),
        ("roblox-nx", "Roblox API (users, games, groups, badges, inventory)"),
        ("discord-nx", "Discord bot framework (client, embeds, slash commands, buttons)"),
    ];

    for (name, desc) in &games {
        println!("  {} {}", name.magenta().bold(), desc.dimmed());
    }

    // 2. Check local nexora_modules/
    let local_modules = PathBuf::from("nexora_modules");
    if local_modules.exists() {
        println!();
        println!("{}", "Local Packages (nexora_modules/):".cyan().bold());

        if let Ok(entries) = fs::read_dir(&local_modules) {
            let mut found = false;
            for entry in entries.flatten() {
                if entry.file_type().map(|t| t.is_dir()).unwrap_or(false) {
                    let name = entry.file_name().to_string_lossy().to_string();
                    let config_path = entry.path().join("nexora.json");
                    let version = if config_path.exists() {
                        fs::read_to_string(&config_path)
                            .ok()
                            .and_then(|c| serde_json::from_str::<serde_json::Value>(&c).ok())
                            .and_then(|v| v.get("version")?.as_str()?.to_string().into())
                            .unwrap_or_else(|| "unknown".to_string())
                    } else {
                        "unknown".to_string()
                    };

                    println!("  {} {}", name.green().bold(), version.yellow());
                    found = true;
                }
            }

            if !found {
                println!("  {}", "(empty)".dimmed());
            }
        }
    }

    // 3. Check global modules
    let global_dir = dirs::home_dir()
        .map(|h| h.join(".nexora").join("packages"))
        .unwrap_or_default();

    if global_dir.exists() {
        println!();
        println!("{}", "Global Packages (~/.nexora/packages/):".cyan().bold());

        if let Ok(entries) = fs::read_dir(&global_dir) {
            let mut found = false;
            for entry in entries.flatten() {
                if entry.file_type().map(|t| t.is_dir()).unwrap_or(false) {
                    let name = entry.file_name().to_string_lossy().to_string();
                    let config_path = entry.path().join("nexora.json");
                    let version = if config_path.exists() {
                        fs::read_to_string(&config_path)
                            .ok()
                            .and_then(|c| serde_json::from_str::<serde_json::Value>(&c).ok())
                            .and_then(|v| v.get("version")?.as_str()?.to_string().into())
                            .unwrap_or_else(|| "unknown".to_string())
                    } else {
                        "unknown".to_string()
                    };

                    println!("  {} {}", name.green().bold(), version.yellow());
                    found = true;
                }
            }

            if !found {
                println!("  {}", "(empty)".dimmed());
            }
        }
    }

    println!();
    println!("{}", "Import with: import { func } from \"module\"".dimmed());

    Ok(())
}
