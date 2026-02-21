//! Scheme evaluator with tail-call optimization (TCO).

use std::cell::RefCell;
use std::rc::Rc;

use crate::builtins::register_builtins;
use crate::parser::parse;
use crate::types::{Env, LambdaParams, Proc, SchemeError, SchemeResult, Val};

/// Create a new global environment with all builtins registered.
pub fn make_global_env() -> Rc<RefCell<Env>> {
    let env = Env::new();
    register_builtins(&env);
    env
}

/// Evaluate a source string in the given environment.
pub fn eval_str(source: &str, env: &Rc<RefCell<Env>>) -> SchemeResult {
    let exprs = parse(source)?;
    let mut result = Val::Void;
    for expr in exprs {
        result = eval(expr, env)?;
    }
    Ok(result)
}

/// Evaluate a Scheme expression with TCO using a trampoline loop.
pub fn eval(expr: Val, env: &Rc<RefCell<Env>>) -> SchemeResult {
    let mut expr = expr;
    let mut env = env.clone();

    loop {
        match expr {
            // Self-evaluating
            Val::Int(_)
            | Val::Float(_)
            | Val::Bool(_)
            | Val::Char(_)
            | Val::Str(_)
            | Val::Vector(_)
            | Val::Void
            | Val::Nil
            | Val::Eof
            | Val::Port(_)
            | Val::Proc(_) => return Ok(expr),

            // Variable lookup
            Val::Symbol(ref sym) => {
                return env
                    .borrow()
                    .get(sym)
                    .ok_or_else(|| SchemeError::UnboundVariable(sym.to_string()));
            }

            // Pair (application or special form)
            Val::Pair(_) => {
                let (operator, operands) = {
                    let car = expr.car()?;
                    let cdr = expr.cdr()?;
                    (car, cdr)
                };

                // Check for special forms
                if let Val::Symbol(ref sym) = operator {
                    match sym.as_ref() {
                        "quote" => {
                            return operands.car();
                        }
                        "if" => {
                            let items = operands.to_vec()?;
                            if items.is_empty() {
                                return Err(SchemeError::Eval("if: missing condition".to_string()));
                            }
                            let cond = eval(items[0].clone(), &env)?;
                            if cond.is_truthy() {
                                if items.len() < 2 {
                                    return Err(SchemeError::Eval(
                                        "if: missing consequent".to_string(),
                                    ));
                                }
                                expr = items[1].clone(); // TCO
                            } else if items.len() >= 3 {
                                expr = items[2].clone(); // TCO
                            } else {
                                return Ok(Val::Void);
                            }
                            continue;
                        }
                        "cond" => {
                            let clauses = operands.to_vec()?;
                            let mut result = Val::Void;
                            let mut found = false;
                            for clause in &clauses {
                                let items = clause.to_vec()?;
                                if items.is_empty() {
                                    return Err(SchemeError::Eval(
                                        "cond: empty clause".to_string(),
                                    ));
                                }
                                let test = &items[0];
                                // else clause
                                let is_else = matches!(test, Val::Bool(true))
                                    || matches!(test, Val::Symbol(s) if s.as_ref() == "else");
                                let test_val = if is_else {
                                    Val::Bool(true)
                                } else {
                                    eval(test.clone(), &env)?
                                };
                                if test_val.is_truthy() {
                                    found = true;
                                    if items.len() == 1 {
                                        result = test_val;
                                    } else if items.len() >= 3
                                        && matches!(&items[1], Val::Symbol(s) if s.as_ref() == "=>")
                                    {
                                        // (cond (test => proc))
                                        let proc = eval(items[2].clone(), &env)?;
                                        result = apply(proc, vec![test_val], &env)?;
                                    } else {
                                        // evaluate body sequentially
                                        for body_expr in &items[1..items.len() - 1] {
                                            eval(body_expr.clone(), &env)?;
                                        }
                                        result = eval(items.last().unwrap().clone(), &env)?;
                                    }
                                    break;
                                }
                            }
                            if !found {
                                return Ok(Val::Void);
                            }
                            return Ok(result);
                        }
                        "and" => {
                            let items = operands.to_vec()?;
                            if items.is_empty() {
                                return Ok(Val::Bool(true));
                            }
                            for item in &items[..items.len() - 1] {
                                let v = eval(item.clone(), &env)?;
                                if !v.is_truthy() {
                                    return Ok(Val::Bool(false));
                                }
                            }
                            expr = items.last().unwrap().clone(); // TCO
                            continue;
                        }
                        "or" => {
                            let items = operands.to_vec()?;
                            if items.is_empty() {
                                return Ok(Val::Bool(false));
                            }
                            for item in &items[..items.len() - 1] {
                                let v = eval(item.clone(), &env)?;
                                if v.is_truthy() {
                                    return Ok(v);
                                }
                            }
                            expr = items.last().unwrap().clone(); // TCO
                            continue;
                        }
                        "when" => {
                            let items = operands.to_vec()?;
                            if items.is_empty() {
                                return Err(SchemeError::Eval("when: missing test".to_string()));
                            }
                            let cond = eval(items[0].clone(), &env)?;
                            if cond.is_truthy() {
                                for body in &items[1..items.len() - 1] {
                                    eval(body.clone(), &env)?;
                                }
                                if items.len() > 1 {
                                    expr = items.last().unwrap().clone();
                                    continue;
                                }
                            }
                            return Ok(Val::Void);
                        }
                        "unless" => {
                            let items = operands.to_vec()?;
                            if items.is_empty() {
                                return Err(SchemeError::Eval("unless: missing test".to_string()));
                            }
                            let cond = eval(items[0].clone(), &env)?;
                            if !cond.is_truthy() {
                                for body in &items[1..items.len() - 1] {
                                    eval(body.clone(), &env)?;
                                }
                                if items.len() > 1 {
                                    expr = items.last().unwrap().clone();
                                    continue;
                                }
                            }
                            return Ok(Val::Void);
                        }
                        "define" => {
                            let items = operands.to_vec()?;
                            if items.is_empty() {
                                return Err(SchemeError::Eval("define: missing name".to_string()));
                            }
                            match &items[0] {
                                Val::Symbol(name) => {
                                    let val = if items.len() > 1 {
                                        eval(items[1].clone(), &env)?
                                    } else {
                                        Val::Void
                                    };
                                    // If defining a lambda, name it
                                    let val = match val {
                                        Val::Proc(Proc::Lambda {
                                            params,
                                            body,
                                            env: lenv,
                                            name: None,
                                        }) => Val::Proc(Proc::Lambda {
                                            params,
                                            body,
                                            env: lenv,
                                            name: Some(name.to_string()),
                                        }),
                                        other => other,
                                    };
                                    env.borrow_mut().define(name.to_string(), val);
                                    return Ok(Val::Void);
                                }
                                Val::Pair(_) => {
                                    // (define (name params...) body...)
                                    let name_and_params = items[0].to_vec()?;
                                    if name_and_params.is_empty() {
                                        return Err(SchemeError::Eval(
                                            "define: invalid form".to_string(),
                                        ));
                                    }
                                    let name = match &name_and_params[0] {
                                        Val::Symbol(s) => s.to_string(),
                                        _ => {
                                            return Err(SchemeError::Eval(
                                                "define: name must be a symbol".to_string(),
                                            ))
                                        }
                                    };
                                    let params = parse_params(&items[0].cdr()?)?;
                                    let body = items[1..].to_vec();
                                    let proc = Val::Proc(Proc::Lambda {
                                        params,
                                        body,
                                        env: env.clone(),
                                        name: Some(name.clone()),
                                    });
                                    env.borrow_mut().define(name, proc);
                                    return Ok(Val::Void);
                                }
                                _ => {
                                    return Err(SchemeError::Eval(
                                        "define: invalid form".to_string(),
                                    ))
                                }
                            }
                        }
                        "define-syntax" => {
                            let items = operands.to_vec()?;
                            if items.len() < 2 {
                                return Err(SchemeError::Eval(
                                    "define-syntax: invalid form".to_string(),
                                ));
                            }
                            let name = match &items[0] {
                                Val::Symbol(s) => s.to_string(),
                                _ => {
                                    return Err(SchemeError::Eval(
                                        "define-syntax: name must be symbol".to_string(),
                                    ))
                                }
                            };
                            let transformer = eval(items[1].clone(), &env)?;
                            env.borrow_mut().define(name, transformer);
                            return Ok(Val::Void);
                        }
                        "syntax-rules" => {
                            // Store as a special syntax-rules value
                            return Ok(Val::Proc(Proc::Lambda {
                                params: LambdaParams::Fixed(vec![]),
                                body: vec![expr.clone()],
                                env: env.clone(),
                                name: Some("syntax-rules".to_string()),
                            }));
                        }
                        "let-syntax" | "letrec-syntax" => {
                            // Simplified: treat bindings as regular let
                            let items = operands.to_vec()?;
                            if items.len() < 2 {
                                return Err(SchemeError::Eval(format!("{}: invalid form", sym)));
                            }
                            let bindings = items[0].to_vec()?;
                            let new_env = Env::extend(env.clone());
                            for b in &bindings {
                                let bv = b.to_vec()?;
                                if bv.len() != 2 {
                                    return Err(SchemeError::Eval(
                                        "let-syntax: invalid binding".to_string(),
                                    ));
                                }
                                let name = match &bv[0] {
                                    Val::Symbol(s) => s.to_string(),
                                    _ => {
                                        return Err(SchemeError::Eval(
                                            "let-syntax: name must be symbol".to_string(),
                                        ))
                                    }
                                };
                                let val = eval(bv[1].clone(), &env)?;
                                new_env.borrow_mut().define(name, val);
                            }
                            env = new_env;
                            let body = &items[1..];
                            for b in &body[..body.len() - 1] {
                                eval(b.clone(), &env)?;
                            }
                            expr = body.last().unwrap().clone();
                            continue;
                        }
                        "set!" => {
                            let items = operands.to_vec()?;
                            if items.len() != 2 {
                                return Err(SchemeError::Eval("set!: expected 2 args".to_string()));
                            }
                            let name = match &items[0] {
                                Val::Symbol(s) => s.to_string(),
                                _ => {
                                    return Err(SchemeError::Eval(
                                        "set!: first arg must be symbol".to_string(),
                                    ))
                                }
                            };
                            let val = eval(items[1].clone(), &env)?;
                            if !env.borrow_mut().set(&name, val) {
                                return Err(SchemeError::UnboundVariable(name));
                            }
                            return Ok(Val::Void);
                        }
                        "lambda" => {
                            let items = operands.to_vec()?;
                            if items.is_empty() {
                                return Err(SchemeError::Eval(
                                    "lambda: missing params".to_string(),
                                ));
                            }
                            let params = parse_params(&items[0])?;
                            let body = items[1..].to_vec();
                            return Ok(Val::Proc(Proc::Lambda {
                                params,
                                body,
                                env: env.clone(),
                                name: None,
                            }));
                        }
                        "begin" => {
                            let items = operands.to_vec()?;
                            if items.is_empty() {
                                return Ok(Val::Void);
                            }
                            for item in &items[..items.len() - 1] {
                                eval(item.clone(), &env)?;
                            }
                            expr = items.last().unwrap().clone(); // TCO
                            continue;
                        }
                        "let" => {
                            let items = operands.to_vec()?;
                            if items.is_empty() {
                                return Err(SchemeError::Eval("let: invalid form".to_string()));
                            }
                            // Named let: (let name ((var init) ...) body...)
                            if let Val::Symbol(ref name) = items[0] {
                                let name = name.to_string();
                                let bindings = items[1].to_vec()?;
                                let body = items[2..].to_vec();
                                let new_env = Env::extend(env.clone());
                                let mut params = Vec::new();
                                let mut inits = Vec::new();
                                for b in &bindings {
                                    let bv = b.to_vec()?;
                                    if bv.len() != 2 {
                                        return Err(SchemeError::Eval(
                                            "let: invalid binding".to_string(),
                                        ));
                                    }
                                    let param = match &bv[0] {
                                        Val::Symbol(s) => s.to_string(),
                                        _ => {
                                            return Err(SchemeError::Eval(
                                                "let: param must be symbol".to_string(),
                                            ))
                                        }
                                    };
                                    params.push(param);
                                    inits.push(eval(bv[1].clone(), &env)?);
                                }
                                let proc = Val::Proc(Proc::Lambda {
                                    params: LambdaParams::Fixed(params),
                                    body,
                                    env: new_env.clone(),
                                    name: Some(name.clone()),
                                });
                                new_env.borrow_mut().define(name, proc.clone());
                                return apply(proc, inits, &env);
                            }
                            // Regular let
                            let bindings = items[0].to_vec()?;
                            let new_env = Env::extend(env.clone());
                            for b in &bindings {
                                let bv = b.to_vec()?;
                                if bv.len() != 2 {
                                    return Err(SchemeError::Eval(
                                        "let: invalid binding".to_string(),
                                    ));
                                }
                                let name = match &bv[0] {
                                    Val::Symbol(s) => s.to_string(),
                                    _ => {
                                        return Err(SchemeError::Eval(
                                            "let: param must be symbol".to_string(),
                                        ))
                                    }
                                };
                                let val = eval(bv[1].clone(), &env)?;
                                new_env.borrow_mut().define(name, val);
                            }
                            env = new_env;
                            let body = &items[1..];
                            if body.is_empty() {
                                return Ok(Val::Void);
                            }
                            for b in &body[..body.len() - 1] {
                                eval(b.clone(), &env)?;
                            }
                            expr = body.last().unwrap().clone(); // TCO
                            continue;
                        }
                        "let*" => {
                            let items = operands.to_vec()?;
                            if items.is_empty() {
                                return Err(SchemeError::Eval("let*: invalid form".to_string()));
                            }
                            let bindings = items[0].to_vec()?;
                            let mut cur_env = env.clone();
                            for b in &bindings {
                                let bv = b.to_vec()?;
                                if bv.len() != 2 {
                                    return Err(SchemeError::Eval(
                                        "let*: invalid binding".to_string(),
                                    ));
                                }
                                let name = match &bv[0] {
                                    Val::Symbol(s) => s.to_string(),
                                    _ => {
                                        return Err(SchemeError::Eval(
                                            "let*: param must be symbol".to_string(),
                                        ))
                                    }
                                };
                                let val = eval(bv[1].clone(), &cur_env)?;
                                let new_env = Env::extend(cur_env);
                                new_env.borrow_mut().define(name, val);
                                cur_env = new_env;
                            }
                            env = cur_env;
                            let body = &items[1..];
                            if body.is_empty() {
                                return Ok(Val::Void);
                            }
                            for b in &body[..body.len() - 1] {
                                eval(b.clone(), &env)?;
                            }
                            expr = body.last().unwrap().clone();
                            continue;
                        }
                        "letrec" | "letrec*" => {
                            let items = operands.to_vec()?;
                            if items.is_empty() {
                                return Err(SchemeError::Eval("letrec: invalid form".to_string()));
                            }
                            let bindings = items[0].to_vec()?;
                            let new_env = Env::extend(env.clone());
                            // Define all variables first (undefined)
                            let mut names = Vec::new();
                            let mut init_exprs = Vec::new();
                            for b in &bindings {
                                let bv = b.to_vec()?;
                                if bv.len() != 2 {
                                    return Err(SchemeError::Eval(
                                        "letrec: invalid binding".to_string(),
                                    ));
                                }
                                let name = match &bv[0] {
                                    Val::Symbol(s) => s.to_string(),
                                    _ => {
                                        return Err(SchemeError::Eval(
                                            "letrec: param must be symbol".to_string(),
                                        ))
                                    }
                                };
                                new_env.borrow_mut().define(name.clone(), Val::Void);
                                names.push(name);
                                init_exprs.push(bv[1].clone());
                            }
                            // Now evaluate and assign
                            for (name, init) in names.iter().zip(init_exprs.iter()) {
                                let val = eval(init.clone(), &new_env)?;
                                new_env.borrow_mut().define(name.clone(), val);
                            }
                            env = new_env;
                            let body = &items[1..];
                            if body.is_empty() {
                                return Ok(Val::Void);
                            }
                            for b in &body[..body.len() - 1] {
                                eval(b.clone(), &env)?;
                            }
                            expr = body.last().unwrap().clone();
                            continue;
                        }
                        "do" => {
                            return eval_do(operands, &env);
                        }
                        "delay" => {
                            // Create a lazy promise
                            let items = operands.to_vec()?;
                            if items.is_empty() {
                                return Err(SchemeError::Eval(
                                    "delay: missing expression".to_string(),
                                ));
                            }
                            return Ok(Val::Proc(Proc::Lambda {
                                params: LambdaParams::Fixed(vec![]),
                                body: items,
                                env: env.clone(),
                                name: Some("promise".to_string()),
                            }));
                        }
                        "delay-force" | "make-promise" => {
                            let items = operands.to_vec()?;
                            if items.is_empty() {
                                return Ok(Val::Void);
                            }
                            return eval(items[0].clone(), &env);
                        }
                        "force" => {
                            let items = operands.to_vec()?;
                            if items.is_empty() {
                                return Err(SchemeError::Eval(
                                    "force: missing argument".to_string(),
                                ));
                            }
                            let p = eval(items[0].clone(), &env)?;
                            match p {
                                Val::Proc(Proc::Lambda {
                                    body,
                                    env: lenv,
                                    ..
                                }) => {
                                    let body_env = Env::extend(lenv);
                                    let b_list = body;
                                    if b_list.is_empty() {
                                        return Ok(Val::Void);
                                    }
                                    for b in &b_list[..b_list.len() - 1] {
                                        eval(b.clone(), &body_env)?;
                                    }
                                    expr = b_list.last().unwrap().clone();
                                    env = body_env;
                                    continue;
                                }
                                other => return Ok(other),
                            }
                        }
                        "quasiquote" => {
                            return eval_quasiquote(operands.car()?, &env, 0);
                        }
                        "case" => {
                            return eval_case(operands, &env);
                        }
                        "values" => {
                            let items = operands.to_vec()?;
                            let mut vals: Vec<Val> = items
                                .iter()
                                .map(|e| eval(e.clone(), &env))
                                .collect::<Result<_, _>>()?;
                            if vals.len() == 1 {
                                return Ok(vals.remove(0));
                            }
                            return Ok(Val::list(vals.into_iter()));
                        }
                        "call-with-values" => {
                            let items = operands.to_vec()?;
                            if items.len() != 2 {
                                return Err(SchemeError::Arity {
                                    expected: "2".to_string(),
                                    got: items.len(),
                                });
                            }
                            let producer = eval(items[0].clone(), &env)?;
                            let consumer = eval(items[1].clone(), &env)?;
                            let values = apply(producer, vec![], &env)?;
                            let args = match values {
                                Val::Pair(_) => values.to_vec()?,
                                other => vec![other],
                            };
                            return apply(consumer, args, &env);
                        }
                        "call-with-current-continuation" | "call/cc" => {
                            // Simplified call/cc - only supports escape continuations
                            let items = operands.to_vec()?;
                            if items.len() != 1 {
                                return Err(SchemeError::Arity {
                                    expected: "1".to_string(),
                                    got: items.len(),
                                });
                            }
                            let proc = eval(items[0].clone(), &env)?;
                            let escape = Val::Proc(Proc::Continuation);
                            return apply(proc, vec![escape], &env);
                        }
                        "apply" => {
                            let items = operands.to_vec()?;
                            if items.len() < 2 {
                                return Err(SchemeError::Arity {
                                    expected: "at least 2".to_string(),
                                    got: items.len(),
                                });
                            }
                            let proc = eval(items[0].clone(), &env)?;
                            let mut args: Vec<Val> = items[1..items.len() - 1]
                                .iter()
                                .map(|e| eval(e.clone(), &env))
                                .collect::<Result<_, _>>()?;
                            let last = eval(items.last().unwrap().clone(), &env)?;
                            let mut extra = last.to_vec()?;
                            args.append(&mut extra);
                            return apply(proc, args, &env);
                        }
                        "map" => {
                            return eval_map(operands, &env);
                        }
                        "for-each" => {
                            return eval_for_each(operands, &env);
                        }
                        "filter" => {
                            return eval_filter(operands, &env);
                        }
                        "fold-right" | "foldr" => {
                            return eval_fold_right(operands, &env);
                        }
                        "fold-left" | "foldl" | "fold" => {
                            return eval_fold_left(operands, &env);
                        }
                        "with-exception-handler" => {
                            let items = operands.to_vec()?;
                            if items.len() != 2 {
                                return Err(SchemeError::Arity {
                                    expected: "2".to_string(),
                                    got: items.len(),
                                });
                            }
                            let handler = eval(items[0].clone(), &env)?;
                            let thunk = eval(items[1].clone(), &env)?;
                            match apply(thunk, vec![], &env) {
                                Ok(v) => return Ok(v),
                                Err(e) => {
                                    let err_val = Val::Str(Rc::new(RefCell::new(e.to_string())));
                                    return apply(handler, vec![err_val], &env);
                                }
                            }
                        }
                        "guard" => {
                            return eval_guard(operands, &env);
                        }
                        "parameterize" => {
                            // Simplified parameterize
                            let items = operands.to_vec()?;
                            if items.is_empty() {
                                return Ok(Val::Void);
                            }
                            let body = &items[1..];
                            let new_env = Env::extend(env.clone());
                            env = new_env;
                            if body.is_empty() {
                                return Ok(Val::Void);
                            }
                            for b in &body[..body.len() - 1] {
                                eval(b.clone(), &env)?;
                            }
                            expr = body.last().unwrap().clone();
                            continue;
                        }
                        "include" => {
                            // Load and evaluate a file
                            let items = operands.to_vec()?;
                            let mut result = Val::Void;
                            for item in items {
                                let path = match item {
                                    Val::Str(ref s) => s.borrow().clone(),
                                    _ => {
                                        return Err(SchemeError::Eval(
                                            "include: expected string".to_string(),
                                        ))
                                    }
                                };
                                let src = std::fs::read_to_string(&path)
                                    .map_err(|e| SchemeError::Io(e.to_string()))?;
                                result = eval_str(&src, &env)?;
                            }
                            return Ok(result);
                        }
                        "load" => {
                            let items = operands.to_vec()?;
                            if items.is_empty() {
                                return Err(SchemeError::Arity {
                                    expected: "1".to_string(),
                                    got: 0,
                                });
                            }
                            let path = match &items[0] {
                                Val::Str(s) => s.borrow().clone(),
                                _ => {
                                    return Err(SchemeError::Eval(
                                        "load: expected string".to_string(),
                                    ))
                                }
                            };
                            let src = std::fs::read_to_string(&path)
                                .map_err(|e| SchemeError::Io(e.to_string()))?;
                            return eval_str(&src, &env);
                        }
                        _ => {} // Fall through to procedure application
                    }
                }

                // Procedure application
                let proc = eval(operator, &env)?;
                let args: Vec<Val> = operands
                    .to_vec()?
                    .iter()
                    .map(|a| eval(a.clone(), &env))
                    .collect::<Result<_, _>>()?;

                match proc {
                    Val::Proc(Proc::Builtin { func, .. }) => {
                        return func(&args);
                    }
                    Val::Proc(Proc::Lambda {
                        params,
                        body,
                        env: closure_env,
                        ..
                    }) => {
                        // Bind parameters
                        let call_env = Env::extend(closure_env);
                        bind_params(&params, args, &call_env)?;

                        if body.is_empty() {
                            return Ok(Val::Void);
                        }
                        for b in &body[..body.len() - 1] {
                            eval(b.clone(), &call_env)?;
                        }
                        // TCO: update expr and env
                        expr = body.last().unwrap().clone();
                        env = call_env;
                        continue;
                    }
                    Val::Proc(Proc::Continuation) => {
                        // Escape continuation - return the value
                        return Ok(args.into_iter().next().unwrap_or(Val::Void));
                    }
                    _ => {
                        return Err(SchemeError::Eval(format!("Not a procedure: {}", proc)));
                    }
                }
            }
        }
    }
}

