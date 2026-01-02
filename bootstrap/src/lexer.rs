//! Lexer for Nova
//!
//! Converts source code into a stream of tokens.
//!
//! # Example
//!
//! ```
//! use nova::lexer::lex;
//!
//! let tokens = lex("let x = 42").unwrap();
//! ```

use crate::error::NovaError;
use crate::token::{Span, Token, TokenKind};

/// Lex source code into tokens
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
                    tokens.push(Token::new(
                        TokenKind::Eof,
                        Span::new(self.current, self.current),
                    ));
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

            '!' => self.match_char('=', TokenKind::NotEq, TokenKind::Bang),

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
                    TokenKind::LtLt
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
                    TokenKind::GtGt
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

            // Numbers
            '0'..='9' => self.lex_number(c)?,

            // Identifiers and keywords
            'a'..='z' | 'A'..='Z' | '_' => self.lex_identifier(c),

            // Unknown character
            _ => {
                return Err(NovaError::InvalidCharacter {
                    char: c,
                    span: Span::new(self.start, self.current),
                });
            }
        };

        Ok(Token::new(kind, Span::new(self.start, self.current)))
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
    fn match_char(&mut self, expected: char, if_match: TokenKind, otherwise: TokenKind) -> TokenKind {
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
                            // Block comment
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

    /// Lex a string literal
    fn lex_string(&mut self) -> Result<TokenKind, NovaError> {
        let mut value = String::new();

        loop {
            match self.advance() {
                Some('"') => break,
                Some('\\') => {
                    // Escape sequence
                    match self.advance() {
                        Some('n') => value.push('\n'),
                        Some('r') => value.push('\r'),
                        Some('t') => value.push('\t'),
                        Some('\\') => value.push('\\'),
                        Some('"') => value.push('"'),
                        Some('0') => value.push('\0'),
                        Some(c) => {
                            return Err(NovaError::InvalidEscape {
                                char: c,
                                span: Span::new(self.current - 1, self.current),
                            });
                        }
                        None => {
                            return Err(NovaError::UnterminatedString {
                                span: Span::new(self.start, self.current),
                            });
                        }
                    }
                }
                Some(c) => value.push(c),
                None => {
                    return Err(NovaError::UnterminatedString {
                        span: Span::new(self.start, self.current),
                    });
                }
            }
        }

        Ok(TokenKind::String(value))
    }

    /// Lex a number literal (integer or float)
    fn lex_number(&mut self, first: char) -> Result<TokenKind, NovaError> {
        let mut value = String::from(first);
        let mut is_float = false;

        // Consume digits
        while let Some(c) = self.peek() {
            if c.is_ascii_digit() {
                value.push(c);
                self.advance();
            } else if c == '.' {
                // Check if it's a decimal point or range operator
                let mut chars = self.chars.clone();
                chars.next(); // consume '.'
                if let Some((_, next)) = chars.peek() {
                    if next.is_ascii_digit() {
                        is_float = true;
                        value.push('.');
                        self.advance(); // consume '.'
                        while let Some(c) = self.peek() {
                            if c.is_ascii_digit() {
                                value.push(c);
                                self.advance();
                            } else {
                                break;
                            }
                        }
                    }
                }
                break;
            } else if c == '_' {
                // Allow underscores in numbers (1_000_000)
                self.advance();
            } else {
                break;
            }
        }

        // Check for exponent
        if let Some('e' | 'E') = self.peek() {
            is_float = true;
            value.push('e');
            self.advance();
            if let Some('+' | '-') = self.peek() {
                value.push(self.advance().unwrap());
            }
            while let Some(c) = self.peek() {
                if c.is_ascii_digit() {
                    value.push(c);
                    self.advance();
                } else {
                    break;
                }
            }
        }

        if is_float {
            let n: f64 = value.parse().map_err(|_| NovaError::InvalidNumber {
                span: Span::new(self.start, self.current),
            })?;
            Ok(TokenKind::Float(n))
        } else {
            let n: i64 = value.parse().map_err(|_| NovaError::InvalidNumber {
                span: Span::new(self.start, self.current),
            })?;
            Ok(TokenKind::Int(n))
        }
    }

    /// Lex an identifier or keyword
    fn lex_identifier(&mut self, first: char) -> TokenKind {
        let mut value = String::from(first);

        while let Some(c) = self.peek() {
            if c.is_ascii_alphanumeric() || c == '_' {
                value.push(c);
                self.advance();
            } else {
                break;
            }
        }

        // Check if it's a keyword
        TokenKind::keyword_from_str(&value).unwrap_or(TokenKind::Ident(value))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lex_simple() {
        let tokens = lex("let x = 42").unwrap();
        assert_eq!(tokens.len(), 5); // let, x, =, 42, EOF
        assert!(matches!(tokens[0].kind, TokenKind::Let));
        assert!(matches!(&tokens[1].kind, TokenKind::Ident(s) if s == "x"));
        assert!(matches!(tokens[2].kind, TokenKind::Eq));
        assert!(matches!(tokens[3].kind, TokenKind::Int(42)));
        assert!(matches!(tokens[4].kind, TokenKind::Eof));
    }

    #[test]
    fn test_lex_string() {
        let tokens = lex("\"hello\"").unwrap();
        assert!(matches!(&tokens[0].kind, TokenKind::String(s) if s == "hello"));
    }

    #[test]
    fn test_lex_operators() {
        let tokens = lex("+ - * / == != <= >=").unwrap();
        assert!(matches!(tokens[0].kind, TokenKind::Plus));
        assert!(matches!(tokens[1].kind, TokenKind::Minus));
        assert!(matches!(tokens[2].kind, TokenKind::Star));
        assert!(matches!(tokens[3].kind, TokenKind::Slash));
        assert!(matches!(tokens[4].kind, TokenKind::EqEq));
        assert!(matches!(tokens[5].kind, TokenKind::NotEq));
        assert!(matches!(tokens[6].kind, TokenKind::LtEq));
        assert!(matches!(tokens[7].kind, TokenKind::GtEq));
    }

    #[test]
    fn test_lex_comments() {
        let tokens = lex("x // this is a comment\ny").unwrap();
        assert_eq!(tokens.len(), 3); // x, y, EOF
        assert!(matches!(&tokens[0].kind, TokenKind::Ident(s) if s == "x"));
        assert!(matches!(&tokens[1].kind, TokenKind::Ident(s) if s == "y"));
    }
}
