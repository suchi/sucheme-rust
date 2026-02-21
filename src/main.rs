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
    let mut load_files: Vec<String> = Vec::new();
    let mut eval_exprs: Vec<String> = Vec::new();
    let mut web_mode = false;
    let mut port: u16 = 8080;
    let mut no_stdlib = false;
    let mut interactive = false;
    let mut quiet = false;
    let mut script_args: Vec<String> = Vec::new();
    let mut collect_script_args = false;
    let mut i = 1;

    while i < args.len() {
        if collect_script_args {
            script_args.push(args[i].clone());
            i += 1;
            continue;
        }

        match args[i].as_str() {
            "--" => {
                // Everything after -- is passed as script arguments
                collect_script_args = true;
            }
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
                    eval_exprs.push(args[i].clone());
                }
            }
            "--load" | "-l" => {
                i += 1;
                if i < args.len() {
                    load_files.push(args[i].clone());
                }
            }
            "--no-stdlib" | "--no-init" | "-n" => {
                no_stdlib = true;
            }
            "--interactive" | "-i" => {
                interactive = true;
            }
            "--quiet" | "-q" => {
                quiet = true;
            }
            "--script" => {
                // Treat the next argument as a script file (implies --no-init for R7RS compat)
                // but we keep stdlib for convenience
                i += 1;
                if i < args.len() {
                    files.push(args[i].clone());
                    // Everything after the script is script args
                    collect_script_args = true;
                }
            }
            arg if !arg.starts_with('-') => {
                files.push(arg.to_string());
                // After the first file, rest are script args (like MIT-Scheme / Guile behavior)
                collect_script_args = true;
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

    // Build the evaluation environment
    let env = eval::make_global_env();
    if !no_stdlib {
        repl::load_stdlib(&env);
    }

    // Make script arguments available as `*argv*`
    if !script_args.is_empty() {
        use std::cell::RefCell;
        use std::rc::Rc;
        let argv = types::Val::list(
            script_args
                .iter()
                .map(|s| types::Val::Str(Rc::new(RefCell::new(s.clone())))),
        );
        env.borrow_mut().define("*argv*".to_string(), argv);
    } else {
        env.borrow_mut()
            .define("*argv*".to_string(), types::Val::Nil);
    }

    // --load files (loaded before any --eval or script file)
    for file in &load_files {
        repl::eval_file(file, &env)?;
    }

    // --eval expressions
    for expr in &eval_exprs {
        match eval::eval_str(expr, &env) {
            Ok(types::Val::Void) => {}
            Ok(val) => {
                if !quiet {
                    println!("{}", types::write_val(&val));
                }
            }
            Err(e) => {
                eprintln!("Error: {}", e);
                process::exit(1);
            }
        }
    }

    // Script files
    if !files.is_empty() {
        for file in &files {
            repl::eval_file(file, &env)?;
        }
        // --interactive flag: drop into REPL after file
        if interactive {
            repl::run_repl_with_env(env)?;
        }
        return Ok(());
    }

    // If only --eval was given (no files), stop here unless -i
    if !eval_exprs.is_empty() && !interactive {
        return Ok(());
    }

    // Interactive REPL
    repl::run_repl_with_env(env)?;
    Ok(())
}

fn print_help() {
    println!("Usage: cprustscheme [OPTIONS] [FILE [ARGS...]]");
    println!();
    println!("Options:");
    println!("  -h, --help           Show this help message");
    println!("  -v, --version        Show version information");
    println!("  -e, --eval EXPR      Evaluate EXPR (can be used multiple times)");
    println!("  -l, --load FILE      Load and evaluate FILE before REPL/script");
    println!("  -i, --interactive    Enter REPL even after evaluating FILE or -e");
    println!("  -q, --quiet          Suppress startup banner");
    println!("  -n, --no-stdlib      Do not load the standard library");
    println!("      --no-init        Alias for --no-stdlib");
    println!("      --script FILE    Execute FILE as a script (args follow)");
    println!("  -w, --web            Start the web REPL server");
    println!("  -p, --port PORT      Web server port (default: 8080)");
    println!("      --               Pass remaining args to script as *argv*");
    println!();
    println!("Examples:");
    println!("  cprustscheme                     # Interactive REPL");
    println!("  cprustscheme -e '(+ 1 2)'        # Evaluate expression");
    println!("  cprustscheme script.scm           # Run a script file");
    println!("  cprustscheme -l lib.scm script.scm  # Load lib then run script");
    println!("  cprustscheme --script script.scm -- arg1 arg2  # Script with args");
    println!("  cprustscheme -e '(display \"hi\")' -i  # Eval then REPL");
    println!("  cprustscheme --web --port 3000    # Web REPL on port 3000");
}
