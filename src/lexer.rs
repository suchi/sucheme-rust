//! Lexer (tokenizer) for Scheme source code.

use crate::types::SchemeError;

/// Scheme tokens.
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    /// (
    LParen,
    /// )
    RParen,
    /// #(
    HashParen,
    /// '
    Quote,
    /// `
    Quasiquote,
    /// ,
    Unquote,
    /// ,@
    UnquoteSplicing,
    /// .
    Dot,
    /// #t or #true
    True,
    /// #f or #false
    False,
    /// Integer literal
    Integer(i64),
    /// Float literal
    Float(f64),
    /// String literal (already unescaped)
    Str(String),
    /// Symbol / identifier
    Symbol(String),
    /// Character literal
    Char(char),
}

/// Maximum allowed source size (1 MiB) to prevent DoS via huge inputs.
const MAX_SOURCE_BYTES: usize = 1 << 20; // 1 048 576

/// Tokenize a Scheme source string.
pub fn tokenize(source: &str) -> Result<Vec<Token>, SchemeError> {
    if source.len() > MAX_SOURCE_BYTES {
        return Err(SchemeError::Eval(format!(
            "source too large: {} bytes (limit: {} bytes)",
            source.len(),
            MAX_SOURCE_BYTES
        )));
    }
    let chars: Vec<char> = source.chars().collect();
    let mut pos = 0;
    let mut tokens = Vec::new();

    while pos < chars.len() {
        // Skip whitespace
        if chars[pos].is_whitespace() {
            pos += 1;
            continue;
        }

        // Line comment
        if chars[pos] == ';' {
            while pos < chars.len() && chars[pos] != '\n' {
                pos += 1;
            }
            continue;
        }

        // Block comment #| ... |#
        if pos + 1 < chars.len() && chars[pos] == '#' && chars[pos + 1] == '|' {
            pos += 2;
            let mut depth = 1usize;
            while pos + 1 < chars.len() {
                if chars[pos] == '#' && chars[pos + 1] == '|' {
                    depth += 1;
                    pos += 2;
                } else if chars[pos] == '|' && chars[pos + 1] == '#' {
                    depth -= 1;
                    pos += 2;
                    if depth == 0 {
                        break;
                    }
                } else {
                    pos += 1;
                }
            }
            if depth != 0 {
                return Err(SchemeError::Lex("Unterminated block comment".to_string()));
            }
            continue;
        }

        // Datum comment #;
        if pos + 1 < chars.len() && chars[pos] == '#' && chars[pos + 1] == ';' {
            tokens.push(Token::Symbol("#;".to_string())); // handled in parser
            pos += 2;
            continue;
        }

        match chars[pos] {
            '(' => {
                tokens.push(Token::LParen);
                pos += 1;
            }
            ')' => {
                tokens.push(Token::RParen);
                pos += 1;
            }
            '\'' => {
                tokens.push(Token::Quote);
                pos += 1;
            }
            '`' => {
                tokens.push(Token::Quasiquote);
                pos += 1;
            }
            ',' => {
                if pos + 1 < chars.len() && chars[pos + 1] == '@' {
                    tokens.push(Token::UnquoteSplicing);
                    pos += 2;
                } else {
                    tokens.push(Token::Unquote);
                    pos += 1;
                }
            }
            '"' => {
                pos += 1;
                let mut s = String::new();
                while pos < chars.len() && chars[pos] != '"' {
                    if chars[pos] == '\\' {
                        pos += 1;
                        if pos >= chars.len() {
                            return Err(SchemeError::Lex("Unterminated string escape".to_string()));
                        }
                        match chars[pos] {
                            'n' => s.push('\n'),
                            't' => s.push('\t'),
                            'r' => s.push('\r'),
                            '"' => s.push('"'),
                            '\\' => s.push('\\'),
                            '0' => s.push('\0'),
                            'a' => s.push('\x07'),
                            'b' => s.push('\x08'),
                            c => s.push(c),
                        }
                    } else {
                        s.push(chars[pos]);
                    }
                    pos += 1;
                }
                if pos >= chars.len() {
                    return Err(SchemeError::Lex("Unterminated string".to_string()));
                }
                pos += 1; // closing "
                tokens.push(Token::Str(s));
            }
            '#' => {
                if pos + 1 < chars.len() && chars[pos + 1] == '(' {
                    tokens.push(Token::HashParen);
                    pos += 2;
                } else if pos + 1 < chars.len() && chars[pos + 1] == '\\' {
                    // Character
                    pos += 2;
                    let ch = parse_char(&chars, &mut pos)?;
                    tokens.push(Token::Char(ch));
                } else {
                    // #t / #f / #true / #false
                    let start = pos;
                    pos += 1;
                    while pos < chars.len()
                        && !chars[pos].is_whitespace()
                        && chars[pos] != '('
                        && chars[pos] != ')'
                        && chars[pos] != '"'
                        && chars[pos] != ';'
                    {
                        pos += 1;
                    }
                    let tok: String = chars[start..pos].iter().collect();
                    match tok.as_str() {
                        "#t" | "#true" => tokens.push(Token::True),
                        "#f" | "#false" => tokens.push(Token::False),
                        _ => {
                            return Err(SchemeError::Lex(format!("Unknown # token: {}", tok)));
                        }
                    }
                }
            }
            _ => {
                // Number or symbol
                let start = pos;
                while pos < chars.len()
                    && !chars[pos].is_whitespace()
                    && chars[pos] != '('
                    && chars[pos] != ')'
                    && chars[pos] != '"'
                    && chars[pos] != ';'
                    && chars[pos] != '\''
                    && chars[pos] != '`'
                    && chars[pos] != ','
                {
                    pos += 1;
                }
                let tok: String = chars[start..pos].iter().collect();
                if tok == "." {
                    tokens.push(Token::Dot);
                } else if let Some(t) = try_parse_number(&tok) {
                    tokens.push(t);
                } else {
                    tokens.push(Token::Symbol(tok.to_lowercase()));
                }
            }
        }
    }

    Ok(tokens)
}

