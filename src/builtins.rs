//! Built-in Scheme procedures (R5RS compliant).

use std::cell::RefCell;
use std::rc::Rc;

use crate::types::{write_val, Env, Proc, SchemeError, SchemeResult, Val};

/// Register all built-in procedures into the environment.
pub fn register_builtins(env: &Rc<RefCell<Env>>) {
    macro_rules! def {
        ($name:expr, $func:expr) => {
            env.borrow_mut().define(
                $name.to_string(),
                Val::Proc(Proc::Builtin {
                    name: $name,
                    func: $func,
                }),
            )
        };
    }

    // Arithmetic
    def!("+", builtin_add);
    def!("-", builtin_sub);
    def!("*", builtin_mul);
    def!("/", builtin_div);
    def!("quotient", builtin_quotient);
    def!("remainder", builtin_remainder);
    def!("modulo", builtin_modulo);
    def!("abs", builtin_abs);
    def!("max", builtin_max);
    def!("min", builtin_min);
    def!("gcd", builtin_gcd);
    def!("lcm", builtin_lcm);
    def!("floor", builtin_floor);
    def!("ceiling", builtin_ceiling);
    def!("truncate", builtin_truncate);
    def!("round", builtin_round);
    def!("exact->inexact", builtin_exact_to_inexact);
    def!("inexact->exact", builtin_inexact_to_exact);
    def!("exact", builtin_exact_to_inexact); // alias
    def!("inexact", builtin_inexact_to_exact); // alias
    def!("number->string", builtin_number_to_string);
    def!("string->number", builtin_string_to_number);
    def!("expt", builtin_expt);
    def!("sqrt", builtin_sqrt);
    def!("exp", builtin_exp);
    def!("log", builtin_log);
    def!("sin", builtin_sin);
    def!("cos", builtin_cos);
    def!("tan", builtin_tan);
    def!("asin", builtin_asin);
    def!("acos", builtin_acos);
    def!("atan", builtin_atan);
    def!("floor/", builtin_floor_div);
    def!("floor-quotient", builtin_floor_quotient);
    def!("floor-remainder", builtin_floor_remainder);
    def!("truncate/", builtin_truncate_div);
    def!("truncate-quotient", builtin_truncate_quotient);
    def!("truncate-remainder", builtin_truncate_remainder);
    def!("exact-integer-sqrt", builtin_exact_integer_sqrt);

    // Comparison
    def!("=", builtin_num_eq);
    def!("<", builtin_lt);
    def!(">", builtin_gt);
    def!("<=", builtin_le);
    def!(">=", builtin_ge);
    def!("zero?", builtin_zero);
    def!("positive?", builtin_positive);
    def!("negative?", builtin_negative);
    def!("odd?", builtin_odd);
    def!("even?", builtin_even);

    // Type predicates
    def!("number?", builtin_numberp);
    def!("integer?", builtin_integerp);
    def!("rational?", builtin_rationalp);
    def!("real?", builtin_realp);
    def!("complex?", builtin_complexp);
    def!("exact?", builtin_exactp);
    def!("inexact?", builtin_inexactp);
    def!("boolean?", builtin_booleanp);
    def!("pair?", builtin_pairp);
    def!("null?", builtin_nullp);
    def!("list?", builtin_listp);
    def!("symbol?", builtin_symbolp);
    def!("string?", builtin_stringp);
    def!("char?", builtin_charp);
    def!("vector?", builtin_vectorp);
    def!("procedure?", builtin_procedurep);
    def!("port?", builtin_portp);
    def!("eof-object?", builtin_eofp);
    def!("eof-object", builtin_eof_object);
    def!("not", builtin_not);

    // Pair/list operations
    def!("cons", builtin_cons);
    def!("car", builtin_car);
    def!("cdr", builtin_cdr);
    def!("set-car!", builtin_set_car);
    def!("set-cdr!", builtin_set_cdr);
    def!("list", builtin_list);
    def!("length", builtin_length);
    def!("append", builtin_append);
    def!("reverse", builtin_reverse);
    def!("list-tail", builtin_list_tail);
    def!("list-ref", builtin_list_ref);
    def!("list-copy", builtin_list_copy);
    def!("memq", builtin_memq);
    def!("memv", builtin_memv);
    def!("member", builtin_member);
    def!("assq", builtin_assq);
    def!("assv", builtin_assv);
    def!("assoc", builtin_assoc);
    // caar, cadr, cdar, cddr, etc. generated below
    register_cxr(env);

    // Equality
    def!("eq?", builtin_eqp);
    def!("eqv?", builtin_eqvp);
    def!("equal?", builtin_equalp);

    // String operations
    def!("make-string", builtin_make_string);
    def!("string", builtin_string);
    def!("string-length", builtin_string_length);
    def!("string-ref", builtin_string_ref);
    def!("string-set!", builtin_string_set);
    def!("string=?", builtin_string_eq);
    def!("string<?", builtin_string_lt);
    def!("string>?", builtin_string_gt);
    def!("string<=?", builtin_string_le);
    def!("string>=?", builtin_string_ge);
    def!("string-ci=?", builtin_string_ci_eq);
    def!("string-ci<?", builtin_string_ci_lt);
    def!("string-ci>?", builtin_string_ci_gt);
    def!("string-ci<=?", builtin_string_ci_le);
    def!("string-ci>=?", builtin_string_ci_ge);
    def!("substring", builtin_substring);
    def!("string-append", builtin_string_append);
    def!("string->list", builtin_string_to_list);
    def!("list->string", builtin_list_to_string);
    def!("string-copy", builtin_string_copy);
    def!("string-fill!", builtin_string_fill);
    def!("string->symbol", builtin_string_to_symbol);
    def!("symbol->string", builtin_symbol_to_string);
    def!("string-upcase", builtin_string_upcase);
    def!("string-downcase", builtin_string_downcase);
    def!("number->string", builtin_number_to_string);

    // Character operations
    def!("char->integer", builtin_char_to_integer);
    def!("integer->char", builtin_integer_to_char);
    def!("char=?", builtin_char_eq);
    def!("char<?", builtin_char_lt);
    def!("char>?", builtin_char_gt);
    def!("char<=?", builtin_char_le);
    def!("char>=?", builtin_char_ge);
    def!("char-ci=?", builtin_char_ci_eq);
    def!("char-ci<?", builtin_char_ci_lt);
    def!("char-ci>?", builtin_char_ci_gt);
    def!("char-ci<=?", builtin_char_ci_le);
    def!("char-ci>=?", builtin_char_ci_ge);
    def!("char-alphabetic?", builtin_char_alpha);
    def!("char-numeric?", builtin_char_numeric);
    def!("char-whitespace?", builtin_char_whitespace);
    def!("char-upper-case?", builtin_char_upper);
    def!("char-lower-case?", builtin_char_lower);
    def!("char-upcase", builtin_char_upcase);
    def!("char-downcase", builtin_char_downcase);

    // Vector operations
    def!("make-vector", builtin_make_vector);
    def!("vector", builtin_vector);
    def!("vector-length", builtin_vector_length);
    def!("vector-ref", builtin_vector_ref);
    def!("vector-set!", builtin_vector_set);
    def!("vector->list", builtin_vector_to_list);
    def!("list->vector", builtin_list_to_vector);
    def!("vector-fill!", builtin_vector_fill);
    def!("vector-copy", builtin_vector_copy);

    // I/O
    def!("display", builtin_display);
    def!("write", builtin_write);
    def!("newline", builtin_newline);
    def!("write-char", builtin_write_char);
    def!("read-char", builtin_read_char);
    def!("peek-char", builtin_peek_char);
    def!("char-ready?", builtin_char_ready);
    def!("read-line", builtin_read_line);
    def!("current-input-port", builtin_current_input_port);
    def!("current-output-port", builtin_current_output_port);
    def!("current-error-port", builtin_current_error_port);
    def!("open-input-file", builtin_open_input_file);
    def!("open-output-file", builtin_open_output_file);
    def!("close-input-port", builtin_close_port);
    def!("close-output-port", builtin_close_port);
    def!("input-port?", builtin_input_portp);
    def!("output-port?", builtin_output_portp);

    // Misc
    def!("error", builtin_error);
    def!("apply", builtin_apply_stub);
    def!("values", builtin_values);
    def!("call-with-values", builtin_call_with_values_stub);
    def!(
        "with-exception-handler",
        builtin_with_exception_handler_stub
    );
    def!("raise", builtin_raise);
    def!("raise-continuable", builtin_raise);
    def!("error-message", builtin_error_message);
    def!("condition/report-string", builtin_error_message);

    // Boolean
    def!("boolean=?", builtin_boolean_eq);

    // System
    def!("exit", builtin_exit);
    def!("force", builtin_force);
    def!("make-promise", builtin_make_promise);
    def!("promise?", builtin_promisep);
    def!("make-parameter", builtin_make_parameter);

    // Constants
    env.borrow_mut().define("else".to_string(), Val::Bool(true));
    env.borrow_mut()
        .define("=>".to_string(), Val::Symbol(Rc::from("=>")));
    env.borrow_mut().define(
        "current-jiffy".to_string(),
        Val::Proc(Proc::Builtin {
            name: "current-jiffy",
            func: builtin_current_jiffy,
        }),
    );
    env.borrow_mut().define(
        "jiffies-per-second".to_string(),
        Val::Proc(Proc::Builtin {
            name: "jiffies-per-second",
            func: builtin_jiffies_per_second,
        }),
    );
}

