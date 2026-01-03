//! Token definitions for Nova
//!
//! This module defines all token types that the lexer produces.

use std::fmt;

// Re-export Span from the span module
pub use crate::span::Span;

/// A token with its kind and location
#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

impl Token {
    pub fn new(kind: TokenKind, span: Span) -> Self {
        Self { kind, span }
    }
}

/// All possible token types in Nova
#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    // Literals
    Int(i64),
    Float(f64),
    String(String),
    Bool(bool),

    // Identifiers and keywords
    Ident(String),

    // Keywords
    Fn,
    Let,
    Mut,
    If,
    Else,
    While,
    For,
    In,
    Return,
    Break,
    Continue,
    Struct,
    Enum,
    Impl,
    Trait,
    Type,
    Pub,
    Use,
    Mod,
    Where,
    Match,
    True,
    False,

    // Operators
    Plus,        // +
    Minus,       // -
    Star,        // *
    Slash,       // /
    Percent,     // %
    Caret,       // ^
    Amp,         // &
    Pipe,        // |
    Tilde,       // ~
    Bang,        // !
    Eq,          // =
    Lt,          // <
    Gt,          // >
    At,          // @
    Dot,         // .
    DotDot,      // ..
    DotDotEq,    // ..=
    Comma,       // ,
    Semi,        // ;
    Colon,       // :
    ColonColon,  // ::
    Arrow,       // ->
    FatArrow,    // =>
    Question,    // ?

    // Compound operators
    PlusEq,      // +=
    MinusEq,     // -=
    StarEq,      // *=
    SlashEq,     // /=
    PercentEq,   // %=
    AmpEq,       // &=
    PipeEq,      // |=
    CaretEq,     // ^=
    EqEq,        // ==
    NotEq,       // !=
    LtEq,        // <=
    GtEq,        // >=
    AmpAmp,      // &&
    PipePipe,    // ||
    LtLt,        // <<
    GtGt,        // >>

    // Delimiters
    LParen,      // (
    RParen,      // )
    LBracket,    // [
    RBracket,    // ]
    LBrace,      // {
    RBrace,      // }

    // Special
    Eof,
}

impl TokenKind {
    /// Returns true if this token is a keyword
    pub fn is_keyword(&self) -> bool {
        matches!(
            self,
            TokenKind::Fn
                | TokenKind::Let
                | TokenKind::Mut
                | TokenKind::If
                | TokenKind::Else
                | TokenKind::While
                | TokenKind::For
                | TokenKind::In
                | TokenKind::Return
                | TokenKind::Break
                | TokenKind::Continue
                | TokenKind::Struct
                | TokenKind::Enum
                | TokenKind::Impl
                | TokenKind::Trait
                | TokenKind::Type
                | TokenKind::Pub
                | TokenKind::Use
                | TokenKind::Mod
                | TokenKind::Where
                | TokenKind::Match
                | TokenKind::True
                | TokenKind::False
        )
    }