/// Apply a procedure to arguments (non-TCO version for internal use).
pub fn apply(proc: Val, args: Vec<Val>, _env: &Rc<RefCell<Env>>) -> SchemeResult {
    match proc {
        Val::Proc(Proc::Builtin { func, .. }) => func(&args),
        Val::Proc(Proc::Lambda {
            params,
            body,
            env: closure_env,
            ..
        }) => {
            let call_env = Env::extend(closure_env);
            bind_params(&params, args, &call_env)?;
            if body.is_empty() {
                return Ok(Val::Void);
            }
            let mut result = Val::Void;
            for b in &body {
                result = eval(b.clone(), &call_env)?;
            }
            Ok(result)
        }
        Val::Proc(Proc::Continuation) => Ok(args.into_iter().next().unwrap_or(Val::Void)),
        _ => Err(SchemeError::Eval(format!("Not a procedure: {}", proc))),
    }
}

/// Parse lambda parameter list.
fn parse_params(params: &Val) -> Result<LambdaParams, SchemeError> {
    match params {
        // (lambda rest ...)  -- single rest arg
        Val::Symbol(s) => Ok(LambdaParams::Variadic {
            required: vec![],
            rest: s.to_string(),
        }),
        Val::Nil => Ok(LambdaParams::Fixed(vec![])),
        Val::Pair(_) => {
            let mut required = Vec::new();
            let mut cur = params.clone();
            loop {
                match cur.clone() {
                    Val::Nil => return Ok(LambdaParams::Fixed(required)),
                    Val::Symbol(rest) => {
                        return Ok(LambdaParams::Variadic {
                            required,
                            rest: rest.to_string(),
                        });
                    }
                    Val::Pair(p) => {
                        let (car, cdr) = {
                            let b = p.borrow();
                            (b.0.clone(), b.1.clone())
                        };
                        match car {
                            Val::Symbol(s) => required.push(s.to_string()),
                            _ => {
                                return Err(SchemeError::Eval(
                                    "lambda: parameter must be a symbol".to_string(),
                                ))
                            }
                        }
                        cur = cdr;
                    }
                    _ => {
                        return Err(SchemeError::Eval(
                            "lambda: invalid parameter list".to_string(),
                        ))
                    }
                }
            }
        }
        _ => Err(SchemeError::Eval(
            "lambda: invalid parameter list".to_string(),
        )),
    }
}