fn register_cxr(env: &Rc<RefCell<Env>>) {
    // Register caar, cadr, cdar, cddr, caaar, ..., cddddr
    let combinations: &[&str] = &[
        "aa", "ad", "da", "dd", "aaa", "aad", "ada", "add", "daa", "dad", "dda", "ddd", "aaaa",
        "aaad", "aada", "aadd", "adaa", "adad", "adda", "addd", "daaa", "daad", "dada", "dadd",
        "ddaa", "ddad", "ddda", "dddd",
    ];
    for combo in combinations {
        let name = format!("c{}r", combo);
        let combo_owned = combo.to_string();
        // We use a closure-based approach but since Proc::Builtin needs fn ptr,
        // we generate them via a helper
        let val = make_cxr_proc(combo_owned.clone());
        env.borrow_mut().define(name, val);
    }
}

fn make_cxr_proc(combo: String) -> Val {
    // Build a lambda that applies c[ad]r in sequence
    // combo is like "ad" meaning cadr = (car (cdr x))
    // We store as a closure lambda with a name
    use crate::types::LambdaParams;

    // Build body as nested calls
    // combo reversed: "ad" -> apply car then cdr reversed = cdr then car
    let param = Val::Symbol(Rc::from("x"));
    let mut body = param;
    for c in combo.chars().rev() {
        let op = if c == 'a' { "car" } else { "cdr" };
        body = Val::cons(Val::Symbol(Rc::from(op)), Val::cons(body, Val::Nil));
    }
    let name = format!("c{}r", combo);
    Val::Proc(crate::types::Proc::Lambda {
        params: LambdaParams::Fixed(vec!["x".to_string()]),
        body: vec![body],
        env: Env::new(),
        name: Some(name),
    })
}

// ---- Arithmetic ----

fn to_num(v: &Val, _name: &str) -> Result<f64, SchemeError> {
    match v {
        Val::Int(n) => Ok(*n as f64),
        Val::Float(f) => Ok(*f),
        _ => Err(SchemeError::Type {
            expected: "number".to_string(),
            got: v.type_name().to_string(),
        }),
    }
}

fn builtin_add(args: &[Val]) -> SchemeResult {
    let mut sum_int: i64 = 0;
    let mut has_float = false;
    let mut sum_float: f64 = 0.0;
    for a in args {
        match a {
            Val::Int(n) => {
                sum_int = sum_int.wrapping_add(*n);
                sum_float += *n as f64;
            }
            Val::Float(f) => {
                has_float = true;
                sum_float += f;
            }
            _ => {
                return Err(SchemeError::Type {
                    expected: "number".to_string(),
                    got: a.type_name().to_string(),
                })
            }
        }
    }
    if has_float {
        Ok(Val::Float(sum_float))
    } else {
        Ok(Val::Int(sum_int))
    }
}

fn builtin_sub(args: &[Val]) -> SchemeResult {
    if args.is_empty() {
        return Err(SchemeError::Arity {
            expected: "at least 1".to_string(),
            got: 0,
        });
    }
    if args.len() == 1 {
        return match &args[0] {
            Val::Int(n) => Ok(Val::Int(-n)),
            Val::Float(f) => Ok(Val::Float(-f)),
            _ => Err(SchemeError::Type {
                expected: "number".to_string(),
                got: args[0].type_name().to_string(),
            }),
        };
    }
    let first = to_num(&args[0], "-")?;
    let rest_sum: f64 = args[1..]
        .iter()
        .map(|a| to_num(a, "-"))
        .collect::<Result<Vec<_>, _>>()?
        .iter()
        .sum();
    let result = first - rest_sum;
    // Preserve exactness
    if args.iter().any(|a| matches!(a, Val::Float(_))) {
        Ok(Val::Float(result))
    } else {
        Ok(Val::Int(result as i64))
    }
}

fn builtin_mul(args: &[Val]) -> SchemeResult {
    let mut prod_int: i64 = 1;
    let mut has_float = false;
    let mut prod_float: f64 = 1.0;
    for a in args {
        match a {
            Val::Int(n) => {
                prod_int = prod_int.wrapping_mul(*n);
                prod_float *= *n as f64;
            }
            Val::Float(f) => {
                has_float = true;
                prod_float *= f;
            }
            _ => {
                return Err(SchemeError::Type {
                    expected: "number".to_string(),
                    got: a.type_name().to_string(),
                })
            }
        }
    }
    if has_float {
        Ok(Val::Float(prod_float))
    } else {
        Ok(Val::Int(prod_int))
    }
}

fn builtin_div(args: &[Val]) -> SchemeResult {
    if args.is_empty() {
        return Err(SchemeError::Arity {
            expected: "at least 1".to_string(),
            got: 0,
        });
    }
    if args.len() == 1 {
        let n = to_num(&args[0], "/")?;
        if n == 0.0 {
            return Err(SchemeError::Eval("Division by zero".to_string()));
        }
        return Ok(Val::Float(1.0 / n));
    }
    let first = to_num(&args[0], "/")?;
    let mut result = first;
    for a in &args[1..] {
        let n = to_num(a, "/")?;
        if n == 0.0 {
            return Err(SchemeError::Eval("Division by zero".to_string()));
        }
        result /= n;
    }
    if args.iter().any(|a| matches!(a, Val::Float(_))) {
        Ok(Val::Float(result))
    } else if result.fract() == 0.0 {
        Ok(Val::Int(result as i64))
    } else {
        Ok(Val::Float(result))
    }
}

fn builtin_quotient(args: &[Val]) -> SchemeResult {
    expect_len(args, 2, "quotient")?;
    let a = as_int(&args[0], "quotient")?;
    let b = as_int(&args[1], "quotient")?;
    if b == 0 {
        return Err(SchemeError::Eval("quotient: division by zero".to_string()));
    }
    Ok(Val::Int(a / b))
}

fn builtin_remainder(args: &[Val]) -> SchemeResult {
    expect_len(args, 2, "remainder")?;
    let a = as_int(&args[0], "remainder")?;
    let b = as_int(&args[1], "remainder")?;
    if b == 0 {
        return Err(SchemeError::Eval("remainder: division by zero".to_string()));
    }
    Ok(Val::Int(a % b))
}

fn builtin_modulo(args: &[Val]) -> SchemeResult {
    expect_len(args, 2, "modulo")?;
    let a = as_int(&args[0], "modulo")?;
    let b = as_int(&args[1], "modulo")?;
    if b == 0 {
        return Err(SchemeError::Eval("modulo: division by zero".to_string()));
    }
    let r = a % b;
    if r == 0 || (r > 0) == (b > 0) {
        Ok(Val::Int(r))
    } else {
        Ok(Val::Int(r + b))
    }
}

fn builtin_abs(args: &[Val]) -> SchemeResult {
    expect_len(args, 1, "abs")?;
    match &args[0] {
        Val::Int(n) => Ok(Val::Int(n.abs())),
        Val::Float(f) => Ok(Val::Float(f.abs())),
        _ => Err(type_err("number", &args[0])),
    }
}

fn builtin_max(args: &[Val]) -> SchemeResult {
    if args.is_empty() {
        return Err(SchemeError::Arity {
            expected: "at least 1".to_string(),
            got: 0,
        });
    }
    let has_float = args.iter().any(|a| matches!(a, Val::Float(_)));
    let mut max = to_num(&args[0], "max")?;
    for a in &args[1..] {
        let n = to_num(a, "max")?;
        if n > max {
            max = n;
        }
    }
    if has_float {
        Ok(Val::Float(max))
    } else {
        Ok(Val::Int(max as i64))
    }
}

fn builtin_min(args: &[Val]) -> SchemeResult {
    if args.is_empty() {
        return Err(SchemeError::Arity {
            expected: "at least 1".to_string(),
            got: 0,
        });
    }
    let has_float = args.iter().any(|a| matches!(a, Val::Float(_)));
    let mut min = to_num(&args[0], "min")?;
    for a in &args[1..] {
        let n = to_num(a, "min")?;
        if n < min {
            min = n;
        }
    }
    if has_float {
        Ok(Val::Float(min))
    } else {
        Ok(Val::Int(min as i64))
    }
}

