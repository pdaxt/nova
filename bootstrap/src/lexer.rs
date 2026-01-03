//! Lexer for Nova
//!
//! Converts source code into a stream of tokens.
//!
//! # Example
//!
//! ```
//! use nova::lexer::lex;
//!
//! let tokens = lex("let x = 42;").unwrap();
//! // Returns: [Let, Ident, Eq, IntLit, Semi, Eof]
//! ```
//!
//! # Design (ADR-005)
//!
//! The lexer produces tokens with spans but NO literal values.
//! Values are extracted from source text when needed:
//!
//! ```ignore
//! let text = &source[span.start()..span.end()];
//! let value: i64 = text.parse()?;
//! ```
//!
//! This keeps tokens small (12 bytes) and cache-friendly.
//!
//! # Contributing
//!
//! **Good first issues in this module:**
//!
//! - [ ] Add raw string literals (`r"..."`, `r#"..."#`)
//! - [ ] Improve error messages for unterminated strings
//! - [ ] Add byte literals (`b'x'`, `b"bytes"`)
//! - [ ] Handle Unicode escapes (`\u{1F600}`)
//!
//! **How to add a new token type:**
//!
//! 1. Add the variant to `TokenKind` in `token.rs`
//! 2. Add lexing logic in `lex_token()` below
//! 3. Add tests in the `tests` module at the bottom
//! 4. Run `cargo test lexer` to verify

use crate::error::NovaError;
use crate::token::{Span, Token, TokenKind};

/// Lex source code into tokens.
///
/// Returns a vector of tokens ending with EOF.
/// Literal values are NOT stored in tokens - use the span to extract from source.
pub fn lex(source: &str) -> Result<Vec<Token>, NovaError> {
    let mut lexer = Lexer::new(source);
    lexer.lex_all()
}

/// The lexer state
struct Lexer<'a> {
    source: &'a str,
    chars: std::iter::Peekable<std::str::CharIndices<'a>>,
    start: usize,
    current: usize,
}

impl<'a> Lexer<'a> {
    fn new(source: &'a str) -> Self {
        Self {
            source,
            chars: source.char_indices().peekable(),
            start: 0,
            current: 0,
        }
    }

    /// Lex all tokens from the source
    fn lex_all(&mut self) -> Result<Vec<Token>, NovaError> {
        let mut tokens = Vec::new();

        loop {
            self.skip_whitespace_and_comments();
            self.start = self.current;

            match self.advance() {
                None => {
                    tokens.push(Token::eof(self.current as u32));
                    break;
                }
                Some(c) => {
                    let token = self.lex_token(c)?;
                    tokens.push(token);
                }
            }
        }

        Ok(tokens)
    }