/// Bind parameters in the environment.
fn bind_params(
    params: &LambdaParams,
    args: Vec<Val>,
    env: &Rc<RefCell<Env>>,
) -> Result<(), SchemeError> {
    match params {
        LambdaParams::Fixed(names) => {
            if names.len() != args.len() {
                return Err(SchemeError::Arity {
                    expected: names.len().to_string(),
                    got: args.len(),
                });
            }
            for (name, val) in names.iter().zip(args.into_iter()) {
                env.borrow_mut().define(name.clone(), val);
            }
        }
        LambdaParams::Variadic { required, rest } => {
            if args.len() < required.len() {
                return Err(SchemeError::Arity {
                    expected: format!("at least {}", required.len()),
                    got: args.len(),
                });
            }
            let (req_args, rest_args) = args.split_at(required.len());
            for (name, val) in required.iter().zip(req_args.iter()) {
                env.borrow_mut().define(name.clone(), val.clone());
            }
            let rest_list = Val::list(rest_args.iter().cloned());
            env.borrow_mut().define(rest.clone(), rest_list);
        }
    }
    Ok(())
}

/// Evaluate a `do` loop expression.
fn eval_do(operands: Val, env: &Rc<RefCell<Env>>) -> SchemeResult {
    let items = operands.to_vec()?;
    if items.len() < 2 {
        return Err(SchemeError::Eval("do: invalid form".to_string()));
    }
    let var_specs = items[0].to_vec()?;
    let test_result = items[1].to_vec()?;
    let commands = items[2..].to_vec();

    // Initialize variables
    let do_env = Env::extend(env.clone());
    let mut var_names: Vec<String> = Vec::new();
    let mut step_exprs: Vec<Option<Val>> = Vec::new();

    for spec in &var_specs {
        let sv = spec.to_vec()?;
        if sv.is_empty() {
            return Err(SchemeError::Eval("do: invalid var spec".to_string()));
        }
        let name = match &sv[0] {
            Val::Symbol(s) => s.to_string(),
            _ => return Err(SchemeError::Eval("do: var name must be symbol".to_string())),
        };
        let init = if sv.len() >= 2 {
            eval(sv[1].clone(), env)?
        } else {
            Val::Void
        };
        do_env.borrow_mut().define(name.clone(), init);
        var_names.push(name);
        step_exprs.push(if sv.len() >= 3 {
            Some(sv[2].clone())
        } else {
            None
        });
    }

    // Test
    if test_result.is_empty() {
        return Err(SchemeError::Eval("do: missing test".to_string()));
    }
    let test = &test_result[0];
    let result_exprs = &test_result[1..];

    loop {
        let test_val = eval(test.clone(), &do_env)?;
        if test_val.is_truthy() {
            // Evaluate result expressions
            let mut result = Val::Void;
            for r in result_exprs {
                result = eval(r.clone(), &do_env)?;
            }
            return Ok(result);
        }
        // Execute commands
        for cmd in &commands {
            eval(cmd.clone(), &do_env)?;
        }
        // Step variables
        let new_vals: Vec<Val> = step_exprs
            .iter()
            .zip(var_names.iter())
            .map(|(step, name)| {
                if let Some(step_expr) = step {
                    eval(step_expr.clone(), &do_env)
                } else {
                    Ok(do_env.borrow().get(name).unwrap_or(Val::Void))
                }
            })
            .collect::<Result<_, _>>()?;
        for (name, val) in var_names.iter().zip(new_vals.into_iter()) {
            do_env.borrow_mut().define(name.clone(), val);
        }
    }
}