fn gcd_impl(a: i64, b: i64) -> i64 {
    let (mut a, mut b) = (a.abs(), b.abs());
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

fn builtin_gcd(args: &[Val]) -> SchemeResult {
    if args.is_empty() {
        return Ok(Val::Int(0));
    }
    let mut result = as_int(&args[0], "gcd")?.abs();
    for a in &args[1..] {
        result = gcd_impl(result, as_int(a, "gcd")?);
    }
    Ok(Val::Int(result))
}

fn builtin_lcm(args: &[Val]) -> SchemeResult {
    if args.is_empty() {
        return Ok(Val::Int(1));
    }
    let mut result = as_int(&args[0], "lcm")?.abs();
    for a in &args[1..] {
        let n = as_int(a, "lcm")?.abs();
        let g = gcd_impl(result, n);
        if g == 0 {
            result = 0;
        } else {
            result = (result / g) * n;
        }
    }
    Ok(Val::Int(result))
}

fn builtin_floor(args: &[Val]) -> SchemeResult {
    expect_len(args, 1, "floor")?;
    match &args[0] {
        Val::Int(n) => Ok(Val::Int(*n)),
        Val::Float(f) => Ok(Val::Float(f.floor())),
        _ => Err(type_err("number", &args[0])),
    }
}

fn builtin_ceiling(args: &[Val]) -> SchemeResult {
    expect_len(args, 1, "ceiling")?;
    match &args[0] {
        Val::Int(n) => Ok(Val::Int(*n)),
        Val::Float(f) => Ok(Val::Float(f.ceil())),
        _ => Err(type_err("number", &args[0])),
    }
}

fn builtin_truncate(args: &[Val]) -> SchemeResult {
    expect_len(args, 1, "truncate")?;
    match &args[0] {
        Val::Int(n) => Ok(Val::Int(*n)),
        Val::Float(f) => Ok(Val::Float(f.trunc())),
        _ => Err(type_err("number", &args[0])),
    }
}

fn builtin_round(args: &[Val]) -> SchemeResult {
    expect_len(args, 1, "round")?;
    match &args[0] {
        Val::Int(n) => Ok(Val::Int(*n)),
        Val::Float(f) => {
            // Round to nearest even (banker's rounding)
            let rounded = f.round();
            let diff = (f - rounded).abs();
            if (diff - 0.5).abs() < f64::EPSILON {
                // Exactly halfway – round to even
                if (rounded as i64) % 2 == 0 {
                    Ok(Val::Float(rounded))
                } else {
                    Ok(Val::Float(rounded - f.signum()))
                }
            } else {
                Ok(Val::Float(rounded))
            }
        }
        _ => Err(type_err("number", &args[0])),
    }
}

fn builtin_exact_to_inexact(args: &[Val]) -> SchemeResult {
    expect_len(args, 1, "exact->inexact")?;
    match &args[0] {
        Val::Int(n) => Ok(Val::Float(*n as f64)),
        Val::Float(f) => Ok(Val::Float(*f)),
        _ => Err(type_err("number", &args[0])),
    }
}

fn builtin_inexact_to_exact(args: &[Val]) -> SchemeResult {
    expect_len(args, 1, "inexact->exact")?;
    match &args[0] {
        Val::Int(n) => Ok(Val::Int(*n)),
        Val::Float(f) => {
            if f.is_finite() {
                Ok(Val::Int(*f as i64))
            } else {
                Err(SchemeError::Eval(
                    "inexact->exact: no exact representation".to_string(),
                ))
            }
        }
        _ => Err(type_err("number", &args[0])),
    }
}

fn builtin_number_to_string(args: &[Val]) -> SchemeResult {
    if args.is_empty() || args.len() > 2 {
        return Err(SchemeError::Arity {
            expected: "1 or 2".to_string(),
            got: args.len(),
        });
    }
    let radix = if args.len() == 2 {
        as_int(&args[1], "number->string")? as u32
    } else {
        10
    };
    let s = match &args[0] {
        Val::Int(n) => match radix {
            2 => format!("{:b}", n),
            8 => format!("{:o}", n),
            10 => format!("{}", n),
            16 => format!("{:x}", n),
            _ => {
                return Err(SchemeError::Eval(
                    "number->string: invalid radix".to_string(),
                ))
            }
        },
        Val::Float(f) => format!("{}", f),
        _ => return Err(type_err("number", &args[0])),
    };
    Ok(Val::Str(Rc::new(RefCell::new(s))))
}

fn builtin_string_to_number(args: &[Val]) -> SchemeResult {
    if args.is_empty() || args.len() > 2 {
        return Err(SchemeError::Arity {
            expected: "1 or 2".to_string(),
            got: args.len(),
        });
    }
    let s = as_str(&args[0], "string->number")?;
    let radix = if args.len() == 2 {
        as_int(&args[1], "string->number")? as u32
    } else {
        10
    };
    if let Ok(n) = i64::from_str_radix(&s, radix) {
        return Ok(Val::Int(n));
    }
    if radix == 10 {
        if let Ok(f) = s.parse::<f64>() {
            return Ok(Val::Float(f));
        }
    }
    Ok(Val::Bool(false))
}

fn builtin_expt(args: &[Val]) -> SchemeResult {
    expect_len(args, 2, "expt")?;
    let base = to_num(&args[0], "expt")?;
    let exp = to_num(&args[1], "expt")?;
    let result = base.powf(exp);
    if matches!((&args[0], &args[1]), (Val::Int(_), Val::Int(e)) if *e >= 0) {
        let exp_i = exp as i32;
        let base_i = base as i64;
        Ok(Val::Int(base_i.pow(exp_i as u32)))
    } else {
        Ok(Val::Float(result))
    }
}

fn builtin_sqrt(args: &[Val]) -> SchemeResult {
    expect_len(args, 1, "sqrt")?;
    let n = to_num(&args[0], "sqrt")?;
    let r = n.sqrt();
    if matches!(&args[0], Val::Int(_)) && r.fract() == 0.0 {
        Ok(Val::Int(r as i64))
    } else {
        Ok(Val::Float(r))
    }
}

macro_rules! float_fn {
    ($name:ident, $func:ident) => {
        fn $name(args: &[Val]) -> SchemeResult {
            expect_len(args, 1, stringify!($func))?;
            let n = to_num(&args[0], stringify!($func))?;
            Ok(Val::Float(n.$func()))
        }
    };
}

float_fn!(builtin_exp, exp);
float_fn!(builtin_sin, sin);
float_fn!(builtin_cos, cos);
float_fn!(builtin_tan, tan);
float_fn!(builtin_asin, asin);
float_fn!(builtin_acos, acos);

fn builtin_log(args: &[Val]) -> SchemeResult {
    if args.is_empty() || args.len() > 2 {
        return Err(SchemeError::Arity {
            expected: "1 or 2".to_string(),
            got: args.len(),
        });
    }
    let n = to_num(&args[0], "log")?;
    if args.len() == 2 {
        let base = to_num(&args[1], "log")?;
        Ok(Val::Float(n.log(base)))
    } else {
        Ok(Val::Float(n.ln()))
    }
}

fn builtin_atan(args: &[Val]) -> SchemeResult {
    if args.len() == 2 {
        let y = to_num(&args[0], "atan")?;
        let x = to_num(&args[1], "atan")?;
        return Ok(Val::Float(y.atan2(x)));
    }
    expect_len(args, 1, "atan")?;
    let n = to_num(&args[0], "atan")?;
    Ok(Val::Float(n.atan()))
}

fn builtin_floor_div(args: &[Val]) -> SchemeResult {
    expect_len(args, 2, "floor/")?;
    let n = as_int(&args[0], "floor/")?;
    let d = as_int(&args[1], "floor/")?;
    if d == 0 {
        return Err(SchemeError::Eval("floor/: division by zero".to_string()));
    }
    let q = n.div_euclid(d);
    let r = n.rem_euclid(d);
    Ok(Val::list([Val::Int(q), Val::Int(r)].into_iter()))
}

fn builtin_floor_quotient(args: &[Val]) -> SchemeResult {
    expect_len(args, 2, "floor-quotient")?;
    let n = as_int(&args[0], "floor-quotient")?;
    let d = as_int(&args[1], "floor-quotient")?;
    if d == 0 {
        return Err(SchemeError::Eval(
            "floor-quotient: division by zero".to_string(),
        ));
    }
    Ok(Val::Int(n.div_euclid(d)))
}

fn builtin_floor_remainder(args: &[Val]) -> SchemeResult {
    expect_len(args, 2, "floor-remainder")?;
    let n = as_int(&args[0], "floor-remainder")?;
    let d = as_int(&args[1], "floor-remainder")?;
    if d == 0 {
        return Err(SchemeError::Eval(
            "floor-remainder: division by zero".to_string(),
        ));
    }
    Ok(Val::Int(n.rem_euclid(d)))
}

fn builtin_truncate_div(args: &[Val]) -> SchemeResult {
    expect_len(args, 2, "truncate/")?;
    let n = as_int(&args[0], "truncate/")?;
    let d = as_int(&args[1], "truncate/")?;
    if d == 0 {
        return Err(SchemeError::Eval("truncate/: division by zero".to_string()));
    }
    Ok(Val::list([Val::Int(n / d), Val::Int(n % d)].into_iter()))
}

fn builtin_truncate_quotient(args: &[Val]) -> SchemeResult {
    expect_len(args, 2, "truncate-quotient")?;
    let n = as_int(&args[0], "truncate-quotient")?;
    let d = as_int(&args[1], "truncate-quotient")?;
    if d == 0 {
        return Err(SchemeError::Eval(
            "truncate-quotient: division by zero".to_string(),
        ));
    }
    Ok(Val::Int(n / d))
}

fn builtin_truncate_remainder(args: &[Val]) -> SchemeResult {
    expect_len(args, 2, "truncate-remainder")?;
    let n = as_int(&args[0], "truncate-remainder")?;
    let d = as_int(&args[1], "truncate-remainder")?;
    if d == 0 {
        return Err(SchemeError::Eval(
            "truncate-remainder: division by zero".to_string(),
        ));
    }
    Ok(Val::Int(n % d))
}

fn builtin_exact_integer_sqrt(args: &[Val]) -> SchemeResult {
    expect_len(args, 1, "exact-integer-sqrt")?;
    let n = as_int(&args[0], "exact-integer-sqrt")?;
    if n < 0 {
        return Err(SchemeError::Eval(
            "exact-integer-sqrt: negative argument".to_string(),
        ));
    }
    let s = (n as f64).sqrt() as i64;
    let r = n - s * s;
    Ok(Val::list([Val::Int(s), Val::Int(r)].into_iter()))
}

// ---- Comparison ----

fn builtin_num_eq(args: &[Val]) -> SchemeResult {
    check_num_chain(args, "=", |a, b| a == b)
}
fn builtin_lt(args: &[Val]) -> SchemeResult {
    check_num_chain(args, "<", |a, b| a < b)
}
fn builtin_gt(args: &[Val]) -> SchemeResult {
    check_num_chain(args, ">", |a, b| a > b)
}
fn builtin_le(args: &[Val]) -> SchemeResult {
    check_num_chain(args, "<=", |a, b| a <= b)
}
fn builtin_ge(args: &[Val]) -> SchemeResult {
    check_num_chain(args, ">=", |a, b| a >= b)
}

fn check_num_chain(args: &[Val], name: &str, pred: impl Fn(f64, f64) -> bool) -> SchemeResult {
    if args.len() < 2 {
        return Err(SchemeError::Arity {
            expected: "at least 2".to_string(),
            got: args.len(),
        });
    }
    for pair in args.windows(2) {
        let a = to_num(&pair[0], name)?;
        let b = to_num(&pair[1], name)?;
        if !pred(a, b) {
            return Ok(Val::Bool(false));
        }
    }
    Ok(Val::Bool(true))
}

fn builtin_zero(args: &[Val]) -> SchemeResult {
    expect_len(args, 1, "zero?")?;
    Ok(Val::Bool(to_num(&args[0], "zero?")? == 0.0))
}
fn builtin_positive(args: &[Val]) -> SchemeResult {
    expect_len(args, 1, "positive?")?;
    Ok(Val::Bool(to_num(&args[0], "positive?")? > 0.0))
}
fn builtin_negative(args: &[Val]) -> SchemeResult {
    expect_len(args, 1, "negative?")?;
    Ok(Val::Bool(to_num(&args[0], "negative?")? < 0.0))
}
fn builtin_odd(args: &[Val]) -> SchemeResult {
    expect_len(args, 1, "odd?")?;
    Ok(Val::Bool(as_int(&args[0], "odd?")? % 2 != 0))
}
fn builtin_even(args: &[Val]) -> SchemeResult {
    expect_len(args, 1, "even?")?;
    Ok(Val::Bool(as_int(&args[0], "even?")? % 2 == 0))
}

// ---- Type predicates ----

fn builtin_numberp(args: &[Val]) -> SchemeResult {
    expect_len(args, 1, "number?")?;
    Ok(Val::Bool(matches!(&args[0], Val::Int(_) | Val::Float(_))))
}
fn builtin_integerp(args: &[Val]) -> SchemeResult {
    expect_len(args, 1, "integer?")?;
    Ok(Val::Bool(match &args[0] {
        Val::Int(_) => true,
        Val::Float(f) => f.fract() == 0.0 && f.is_finite(),
        _ => false,
    }))
}
fn builtin_rationalp(args: &[Val]) -> SchemeResult {
    expect_len(args, 1, "rational?")?;
    Ok(Val::Bool(match &args[0] {
        Val::Int(_) => true,
        Val::Float(f) => f.is_finite(),
        _ => false,
    }))
}
fn builtin_realp(args: &[Val]) -> SchemeResult {
    expect_len(args, 1, "real?")?;
    Ok(Val::Bool(matches!(&args[0], Val::Int(_) | Val::Float(_))))
}
fn builtin_complexp(args: &[Val]) -> SchemeResult {
    builtin_realp(args)
}
fn builtin_exactp(args: &[Val]) -> SchemeResult {
    expect_len(args, 1, "exact?")?;
    Ok(Val::Bool(matches!(&args[0], Val::Int(_))))
}
fn builtin_inexactp(args: &[Val]) -> SchemeResult {
    expect_len(args, 1, "inexact?")?;
    Ok(Val::Bool(matches!(&args[0], Val::Float(_))))
}
fn builtin_booleanp(args: &[Val]) -> SchemeResult {
    expect_len(args, 1, "boolean?")?;
    Ok(Val::Bool(matches!(&args[0], Val::Bool(_))))
}
fn builtin_pairp(args: &[Val]) -> SchemeResult {
    expect_len(args, 1, "pair?")?;
    Ok(Val::Bool(matches!(&args[0], Val::Pair(_))))
}
fn builtin_nullp(args: &[Val]) -> SchemeResult {
    expect_len(args, 1, "null?")?;
    Ok(Val::Bool(matches!(&args[0], Val::Nil)))
}
fn builtin_listp(args: &[Val]) -> SchemeResult {
    expect_len(args, 1, "list?")?;
    let mut v = args[0].clone();
    loop {
        match v {
            Val::Nil => return Ok(Val::Bool(true)),
            Val::Pair(ref p) => {
                let cdr = p.borrow().1.clone();
                v = cdr;
            }
            _ => return Ok(Val::Bool(false)),
        }
    }
}
fn builtin_symbolp(args: &[Val]) -> SchemeResult {
    expect_len(args, 1, "symbol?")?;
    Ok(Val::Bool(matches!(&args[0], Val::Symbol(_))))
}
fn builtin_stringp(args: &[Val]) -> SchemeResult {
    expect_len(args, 1, "string?")?;
    Ok(Val::Bool(matches!(&args[0], Val::Str(_))))
}
fn builtin_charp(args: &[Val]) -> SchemeResult {
    expect_len(args, 1, "char?")?;
    Ok(Val::Bool(matches!(&args[0], Val::Char(_))))
}
fn builtin_vectorp(args: &[Val]) -> SchemeResult {
    expect_len(args, 1, "vector?")?;
    Ok(Val::Bool(matches!(&args[0], Val::Vector(_))))
}
fn builtin_procedurep(args: &[Val]) -> SchemeResult {
    expect_len(args, 1, "procedure?")?;
    Ok(Val::Bool(matches!(&args[0], Val::Proc(_))))
}
fn builtin_portp(args: &[Val]) -> SchemeResult {
    expect_len(args, 1, "port?")?;
    Ok(Val::Bool(matches!(&args[0], Val::Port(_))))
}
fn builtin_eofp(args: &[Val]) -> SchemeResult {
    expect_len(args, 1, "eof-object?")?;
    Ok(Val::Bool(matches!(&args[0], Val::Eof)))
}
fn builtin_eof_object(args: &[Val]) -> SchemeResult {
    expect_len(args, 0, "eof-object")?;
    Ok(Val::Eof)
}
fn builtin_not(args: &[Val]) -> SchemeResult {
    expect_len(args, 1, "not")?;
    Ok(Val::Bool(matches!(&args[0], Val::Bool(false))))
}

// ---- Pair/list ----

fn builtin_cons(args: &[Val]) -> SchemeResult {
    expect_len(args, 2, "cons")?;
    Ok(Val::cons(args[0].clone(), args[1].clone()))
}

fn builtin_car(args: &[Val]) -> SchemeResult {
    expect_len(args, 1, "car")?;
    args[0].car()
}

fn builtin_cdr(args: &[Val]) -> SchemeResult {
    expect_len(args, 1, "cdr")?;
    args[0].cdr()
}

fn builtin_set_car(args: &[Val]) -> SchemeResult {
    expect_len(args, 2, "set-car!")?;
    match &args[0] {
        Val::Pair(p) => {
            p.borrow_mut().0 = args[1].clone();
            Ok(Val::Void)
        }
        _ => Err(type_err("pair", &args[0])),
    }
}

fn builtin_set_cdr(args: &[Val]) -> SchemeResult {
    expect_len(args, 2, "set-cdr!")?;
    match &args[0] {
        Val::Pair(p) => {
            p.borrow_mut().1 = args[1].clone();
            Ok(Val::Void)
        }
        _ => Err(type_err("pair", &args[0])),
    }
}

fn builtin_list(args: &[Val]) -> SchemeResult {
    Ok(Val::list(args.iter().cloned()))
}

fn builtin_length(args: &[Val]) -> SchemeResult {
    expect_len(args, 1, "length")?;
    let mut count = 0i64;
    let mut v = args[0].clone();
    loop {
        match v {
            Val::Nil => return Ok(Val::Int(count)),
            Val::Pair(p) => {
                count += 1;
                v = p.borrow().1.clone();
            }
            _ => {
                return Err(SchemeError::Eval(format!(
                    "length: not a proper list: {}",
                    args[0]
                )))
            }
        }
    }
}

fn builtin_append(args: &[Val]) -> SchemeResult {
    if args.is_empty() {
        return Ok(Val::Nil);
    }
    if args.len() == 1 {
        return Ok(args[0].clone());
    }
    // Collect all but last list
    let mut result_items: Vec<Val> = Vec::new();
    for list in &args[..args.len() - 1] {
        let mut v = list.clone();
        loop {
            match v {
                Val::Nil => break,
                Val::Pair(p) => {
                    let (car, cdr) = {
                        let b = p.borrow();
                        (b.0.clone(), b.1.clone())
                    };
                    result_items.push(car);
                    v = cdr;
                }
                _ => return Err(SchemeError::Eval("append: not a proper list".to_string())),
            }
        }
    }
    // Build the result from the last element (can be non-list)
    let mut result = args.last().unwrap().clone();
    for item in result_items.into_iter().rev() {
        result = Val::cons(item, result);
    }
    Ok(result)
}

fn builtin_reverse(args: &[Val]) -> SchemeResult {
    expect_len(args, 1, "reverse")?;
    let items = args[0].to_vec()?;
    Ok(Val::list(items.into_iter().rev()))
}

fn builtin_list_tail(args: &[Val]) -> SchemeResult {
    expect_len(args, 2, "list-tail")?;
    let k = as_int(&args[1], "list-tail")? as usize;
    let mut v = args[0].clone();
    for _ in 0..k {
        match v {
            Val::Pair(p) => v = p.borrow().1.clone(),
            _ => {
                return Err(SchemeError::Eval(
                    "list-tail: index out of range".to_string(),
                ))
            }
        }
    }
    Ok(v)
}

fn builtin_list_ref(args: &[Val]) -> SchemeResult {
    let tail = builtin_list_tail(args)?;
    match tail {
        Val::Pair(p) => Ok(p.borrow().0.clone()),
        _ => Err(SchemeError::Eval(
            "list-ref: index out of range".to_string(),
        )),
    }
}

fn builtin_list_copy(args: &[Val]) -> SchemeResult {
    expect_len(args, 1, "list-copy")?;
    let items = args[0].to_vec()?;
    Ok(Val::list(items.into_iter()))
}

fn builtin_memq(args: &[Val]) -> SchemeResult {
    expect_len(args, 2, "memq")?;
    let needle = &args[0];
    let mut v = args[1].clone();
    loop {
        match v {
            Val::Nil => return Ok(Val::Bool(false)),
            Val::Pair(ref p) => {
                let (car, cdr) = {
                    let b = p.borrow();
                    (b.0.clone(), b.1.clone())
                };
                if scheme_eq(needle, &car) {
                    return Ok(v.clone());
                }
                v = cdr;
            }
            _ => return Ok(Val::Bool(false)),
        }
    }
}

fn builtin_memv(args: &[Val]) -> SchemeResult {
    builtin_memq(args) // eqv? same as eq? for our purposes
}

fn builtin_member(args: &[Val]) -> SchemeResult {
    expect_len(args, 2, "member")?;
    let needle = &args[0];
    let mut v = args[1].clone();
    loop {
        match v {
            Val::Nil => return Ok(Val::Bool(false)),
            Val::Pair(ref p) => {
                let (car, cdr) = {
                    let b = p.borrow();
                    (b.0.clone(), b.1.clone())
                };
                if scheme_equal(needle, &car) {
                    return Ok(v.clone());
                }
                v = cdr;
            }
            _ => return Ok(Val::Bool(false)),
        }
    }
}

fn builtin_assq(args: &[Val]) -> SchemeResult {
    expect_len(args, 2, "assq")?;
    let key = &args[0];
    let mut v = args[1].clone();
    loop {
        match v {
            Val::Nil => return Ok(Val::Bool(false)),
            Val::Pair(p) => {
                let (pair, rest) = {
                    let b = p.borrow();
                    (b.0.clone(), b.1.clone())
                };
                if let Val::Pair(ref kv) = pair {
                    if scheme_eq(key, &kv.borrow().0) {
                        return Ok(pair.clone());
                    }
                }
                v = rest;
            }
            _ => return Ok(Val::Bool(false)),
        }
    }
}

fn builtin_assv(args: &[Val]) -> SchemeResult {
    builtin_assq(args)
}

fn builtin_assoc(args: &[Val]) -> SchemeResult {
    expect_len(args, 2, "assoc")?;
    let key = &args[0];
    let mut v = args[1].clone();
    loop {
        match v {
            Val::Nil => return Ok(Val::Bool(false)),
            Val::Pair(p) => {
                let (pair, rest) = {
                    let b = p.borrow();
                    (b.0.clone(), b.1.clone())
                };
                if let Val::Pair(ref kv) = pair {
                    if scheme_equal(key, &kv.borrow().0) {
                        return Ok(pair.clone());
                    }
                }
                v = rest;
            }
            _ => return Ok(Val::Bool(false)),
        }
    }
}

// ---- Equality ----

fn scheme_eq(a: &Val, b: &Val) -> bool {
    match (a, b) {
        (Val::Bool(x), Val::Bool(y)) => x == y,
        (Val::Symbol(x), Val::Symbol(y)) => x == y,
        (Val::Nil, Val::Nil) => true,
        (Val::Int(x), Val::Int(y)) => x == y,
        (Val::Char(x), Val::Char(y)) => x == y,
        (Val::Pair(x), Val::Pair(y)) => Rc::ptr_eq(x, y),
        (Val::Vector(x), Val::Vector(y)) => Rc::ptr_eq(x, y),
        (Val::Str(x), Val::Str(y)) => Rc::ptr_eq(x, y),
        _ => false,
    }
}

fn scheme_equal(a: &Val, b: &Val) -> bool {
    match (a, b) {
        (Val::Int(x), Val::Int(y)) => x == y,
        (Val::Float(x), Val::Float(y)) => x == y,
        (Val::Int(x), Val::Float(y)) => (*x as f64) == *y,
        (Val::Float(x), Val::Int(y)) => *x == (*y as f64),
        (Val::Bool(x), Val::Bool(y)) => x == y,
        (Val::Char(x), Val::Char(y)) => x == y,
        (Val::Symbol(x), Val::Symbol(y)) => x == y,
        (Val::Nil, Val::Nil) => true,
        (Val::Str(x), Val::Str(y)) => *x.borrow() == *y.borrow(),
        (Val::Pair(x), Val::Pair(y)) => {
            let (xa, xd) = {
                let b = x.borrow();
                (b.0.clone(), b.1.clone())
            };
            let (ya, yd) = {
                let b = y.borrow();
                (b.0.clone(), b.1.clone())
            };
            scheme_equal(&xa, &ya) && scheme_equal(&xd, &yd)
        }
        (Val::Vector(x), Val::Vector(y)) => {
            let xb = x.borrow();
            let yb = y.borrow();
            xb.len() == yb.len() && xb.iter().zip(yb.iter()).all(|(a, b)| scheme_equal(a, b))
        }
        _ => false,
    }
}

fn builtin_eqp(args: &[Val]) -> SchemeResult {
    expect_len(args, 2, "eq?")?;
    Ok(Val::Bool(scheme_eq(&args[0], &args[1])))
}

fn builtin_eqvp(args: &[Val]) -> SchemeResult {
    expect_len(args, 2, "eqv?")?;
    Ok(Val::Bool(match (&args[0], &args[1]) {
        (Val::Int(a), Val::Int(b)) => a == b,
        (Val::Float(a), Val::Float(b)) => a == b,
        _ => scheme_eq(&args[0], &args[1]),
    }))
}

fn builtin_equalp(args: &[Val]) -> SchemeResult {
    expect_len(args, 2, "equal?")?;
    Ok(Val::Bool(scheme_equal(&args[0], &args[1])))
}

// ---- String operations ----

fn builtin_make_string(args: &[Val]) -> SchemeResult {
    if args.is_empty() || args.len() > 2 {
        return Err(SchemeError::Arity {
            expected: "1 or 2".to_string(),
            got: args.len(),
        });
    }
    let n = as_int(&args[0], "make-string")? as usize;
    let ch = if args.len() == 2 {
        as_char(&args[1], "make-string")?
    } else {
        '\0'
    };
    Ok(Val::Str(Rc::new(RefCell::new(
        std::iter::repeat_n(ch, n).collect(),
    ))))
}

fn builtin_string(args: &[Val]) -> SchemeResult {
    let mut s = String::new();
    for a in args {
        s.push(as_char(a, "string")?);
    }
    Ok(Val::Str(Rc::new(RefCell::new(s))))
}

fn builtin_string_length(args: &[Val]) -> SchemeResult {
    expect_len(args, 1, "string-length")?;
    let s = as_str(&args[0], "string-length")?;
    Ok(Val::Int(s.chars().count() as i64))
}

fn builtin_string_ref(args: &[Val]) -> SchemeResult {
    expect_len(args, 2, "string-ref")?;
    let s = as_str(&args[0], "string-ref")?;
    let k = as_int(&args[1], "string-ref")? as usize;
    s.chars()
        .nth(k)
        .map(Val::Char)
        .ok_or_else(|| SchemeError::Eval("string-ref: index out of range".to_string()))
}

fn builtin_string_set(args: &[Val]) -> SchemeResult {
    expect_len(args, 3, "string-set!")?;
    let k = as_int(&args[1], "string-set!")? as usize;
    let ch = as_char(&args[2], "string-set!")?;
    match &args[0] {
        Val::Str(s) => {
            let mut b = s.borrow_mut();
            let mut chars: Vec<char> = b.chars().collect();
            if k >= chars.len() {
                return Err(SchemeError::Eval(
                    "string-set!: index out of range".to_string(),
                ));
            }
            chars[k] = ch;
            *b = chars.into_iter().collect();
            Ok(Val::Void)
        }
        _ => Err(type_err("string", &args[0])),
    }
}

macro_rules! string_cmp {
    ($name:ident, $func:expr) => {
        fn $name(args: &[Val]) -> SchemeResult {
            if args.len() < 2 {
                return Err(SchemeError::Arity {
                    expected: "at least 2".to_string(),
                    got: args.len(),
                });
            }
            for w in args.windows(2) {
                let a = as_str(&w[0], stringify!($name))?;
                let b = as_str(&w[1], stringify!($name))?;
                if !($func)(&a, &b) {
                    return Ok(Val::Bool(false));
                }
            }
            Ok(Val::Bool(true))
        }
    };
}

string_cmp!(builtin_string_eq, |a: &str, b: &str| a == b);
string_cmp!(builtin_string_lt, |a: &str, b: &str| a < b);
string_cmp!(builtin_string_gt, |a: &str, b: &str| a > b);
string_cmp!(builtin_string_le, |a: &str, b: &str| a <= b);
string_cmp!(builtin_string_ge, |a: &str, b: &str| a >= b);
string_cmp!(builtin_string_ci_eq, |a: &str, b: &str| a.to_lowercase()
    == b.to_lowercase());
string_cmp!(builtin_string_ci_lt, |a: &str, b: &str| a.to_lowercase()
    < b.to_lowercase());
string_cmp!(builtin_string_ci_gt, |a: &str, b: &str| a.to_lowercase()
    > b.to_lowercase());
string_cmp!(builtin_string_ci_le, |a: &str, b: &str| a.to_lowercase()
    <= b.to_lowercase());
string_cmp!(builtin_string_ci_ge, |a: &str, b: &str| a.to_lowercase()
    >= b.to_lowercase());

fn builtin_substring(args: &[Val]) -> SchemeResult {
    if args.len() < 2 || args.len() > 3 {
        return Err(SchemeError::Arity {
            expected: "2 or 3".to_string(),
            got: args.len(),
        });
    }
    let s = as_str(&args[0], "substring")?;
    let chars: Vec<char> = s.chars().collect();
    let start = as_int(&args[1], "substring")? as usize;
    let end = if args.len() == 3 {
        as_int(&args[2], "substring")? as usize
    } else {
        chars.len()
    };
    if start > end || end > chars.len() {
        return Err(SchemeError::Eval(
            "substring: index out of range".to_string(),
        ));
    }
    Ok(Val::Str(Rc::new(RefCell::new(
        chars[start..end].iter().collect(),
    ))))
}

fn builtin_string_append(args: &[Val]) -> SchemeResult {
    let mut result = String::new();
    for a in args {
        result.push_str(&as_str(a, "string-append")?);
    }
    Ok(Val::Str(Rc::new(RefCell::new(result))))
}

fn builtin_string_to_list(args: &[Val]) -> SchemeResult {
    if args.is_empty() || args.len() > 3 {
        return Err(SchemeError::Arity {
            expected: "1-3".to_string(),
            got: args.len(),
        });
    }
    let s = as_str(&args[0], "string->list")?;
    let chars: Vec<char> = s.chars().collect();
    let start = if args.len() >= 2 {
        as_int(&args[1], "string->list")? as usize
    } else {
        0
    };
    let end = if args.len() == 3 {
        as_int(&args[2], "string->list")? as usize
    } else {
        chars.len()
    };
    Ok(Val::list(chars[start..end].iter().map(|&c| Val::Char(c))))
}

fn builtin_list_to_string(args: &[Val]) -> SchemeResult {
    expect_len(args, 1, "list->string")?;
    let items = args[0].to_vec()?;
    let mut s = String::new();
    for item in items {
        s.push(as_char(&item, "list->string")?);
    }
    Ok(Val::Str(Rc::new(RefCell::new(s))))
}

fn builtin_string_copy(args: &[Val]) -> SchemeResult {
    if args.is_empty() || args.len() > 3 {
        return Err(SchemeError::Arity {
            expected: "1-3".to_string(),
            got: args.len(),
        });
    }
    let s = as_str(&args[0], "string-copy")?;
    let chars: Vec<char> = s.chars().collect();
    let start = if args.len() >= 2 {
        as_int(&args[1], "string-copy")? as usize
    } else {
        0
    };
    let end = if args.len() == 3 {
        as_int(&args[2], "string-copy")? as usize
    } else {
        chars.len()
    };
    Ok(Val::Str(Rc::new(RefCell::new(
        chars[start..end].iter().collect(),
    ))))
}

fn builtin_string_fill(args: &[Val]) -> SchemeResult {
    if args.len() < 2 || args.len() > 4 {
        return Err(SchemeError::Arity {
            expected: "2-4".to_string(),
            got: args.len(),
        });
    }
    let ch = as_char(&args[1], "string-fill!")?;
    match &args[0] {
        Val::Str(s) => {
            let mut b = s.borrow_mut();
            let n = b.chars().count();
            let start = if args.len() >= 3 {
                as_int(&args[2], "string-fill!")? as usize
            } else {
                0
            };
            let end = if args.len() == 4 {
                as_int(&args[3], "string-fill!")? as usize
            } else {
                n
            };
            let mut chars: Vec<char> = b.chars().collect();
            for i in start..end {
                if i < chars.len() {
                    chars[i] = ch;
                }
            }
            *b = chars.into_iter().collect();
            Ok(Val::Void)
        }
        _ => Err(type_err("string", &args[0])),
    }
}

fn builtin_string_to_symbol(args: &[Val]) -> SchemeResult {
    expect_len(args, 1, "string->symbol")?;
    let s = as_str(&args[0], "string->symbol")?;
    Ok(Val::Symbol(Rc::from(s.as_str())))
}

fn builtin_symbol_to_string(args: &[Val]) -> SchemeResult {
    expect_len(args, 1, "symbol->string")?;
    match &args[0] {
        Val::Symbol(s) => Ok(Val::Str(Rc::new(RefCell::new(s.to_string())))),
        _ => Err(type_err("symbol", &args[0])),
    }
}

fn builtin_string_upcase(args: &[Val]) -> SchemeResult {
    expect_len(args, 1, "string-upcase")?;
    let s = as_str(&args[0], "string-upcase")?;
    Ok(Val::Str(Rc::new(RefCell::new(s.to_uppercase()))))
}

fn builtin_string_downcase(args: &[Val]) -> SchemeResult {
    expect_len(args, 1, "string-downcase")?;
    let s = as_str(&args[0], "string-downcase")?;
    Ok(Val::Str(Rc::new(RefCell::new(s.to_lowercase()))))
}

// ---- Character operations ----

fn builtin_char_to_integer(args: &[Val]) -> SchemeResult {
    expect_len(args, 1, "char->integer")?;
    Ok(Val::Int(as_char(&args[0], "char->integer")? as i64))
}

fn builtin_integer_to_char(args: &[Val]) -> SchemeResult {
    expect_len(args, 1, "integer->char")?;
    let n = as_int(&args[0], "integer->char")?;
    char::from_u32(n as u32)
        .map(Val::Char)
        .ok_or_else(|| SchemeError::Eval("integer->char: invalid codepoint".to_string()))
}

macro_rules! char_cmp {
    ($name:ident, $func:expr) => {
        fn $name(args: &[Val]) -> SchemeResult {
            if args.len() < 2 {
                return Err(SchemeError::Arity {
                    expected: "at least 2".to_string(),
                    got: args.len(),
                });
            }
            for w in args.windows(2) {
                let a = as_char(&w[0], stringify!($name))?;
                let b = as_char(&w[1], stringify!($name))?;
                if !($func)(a, b) {
                    return Ok(Val::Bool(false));
                }
            }
            Ok(Val::Bool(true))
        }
    };
}

char_cmp!(builtin_char_eq, |a: char, b: char| a == b);
char_cmp!(builtin_char_lt, |a: char, b: char| a < b);
char_cmp!(builtin_char_gt, |a: char, b: char| a > b);
char_cmp!(builtin_char_le, |a: char, b: char| a <= b);
char_cmp!(builtin_char_ge, |a: char, b: char| a >= b);
char_cmp!(builtin_char_ci_eq, |a: char, b: char| a
    .to_lowercase()
    .eq(b.to_lowercase()));
char_cmp!(builtin_char_ci_lt, |a: char, b: char| a
    .to_lowercase()
    .to_string()
    < b.to_lowercase().to_string());
char_cmp!(builtin_char_ci_gt, |a: char, b: char| a
    .to_lowercase()
    .to_string()
    > b.to_lowercase().to_string());
char_cmp!(builtin_char_ci_le, |a: char, b: char| a
    .to_lowercase()
    .to_string()
    <= b.to_lowercase().to_string());
char_cmp!(builtin_char_ci_ge, |a: char, b: char| a
    .to_lowercase()
    .to_string()
    >= b.to_lowercase().to_string());

fn builtin_char_alpha(args: &[Val]) -> SchemeResult {
    expect_len(args, 1, "char-alphabetic?")?;
    Ok(Val::Bool(
        as_char(&args[0], "char-alphabetic?")?.is_alphabetic(),
    ))
}
fn builtin_char_numeric(args: &[Val]) -> SchemeResult {
    expect_len(args, 1, "char-numeric?")?;
    Ok(Val::Bool(as_char(&args[0], "char-numeric?")?.is_numeric()))
}
fn builtin_char_whitespace(args: &[Val]) -> SchemeResult {
    expect_len(args, 1, "char-whitespace?")?;
    Ok(Val::Bool(
        as_char(&args[0], "char-whitespace?")?.is_whitespace(),
    ))
}
fn builtin_char_upper(args: &[Val]) -> SchemeResult {
    expect_len(args, 1, "char-upper-case?")?;
    Ok(Val::Bool(
        as_char(&args[0], "char-upper-case?")?.is_uppercase(),
    ))
}
fn builtin_char_lower(args: &[Val]) -> SchemeResult {
    expect_len(args, 1, "char-lower-case?")?;
    Ok(Val::Bool(
        as_char(&args[0], "char-lower-case?")?.is_lowercase(),
    ))
}
fn builtin_char_upcase(args: &[Val]) -> SchemeResult {
    expect_len(args, 1, "char-upcase")?;
    let c = as_char(&args[0], "char-upcase")?;
    Ok(Val::Char(c.to_uppercase().next().unwrap_or(c)))
}
fn builtin_char_downcase(args: &[Val]) -> SchemeResult {
    expect_len(args, 1, "char-downcase")?;
    let c = as_char(&args[0], "char-downcase")?;
    Ok(Val::Char(c.to_lowercase().next().unwrap_or(c)))
}

// ---- Vector operations ----

fn builtin_make_vector(args: &[Val]) -> SchemeResult {
    if args.is_empty() || args.len() > 2 {
        return Err(SchemeError::Arity {
            expected: "1 or 2".to_string(),
            got: args.len(),
        });
    }
    let n = as_int(&args[0], "make-vector")? as usize;
    let fill = if args.len() == 2 {
        args[1].clone()
    } else {
        Val::Int(0)
    };
    Ok(Val::Vector(Rc::new(RefCell::new(vec![fill; n]))))
}

fn builtin_vector(args: &[Val]) -> SchemeResult {
    Ok(Val::Vector(Rc::new(RefCell::new(args.to_vec()))))
}

fn builtin_vector_length(args: &[Val]) -> SchemeResult {
    expect_len(args, 1, "vector-length")?;
    match &args[0] {
        Val::Vector(v) => Ok(Val::Int(v.borrow().len() as i64)),
        _ => Err(type_err("vector", &args[0])),
    }
}

fn builtin_vector_ref(args: &[Val]) -> SchemeResult {
    expect_len(args, 2, "vector-ref")?;
    let k = as_int(&args[1], "vector-ref")? as usize;
    match &args[0] {
        Val::Vector(v) => {
            let b = v.borrow();
            b.get(k)
                .cloned()
                .ok_or_else(|| SchemeError::Eval("vector-ref: index out of range".to_string()))
        }
        _ => Err(type_err("vector", &args[0])),
    }
}

fn builtin_vector_set(args: &[Val]) -> SchemeResult {
    expect_len(args, 3, "vector-set!")?;
    let k = as_int(&args[1], "vector-set!")? as usize;
    match &args[0] {
        Val::Vector(v) => {
            let mut b = v.borrow_mut();
            if k >= b.len() {
                return Err(SchemeError::Eval(
                    "vector-set!: index out of range".to_string(),
                ));
            }
            b[k] = args[2].clone();
            Ok(Val::Void)
        }
        _ => Err(type_err("vector", &args[0])),
    }
}

fn builtin_vector_to_list(args: &[Val]) -> SchemeResult {
    if args.is_empty() || args.len() > 3 {
        return Err(SchemeError::Arity {
            expected: "1-3".to_string(),
            got: args.len(),
        });
    }
    match &args[0] {
        Val::Vector(v) => {
            let b = v.borrow();
            let start = if args.len() >= 2 {
                as_int(&args[1], "vector->list")? as usize
            } else {
                0
            };
            let end = if args.len() == 3 {
                as_int(&args[2], "vector->list")? as usize
            } else {
                b.len()
            };
            Ok(Val::list(b[start..end].iter().cloned()))
        }
        _ => Err(type_err("vector", &args[0])),
    }
}

fn builtin_list_to_vector(args: &[Val]) -> SchemeResult {
    expect_len(args, 1, "list->vector")?;
    let items = args[0].to_vec()?;
    Ok(Val::Vector(Rc::new(RefCell::new(items))))
}

fn builtin_vector_fill(args: &[Val]) -> SchemeResult {
    if args.len() < 2 || args.len() > 4 {
        return Err(SchemeError::Arity {
            expected: "2-4".to_string(),
            got: args.len(),
        });
    }
    let fill = args[1].clone();
    match &args[0] {
        Val::Vector(v) => {
            let mut b = v.borrow_mut();
            let n = b.len();
            let start = if args.len() >= 3 {
                as_int(&args[2], "vector-fill!")? as usize
            } else {
                0
            };
            let end = if args.len() == 4 {
                as_int(&args[3], "vector-fill!")? as usize
            } else {
                n
            };
            for i in start..end {
                if i < b.len() {
                    b[i] = fill.clone();
                }
            }
            Ok(Val::Void)
        }
        _ => Err(type_err("vector", &args[0])),
    }
}

fn builtin_vector_copy(args: &[Val]) -> SchemeResult {
    if args.is_empty() || args.len() > 3 {
        return Err(SchemeError::Arity {
            expected: "1-3".to_string(),
            got: args.len(),
        });
    }
    match &args[0] {
        Val::Vector(v) => {
            let b = v.borrow();
            let start = if args.len() >= 2 {
                as_int(&args[1], "vector-copy")? as usize
            } else {
                0
            };
            let end = if args.len() == 3 {
                as_int(&args[2], "vector-copy")? as usize
            } else {
                b.len()
            };
            Ok(Val::Vector(Rc::new(RefCell::new(b[start..end].to_vec()))))
        }
        _ => Err(type_err("vector", &args[0])),
    }
}

// ---- I/O ----

fn builtin_display(args: &[Val]) -> SchemeResult {
    if args.is_empty() || args.len() > 2 {
        return Err(SchemeError::Arity {
            expected: "1 or 2".to_string(),
            got: args.len(),
        });
    }
    print!("{}", args[0]);
    Ok(Val::Void)
}

fn builtin_write(args: &[Val]) -> SchemeResult {
    if args.is_empty() || args.len() > 2 {
        return Err(SchemeError::Arity {
            expected: "1 or 2".to_string(),
            got: args.len(),
        });
    }
    print!("{}", write_val(&args[0]));
    Ok(Val::Void)
}

fn builtin_newline(args: &[Val]) -> SchemeResult {
    if args.len() > 1 {
        return Err(SchemeError::Arity {
            expected: "0 or 1".to_string(),
            got: args.len(),
        });
    }
    println!();
    Ok(Val::Void)
}

fn builtin_write_char(args: &[Val]) -> SchemeResult {
    if args.is_empty() || args.len() > 2 {
        return Err(SchemeError::Arity {
            expected: "1 or 2".to_string(),
            got: args.len(),
        });
    }
    let c = as_char(&args[0], "write-char")?;
    print!("{}", c);
    Ok(Val::Void)
}

fn builtin_read_char(_args: &[Val]) -> SchemeResult {
    use std::io::Read;
    let mut buf = [0u8; 1];
    match std::io::stdin().read(&mut buf) {
        Ok(0) => Ok(Val::Eof),
        Ok(_) => Ok(Val::Char(buf[0] as char)),
        Err(e) => Err(SchemeError::Io(e.to_string())),
    }
}

fn builtin_peek_char(_args: &[Val]) -> SchemeResult {
    // Simple implementation - not truly buffered
    Ok(Val::Eof)
}

fn builtin_char_ready(_args: &[Val]) -> SchemeResult {
    Ok(Val::Bool(true))
}

fn builtin_read_line(_args: &[Val]) -> SchemeResult {
    let mut line = String::new();
    match std::io::stdin().read_line(&mut line) {
        Ok(0) => Ok(Val::Eof),
        Ok(_) => {
            if line.ends_with('\n') {
                line.pop();
                if line.ends_with('\r') {
                    line.pop();
                }
            }
            Ok(Val::Str(Rc::new(RefCell::new(line))))
        }
        Err(e) => Err(SchemeError::Io(e.to_string())),
    }
}

fn builtin_current_input_port(_args: &[Val]) -> SchemeResult {
    Ok(Val::Port(crate::types::Port::StdIn))
}
fn builtin_current_output_port(_args: &[Val]) -> SchemeResult {
    Ok(Val::Port(crate::types::Port::StdOut))
}
fn builtin_current_error_port(_args: &[Val]) -> SchemeResult {
    Ok(Val::Port(crate::types::Port::StdErr))
}
fn builtin_open_input_file(args: &[Val]) -> SchemeResult {
    expect_len(args, 1, "open-input-file")?;
    // Basic stub - full port implementation would need more work
    Err(SchemeError::Eval(
        "open-input-file: not fully implemented".to_string(),
    ))
}
fn builtin_open_output_file(args: &[Val]) -> SchemeResult {
    expect_len(args, 1, "open-output-file")?;
    Err(SchemeError::Eval(
        "open-output-file: not fully implemented".to_string(),
    ))
}
fn builtin_close_port(_args: &[Val]) -> SchemeResult {
    Ok(Val::Void)
}
fn builtin_input_portp(args: &[Val]) -> SchemeResult {
    expect_len(args, 1, "input-port?")?;
    Ok(Val::Bool(matches!(
        &args[0],
        Val::Port(crate::types::Port::StdIn)
    )))
}
fn builtin_output_portp(args: &[Val]) -> SchemeResult {
    expect_len(args, 1, "output-port?")?;
    Ok(Val::Bool(matches!(
        &args[0],
        Val::Port(crate::types::Port::StdOut | crate::types::Port::StdErr)
    )))
}

// ---- Misc ----

fn builtin_error(args: &[Val]) -> SchemeResult {
    if args.is_empty() {
        return Err(SchemeError::Eval("error: no message".to_string()));
    }
    let msg = match &args[0] {
        Val::Str(s) => s.borrow().clone(),
        other => format!("{}", other),
    };
    let irritants: Vec<String> = args[1..].iter().map(write_val).collect();
    if irritants.is_empty() {
        Err(SchemeError::Eval(msg))
    } else {
        Err(SchemeError::Eval(format!(
            "{} {}",
            msg,
            irritants.join(" ")
        )))
    }
}

fn builtin_apply_stub(_args: &[Val]) -> SchemeResult {
    // Real apply is handled in eval.rs as a special case
    Err(SchemeError::Eval("apply: internal error".to_string()))
}

fn builtin_values(args: &[Val]) -> SchemeResult {
    // For single value, just return it
    if args.len() == 1 {
        return Ok(args[0].clone());
    }
    // Multiple values as a list (simplified)
    Ok(Val::list(args.iter().cloned()))
}

fn builtin_call_with_values_stub(_args: &[Val]) -> SchemeResult {
    Err(SchemeError::Eval(
        "call-with-values: use eval call".to_string(),
    ))
}

fn builtin_with_exception_handler_stub(_args: &[Val]) -> SchemeResult {
    Err(SchemeError::Eval(
        "with-exception-handler: use eval call".to_string(),
    ))
}

fn builtin_raise(args: &[Val]) -> SchemeResult {
    if args.is_empty() {
        return Err(SchemeError::Eval("raise: no argument".to_string()));
    }
    Err(SchemeError::Eval(format!("{}", args[0])))
}

fn builtin_error_message(args: &[Val]) -> SchemeResult {
    expect_len(args, 1, "error-message")?;
    match &args[0] {
        Val::Str(s) => Ok(Val::Str(s.clone())),
        other => Ok(Val::Str(Rc::new(RefCell::new(format!("{}", other))))),
    }
}

fn builtin_boolean_eq(args: &[Val]) -> SchemeResult {
    if args.len() < 2 {
        return Err(SchemeError::Arity {
            expected: "at least 2".to_string(),
            got: args.len(),
        });
    }
    let first = match &args[0] {
        Val::Bool(b) => *b,
        _ => return Err(type_err("boolean", &args[0])),
    };
    for a in &args[1..] {
        match a {
            Val::Bool(b) if *b == first => {}
            Val::Bool(_) => return Ok(Val::Bool(false)),
            _ => return Err(type_err("boolean", a)),
        }
    }
    Ok(Val::Bool(true))
}

fn builtin_exit(args: &[Val]) -> SchemeResult {
    let code = if args.is_empty() {
        0
    } else {
        as_int(&args[0], "exit").unwrap_or(0) as i32
    };
    std::process::exit(code);
}

fn builtin_force(_args: &[Val]) -> SchemeResult {
    // Simplified: promises handled in eval
    if _args.is_empty() {
        return Err(SchemeError::Arity {
            expected: "1".to_string(),
            got: 0,
        });
    }
    Ok(_args[0].clone())
}

fn builtin_make_promise(_args: &[Val]) -> SchemeResult {
    Ok(Val::Void)
}

fn builtin_promisep(_args: &[Val]) -> SchemeResult {
    Ok(Val::Bool(false))
}

fn builtin_make_parameter(args: &[Val]) -> SchemeResult {
    if args.is_empty() {
        return Err(SchemeError::Arity {
            expected: "1".to_string(),
            got: 0,
        });
    }
    let val = args[0].clone();
    // Return a procedure that returns the value when called with no args
    // For simplicity, we store the value in a cell
    let cell = Rc::new(RefCell::new(val));
    // We can't really make this a builtin fn ptr without closures,
    // so we use a Lambda with a captured environment
    let inner_env = Env::new();
    inner_env
        .borrow_mut()
        .define("__param_val__".to_string(), cell.borrow().clone());
    Ok(Val::Proc(crate::types::Proc::Lambda {
        params: crate::types::LambdaParams::Variadic {
            required: vec![],
            rest: "args".to_string(),
        },
        body: vec![Val::Symbol(Rc::from("__param_val__"))],
        env: inner_env,
        name: Some("parameter".to_string()),
    }))
}

fn builtin_current_jiffy(_args: &[Val]) -> SchemeResult {
    use std::time::{SystemTime, UNIX_EPOCH};
    let ms = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis() as i64)
        .unwrap_or(0);
    Ok(Val::Int(ms))
}

