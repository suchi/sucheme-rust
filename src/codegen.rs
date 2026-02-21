//! LLVM 17 JIT back-end for Scheme integer arithmetic.
//!
//! Enabled via the `llvm-backend` Cargo feature:
//! ```sh
//! cargo build --features llvm-backend
//! cargo run  --features llvm-backend -- --jit -e "(let ((n 1000)) (* n n))"
//! ```
//!
//! # Supported subset
//! The JIT handles integer-only sub-expressions built from:
//! - Integer / boolean literals
//! - Arithmetic: `+`, `-`, `*`, `quotient`, `remainder`
//! - Comparisons: `=`, `<`, `>`, `<=`, `>=`
//! - Logic: `not`
//! - Control: `if`, `begin`
//! - Binding: `let`, `let*`
//!
//! Anything outside this set returns `None` and the caller falls back to the
//! tree-walking interpreter transparently.
//!
//! # Debugging
//! Set `SCHEME_DUMP_IR=1` to print the generated LLVM IR to stderr.

use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::execution_engine::JitFunction;
use inkwell::values::{BasicValue, IntValue};
use inkwell::{IntPredicate, OptimizationLevel};
use std::collections::HashMap;

use crate::types::Val;

// ── Public API ─────────────────────────────────────────────────────────────

/// Attempt to compile and JIT-execute a Scheme value as an `i64` integer.
///
/// Returns `Some(Val::Int(n))` when the expression is JIT-eligible and
/// executed successfully.  Returns `None` when the expression uses types or
/// operations outside the JIT subset; the caller should invoke the
/// tree-walking interpreter instead.
pub fn try_jit_eval(val: &Val) -> Option<Val> {
    let ctx = Context::create();
    compile_and_run(&ctx, val).map(Val::Int)
}

/// Return the LLVM IR string for a Scheme expression (for diagnostics).
pub fn emit_llvm_ir(val: &Val) -> Option<String> {
    let ctx = Context::create();
    let module = ctx.create_module("scheme_jit");
    let builder = ctx.create_builder();

    let i64_t = ctx.i64_type();
    let fn_ty = i64_t.fn_type(&[], false);
    let func = module.add_function("__entry", fn_ty, None);
    let entry_bb = ctx.append_basic_block(func, "entry");
    builder.position_at_end(entry_bb);

    let mut cg = CodeGen {
        ctx: &ctx,
        builder: &builder,
        vars: HashMap::new(),
    };

    let result = cg.compile(val).ok()?;
    builder.build_return(Some(&result)).ok()?;

    if func.verify(false) {
        Some(module.print_to_string().to_string())
    } else {
        None
    }
}

/// Environment variable name for enabling LLVM IR debug dumps.
const DUMP_IR_ENV: &str = "SCHEME_DUMP_IR";

fn compile_and_run(ctx: &Context, val: &Val) -> Option<i64> {
    let module = ctx.create_module("scheme_jit");
    let builder = ctx.create_builder();

    let i64_t = ctx.i64_type();
    let fn_ty = i64_t.fn_type(&[], false);
    let func = module.add_function("__entry", fn_ty, None);
    let entry_bb = ctx.append_basic_block(func, "entry");
    builder.position_at_end(entry_bb);

    let mut cg = CodeGen {
        ctx,
        builder: &builder,
        vars: HashMap::new(),
    };

    let result = cg.compile(val).ok()?;
    builder.build_return(Some(&result)).ok()?;

    if !func.verify(false) {
        return None;
    }

    // Optional IR dump for debugging
    if std::env::var(DUMP_IR_ENV).as_deref() == Ok("1") {
        module.print_to_stderr();
    }

    let ee = module
        .create_jit_execution_engine(OptimizationLevel::Default)
        .ok()?;

    type EntryFn = unsafe extern "C" fn() -> i64;
    unsafe {
        let f: JitFunction<EntryFn> = ee.get_function("__entry").ok()?;
        Some(f.call())
    }
}

// ── Code generator ─────────────────────────────────────────────────────────

struct CodeGen<'ctx> {
    ctx: &'ctx Context,
    builder: &'ctx Builder<'ctx>,
    /// Currently in-scope JIT integer variables.
    vars: HashMap<String, IntValue<'ctx>>,
}

