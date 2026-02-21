//! Scheme value types and error definitions.

use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;

use thiserror::Error;

/// Scheme interpreter errors.
#[derive(Debug, Error)]
pub enum SchemeError {
    #[error("Lexer error: {0}")]
    Lex(String),
    #[error("Parse error: {0}")]
    Parse(String),
    #[error("Eval error: {0}")]
    Eval(String),
    #[error("Arity error: expected {expected} args, got {got}")]
    Arity { expected: String, got: usize },
    #[error("Type error: expected {expected}, got {got}")]
    Type { expected: String, got: String },
    #[error("Unbound variable: {0}")]
    UnboundVariable(String),
    #[error("IO error: {0}")]
    Io(String),
}

pub type SchemeResult = Result<Val, SchemeError>;

/// The core Scheme value type.
#[derive(Clone)]
pub enum Val {
    /// Exact integer
    Int(i64),
    /// Inexact floating-point number
    Float(f64),
    /// Boolean
    Bool(bool),
    /// Character
    Char(char),
    /// String (mutable)
    Str(Rc<RefCell<String>>),
    /// Symbol (interned string)
    Symbol(Rc<str>),
    /// Pair (cons cell)
    Pair(Rc<RefCell<(Val, Val)>>),
    /// Empty list
    Nil,
    /// Vector (mutable)
    Vector(Rc<RefCell<Vec<Val>>>),
    /// Procedure (lambda or built-in)
    Proc(Proc),
    /// Port
    Port(Port),
    /// Unspecified (void)
    Void,
    /// EOF object
    Eof,
}

/// Scheme procedure variants.
#[derive(Clone)]
pub enum Proc {
    /// Native Rust built-in
    Builtin {
        name: &'static str,
        func: fn(&[Val]) -> SchemeResult,
    },
    /// Lambda closure
    Lambda {
        params: LambdaParams,
        body: Vec<Val>,
        env: Rc<RefCell<Env>>,
        name: Option<String>,
    },
    /// Tail-call continuation (internal use)
    Continuation,
}

/// Lambda parameter list representation.
#[derive(Clone, Debug)]
pub enum LambdaParams {
    /// Fixed arity: (lambda (a b c) ...)
    Fixed(Vec<String>),
    /// Variadic: (lambda (a b . rest) ...) or (lambda rest ...)
    Variadic { required: Vec<String>, rest: String },
}

/// Port type for I/O.
#[derive(Clone, Debug)]
#[allow(clippy::enum_variant_names)]
pub enum Port {
    StdIn,
    StdOut,
    StdErr,
}

/// Environment (scope chain).
pub struct Env {
    bindings: HashMap<String, Val>,
    parent: Option<Rc<RefCell<Env>>>,
}

impl Env {
    /// Create a new top-level environment.
    pub fn new() -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Env {
            bindings: HashMap::new(),
            parent: None,
        }))
    }

    /// Extend environment with a new scope.
    pub fn extend(parent: Rc<RefCell<Env>>) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Env {
            bindings: HashMap::new(),
            parent: Some(parent),
        }))
    }

    /// Look up a variable.
    pub fn get(&self, name: &str) -> Option<Val> {
        if let Some(val) = self.bindings.get(name) {
            Some(val.clone())
        } else if let Some(parent) = &self.parent {
            parent.borrow().get(name)
        } else {
            None
        }
    }

    /// Define a variable in this scope.
    pub fn define(&mut self, name: String, val: Val) {
        self.bindings.insert(name, val);
    }

    /// Set an existing variable (walks the chain).
    pub fn set(&mut self, name: &str, val: Val) -> bool {
        if self.bindings.contains_key(name) {
            self.bindings.insert(name.to_string(), val);
            true
        } else if let Some(parent) = &self.parent {
            parent.borrow_mut().set(name, val)
        } else {
            false
        }
    }
}

impl Default for Env {
    fn default() -> Self {
        Env {
            bindings: HashMap::new(),
            parent: None,
        }
    }
}

// ---- Val helpers ----

impl Val {
    /// Create a Scheme list from an iterator of values.
    pub fn list(items: impl DoubleEndedIterator<Item = Val>) -> Val {
        let mut result = Val::Nil;
        for item in items.rev() {
            result = Val::cons(item, result);
        }
        result
    }

    /// Create a cons pair.
    pub fn cons(car: Val, cdr: Val) -> Val {
        Val::Pair(Rc::new(RefCell::new((car, cdr))))
    }

    /// Access the car of a pair.
    pub fn car(&self) -> Result<Val, SchemeError> {
        match self {
            Val::Pair(p) => Ok(p.borrow().0.clone()),
            _ => Err(SchemeError::Type {
                expected: "pair".to_string(),
                got: self.type_name().to_string(),
            }),
        }
    }

    /// Access the cdr of a pair.
    pub fn cdr(&self) -> Result<Val, SchemeError> {
        match self {
            Val::Pair(p) => Ok(p.borrow().1.clone()),
            _ => Err(SchemeError::Type {
                expected: "pair".to_string(),
                got: self.type_name().to_string(),
            }),
        }
    }

    /// Return the type name as a string.
    pub fn type_name(&self) -> &str {
        match self {
            Val::Int(_) => "integer",
            Val::Float(_) => "float",
            Val::Bool(_) => "boolean",
            Val::Char(_) => "char",
            Val::Str(_) => "string",
            Val::Symbol(_) => "symbol",
            Val::Pair(_) => "pair",
            Val::Nil => "null",
            Val::Vector(_) => "vector",
            Val::Proc(_) => "procedure",
            Val::Port(_) => "port",
            Val::Void => "void",
            Val::Eof => "eof-object",
        }
    }