fn builtin_jiffies_per_second(_args: &[Val]) -> SchemeResult {
    Ok(Val::Int(1000))
}

// ---- Helper functions ----

fn expect_len(args: &[Val], n: usize, _name: &str) -> Result<(), SchemeError> {
    if args.len() != n {
        Err(SchemeError::Arity {
            expected: n.to_string(),
            got: args.len(),
        })
    } else {
        Ok(())
    }
}

fn type_err(expected: &str, got: &Val) -> SchemeError {
    SchemeError::Type {
        expected: expected.to_string(),
        got: got.type_name().to_string(),
    }
}

fn as_int(v: &Val, _name: &str) -> Result<i64, SchemeError> {
    match v {
        Val::Int(n) => Ok(*n),
        Val::Float(f) if f.fract() == 0.0 => Ok(*f as i64),
        _ => Err(SchemeError::Type {
            expected: "integer".to_string(),
            got: v.type_name().to_string(),
        }),
    }
}

fn as_str(v: &Val, _name: &str) -> Result<String, SchemeError> {
    match v {
        Val::Str(s) => Ok(s.borrow().clone()),
        _ => Err(SchemeError::Type {
            expected: "string".to_string(),
            got: v.type_name().to_string(),
        }),
    }
}

fn as_char(v: &Val, _name: &str) -> Result<char, SchemeError> {
    match v {
        Val::Char(c) => Ok(*c),
        _ => Err(SchemeError::Type {
            expected: "char".to_string(),
            got: v.type_name().to_string(),
        }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn env() -> Rc<RefCell<Env>> {
        let e = Env::new();
        register_builtins(&e);
        e
    }

    fn call(name: &str, args: Vec<Val>) -> SchemeResult {
        let e = env();
        let proc = e.borrow().get(name).unwrap();
        if let Val::Proc(Proc::Builtin { func, .. }) = proc {
            func(&args)
        } else {
            panic!("{} is not a builtin", name)
        }
    }

    #[test]
    fn test_add() {
        assert_eq!(
            call("+", vec![Val::Int(1), Val::Int(2)]).unwrap(),
            Val::Int(3)
        );
    }

    #[test]
    fn test_mul() {
        assert_eq!(
            call("*", vec![Val::Int(3), Val::Int(4)]).unwrap(),
            Val::Int(12)
        );
    }

    #[test]
    fn test_eq() {
        assert_eq!(
            call("equal?", vec![Val::Int(1), Val::Int(1)]).unwrap(),
            Val::Bool(true)
        );
    }

    #[test]
    fn test_string_append() {
        let a = Val::Str(Rc::new(RefCell::new("hello".to_string())));
        let b = Val::Str(Rc::new(RefCell::new(" world".to_string())));
        let result = call("string-append", vec![a, b]).unwrap();
        assert_eq!(result.to_string(), "hello world");
    }
}
