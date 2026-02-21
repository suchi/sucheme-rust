//! Parser: converts token stream to Scheme Val AST.

use crate::lexer::{tokenize, Token};
use crate::types::{SchemeError, Val};
use std::rc::Rc;

/// Parse a string into a list of Scheme expressions.
pub fn parse(source: &str) -> Result<Vec<Val>, SchemeError> {
    let tokens = tokenize(source)?;
    let mut pos = 0;
    let mut exprs = Vec::new();
    while pos < tokens.len() {
        let expr = parse_expr(&tokens, &mut pos)?;
        exprs.push(expr);
    }
    Ok(exprs)
}

/// Parse a single expression from the token stream.
fn parse_expr(tokens: &[Token], pos: &mut usize) -> Result<Val, SchemeError> {
    if *pos >= tokens.len() {
        return Err(SchemeError::Parse("Unexpected end of input".to_string()));
    }

    // Datum comment: skip next expression
    if let Token::Symbol(ref s) = tokens[*pos] {
        if s == "#;" {
            *pos += 1;
            parse_expr(tokens, pos)?; // consume and discard
            return parse_expr(tokens, pos);
        }
    }

    match tokens[*pos].clone() {
        Token::LParen => {
            *pos += 1;
            parse_list(tokens, pos)
        }
        Token::HashParen => {
            *pos += 1;
            let mut items = Vec::new();
            while *pos < tokens.len() {
                if tokens[*pos] == Token::RParen {
                    *pos += 1;
                    break;
                }
                items.push(parse_expr(tokens, pos)?);
            }
            Ok(Val::Vector(Rc::new(std::cell::RefCell::new(items))))
        }
        Token::RParen => Err(SchemeError::Parse("Unexpected ')'".to_string())),
        Token::Quote => {
            *pos += 1;
            let expr = parse_expr(tokens, pos)?;
            Ok(Val::list([symbol("quote"), expr].into_iter()))
        }
        Token::Quasiquote => {
            *pos += 1;
            let expr = parse_expr(tokens, pos)?;
            Ok(Val::list([symbol("quasiquote"), expr].into_iter()))
        }
        Token::Unquote => {
            *pos += 1;
            let expr = parse_expr(tokens, pos)?;
            Ok(Val::list([symbol("unquote"), expr].into_iter()))
        }
        Token::UnquoteSplicing => {
            *pos += 1;
            let expr = parse_expr(tokens, pos)?;
            Ok(Val::list([symbol("unquote-splicing"), expr].into_iter()))
        }
        Token::True => {
            *pos += 1;
            Ok(Val::Bool(true))
        }
        Token::False => {
            *pos += 1;
            Ok(Val::Bool(false))
        }
        Token::Integer(n) => {
            *pos += 1;
            Ok(Val::Int(n))
        }
        Token::Float(f) => {
            *pos += 1;
            Ok(Val::Float(f))
        }
        Token::Str(s) => {
            *pos += 1;
            Ok(Val::Str(Rc::new(std::cell::RefCell::new(s))))
        }
        Token::Symbol(s) => {
            *pos += 1;
            Ok(Val::Symbol(Rc::from(s.as_str())))
        }
        Token::Char(c) => {
            *pos += 1;
            Ok(Val::Char(c))
        }
        Token::Dot => Err(SchemeError::Parse(
            "Unexpected '.' outside list".to_string(),
        )),
    }
}

/// Parse a list starting after the opening `(`.
fn parse_list(tokens: &[Token], pos: &mut usize) -> Result<Val, SchemeError> {
    if *pos >= tokens.len() {
        return Err(SchemeError::Parse("Unterminated list".to_string()));
    }

    if tokens[*pos] == Token::RParen {
        *pos += 1;
        return Ok(Val::Nil);
    }

    let car = parse_expr(tokens, pos)?;

    // Check for dotted pair: (a . b)
    if *pos < tokens.len() && tokens[*pos] == Token::Dot {
        *pos += 1;
        let cdr = parse_expr(tokens, pos)?;
        if *pos >= tokens.len() || tokens[*pos] != Token::RParen {
            return Err(SchemeError::Parse(
                "Expected ')' after dotted pair".to_string(),
            ));
        }
        *pos += 1;
        return Ok(Val::cons(car, cdr));
    }

    let cdr = parse_list(tokens, pos)?;
    Ok(Val::cons(car, cdr))
}

fn symbol(s: &str) -> Val {
    Val::Symbol(Rc::from(s))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_atom() {
        let exprs = parse("42").unwrap();
        assert_eq!(exprs.len(), 1);
        assert_eq!(exprs[0], Val::Int(42));
    }

    #[test]
    fn test_parse_list() {
        let exprs = parse("(+ 1 2)").unwrap();
        assert_eq!(exprs.len(), 1);
        let list = exprs[0].to_vec().unwrap();
        assert_eq!(list.len(), 3);
        assert_eq!(list[0], Val::Symbol(Rc::from("+")));
        assert_eq!(list[1], Val::Int(1));
        assert_eq!(list[2], Val::Int(2));
    }

    #[test]
    fn test_parse_nested() {
        let exprs = parse("(define (square x) (* x x))").unwrap();
        assert!(exprs.len() == 1);
    }

    #[test]
    fn test_parse_quote() {
        let exprs = parse("'(1 2)").unwrap();
        assert_eq!(exprs.len(), 1);
        let list = exprs[0].to_vec().unwrap();
        assert_eq!(list[0], Val::Symbol(Rc::from("quote")));
    }

    #[test]
    fn test_parse_dotted() {
        let exprs = parse("(1 . 2)").unwrap();
        assert_eq!(exprs.len(), 1);
        assert_eq!(exprs[0].car().unwrap(), Val::Int(1));
        assert_eq!(exprs[0].cdr().unwrap(), Val::Int(2));
    }
}