/// Evaluate a quasiquote expression.
fn eval_quasiquote(tmpl: Val, env: &Rc<RefCell<Env>>, depth: usize) -> SchemeResult {
    match &tmpl {
        Val::Pair(_) => {
            let car = tmpl.car()?;
            // Check for unquote
            if let Val::Symbol(ref s) = car {
                if s.as_ref() == "unquote" {
                    let inner = tmpl.cdr()?.car()?;
                    if depth == 0 {
                        return eval(inner, env);
                    } else {
                        let inner_qq = eval_quasiquote(inner, env, depth - 1)?;
                        return Ok(Val::list(
                            [Val::Symbol(Rc::from("unquote")), inner_qq].into_iter(),
                        ));
                    }
                }
                if s.as_ref() == "quasiquote" {
                    let inner = tmpl.cdr()?.car()?;
                    let inner_qq = eval_quasiquote(inner, env, depth + 1)?;
                    return Ok(Val::list(
                        [Val::Symbol(Rc::from("quasiquote")), inner_qq].into_iter(),
                    ));
                }
            }
            // Check for unquote-splicing in car
            let mut result = Vec::new();
            let mut cur = tmpl.clone();
            loop {
                match cur.clone() {
                    Val::Nil => break,
                    Val::Pair(_) => {
                        let head = cur.car()?;
                        let tail = cur.cdr()?;
                        // Check for ,@ splice
                        if let Val::Pair(_) = &head {
                            let head_car = head.car()?;
                            if let Val::Symbol(ref s) = head_car {
                                if s.as_ref() == "unquote-splicing" {
                                    let inner = head.cdr()?.car()?;
                                    if depth == 0 {
                                        let spliced = eval(inner, env)?;
                                        let items = spliced.to_vec()?;
                                        result.extend(items);
                                        cur = tail;
                                        continue;
                                    }
                                }
                            }
                        }
                        result.push(eval_quasiquote(head, env, depth)?);
                        cur = tail;
                    }
                    other => {
                        // Dotted pair tail
                        let tail_val = eval_quasiquote(other, env, depth)?;
                        // Build from result
                        let mut v = tail_val;
                        for item in result.into_iter().rev() {
                            v = Val::cons(item, v);
                        }
                        return Ok(v);
                    }
                }
            }
            Ok(Val::list(result.into_iter()))
        }
        Val::Vector(v) => {
            let items = v.borrow().clone();
            let new_items: Vec<Val> = items
                .iter()
                .map(|item| eval_quasiquote(item.clone(), env, depth))
                .collect::<Result<_, _>>()?;
            Ok(Val::Vector(Rc::new(RefCell::new(new_items))))
        }
        // Atoms are self-quoting
        other => Ok(other.clone()),
    }
}

