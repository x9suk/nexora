mod ast;
mod compiler;
mod error;
mod interpreter;
mod lexer;
mod parser;
mod repl;
mod value;

use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    match args.len() {
        1 => {
            // No arguments: start REPL
            repl::run_repl();
        }
        2 => {
            if args[1] == "repl" {
                repl::run_repl();
            } else if args[1] == "help" || args[1] == "--help" || args[1] == "-h" {
                print_usage();
            } else {
                // Treat as file path
                run_file(&args[1]);
            }
        }
        3 => {
            if args[1] == "run" {
                run_file(&args[2]);
            } else if args[1] == "tokens" {
                run_tokens(&args[2]);
            } else {
                eprintln!("Unknown command: {}", args[1]);
                print_usage();
                process::exit(1);
            }
        }
        _ => {
            eprintln!("Too many arguments");
            print_usage();
            process::exit(1);
        }
    }
}

fn print_usage() {
    println!("Nexora - A simple programming language");
    println!();
    println!("Usage:");
    println!("  nexora              Start REPL");
    println!("  nexora <file>       Run a .nx file");
    println!("  nexora run <file>   Run a .nx file");
    println!("  nexora repl         Start REPL");
    println!("  nexora help         Show this help");
}

fn run_file(path: &str) {
    let source = match fs::read_to_string(path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Error reading file '{}': {}", path, e);
            process::exit(1);
        }
    };

    let mut lexer = lexer::Lexer::new(&source);
    let tokens = lexer.tokenize();

    let mut parser = parser::Parser::new(tokens);
    let stmts = parser.parse();

    let mut interp = interpreter::Interpreter::new();
    interp.set_base_path(path);
    if let Err(e) = interp.run(&stmts) {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}

fn run_tokens(path: &str) {
    let source = match fs::read_to_string(path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Error reading file '{}': {}", path, e);
            process::exit(1);
        }
    };

    let mut lexer = lexer::Lexer::new(&source);
    let tokens = lexer.tokenize();
    for (i, tok) in tokens.iter().enumerate() {
        println!("{:3}: {:?}", i, tok);
    }
}