    /// Lex a single token starting with the given character
    fn lex_token(&mut self, c: char) -> Result<Token, NovaError> {
        let kind = match c {
            // Single-character tokens
            '(' => TokenKind::LParen,
            ')' => TokenKind::RParen,
            '[' => TokenKind::LBracket,
            ']' => TokenKind::RBracket,
            '{' => TokenKind::LBrace,
            '}' => TokenKind::RBrace,
            ',' => TokenKind::Comma,
            ';' => TokenKind::Semi,
            '~' => TokenKind::Tilde,
            '@' => TokenKind::At,
            '?' => TokenKind::Question,
            '#' => TokenKind::Hash,
            '$' => TokenKind::Dollar,

            // Potentially multi-character tokens
            '+' => self.match_char('=', TokenKind::PlusEq, TokenKind::Plus),
            '-' => {
                if self.check('>') {
                    self.advance();
                    TokenKind::Arrow
                } else if self.check('=') {
                    self.advance();
                    TokenKind::MinusEq
                } else {
                    TokenKind::Minus
                }
            }
            '*' => self.match_char('=', TokenKind::StarEq, TokenKind::Star),
            '/' => self.match_char('=', TokenKind::SlashEq, TokenKind::Slash),
            '%' => self.match_char('=', TokenKind::PercentEq, TokenKind::Percent),
            '^' => self.match_char('=', TokenKind::CaretEq, TokenKind::Caret),

            '&' => {
                if self.check('&') {
                    self.advance();
                    TokenKind::AmpAmp
                } else if self.check('=') {
                    self.advance();
                    TokenKind::AmpEq
                } else {
                    TokenKind::Amp
                }
            }

            '|' => {
                if self.check('|') {
                    self.advance();
                    TokenKind::PipePipe
                } else if self.check('=') {
                    self.advance();
                    TokenKind::PipeEq
                } else {
                    TokenKind::Pipe
                }
            }

            '!' => self.match_char('=', TokenKind::BangEq, TokenKind::Bang),

            '=' => {
                if self.check('=') {
                    self.advance();
                    TokenKind::EqEq
                } else if self.check('>') {
                    self.advance();
                    TokenKind::FatArrow
                } else {
                    TokenKind::Eq
                }
            }

            '<' => {
                if self.check('=') {
                    self.advance();
                    TokenKind::LtEq
                } else if self.check('<') {
                    self.advance();
                    if self.check('=') {
                        self.advance();
                        TokenKind::LtLtEq
                    } else {
                        TokenKind::LtLt
                    }
                } else {
                    TokenKind::Lt
                }
            }

            '>' => {
                if self.check('=') {
                    self.advance();
                    TokenKind::GtEq
                } else if self.check('>') {
                    self.advance();
                    if self.check('=') {
                        self.advance();
                        TokenKind::GtGtEq
                    } else {
                        TokenKind::GtGt
                    }
                } else {
                    TokenKind::Gt
                }
            }

            ':' => self.match_char(':', TokenKind::ColonColon, TokenKind::Colon),

            '.' => {
                if self.check('.') {
                    self.advance();
                    if self.check('=') {
                        self.advance();
                        TokenKind::DotDotEq
                    } else {
                        TokenKind::DotDot
                    }
                } else {
                    TokenKind::Dot
                }
            }

            // String literals
            '"' => self.lex_string()?,

            // Character literals
            '\'' => self.lex_char()?,

            // Numbers
            '0'..='9' => self.lex_number()?,

            // Identifiers and keywords
            'a'..='z' | 'A'..='Z' | '_' => self.lex_identifier(),

            // Unknown character
            _ => {
                return Err(NovaError::InvalidCharacter {
                    char: c,
                    span: Span::new(self.start as u32, self.current as u32),
                });
            }
        };

        Ok(Token::new(
            kind,
            Span::new(self.start as u32, self.current as u32),
        ))
    }

    /// Advance and return the next character
    fn advance(&mut self) -> Option<char> {
        match self.chars.next() {
            Some((i, c)) => {
                self.current = i + c.len_utf8();
                Some(c)
            }
            None => None,
        }
    }

    /// Peek at the next character without consuming
    fn peek(&mut self) -> Option<char> {
        self.chars.peek().map(|(_, c)| *c)
    }

    /// Check if the next character matches
    fn check(&mut self, expected: char) -> bool {
        self.peek() == Some(expected)
    }

    /// Match a character, returning one of two token kinds
    fn match_char(
        &mut self,
        expected: char,
        if_match: TokenKind,
        otherwise: TokenKind,
    ) -> TokenKind {
        if self.check(expected) {
            self.advance();
            if_match
        } else {
            otherwise
        }
    }

    /// Skip whitespace and comments
    fn skip_whitespace_and_comments(&mut self) {
        loop {
            match self.peek() {
                Some(' ' | '\t' | '\n' | '\r') => {
                    self.advance();
                }
                Some('/') => {
                    // Check for comment
                    let mut chars = self.chars.clone();
                    chars.next(); // consume '/'
                    match chars.peek() {
                        Some((_, '/')) => {
                            // Line comment
                            self.advance(); // '/'
                            self.advance(); // '/'
                            while let Some(c) = self.peek() {
                                if c == '\n' {
                                    break;
                                }
                                self.advance();
                            }
                        }
                        Some((_, '*')) => {
                            // Block comment (supports nesting)
                            self.advance(); // '/'
                            self.advance(); // '*'
                            let mut depth = 1;
                            while depth > 0 {
                                match self.advance() {
                                    Some('*') if self.check('/') => {
                                        self.advance();
                                        depth -= 1;
                                    }
                                    Some('/') if self.check('*') => {
                                        self.advance();
                                        depth += 1;
                                    }
                                    Some(_) => {}
                                    None => break, // Unterminated, will error later
                                }
                            }
                        }
                        _ => break,
                    }
                }
                _ => break,
            }
        }
    }