/// Try to parse a token as a number.
fn try_parse_number(s: &str) -> Option<Token> {
    // Exact integer (decimal)
    if let Ok(n) = s.parse::<i64>() {
        return Some(Token::Integer(n));
    }
    // Radix prefixes
    if s.starts_with("#b") || s.starts_with("#B") {
        if let Ok(n) = i64::from_str_radix(&s[2..], 2) {
            return Some(Token::Integer(n));
        }
    }
    if s.starts_with("#o") || s.starts_with("#O") {
        if let Ok(n) = i64::from_str_radix(&s[2..], 8) {
            return Some(Token::Integer(n));
        }
    }
    if s.starts_with("#d") || s.starts_with("#D") {
        if let Ok(n) = s[2..].parse::<i64>() {
            return Some(Token::Integer(n));
        }
    }
    if s.starts_with("#x") || s.starts_with("#X") {
        if let Ok(n) = i64::from_str_radix(&s[2..], 16) {
            return Some(Token::Integer(n));
        }
    }
    // Float
    if let Ok(f) = s.parse::<f64>() {
        return Some(Token::Float(f));
    }
    // Special float literals
    match s {
        "+inf.0" => return Some(Token::Float(f64::INFINITY)),
        "-inf.0" => return Some(Token::Float(f64::NEG_INFINITY)),
        "+nan.0" | "-nan.0" => return Some(Token::Float(f64::NAN)),
        _ => {}
    }
    None
}

/// Parse a character literal starting after `#\`.
fn parse_char(chars: &[char], pos: &mut usize) -> Result<char, SchemeError> {
    if *pos >= chars.len() {
        return Err(SchemeError::Lex("Expected character after #\\".to_string()));
    }
    let start = *pos;
    // Read until delimiter
    let mut end = *pos;
    while end < chars.len() && !chars[end].is_whitespace() && chars[end] != '(' && chars[end] != ')'
    {
        end += 1;
    }
    let name: String = chars[start..end].iter().collect();
    *pos = end;
    match name.to_lowercase().as_str() {
        "space" => Ok(' '),
        "newline" => Ok('\n'),
        "tab" => Ok('\t'),
        "return" => Ok('\r'),
        "null" | "nul" => Ok('\0'),
        "backspace" => Ok('\x08'),
        "page" => Ok('\x0C'),
        "escape" | "esc" => Ok('\x1B'),
        "delete" | "del" | "rubout" => Ok('\x7F'),
        "altmode" => Ok('\x1B'),
        _ => {
            if name.len() == 1 {
                Ok(name.chars().next().unwrap())
            } else {
                Err(SchemeError::Lex(format!(
                    "Unknown character name: {}",
                    name
                )))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize_basic() {
        let tokens = tokenize("(+ 1 2)").unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::LParen,
                Token::Symbol("+".to_string()),
                Token::Integer(1),
                Token::Integer(2),
                Token::RParen,
            ]
        );
    }

    #[test]
    fn test_tokenize_string() {
        let tokens = tokenize(r#""hello\nworld""#).unwrap();
        assert_eq!(tokens, vec![Token::Str("hello\nworld".to_string())]);
    }

    #[test]
    fn test_tokenize_bool() {
        let tokens = tokenize("#t #f #true #false").unwrap();
        assert_eq!(
            tokens,
            vec![Token::True, Token::False, Token::True, Token::False]
        );
    }

    #[test]
    fn test_tokenize_char() {
        let tokens = tokenize(r"#\space #\a").unwrap();
        assert_eq!(tokens, vec![Token::Char(' '), Token::Char('a')]);
    }

    #[test]
    fn test_tokenize_float() {
        let tokens = tokenize("3.14 +inf.0").unwrap();
        assert_eq!(
            tokens,
            vec![Token::Float(3.14), Token::Float(f64::INFINITY)]
        );
    }

    #[test]
    fn test_line_comment() {
        let tokens = tokenize("1 ; this is a comment\n2").unwrap();
        assert_eq!(tokens, vec![Token::Integer(1), Token::Integer(2)]);
    }
}