/// Arithmetic fold operations.
enum ArithOp {
    Add,
    Sub,
    Mul,
}

impl<'ctx> CodeGen<'ctx> {
    fn i64_t(&self) -> inkwell::types::IntType<'ctx> {
        self.ctx.i64_type()
    }

    fn const_i64(&self, n: i64) -> IntValue<'ctx> {
        self.i64_t().const_int(n as u64, true)
    }

    /// Compile one Scheme value to an LLVM `i64` value.
    fn compile(&mut self, val: &Val) -> Result<IntValue<'ctx>, ()> {
        match val {
            Val::Int(n) => Ok(self.const_i64(*n)),
            Val::Bool(true) => Ok(self.const_i64(1)),
            Val::Bool(false) => Ok(self.const_i64(0)),
            Val::Symbol(s) => self.vars.get(s.as_ref()).copied().ok_or(()),
            Val::Pair(_) => {
                let items = val.to_vec().map_err(|_| ())?;
                self.compile_call(&items)
            }
            _ => Err(()),
        }
    }

    /// Compile a Scheme procedure call expression.
    fn compile_call(&mut self, items: &[Val]) -> Result<IntValue<'ctx>, ()> {
        if items.is_empty() {
            return Err(());
        }
        let op = match &items[0] {
            Val::Symbol(s) => s.as_ref().to_string(),
            _ => return Err(()),
        };
        let args = &items[1..];

        match op.as_str() {
            "+" => self.fold(args, ArithOp::Add),
            "-" => {
                if args.len() == 1 {
                    let v = self.compile(&args[0])?;
                    self.builder.build_int_neg(v, "neg").map_err(|_| ())
                } else {
                    self.fold(args, ArithOp::Sub)
                }
            }
            "*" => self.fold(args, ArithOp::Mul),
            "quotient" => {
                let (a, b) = self.two_args(args)?;
                // If divisor is a compile-time zero constant, fall back to interpreter
                // to produce a proper Scheme error rather than undefined behavior.
                if b.is_null() || b == self.i64_t().const_zero() {
                    return Err(());
                }
                self.builder
                    .build_int_signed_div(a, b, "quot")
                    .map_err(|_| ())
            }
            "remainder" => {
                let (a, b) = self.two_args(args)?;
                // Same zero-divisor guard as quotient.
                if b.is_null() || b == self.i64_t().const_zero() {
                    return Err(());
                }
                self.builder
                    .build_int_signed_rem(a, b, "rem")
                    .map_err(|_| ())
            }
            "not" => {
                if args.len() != 1 {
                    return Err(());
                }
                let v = self.compile(&args[0])?;
                let cmp = self
                    .builder
                    .build_int_compare(IntPredicate::EQ, v, self.i64_t().const_zero(), "not_c")
                    .map_err(|_| ())?;
                self.builder
                    .build_int_z_extend(cmp, self.i64_t(), "not_e")
                    .map_err(|_| ())
            }
            "=" | "<" | ">" | "<=" | ">=" => {
                let pred = match op.as_str() {
                    "=" => IntPredicate::EQ,
                    "<" => IntPredicate::SLT,
                    ">" => IntPredicate::SGT,
                    "<=" => IntPredicate::SLE,
                    _ => IntPredicate::SGE,
                };
                let (a, b) = self.two_args(args)?;
                let cmp = self
                    .builder
                    .build_int_compare(pred, a, b, "cmp")
                    .map_err(|_| ())?;
                self.builder
                    .build_int_z_extend(cmp, self.i64_t(), "cmp_e")
                    .map_err(|_| ())
            }
            "if" => self.compile_if(items),
            "begin" => self.compile_begin(args),
            "let" => self.compile_let(args, false),
            "let*" => self.compile_let(args, true),
            _ => Err(()),
        }
    }

    /// Fold a list of arguments with an arithmetic operation.
    fn fold(&mut self, args: &[Val], op: ArithOp) -> Result<IntValue<'ctx>, ()> {
        if args.is_empty() {
            return Ok(self.const_i64(match op {
                ArithOp::Mul => 1,
                _ => 0,
            }));
        }
        let mut acc = self.compile(&args[0])?;
        for arg in &args[1..] {
            let v = self.compile(arg)?;
            acc = match op {
                ArithOp::Add => self.builder.build_int_add(acc, v, "add").map_err(|_| ())?,
                ArithOp::Sub => self.builder.build_int_sub(acc, v, "sub").map_err(|_| ())?,
                ArithOp::Mul => self.builder.build_int_mul(acc, v, "mul").map_err(|_| ())?,
            };
        }
        Ok(acc)
    }

    /// Compile exactly two arguments.
    fn two_args(&mut self, args: &[Val]) -> Result<(IntValue<'ctx>, IntValue<'ctx>), ()> {
        if args.len() != 2 {
            return Err(());
        }
        Ok((self.compile(&args[0])?, self.compile(&args[1])?))
    }

    /// Compile `(if cond then else?)` using LLVM basic blocks and a phi node.
    fn compile_if(&mut self, items: &[Val]) -> Result<IntValue<'ctx>, ()> {
        // items: ["if", cond, then, else?]
        if items.len() < 3 {
            return Err(());
        }
        let cond = self.compile(&items[1])?;
        let cond_bool = self
            .builder
            .build_int_compare(IntPredicate::NE, cond, self.i64_t().const_zero(), "if_c")
            .map_err(|_| ())?;

        let func = self
            .builder
            .get_insert_block()
            .ok_or(())?
            .get_parent()
            .ok_or(())?;
        let then_bb = self.ctx.append_basic_block(func, "then");
        let else_bb = self.ctx.append_basic_block(func, "else");
        let merge_bb = self.ctx.append_basic_block(func, "merge");

        self.builder
            .build_conditional_branch(cond_bool, then_bb, else_bb)
            .map_err(|_| ())?;

        // then branch
        self.builder.position_at_end(then_bb);
        let then_val = self.compile(&items[2])?;
        self.builder
            .build_unconditional_branch(merge_bb)
            .map_err(|_| ())?;
        let then_end = self.builder.get_insert_block().ok_or(())?;

        // else branch
        self.builder.position_at_end(else_bb);
        let else_val = if items.len() >= 4 {
            self.compile(&items[3])?
        } else {
            self.const_i64(0)
        };
        self.builder
            .build_unconditional_branch(merge_bb)
            .map_err(|_| ())?;
        let else_end = self.builder.get_insert_block().ok_or(())?;

        // merge with phi
        self.builder.position_at_end(merge_bb);
        let phi = self
            .builder
            .build_phi(self.i64_t(), "if_r")
            .map_err(|_| ())?;
        phi.add_incoming(&[
            (&then_val as &dyn BasicValue<'ctx>, then_end),
            (&else_val as &dyn BasicValue<'ctx>, else_end),
        ]);
        Ok(phi.as_basic_value().into_int_value())
    }

    fn compile_begin(&mut self, args: &[Val]) -> Result<IntValue<'ctx>, ()> {
        if args.is_empty() {
            return Err(());
        }
        let mut last = self.const_i64(0);
        for v in args {
            last = self.compile(v)?;
        }
        Ok(last)
    }

    /// Compile `(let bindings body...)` or `(let* bindings body...)`.
    ///
    /// `args` = everything after the `let`/`let*` symbol: `[bindings, body...]`
    fn compile_let(&mut self, args: &[Val], sequential: bool) -> Result<IntValue<'ctx>, ()> {
        if args.len() < 2 {
            return Err(());
        }
        let bindings = args[0].to_vec().map_err(|_| ())?;
        let mut saved: Vec<(String, Option<IntValue<'ctx>>)> = Vec::new();

        if sequential {
            // let*: evaluate and bind each variable in order
            for b in &bindings {
                let pair = b.to_vec().map_err(|_| ())?;
                if pair.len() != 2 {
                    return Err(());
                }
                let name = match &pair[0] {
                    Val::Symbol(s) => s.as_ref().to_string(),
                    _ => return Err(()),
                };
                let val = self.compile(&pair[1])?;
                let old = self.vars.insert(name.clone(), val);
                saved.push((name, old));
            }
        } else {
            // let: evaluate all inits first (in parallel), then bind
            let mut new_vals: Vec<(String, IntValue<'ctx>)> = Vec::new();
            for b in &bindings {
                let pair = b.to_vec().map_err(|_| ())?;
                if pair.len() != 2 {
                    return Err(());
                }
                let name = match &pair[0] {
                    Val::Symbol(s) => s.as_ref().to_string(),
                    _ => return Err(()),
                };
                let val = self.compile(&pair[1])?;
                new_vals.push((name, val));
            }
            for (name, val) in new_vals {
                let old = self.vars.insert(name.clone(), val);
                saved.push((name, old));
            }
        }

        let mut result = self.const_i64(0);
        for expr in &args[1..] {
            result = self.compile(expr)?;
        }

        // Restore scope
        for (name, old) in saved.into_iter().rev() {
            match old {
                Some(v) => {
                    self.vars.insert(name, v);
                }
                None => {
                    self.vars.remove(&name);
                }
            }
        }
        Ok(result)
    }
}