/// Evaluate a `case` expression.
fn eval_case(operands: Val, env: &Rc<RefCell<Env>>) -> SchemeResult {
    let items = operands.to_vec()?;
    if items.is_empty() {
        return Err(SchemeError::Eval("case: missing key".to_string()));
    }
    let key = eval(items[0].clone(), env)?;
    for clause in &items[1..] {
        let cv = clause.to_vec()?;
        if cv.is_empty() {
            continue;
        }
        let datums = &cv[0];
        let is_else = matches!(datums, Val::Bool(true))
            || matches!(datums, Val::Symbol(s) if s.as_ref() == "else");
        let matched = if is_else {
            true
        } else {
            let datum_list = datums.to_vec()?;
            datum_list.iter().any(|d| scheme_eqv(&key, d))
        };
        if matched {
            // Check for => syntax
            if cv.len() >= 3 && matches!(&cv[1], Val::Symbol(s) if s.as_ref() == "=>") {
                let proc = eval(cv[2].clone(), env)?;
                return apply(proc, vec![key], env);
            }
            let mut result = Val::Void;
            for body in &cv[1..] {
                result = eval(body.clone(), env)?;
            }
            return Ok(result);
        }
    }
    Ok(Val::Void)
}

fn scheme_eqv(a: &Val, b: &Val) -> bool {
    match (a, b) {
        (Val::Bool(x), Val::Bool(y)) => x == y,
        (Val::Int(x), Val::Int(y)) => x == y,
        (Val::Char(x), Val::Char(y)) => x == y,
        (Val::Symbol(x), Val::Symbol(y)) => x == y,
        (Val::Nil, Val::Nil) => true,
        _ => false,
    }
}

