# cprustscheme

R5RS-compliant Scheme interpreter written in Rust, built with GitHub Copilot.

[![CI](https://github.com/suchi/cprustscheme/actions/workflows/ci.yml/badge.svg)](https://github.com/suchi/cprustscheme/actions/workflows/ci.yml)

## Features

- **R5RS-compliant** – implements the full R5RS specification including the appendix macros
- **Tail Call Optimization** – proper tail calls via trampoline loop (no stack overflow)
- **Interactive REPL** – with Emacs key bindings and command history (rustyline)
- **Web UI** – browser-based REPL via built-in HTTP server
- **Standard Library** – R5RS macros and utilities implemented in Scheme itself

## Quick Start

```bash
# Build
cargo build --release

# Interactive REPL
./target/release/cprustscheme

# Evaluate an expression
./target/release/cprustscheme -e "(+ 1 2 3)"

# Run a Scheme file
./target/release/cprustscheme myfile.scm

# Start web server (http://localhost:8080)
./target/release/cprustscheme --web
```

## Documentation

- [Statement of Work (SOW)](docs/SOW.md)
- [Onboarding Guide](docs/ONBOARDING.md)
- [ADR-001: Language Choice](docs/adr/ADR-001-language-choice.md)
- [ADR-002: Tail Call Optimization](docs/adr/ADR-002-tco-trampoline.md)
- [ADR-003: Standard Library in Scheme](docs/adr/ADR-003-stdlib-in-scheme.md)
- [ADR-004: Web UI with actix-web](docs/adr/ADR-004-web-ui-actix.md)

## Project Structure

```
src/
  main.rs      - Entry point and CLI
  types.rs     - Scheme value types
  lexer.rs     - Tokenizer
  parser.rs    - S-expression parser
  eval.rs      - Evaluator with TCO
  builtins.rs  - Built-in procedures
  repl.rs      - Interactive REPL
  web.rs       - Web server
scheme/
  stdlib.scm   - Standard library in Scheme
web/static/
  index.html   - Web REPL UI
```

## License

MIT
 