// ── Tests ───────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn jit_i64(src: &str) -> Option<i64> {
        let vals = crate::parser::parse(src).ok()?;
        let val = vals.into_iter().next()?;
        match try_jit_eval(&val)? {
            Val::Int(n) => Some(n),
            _ => None,
        }
    }

    #[test]
    fn jit_literals() {
        assert_eq!(jit_i64("42"), Some(42));
        assert_eq!(jit_i64("-7"), Some(-7));
    }

    #[test]
    fn jit_arithmetic() {
        assert_eq!(jit_i64("(+ 1 2 3)"), Some(6));
        assert_eq!(jit_i64("(* 2 3 4)"), Some(24));
        assert_eq!(jit_i64("(- 10 3)"), Some(7));
        assert_eq!(jit_i64("(- 5)"), Some(-5));
        assert_eq!(jit_i64("(quotient 10 3)"), Some(3));
        assert_eq!(jit_i64("(remainder 10 3)"), Some(1));
    }

    #[test]
    fn jit_comparison() {
        assert_eq!(jit_i64("(if (= 2 2) 1 0)"), Some(1));
        assert_eq!(jit_i64("(if (< 1 2) 1 0)"), Some(1));
        assert_eq!(jit_i64("(if (> 1 2) 1 0)"), Some(0));
        assert_eq!(jit_i64("(if (<= 3 3) 1 0)"), Some(1));
        assert_eq!(jit_i64("(if (>= 4 3) 1 0)"), Some(1));
    }

    #[test]
    fn jit_boolean() {
        assert_eq!(jit_i64("(if #t 1 0)"), Some(1));
        assert_eq!(jit_i64("(if #f 1 0)"), Some(0));
        assert_eq!(jit_i64("(not 0)"), Some(1));
        assert_eq!(jit_i64("(not 1)"), Some(0));
    }

    #[test]
    fn jit_let() {
        assert_eq!(jit_i64("(let ((x 5) (y 3)) (+ x y))"), Some(8));
        assert_eq!(jit_i64("(let ((x 10)) (* x x))"), Some(100));
    }

    #[test]
    fn jit_let_star() {
        assert_eq!(jit_i64("(let* ((x 5) (y (* x 2))) y)"), Some(10));
    }

    #[test]
    fn jit_begin() {
        assert_eq!(jit_i64("(begin 1 2 3)"), Some(3));
    }

    #[test]
    fn jit_nested() {
        assert_eq!(jit_i64("(* (+ 2 3) (- 10 4))"), Some(30));
    }

    #[test]
    fn jit_not_eligible_returns_none() {
        // Strings are not JIT-eligible
        let vals = crate::parser::parse("\"hello\"").unwrap();
        let val = vals.into_iter().next().unwrap();
        assert!(try_jit_eval(&val).is_none());
    }

    #[test]
    fn jit_emit_ir_produces_output() {
        let vals = crate::parser::parse("(+ 1 2)").unwrap();
        let val = vals.into_iter().next().unwrap();
        let ir = emit_llvm_ir(&val);
        assert!(ir.is_some());
        let ir_text = ir.unwrap();
        assert!(ir_text.contains("__entry"));
    }
}