fn eval_map(operands: Val, env: &Rc<RefCell<Env>>) -> SchemeResult {
    let items = operands.to_vec()?;
    if items.len() < 2 {
        return Err(SchemeError::Arity {
            expected: "at least 2".to_string(),
            got: items.len(),
        });
    }
    let proc = eval(items[0].clone(), env)?;
    let lists: Vec<Vec<Val>> = items[1..]
        .iter()
        .map(|l| -> Result<Vec<Val>, SchemeError> { eval(l.clone(), env)?.to_vec() })
        .collect::<Result<_, _>>()?;

    if lists.is_empty() {
        return Ok(Val::Nil);
    }
    let len = lists[0].len();
    let mut results = Vec::with_capacity(len);
    for i in 0..len {
        let args: Vec<Val> = lists.iter().map(|l| l[i].clone()).collect();
        results.push(apply(proc.clone(), args, env)?);
    }
    Ok(Val::list(results.into_iter()))
}

fn eval_for_each(operands: Val, env: &Rc<RefCell<Env>>) -> SchemeResult {
    let items = operands.to_vec()?;
    if items.len() < 2 {
        return Err(SchemeError::Arity {
            expected: "at least 2".to_string(),
            got: items.len(),
        });
    }
    let proc = eval(items[0].clone(), env)?;
    let lists: Vec<Vec<Val>> = items[1..]
        .iter()
        .map(|l| -> Result<Vec<Val>, SchemeError> { eval(l.clone(), env)?.to_vec() })
        .collect::<Result<_, _>>()?;

    if lists.is_empty() {
        return Ok(Val::Void);
    }
    let len = lists[0].len();
    for i in 0..len {
        let args: Vec<Val> = lists.iter().map(|l| l[i].clone()).collect();
        apply(proc.clone(), args, env)?;
    }
    Ok(Val::Void)
}