    /// Is this value truthy? Only #f is false in Scheme.
    pub fn is_truthy(&self) -> bool {
        !matches!(self, Val::Bool(false))
    }

    /// Convert to Scheme list as a Vec<Val>.
    pub fn to_vec(&self) -> Result<Vec<Val>, SchemeError> {
        let mut items = Vec::new();
        let mut cur = self.clone();
        loop {
            match cur {
                Val::Nil => break,
                Val::Pair(ref p) => {
                    let (car, cdr) = {
                        let b = p.borrow();
                        (b.0.clone(), b.1.clone())
                    };
                    items.push(car);
                    cur = cdr;
                }
                _ => {
                    return Err(SchemeError::Eval(format!("improper list: {}", self)));
                }
            }
        }
        Ok(items)
    }
}

// ---- Display ----

impl fmt::Display for Val {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", display_val(self, false))
    }
}

impl fmt::Debug for Val {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", display_val(self, true))
    }
}

/// `write` mode = true uses `write` semantics (strings quoted, chars written).
fn display_val(val: &Val, write_mode: bool) -> String {
    match val {
        Val::Int(n) => n.to_string(),
        Val::Float(f) => {
            if f.is_infinite() {
                if *f > 0.0 {
                    "+inf.0".to_string()
                } else {
                    "-inf.0".to_string()
                }
            } else if f.is_nan() {
                "+nan.0".to_string()
            } else {
                let s = format!("{}", f);
                // Ensure a decimal point for inexact numbers
                if s.contains('.') || s.contains('e') || s.contains('E') {
                    s
                } else {
                    format!("{}.0", s)
                }
            }
        }
        Val::Bool(true) => "#t".to_string(),
        Val::Bool(false) => "#f".to_string(),
        Val::Char(c) => {
            if write_mode {
                match c {
                    ' ' => "#\\space".to_string(),
                    '\n' => "#\\newline".to_string(),
                    '\t' => "#\\tab".to_string(),
                    '\r' => "#\\return".to_string(),
                    '\0' => "#\\null".to_string(),
                    '\x08' => "#\\backspace".to_string(),
                    '\x0C' => "#\\page".to_string(),
                    '\x1B' => "#\\escape".to_string(),
                    '\x7F' => "#\\delete".to_string(),
                    _ => format!("#\\{}", c),
                }
            } else {
                c.to_string()
            }
        }
        Val::Str(s) => {
            if write_mode {
                let inner = s.borrow();
                let escaped = inner
                    .chars()
                    .flat_map(|c| match c {
                        '"' => vec!['\\', '"'],
                        '\\' => vec!['\\', '\\'],
                        '\n' => vec!['\\', 'n'],
                        '\t' => vec!['\\', 't'],
                        '\r' => vec!['\\', 'r'],
                        c => vec![c],
                    })
                    .collect::<String>();
                format!("\"{}\"", escaped)
            } else {
                s.borrow().clone()
            }
        }
        Val::Symbol(s) => s.to_string(),
        Val::Nil => "()".to_string(),
        Val::Pair(_) => {
            let mut parts = Vec::new();
            let mut cur = val.clone();
            loop {
                match cur {
                    Val::Pair(ref p) => {
                        let (car, cdr) = {
                            let b = p.borrow();
                            (b.0.clone(), b.1.clone())
                        };
                        parts.push(display_val(&car, write_mode));
                        cur = cdr;
                    }
                    Val::Nil => break,
                    other => {
                        parts.push(".".to_string());
                        parts.push(display_val(&other, write_mode));
                        break;
                    }
                }
            }
            format!("({})", parts.join(" "))
        }
        Val::Vector(v) => {
            let items: Vec<String> = v
                .borrow()
                .iter()
                .map(|x| display_val(x, write_mode))
                .collect();
            format!("#({})", items.join(" "))
        }
        Val::Proc(p) => match p {
            Proc::Builtin { name, .. } => format!("#<procedure:{}>", name),
            Proc::Lambda { name, .. } => {
                if let Some(n) = name {
                    format!("#<procedure:{}>", n)
                } else {
                    "#<procedure>".to_string()
                }
            }
            Proc::Continuation => "#<continuation>".to_string(),
        },
        Val::Port(p) => match p {
            Port::StdIn => "#<port:stdin>".to_string(),
            Port::StdOut => "#<port:stdout>".to_string(),
            Port::StdErr => "#<port:stderr>".to_string(),
        },
        Val::Void => "".to_string(),
        Val::Eof => "#<eof>".to_string(),
    }
}

/// Write a value in `write` mode (quoted strings, etc.).
pub fn write_val(val: &Val) -> String {
    display_val(val, true)
}

impl PartialEq for Val {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Val::Int(a), Val::Int(b)) => a == b,
            (Val::Float(a), Val::Float(b)) => a == b,
            (Val::Int(a), Val::Float(b)) => (*a as f64) == *b,
            (Val::Float(a), Val::Int(b)) => *a == (*b as f64),
            (Val::Bool(a), Val::Bool(b)) => a == b,
            (Val::Char(a), Val::Char(b)) => a == b,
            (Val::Str(a), Val::Str(b)) => *a.borrow() == *b.borrow(),
            (Val::Symbol(a), Val::Symbol(b)) => a == b,
            (Val::Nil, Val::Nil) => true,
            _ => false,
        }
    }
}
