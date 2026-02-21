//! Interactive REPL with rustyline for line editing and history.

use std::cell::RefCell;
use std::rc::Rc;

use rustyline::{config::Configurer, error::ReadlineError, DefaultEditor, EditMode};

use crate::eval::{eval_str, make_global_env};
use crate::types::{write_val, Env, Val};

const HISTORY_FILE: &str = ".scheme_history";
const PROMPT: &str = "scheme> ";
const PROMPT_CONT: &str = "       ";

/// Run the interactive REPL (new environment with stdlib).
#[allow(dead_code)]
pub fn run_repl() -> anyhow::Result<()> {
    let env = make_global_env();
    load_stdlib(&env);
    run_repl_with_env(env)
}

/// Run the interactive REPL with a pre-built environment.
pub fn run_repl_with_env(env: Rc<RefCell<Env>>) -> anyhow::Result<()> {
    let mut rl = DefaultEditor::new()?;
    rl.set_edit_mode(EditMode::Emacs);
    rl.set_auto_add_history(true);

    let _ = rl.load_history(HISTORY_FILE);

    println!("cprustscheme REPL (R5RS-compatible Scheme)");
    println!("Type (exit) or Ctrl-D to quit.");
    println!();

    let mut pending = String::new();

    loop {
        let prompt = if pending.is_empty() {
            PROMPT
        } else {
            PROMPT_CONT
        };

        match rl.readline(prompt) {
            Ok(line) => {
                pending.push_str(&line);
                pending.push('\n');

                // Check if the expression is complete
                match is_complete(&pending) {
                    InputStatus::Complete => {
                        let src = std::mem::take(&mut pending);
                        match eval_str(src.trim(), &env) {
                            Ok(Val::Void) => {} // don't print void
                            Ok(val) => println!("{}", write_val(&val)),
                            Err(e) => eprintln!("Error: {}", e),
                        }
                    }
                    InputStatus::Incomplete => {
                        // Keep accumulating
                    }
                    InputStatus::Empty => {
                        pending.clear();
                    }
                }
            }
            Err(ReadlineError::Interrupted) => {
                pending.clear();
                println!();
            }
            Err(ReadlineError::Eof) => {
                println!();
                break;
            }
            Err(e) => {
                eprintln!("Error: {}", e);
                break;
            }
        }
    }

    let _ = rl.save_history(HISTORY_FILE);
    Ok(())
}

/// Evaluate a file path in the given environment.
pub fn eval_file(path: &str, env: &Rc<RefCell<Env>>) -> anyhow::Result<()> {
    let src = std::fs::read_to_string(path)?;
    match eval_str(&src, env) {
        Ok(_) => {}
        Err(e) => eprintln!("Error: {}", e),
    }
    Ok(())
}

/// Input completion status.
enum InputStatus {
    /// Complete expression(s)
    Complete,
    /// Needs more input (unbalanced parens)
    Incomplete,
    /// Empty or whitespace only
    Empty,
}

/// Check whether a string contains a complete Scheme expression.
fn is_complete(input: &str) -> InputStatus {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return InputStatus::Empty;
    }

    let mut depth: i32 = 0;
    let mut in_string = false;
    let mut escape = false;
    let mut in_comment = false;

    for c in trimmed.chars() {
        if in_comment {
            if c == '\n' {
                in_comment = false;
            }
            continue;
        }
        if escape {
            escape = false;
            continue;
        }
        if in_string {
            if c == '\\' {
                escape = true;
            } else if c == '"' {
                in_string = false;
            }
            continue;
        }
        match c {
            ';' => in_comment = true,
            '"' => in_string = true,
            '(' | '[' => depth += 1,
            ')' | ']' => {
                depth -= 1;
                if depth < 0 {
                    depth = 0;
                }
            }
            _ => {}
        }
    }

    if in_string {
        return InputStatus::Incomplete;
    }

    if depth > 0 {
        InputStatus::Incomplete
    } else {
        InputStatus::Complete
    }
}

/// Load the standard library if available.
pub fn load_stdlib(env: &Rc<RefCell<Env>>) {
    // Try locations in priority order
    let locations = [
        "scheme/stdlib.scm",
        "../scheme/stdlib.scm",
        "/usr/share/cprustscheme/stdlib.scm",
    ];

    // Find binary directory
    if let Ok(exe) = std::env::current_exe() {
        if let Some(dir) = exe.parent() {
            let stdlib_path = dir.join("../scheme/stdlib.scm");
            if stdlib_path.exists() {
                if let Ok(src) = std::fs::read_to_string(&stdlib_path) {
                    let _ = eval_str(&src, env);
                    return;
                }
            }
        }
    }

    for loc in &locations {
        if let Ok(src) = std::fs::read_to_string(loc) {
            let _ = eval_str(&src, env);
            return;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_complete() {
        assert!(matches!(is_complete("(+ 1 2)"), InputStatus::Complete));
        assert!(matches!(is_complete("(+ 1"), InputStatus::Incomplete));
        assert!(matches!(is_complete(""), InputStatus::Empty));
        assert!(matches!(is_complete("\"hello"), InputStatus::Incomplete));
    }
}
