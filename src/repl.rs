use rustyline::Editor;
use rustyline::error::ReadlineError;

use crate::interpreter::Interpreter;
use crate::lexer::Lexer;
use crate::parser::Parser;

const VERSION: &str = "0.4.0";

const RESET: &str = "\x1b[0m";
const BOLD: &str = "\x1b[1m";
const DIM: &str = "\x1b[2m";
const RED: &str = "\x1b[31m";
const GREEN: &str = "\x1b[32m";
const YELLOW: &str = "\x1b[33m";
const BLUE: &str = "\x1b[34m";
const _MAGENTA: &str = "\x1b[35m";
const CYAN: &str = "\x1b[36m";
const _WHITE: &str = "\x1b[37m";

pub fn run_repl() {
    println!("{}{}Nexora REPL v{}{}", BOLD, CYAN, VERSION, RESET);
    println!("{}Type 'help' for commands, 'exit' to quit{}\n", DIM, RESET);

    let mut rl = Editor::<()>::new().expect("Failed to create readline editor");
    let mut interp = Interpreter::new();

    loop {
        let readline = rl.readline(&format!("{}nexora{}>{} ", BLUE, RESET, GREEN));
        match readline {
            Ok(line) => {
                let line = line.trim();
                if line.is_empty() {
                    continue;
                }

                match line {
                    "exit" | "quit" => {
                        println!("{}Goodbye!{}", DIM, RESET);
                        break;
                    }
                    "help" => {
                        println!("\n{}{}Commands:{}", BOLD, CYAN, RESET);
                        println!("  {}exit{} / {}quit{}     - Exit the REPL", YELLOW, RESET, YELLOW, RESET);
                        println!("  {}help{}          - Show this help", YELLOW, RESET);
                        println!("  {}version{}       - Show version", YELLOW, RESET);
                        println!("  {}clear{}         - Clear variables", YELLOW, RESET);
                        println!("  {}vars{}          - Show all variables", YELLOW, RESET);
                        println!();
                        println!("{}{}Syntax:{}", BOLD, CYAN, RESET);
                        println!("  {}let x = 10{}          - Variable", DIM, RESET);
                        println!("  {}func add(a, b) {}{}   - Function", DIM, "{}", RESET);
                        println!("  {}print(x){}            - Print", DIM, RESET);
                        println!("  {}import \"math\"{}       - Import", DIM, RESET);
                        println!("  {}class Dog {{ ... }}{}  - Class", DIM, RESET);
                        println!();
                        continue;
                    }
                    "version" => {
                        println!("{}Nexora v{}{}", CYAN, VERSION, RESET);
                        continue;
                    }
                    "clear" => {
                        interp = Interpreter::new();
                        println!("{}Variables cleared{}", GREEN, RESET);
                        continue;
                    }
                    "vars" => {
                        let globals = interp.get_globals();
                        if globals.is_empty() {
                            println!("{}No variables defined{}", DIM, RESET);
                        } else {
                            println!("\n{}{}Variables:{}", BOLD, CYAN, RESET);
                            for (name, val) in globals {
                                println!("  {}{}{} = {}", YELLOW, name, RESET, val);
                            }
                        }
                        println!();
                        continue;
                    }
                    _ => {}
                }

                let mut lexer = Lexer::new(line);
                let tokens = lexer.tokenize();
                let mut parser = Parser::new(tokens);
                let stmts = parser.parse();

                match interp.run(&stmts) {
                    Ok(_) => {}
                    Err(e) => {
                        eprintln!("{}Error: {}{}", RED, e, RESET);
                    }
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("{}Ctrl-C{}", YELLOW, RESET);
            }
            Err(ReadlineError::Eof) => {
                println!("{}Goodbye!{}", DIM, RESET);
                break;
            }
            Err(err) => {
                eprintln!("{}Error: {:?}{}", RED, err, RESET);
                break;
            }
        }
    }
}