fn eval_filter(operands: Val, env: &Rc<RefCell<Env>>) -> SchemeResult {
    let items = operands.to_vec()?;
    if items.len() != 2 {
        return Err(SchemeError::Arity {
            expected: "2".to_string(),
            got: items.len(),
        });
    }
    let pred = eval(items[0].clone(), env)?;
    let list = eval(items[1].clone(), env)?.to_vec()?;
    let mut result = Vec::new();
    for item in list {
        if apply(pred.clone(), vec![item.clone()], env)?.is_truthy() {
            result.push(item);
        }
    }
    Ok(Val::list(result.into_iter()))
}

fn eval_fold_right(operands: Val, env: &Rc<RefCell<Env>>) -> SchemeResult {
    let items = operands.to_vec()?;
    if items.len() < 3 {
        return Err(SchemeError::Arity {
            expected: "at least 3".to_string(),
            got: items.len(),
        });
    }
    let proc = eval(items[0].clone(), env)?;
    let init = eval(items[1].clone(), env)?;
    let list = eval(items[2].clone(), env)?.to_vec()?;
    let mut acc = init;
    for item in list.into_iter().rev() {
        acc = apply(proc.clone(), vec![item, acc], env)?;
    }
    Ok(acc)
}

fn eval_fold_left(operands: Val, env: &Rc<RefCell<Env>>) -> SchemeResult {
    let items = operands.to_vec()?;
    if items.len() < 3 {
        return Err(SchemeError::Arity {
            expected: "at least 3".to_string(),
            got: items.len(),
        });
    }
    let proc = eval(items[0].clone(), env)?;
    let init = eval(items[1].clone(), env)?;
    let list = eval(items[2].clone(), env)?.to_vec()?;
    let mut acc = init;
    for item in list {
        acc = apply(proc.clone(), vec![acc, item], env)?;
    }
    Ok(acc)
}