    /// Lex a string literal (just identifies it, doesn't parse escape sequences)
    fn lex_string(&mut self) -> Result<TokenKind, NovaError> {
        loop {
            match self.advance() {
                Some('"') => break,
                Some('\\') => {
                    // Skip the escaped character
                    if self.advance().is_none() {
                        return Err(NovaError::UnterminatedString {
                            span: Span::new(self.start as u32, self.current as u32),
                        });
                    }
                }
                Some(_) => {}
                None => {
                    return Err(NovaError::UnterminatedString {
                        span: Span::new(self.start as u32, self.current as u32),
                    });
                }
            }
        }

        Ok(TokenKind::StringLit)
    }

    /// Lex a character literal
    fn lex_char(&mut self) -> Result<TokenKind, NovaError> {
        match self.advance() {
            Some('\\') => {
                // Escape sequence
                if self.advance().is_none() {
                    return Err(NovaError::UnterminatedString {
                        span: Span::new(self.start as u32, self.current as u32),
                    });
                }
            }
            Some('\'') => {
                // Empty char literal
                return Err(NovaError::InvalidCharacter {
                    char: '\'',
                    span: Span::new(self.start as u32, self.current as u32),
                });
            }
            Some(_) => {}
            None => {
                return Err(NovaError::UnterminatedString {
                    span: Span::new(self.start as u32, self.current as u32),
                });
            }
        }

        // Expect closing quote
        if !self.check('\'') {
            return Err(NovaError::UnterminatedString {
                span: Span::new(self.start as u32, self.current as u32),
            });
        }
        self.advance();

        Ok(TokenKind::CharLit)
    }

    /// Lex a number literal (integer or float)
    fn lex_number(&mut self) -> Result<TokenKind, NovaError> {
        let mut is_float = false;

        // Check for hex, binary, octal
        if self.source.as_bytes().get(self.start) == Some(&b'0') {
            match self.peek() {
                Some('x' | 'X') => {
                    self.advance();
                    while let Some(c) = self.peek() {
                        if c.is_ascii_hexdigit() || c == '_' {
                            self.advance();
                        } else {
                            break;
                        }
                    }
                    return Ok(TokenKind::IntLit);
                }
                Some('b' | 'B') => {
                    self.advance();
                    while let Some(c) = self.peek() {
                        if c == '0' || c == '1' || c == '_' {
                            self.advance();
                        } else {
                            break;
                        }
                    }
                    return Ok(TokenKind::IntLit);
                }
                Some('o' | 'O') => {
                    self.advance();
                    while let Some(c) = self.peek() {
                        if ('0'..='7').contains(&c) || c == '_' {
                            self.advance();
                        } else {
                            break;
                        }
                    }
                    return Ok(TokenKind::IntLit);
                }
                _ => {}
            }
        }

        // Consume digits
        while let Some(c) = self.peek() {
            if c.is_ascii_digit() || c == '_' {
                self.advance();
            } else {
                break;
            }
        }

        // Check for decimal point
        if self.check('.') {
            // Look ahead to distinguish from range operator
            let mut chars = self.chars.clone();
            chars.next(); // consume '.'
            if let Some((_, c)) = chars.peek() {
                if c.is_ascii_digit() {
                    is_float = true;
                    self.advance(); // consume '.'
                    while let Some(c) = self.peek() {
                        if c.is_ascii_digit() || c == '_' {
                            self.advance();
                        } else {
                            break;
                        }
                    }
                }
            }
        }

        // Check for exponent
        if let Some('e' | 'E') = self.peek() {
            is_float = true;
            self.advance();
            if let Some('+' | '-') = self.peek() {
                self.advance();
            }
            while let Some(c) = self.peek() {
                if c.is_ascii_digit() || c == '_' {
                    self.advance();
                } else {
                    break;
                }
            }
        }

        Ok(if is_float {
            TokenKind::FloatLit
        } else {
            TokenKind::IntLit
        })
    }

