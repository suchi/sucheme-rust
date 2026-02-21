//! cprustscheme - R5RS-compliant Scheme interpreter in Rust.

mod builtins;
mod eval;
mod lexer;
mod parser;
mod repl;
mod types;
mod web;

use std::process;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    match run(args) {
        Ok(()) => {}
        Err(e) => {
            eprintln!("Fatal error: {}", e);
            process::exit(1);
        }
    }
}

fn run(args: Vec<String>) -> anyhow::Result<()> {
    // Parse command-line options
    let mut files: Vec<String> = Vec::new();
    let mut web_mode = false;
    let mut port: u16 = 8080;
    let mut i = 1;

    while i < args.len() {
        match args[i].as_str() {
            "--web" | "-w" => {
                web_mode = true;
            }
            "--port" | "-p" => {
                i += 1;
                if i < args.len() {
                    port = args[i].parse().unwrap_or(8080);
                }
            }
            "--help" | "-h" => {
                print_help();
                return Ok(());
            }
            "--version" | "-v" => {
                println!("cprustscheme {}", env!("CARGO_PKG_VERSION"));
                return Ok(());
            }
            "--eval" | "-e" => {
                i += 1;
                if i < args.len() {
                    let env = eval::make_global_env();
                    repl::load_stdlib(&env);
                    match eval::eval_str(&args[i], &env) {
                        Ok(types::Val::Void) => {}
                        Ok(val) => println!("{}", types::write_val(&val)),
                        Err(e) => {
                            eprintln!("Error: {}", e);
                            process::exit(1);
                        }
                    }
                    return Ok(());
                }
            }
            arg if !arg.starts_with('-') => {
                files.push(arg.to_string());
            }
            other => {
                eprintln!("Unknown option: {}", other);
                print_help();
                process::exit(1);
            }
        }
        i += 1;
    }

    if web_mode {
        // Start the web server
        actix_web::rt::System::new().block_on(web::run_web_server(port))?;
        return Ok(());
    }

    if !files.is_empty() {
        // Batch mode: evaluate files
        let env = eval::make_global_env();
        repl::load_stdlib(&env);
        for file in &files {
            repl::eval_file(file, &env)?;
        }
        return Ok(());
    }

    // Interactive REPL
    repl::run_repl()?;
    Ok(())
}

fn print_help() {
    println!("Usage: cprustscheme [OPTIONS] [FILE...]");
    println!();
    println!("Options:");
    println!("  -h, --help           Show this help");
    println!("  -v, --version        Show version");
    println!("  -e, --eval CODE      Evaluate expression");
    println!("  -w, --web            Start web server");
    println!("  -p, --port PORT      Web server port (default: 8080)");
    println!();
    println!("If FILE is given, evaluate it in batch mode.");
    println!("If no arguments are given, start interactive REPL.");
}