fn eval_guard(operands: Val, env: &Rc<RefCell<Env>>) -> SchemeResult {
    let items = operands.to_vec()?;
    if items.len() < 2 {
        return Err(SchemeError::Eval("guard: invalid form".to_string()));
    }
    let var_clauses = items[0].to_vec()?;
    if var_clauses.is_empty() {
        return Err(SchemeError::Eval("guard: missing variable".to_string()));
    }
    let var_name = match &var_clauses[0] {
        Val::Symbol(s) => s.to_string(),
        _ => return Err(SchemeError::Eval("guard: var must be symbol".to_string())),
    };
    let clauses = var_clauses[1..].to_vec();
    let body = items[1..].to_vec();

    // Try to run body
    let body_result = {
        let mut result = Val::Void;
        let mut err = None;
        for b in &body {
            match eval(b.clone(), env) {
                Ok(v) => result = v,
                Err(e) => {
                    err = Some(e);
                    break;
                }
            }
        }
        (result, err)
    };

    match body_result {
        (result, None) => Ok(result),
        (_, Some(e)) => {
            // Bind the variable
            let guard_env = Env::extend(env.clone());
            let err_val = Val::Str(Rc::new(RefCell::new(e.to_string())));
            guard_env.borrow_mut().define(var_name, err_val.clone());
            // Evaluate clauses
            for clause in &clauses {
                let cv = clause.to_vec()?;
                if cv.is_empty() {
                    continue;
                }
                let test = &cv[0];
                let is_else = matches!(test, Val::Bool(true))
                    || matches!(test, Val::Symbol(s) if s.as_ref() == "else");
                let test_val = if is_else {
                    Val::Bool(true)
                } else {
                    eval(test.clone(), &guard_env)?
                };
                if test_val.is_truthy() {
                    let mut result = test_val;
                    for body_expr in &cv[1..] {
                        result = eval(body_expr.clone(), &guard_env)?;
                    }
                    return Ok(result);
                }
            }
            // Re-raise
            Err(e)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run(code: &str) -> SchemeResult {
        let env = make_global_env();
        eval_str(code, &env)
    }

    #[test]
    fn test_arithmetic() {
        assert_eq!(run("(+ 1 2)").unwrap(), Val::Int(3));
        assert_eq!(run("(* 3 4)").unwrap(), Val::Int(12));
        assert_eq!(run("(- 10 3)").unwrap(), Val::Int(7));
    }

    #[test]
    fn test_define() {
        let env = make_global_env();
        eval_str("(define x 42)", &env).unwrap();
        assert_eq!(eval_str("x", &env).unwrap(), Val::Int(42));
    }

    #[test]
    fn test_lambda() {
        let result = run("((lambda (x) (* x x)) 5)").unwrap();
        assert_eq!(result, Val::Int(25));
    }

    #[test]
    fn test_if() {
        assert_eq!(run("(if #t 1 2)").unwrap(), Val::Int(1));
        assert_eq!(run("(if #f 1 2)").unwrap(), Val::Int(2));
    }

    #[test]
    fn test_let() {
        assert_eq!(run("(let ((x 3) (y 4)) (+ x y))").unwrap(), Val::Int(7));
    }

    #[test]
    fn test_recursion() {
        let code = r"
        (define (fact n)
          (if (= n 0) 1 (* n (fact (- n 1)))))
        (fact 10)
        ";
        assert_eq!(run(code).unwrap(), Val::Int(3628800));
    }

    #[test]
    fn test_tail_recursion() {
        let code = r"
        (define (loop n)
          (if (= n 0) 'done (loop (- n 1))))
        (loop 100000)
        ";
        let result = run(code).unwrap();
        assert_eq!(result, Val::Symbol(Rc::from("done")));
    }

    #[test]
    fn test_quasiquote() {
        let env = make_global_env();
        eval_str("(define x 42)", &env).unwrap();
        let result = eval_str("`(the answer is ,x)", &env).unwrap();
        let items = result.to_vec().unwrap();
        assert_eq!(items.len(), 4);
        assert_eq!(items[3], Val::Int(42));
    }

    #[test]
    fn test_closure() {
        let code = r"
        (define (make-adder n)
          (lambda (x) (+ x n)))
        (define add5 (make-adder 5))
        (add5 3)
        ";
        assert_eq!(run(code).unwrap(), Val::Int(8));
    }

    #[test]
    fn test_map() {
        let code = r"
        (map (lambda (x) (* x x)) '(1 2 3))
        ";
        let result = run(code).unwrap();
        let items = result.to_vec().unwrap();
        assert_eq!(items, vec![Val::Int(1), Val::Int(4), Val::Int(9)]);
    }
}