    /// Lex an identifier or keyword
    fn lex_identifier(&mut self) -> TokenKind {
        while let Some(c) = self.peek() {
            if c.is_ascii_alphanumeric() || c == '_' {
                self.advance();
            } else {
                break;
            }
        }

        // Check if it's a keyword
        let text = &self.source[self.start..self.current];
        TokenKind::from_keyword(text).unwrap_or(TokenKind::Ident)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lex_simple() {
        let tokens = lex("let x = 42").unwrap();
        assert_eq!(tokens.len(), 5); // let, x, =, 42, EOF
        assert_eq!(tokens[0].kind(), TokenKind::Let);
        assert_eq!(tokens[1].kind(), TokenKind::Ident);
        assert_eq!(tokens[2].kind(), TokenKind::Eq);
        assert_eq!(tokens[3].kind(), TokenKind::IntLit);
        assert_eq!(tokens[4].kind(), TokenKind::Eof);
    }

    #[test]
    fn test_lex_string() {
        let tokens = lex("\"hello\"").unwrap();
        assert_eq!(tokens[0].kind(), TokenKind::StringLit);
    }

    #[test]
    fn test_lex_char() {
        let tokens = lex("'a' '\\n'").unwrap();
        assert_eq!(tokens[0].kind(), TokenKind::CharLit);
        assert_eq!(tokens[1].kind(), TokenKind::CharLit);
    }

    #[test]
    fn test_lex_operators() {
        let tokens = lex("+ - * / == != <= >=").unwrap();
        assert_eq!(tokens[0].kind(), TokenKind::Plus);
        assert_eq!(tokens[1].kind(), TokenKind::Minus);
        assert_eq!(tokens[2].kind(), TokenKind::Star);
        assert_eq!(tokens[3].kind(), TokenKind::Slash);
        assert_eq!(tokens[4].kind(), TokenKind::EqEq);
        assert_eq!(tokens[5].kind(), TokenKind::BangEq);
        assert_eq!(tokens[6].kind(), TokenKind::LtEq);
        assert_eq!(tokens[7].kind(), TokenKind::GtEq);
    }

    #[test]
    fn test_lex_shift_assign() {
        let tokens = lex("<<= >>=").unwrap();
        assert_eq!(tokens[0].kind(), TokenKind::LtLtEq);
        assert_eq!(tokens[1].kind(), TokenKind::GtGtEq);
    }

    #[test]
    fn test_lex_comments() {
        let tokens = lex("x // this is a comment\ny").unwrap();
        assert_eq!(tokens.len(), 3); // x, y, EOF
        assert_eq!(tokens[0].kind(), TokenKind::Ident);
        assert_eq!(tokens[1].kind(), TokenKind::Ident);
    }

    #[test]
    fn test_lex_nested_comments() {
        let tokens = lex("a /* outer /* inner */ outer */ b").unwrap();
        assert_eq!(tokens.len(), 3); // a, b, EOF
    }

    #[test]
    fn test_lex_hex_binary_octal() {
        let tokens = lex("0xFF 0b1010 0o777").unwrap();
        assert_eq!(tokens[0].kind(), TokenKind::IntLit);
        assert_eq!(tokens[1].kind(), TokenKind::IntLit);
        assert_eq!(tokens[2].kind(), TokenKind::IntLit);
    }

    #[test]
    fn test_lex_float() {
        let tokens = lex("3.14 1e10 2.5e-3").unwrap();
        assert_eq!(tokens[0].kind(), TokenKind::FloatLit);
        assert_eq!(tokens[1].kind(), TokenKind::FloatLit);
        assert_eq!(tokens[2].kind(), TokenKind::FloatLit);
    }

    #[test]
    fn test_lex_keywords() {
        let tokens = lex("fn let if else while for return true false").unwrap();
        assert_eq!(tokens[0].kind(), TokenKind::Fn);
        assert_eq!(tokens[1].kind(), TokenKind::Let);
        assert_eq!(tokens[2].kind(), TokenKind::If);
        assert_eq!(tokens[3].kind(), TokenKind::Else);
        assert_eq!(tokens[4].kind(), TokenKind::While);
        assert_eq!(tokens[5].kind(), TokenKind::For);
        assert_eq!(tokens[6].kind(), TokenKind::Return);
        assert_eq!(tokens[7].kind(), TokenKind::True);
        assert_eq!(tokens[8].kind(), TokenKind::False);
    }

    #[test]
    fn test_span_accuracy() {
        let source = "let x = 42";
        let tokens = lex(source).unwrap();

        // "let" at 0..3
        assert_eq!(tokens[0].span().start(), 0);
        assert_eq!(tokens[0].span().end(), 3);
        assert_eq!(&source[0..3], "let");

        // "x" at 4..5
        assert_eq!(tokens[1].span().start(), 4);
        assert_eq!(tokens[1].span().end(), 5);
        assert_eq!(&source[4..5], "x");

        // "42" at 8..10
        assert_eq!(tokens[3].span().start(), 8);
        assert_eq!(tokens[3].span().end(), 10);
        assert_eq!(&source[8..10], "42");
    }
}