    /// Try to convert an identifier to a keyword
    pub fn keyword_from_str(s: &str) -> Option<TokenKind> {
        match s {
            "fn" => Some(TokenKind::Fn),
            "let" => Some(TokenKind::Let),
            "mut" => Some(TokenKind::Mut),
            "if" => Some(TokenKind::If),
            "else" => Some(TokenKind::Else),
            "while" => Some(TokenKind::While),
            "for" => Some(TokenKind::For),
            "in" => Some(TokenKind::In),
            "return" => Some(TokenKind::Return),
            "break" => Some(TokenKind::Break),
            "continue" => Some(TokenKind::Continue),
            "struct" => Some(TokenKind::Struct),
            "enum" => Some(TokenKind::Enum),
            "impl" => Some(TokenKind::Impl),
            "trait" => Some(TokenKind::Trait),
            "type" => Some(TokenKind::Type),
            "pub" => Some(TokenKind::Pub),
            "use" => Some(TokenKind::Use),
            "mod" => Some(TokenKind::Mod),
            "where" => Some(TokenKind::Where),
            "match" => Some(TokenKind::Match),
            "true" => Some(TokenKind::True),
            "false" => Some(TokenKind::False),
            _ => None,
        }
    }
}

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TokenKind::Int(n) => write!(f, "{}", n),
            TokenKind::Float(n) => write!(f, "{}", n),
            TokenKind::String(s) => write!(f, "\"{}\"", s),
            TokenKind::Bool(b) => write!(f, "{}", b),
            TokenKind::Ident(s) => write!(f, "{}", s),
            TokenKind::Fn => write!(f, "fn"),
            TokenKind::Let => write!(f, "let"),
            TokenKind::Mut => write!(f, "mut"),
            TokenKind::If => write!(f, "if"),
            TokenKind::Else => write!(f, "else"),
            TokenKind::While => write!(f, "while"),
            TokenKind::For => write!(f, "for"),
            TokenKind::In => write!(f, "in"),
            TokenKind::Return => write!(f, "return"),
            TokenKind::Break => write!(f, "break"),
            TokenKind::Continue => write!(f, "continue"),
            TokenKind::Struct => write!(f, "struct"),
            TokenKind::Enum => write!(f, "enum"),
            TokenKind::Impl => write!(f, "impl"),
            TokenKind::Trait => write!(f, "trait"),
            TokenKind::Type => write!(f, "type"),
            TokenKind::Pub => write!(f, "pub"),
            TokenKind::Use => write!(f, "use"),
            TokenKind::Mod => write!(f, "mod"),
            TokenKind::Where => write!(f, "where"),
            TokenKind::Match => write!(f, "match"),
            TokenKind::True => write!(f, "true"),
            TokenKind::False => write!(f, "false"),
            TokenKind::Plus => write!(f, "+"),
            TokenKind::Minus => write!(f, "-"),
            TokenKind::Star => write!(f, "*"),
            TokenKind::Slash => write!(f, "/"),
            TokenKind::Percent => write!(f, "%"),
            TokenKind::Caret => write!(f, "^"),
            TokenKind::Amp => write!(f, "&"),
            TokenKind::Pipe => write!(f, "|"),
            TokenKind::Tilde => write!(f, "~"),
            TokenKind::Bang => write!(f, "!"),
            TokenKind::Eq => write!(f, "="),
            TokenKind::Lt => write!(f, "<"),
            TokenKind::Gt => write!(f, ">"),
            TokenKind::At => write!(f, "@"),
            TokenKind::Dot => write!(f, "."),
            TokenKind::DotDot => write!(f, ".."),
            TokenKind::DotDotEq => write!(f, "..="),
            TokenKind::Comma => write!(f, ","),
            TokenKind::Semi => write!(f, ";"),
            TokenKind::Colon => write!(f, ":"),
            TokenKind::ColonColon => write!(f, "::"),
            TokenKind::Arrow => write!(f, "->"),
            TokenKind::FatArrow => write!(f, "=>"),
            TokenKind::Question => write!(f, "?"),
            TokenKind::PlusEq => write!(f, "+="),
            TokenKind::MinusEq => write!(f, "-="),
            TokenKind::StarEq => write!(f, "*="),
            TokenKind::SlashEq => write!(f, "/="),
            TokenKind::PercentEq => write!(f, "%="),
            TokenKind::AmpEq => write!(f, "&="),
            TokenKind::PipeEq => write!(f, "|="),
            TokenKind::CaretEq => write!(f, "^="),
            TokenKind::EqEq => write!(f, "=="),
            TokenKind::NotEq => write!(f, "!="),
            TokenKind::LtEq => write!(f, "<="),
            TokenKind::GtEq => write!(f, ">="),
            TokenKind::AmpAmp => write!(f, "&&"),
            TokenKind::PipePipe => write!(f, "||"),
            TokenKind::LtLt => write!(f, "<<"),
            TokenKind::GtGt => write!(f, ">>"),
            TokenKind::LParen => write!(f, "("),
            TokenKind::RParen => write!(f, ")"),
            TokenKind::LBracket => write!(f, "["),
            TokenKind::RBracket => write!(f, "]"),
            TokenKind::LBrace => write!(f, "{{"),
            TokenKind::RBrace => write!(f, "}}"),
            TokenKind::Eof => write!(f, "<eof>"),
        }
    }
}
