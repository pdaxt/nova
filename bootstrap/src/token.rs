//! Token definitions for Nova
//!
//! This module defines tokens - the atomic units produced by the lexer.
//!
//! # Design (ADR-004, ADR-005)
//!
//! Tokens are designed for minimal memory footprint:
//! - `TokenKind`: 1 byte (enum discriminant, no payload)
//! - `Token`: 12 bytes (kind + padding + span)
//!
//! Literal values (integers, strings, etc.) are NOT stored in tokens.
//! They are extracted from the source text using the span when needed.
//! This keeps every token the same size and cache-friendly.

use std::fmt;

// Re-export Span from the span module
pub use crate::span::Span;

// ============================================================================
// TokenKind - 1 byte enum
// ============================================================================

/// The kind of a token, without any payload.
///
/// # Size Guarantee
///
/// This enum is exactly 1 byte (`#[repr(u8)]`), enabling Token to be 12 bytes.
/// All literal values are extracted from source text, not stored here.
///
/// # Variant Count
///
/// Currently ~75 variants, well under the 256 limit of u8.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum TokenKind {
    // ========================================================================
    // Literals (values extracted from source via span)
    // ========================================================================
    /// Integer literal: `42`, `0xFF`, `0b1010`, `1_000`
    IntLit = 0,
    /// Float literal: `3.14`, `1e10`, `2.5e-3`
    FloatLit = 1,
    /// String literal: `"hello"`, `"line\nbreak"`
    StringLit = 2,
    /// Character literal: `'a'`, `'\n'`
    CharLit = 3,

    // ========================================================================
    // Identifier (text extracted from source via span)
    // ========================================================================
    /// Identifier: `foo`, `_bar`, `Vec3`
    Ident = 4,

    // ========================================================================
    // Keywords (alphabetically sorted for binary search potential)
    // ========================================================================
    /// `as`
    As = 10,
    /// `async`
    Async = 11,
    /// `await`
    Await = 12,
    /// `break`
    Break = 13,
    /// `const`
    Const = 14,
    /// `continue`
    Continue = 15,
    /// `else`
    Else = 16,
    /// `enum`
    Enum = 17,
    /// `false`
    False = 18,
    /// `fn`
    Fn = 19,
    /// `for`
    For = 20,
    /// `if`
    If = 21,
    /// `impl`
    Impl = 22,
    /// `in`
    In = 23,
    /// `let`
    Let = 24,
    /// `loop`
    Loop = 25,
    /// `match`
    Match = 26,
    /// `mod`
    Mod = 27,
    /// `mut`
    Mut = 28,
    /// `pub`
    Pub = 29,
    /// `return`
    Return = 30,
    /// `self`
    SelfLower = 31,
    /// `Self`
    SelfUpper = 32,
    /// `static`
    Static = 33,
    /// `struct`
    Struct = 34,
    /// `trait`
    Trait = 35,
    /// `true`
    True = 36,
    /// `type`
    Type = 37,
    /// `unsafe`
    Unsafe = 38,
    /// `use`
    Use = 39,
    /// `where`
    Where = 40,
    /// `while`
    While = 41,

    // ========================================================================
    // Single-character operators and punctuation
    // ========================================================================
    /// `+`
    Plus = 50,
    /// `-`
    Minus = 51,
    /// `*`
    Star = 52,
    /// `/`
    Slash = 53,
    /// `%`
    Percent = 54,
    /// `^`
    Caret = 55,
    /// `&`
    Amp = 56,
    /// `|`
    Pipe = 57,
    /// `~`
    Tilde = 58,
    /// `!`
    Bang = 59,
    /// `=`
    Eq = 60,
    /// `<`
    Lt = 61,
    /// `>`
    Gt = 62,
    /// `@`
    At = 63,
    /// `.`
    Dot = 64,
    /// `,`
    Comma = 65,
    /// `;`
    Semi = 66,
    /// `:`
    Colon = 67,
    /// `#`
    Hash = 68,
    /// `$`
    Dollar = 69,
    /// `?`
    Question = 70,
    /// `_` (underscore as a token, not part of identifier)
    #[allow(dead_code)]
    Underscore = 71,

    // ========================================================================
    // Multi-character operators
    // ========================================================================
    /// `..`
    DotDot = 80,
    /// `..=`
    DotDotEq = 81,
    /// `::`
    ColonColon = 82,
    /// `->`
    Arrow = 83,
    /// `=>`
    FatArrow = 84,
    /// `+=`
    PlusEq = 85,
    /// `-=`
    MinusEq = 86,
    /// `*=`
    StarEq = 87,
    /// `/=`
    SlashEq = 88,
    /// `%=`
    PercentEq = 89,
    /// `^=`
    CaretEq = 90,
    /// `&=`
    AmpEq = 91,
    /// `|=`
    PipeEq = 92,
    /// `==`
    EqEq = 93,
    /// `!=`
    BangEq = 94,
    /// `<=`
    LtEq = 95,
    /// `>=`
    GtEq = 96,
    /// `&&`
    AmpAmp = 97,
    /// `||`
    PipePipe = 98,
    /// `<<`
    LtLt = 99,
    /// `>>`
    GtGt = 100,
    /// `<<=`
    LtLtEq = 101,
    /// `>>=`
    GtGtEq = 102,

    // ========================================================================
    // Delimiters
    // ========================================================================
    /// `(`
    LParen = 110,
    /// `)`
    RParen = 111,
    /// `[`
    LBracket = 112,
    /// `]`
    RBracket = 113,
    /// `{`
    LBrace = 114,
    /// `}`
    RBrace = 115,

    // ========================================================================
    // Special tokens
    // ========================================================================
    /// End of file
    Eof = 250,
    /// Lexer error (invalid character, unterminated string, etc.)
    #[allow(dead_code)]
    Error = 251,
}

impl TokenKind {
    /// Returns true if this token is a keyword.
    #[inline]
    #[allow(dead_code)]
    pub const fn is_keyword(self) -> bool {
        matches!(self as u8, 10..=41)
    }

    /// Returns true if this token is a literal.
    #[inline]
    #[allow(dead_code)]
    pub const fn is_literal(self) -> bool {
        matches!(self as u8, 0..=3)
    }

    /// Returns true if this token is an operator.
    #[inline]
    #[allow(dead_code)]
    pub const fn is_operator(self) -> bool {
        matches!(self as u8, 50..=102)
    }

    /// Returns true if this token is a delimiter.
    #[inline]
    #[allow(dead_code)]
    pub const fn is_delimiter(self) -> bool {
        matches!(self as u8, 110..=115)
    }

    /// Returns the precedence of binary operators (higher = binds tighter).
    /// Returns None for non-binary operators.
    #[inline]
    #[allow(dead_code)]
    pub const fn precedence(self) -> Option<u8> {
        match self {
            // Assignment (right-to-left, lowest precedence)
            TokenKind::Eq
            | TokenKind::PlusEq
            | TokenKind::MinusEq
            | TokenKind::StarEq
            | TokenKind::SlashEq
            | TokenKind::PercentEq
            | TokenKind::AmpEq
            | TokenKind::PipeEq
            | TokenKind::CaretEq
            | TokenKind::LtLtEq
            | TokenKind::GtGtEq => Some(1),

            // Logical OR
            TokenKind::PipePipe => Some(2),

            // Logical AND
            TokenKind::AmpAmp => Some(3),

            // Comparison
            TokenKind::EqEq
            | TokenKind::BangEq
            | TokenKind::Lt
            | TokenKind::Gt
            | TokenKind::LtEq
            | TokenKind::GtEq => Some(4),

            // Bitwise OR
            TokenKind::Pipe => Some(5),

            // Bitwise XOR
            TokenKind::Caret => Some(6),

            // Bitwise AND
            TokenKind::Amp => Some(7),

            // Shift
            TokenKind::LtLt | TokenKind::GtGt => Some(8),

            // Addition/Subtraction
            TokenKind::Plus | TokenKind::Minus => Some(9),

            // Multiplication/Division/Remainder
            TokenKind::Star | TokenKind::Slash | TokenKind::Percent => Some(10),

            // Range
            TokenKind::DotDot | TokenKind::DotDotEq => Some(11),

            _ => None,
        }
    }

    /// Converts a string to its keyword TokenKind, if it is a keyword.
    pub fn from_keyword(s: &str) -> Option<TokenKind> {
        match s {
            "as" => Some(TokenKind::As),
            "async" => Some(TokenKind::Async),
            "await" => Some(TokenKind::Await),
            "break" => Some(TokenKind::Break),
            "const" => Some(TokenKind::Const),
            "continue" => Some(TokenKind::Continue),
            "else" => Some(TokenKind::Else),
            "enum" => Some(TokenKind::Enum),
            "false" => Some(TokenKind::False),
            "fn" => Some(TokenKind::Fn),
            "for" => Some(TokenKind::For),
            "if" => Some(TokenKind::If),
            "impl" => Some(TokenKind::Impl),
            "in" => Some(TokenKind::In),
            "let" => Some(TokenKind::Let),
            "loop" => Some(TokenKind::Loop),
            "match" => Some(TokenKind::Match),
            "mod" => Some(TokenKind::Mod),
            "mut" => Some(TokenKind::Mut),
            "pub" => Some(TokenKind::Pub),
            "return" => Some(TokenKind::Return),
            "self" => Some(TokenKind::SelfLower),
            "Self" => Some(TokenKind::SelfUpper),
            "static" => Some(TokenKind::Static),
            "struct" => Some(TokenKind::Struct),
            "trait" => Some(TokenKind::Trait),
            "true" => Some(TokenKind::True),
            "type" => Some(TokenKind::Type),
            "unsafe" => Some(TokenKind::Unsafe),
            "use" => Some(TokenKind::Use),
            "where" => Some(TokenKind::Where),
            "while" => Some(TokenKind::While),
            _ => None,
        }
    }

    /// Returns the static string representation of this token kind.
    /// For literals and identifiers, returns a placeholder.
    pub const fn as_str(self) -> &'static str {
        match self {
            // Literals (actual value comes from source)
            TokenKind::IntLit => "<int>",
            TokenKind::FloatLit => "<float>",
            TokenKind::StringLit => "<string>",
            TokenKind::CharLit => "<char>",
            TokenKind::Ident => "<ident>",

            // Keywords
            TokenKind::As => "as",
            TokenKind::Async => "async",
            TokenKind::Await => "await",
            TokenKind::Break => "break",
            TokenKind::Const => "const",
            TokenKind::Continue => "continue",
            TokenKind::Else => "else",
            TokenKind::Enum => "enum",
            TokenKind::False => "false",
            TokenKind::Fn => "fn",
            TokenKind::For => "for",
            TokenKind::If => "if",
            TokenKind::Impl => "impl",
            TokenKind::In => "in",
            TokenKind::Let => "let",
            TokenKind::Loop => "loop",
            TokenKind::Match => "match",
            TokenKind::Mod => "mod",
            TokenKind::Mut => "mut",
            TokenKind::Pub => "pub",
            TokenKind::Return => "return",
            TokenKind::SelfLower => "self",
            TokenKind::SelfUpper => "Self",
            TokenKind::Static => "static",
            TokenKind::Struct => "struct",
            TokenKind::Trait => "trait",
            TokenKind::True => "true",
            TokenKind::Type => "type",
            TokenKind::Unsafe => "unsafe",
            TokenKind::Use => "use",
            TokenKind::Where => "where",
            TokenKind::While => "while",

            // Operators
            TokenKind::Plus => "+",
            TokenKind::Minus => "-",
            TokenKind::Star => "*",
            TokenKind::Slash => "/",
            TokenKind::Percent => "%",
            TokenKind::Caret => "^",
            TokenKind::Amp => "&",
            TokenKind::Pipe => "|",
            TokenKind::Tilde => "~",
            TokenKind::Bang => "!",
            TokenKind::Eq => "=",
            TokenKind::Lt => "<",
            TokenKind::Gt => ">",
            TokenKind::At => "@",
            TokenKind::Dot => ".",
            TokenKind::Comma => ",",
            TokenKind::Semi => ";",
            TokenKind::Colon => ":",
            TokenKind::Hash => "#",
            TokenKind::Dollar => "$",
            TokenKind::Question => "?",
            TokenKind::Underscore => "_",

            TokenKind::DotDot => "..",
            TokenKind::DotDotEq => "..=",
            TokenKind::ColonColon => "::",
            TokenKind::Arrow => "->",
            TokenKind::FatArrow => "=>",
            TokenKind::PlusEq => "+=",
            TokenKind::MinusEq => "-=",
            TokenKind::StarEq => "*=",
            TokenKind::SlashEq => "/=",
            TokenKind::PercentEq => "%=",
            TokenKind::CaretEq => "^=",
            TokenKind::AmpEq => "&=",
            TokenKind::PipeEq => "|=",
            TokenKind::EqEq => "==",
            TokenKind::BangEq => "!=",
            TokenKind::LtEq => "<=",
            TokenKind::GtEq => ">=",
            TokenKind::AmpAmp => "&&",
            TokenKind::PipePipe => "||",
            TokenKind::LtLt => "<<",
            TokenKind::GtGt => ">>",
            TokenKind::LtLtEq => "<<=",
            TokenKind::GtGtEq => ">>=",

            // Delimiters
            TokenKind::LParen => "(",
            TokenKind::RParen => ")",
            TokenKind::LBracket => "[",
            TokenKind::RBracket => "]",
            TokenKind::LBrace => "{",
            TokenKind::RBrace => "}",

            // Special
            TokenKind::Eof => "<eof>",
            TokenKind::Error => "<error>",
        }
    }
}

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

// ============================================================================
// Token - 12 bytes
// ============================================================================

/// A token with its kind and source location.
///
/// # Size Guarantee
///
/// This struct is exactly 12 bytes:
/// - `kind: TokenKind` = 1 byte
/// - padding = 3 bytes (for Span alignment)
/// - `span: Span` = 8 bytes
///
/// This is enforced by compile-time assertions in tests.
///
/// # Extracting Values
///
/// Token does not store literal values. To get the actual text:
/// ```ignore
/// let text = source.slice(token.span);
/// let value: i64 = text.parse()?;
/// ```
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct Token {
    kind: TokenKind,
    // 3 bytes padding here (implicit)
    span: Span,
}

impl Token {
    /// Creates a new token with the given kind and span.
    #[inline]
    pub const fn new(kind: TokenKind, span: Span) -> Self {
        Self { kind, span }
    }

    /// Returns the kind of this token.
    #[inline]
    pub const fn kind(&self) -> TokenKind {
        self.kind
    }

    /// Returns the span of this token in source.
    #[inline]
    pub const fn span(&self) -> Span {
        self.span
    }

    /// Returns true if this token is the given kind.
    #[inline]
    #[allow(dead_code)]
    pub const fn is(&self, kind: TokenKind) -> bool {
        self.kind as u8 == kind as u8
    }

    /// Returns true if this token is a keyword.
    #[inline]
    #[allow(dead_code)]
    pub const fn is_keyword(&self) -> bool {
        self.kind.is_keyword()
    }

    /// Returns true if this token is a literal.
    #[inline]
    #[allow(dead_code)]
    pub const fn is_literal(&self) -> bool {
        self.kind.is_literal()
    }

    /// Returns true if this token is an operator.
    #[inline]
    #[allow(dead_code)]
    pub const fn is_operator(&self) -> bool {
        self.kind.is_operator()
    }

    /// Creates an EOF token at the given position.
    #[inline]
    pub const fn eof(pos: u32) -> Self {
        Self::new(TokenKind::Eof, Span::new(pos, pos))
    }

    /// Creates an error token at the given span.
    #[inline]
    #[allow(dead_code)]
    pub const fn error(span: Span) -> Self {
        Self::new(TokenKind::Error, span)
    }
}

impl fmt::Debug for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?} @ {}", self.kind, self.span)
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.kind)
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem::{align_of, size_of};

    // ========================================================================
    // Size guarantees (ADR-004)
    // ========================================================================

    #[test]
    fn token_kind_is_1_byte() {
        assert_eq!(
            size_of::<TokenKind>(),
            1,
            "TokenKind must be exactly 1 byte"
        );
    }

    #[test]
    fn token_is_12_bytes() {
        assert_eq!(size_of::<Token>(), 12, "Token must be exactly 12 bytes");
    }

    #[test]
    fn token_alignment() {
        assert_eq!(align_of::<Token>(), 4, "Token should align to 4 bytes");
    }

    // Compile-time size assertions
    const _: () = assert!(size_of::<TokenKind>() == 1);
    const _: () = assert!(size_of::<Token>() == 12);

    // ========================================================================
    // TokenKind classification
    // ========================================================================

    #[test]
    fn keywords_are_keywords() {
        assert!(TokenKind::Fn.is_keyword());
        assert!(TokenKind::Let.is_keyword());
        assert!(TokenKind::If.is_keyword());
        assert!(TokenKind::Else.is_keyword());
        assert!(TokenKind::While.is_keyword());
        assert!(TokenKind::Return.is_keyword());
        assert!(TokenKind::True.is_keyword());
        assert!(TokenKind::False.is_keyword());
    }

    #[test]
    fn non_keywords_are_not_keywords() {
        assert!(!TokenKind::IntLit.is_keyword());
        assert!(!TokenKind::Ident.is_keyword());
        assert!(!TokenKind::Plus.is_keyword());
        assert!(!TokenKind::LParen.is_keyword());
        assert!(!TokenKind::Eof.is_keyword());
    }

    #[test]
    fn literals_are_literals() {
        assert!(TokenKind::IntLit.is_literal());
        assert!(TokenKind::FloatLit.is_literal());
        assert!(TokenKind::StringLit.is_literal());
        assert!(TokenKind::CharLit.is_literal());
    }

    #[test]
    fn non_literals_are_not_literals() {
        assert!(!TokenKind::Ident.is_literal());
        assert!(!TokenKind::Fn.is_literal());
        assert!(!TokenKind::Plus.is_literal());
        assert!(!TokenKind::True.is_literal()); // true is a keyword, not a literal
    }

    #[test]
    fn operators_are_operators() {
        assert!(TokenKind::Plus.is_operator());
        assert!(TokenKind::Minus.is_operator());
        assert!(TokenKind::EqEq.is_operator());
        assert!(TokenKind::AmpAmp.is_operator());
    }

    #[test]
    fn delimiters_are_delimiters() {
        assert!(TokenKind::LParen.is_delimiter());
        assert!(TokenKind::RParen.is_delimiter());
        assert!(TokenKind::LBrace.is_delimiter());
        assert!(TokenKind::RBrace.is_delimiter());
    }

    // ========================================================================
    // Keyword lookup
    // ========================================================================

    #[test]
    fn keyword_from_str_works() {
        assert_eq!(TokenKind::from_keyword("fn"), Some(TokenKind::Fn));
        assert_eq!(TokenKind::from_keyword("let"), Some(TokenKind::Let));
        assert_eq!(TokenKind::from_keyword("if"), Some(TokenKind::If));
        assert_eq!(TokenKind::from_keyword("true"), Some(TokenKind::True));
        assert_eq!(TokenKind::from_keyword("false"), Some(TokenKind::False));
    }

    #[test]
    fn non_keyword_from_str_returns_none() {
        assert_eq!(TokenKind::from_keyword("foo"), None);
        assert_eq!(TokenKind::from_keyword("bar"), None);
        assert_eq!(TokenKind::from_keyword(""), None);
        assert_eq!(TokenKind::from_keyword("FN"), None); // case sensitive
    }

    // ========================================================================
    // Precedence
    // ========================================================================

    #[test]
    fn precedence_ordering() {
        // Multiplication binds tighter than addition
        assert!(TokenKind::Star.precedence() > TokenKind::Plus.precedence());

        // Comparison binds tighter than logical operators
        assert!(TokenKind::EqEq.precedence() > TokenKind::AmpAmp.precedence());

        // Logical AND binds tighter than logical OR
        assert!(TokenKind::AmpAmp.precedence() > TokenKind::PipePipe.precedence());
    }

    #[test]
    fn non_operators_have_no_precedence() {
        assert_eq!(TokenKind::Ident.precedence(), None);
        assert_eq!(TokenKind::IntLit.precedence(), None);
        assert_eq!(TokenKind::LParen.precedence(), None);
        assert_eq!(TokenKind::Fn.precedence(), None);
    }

    // ========================================================================
    // Token construction
    // ========================================================================

    #[test]
    fn token_new() {
        let token = Token::new(TokenKind::Fn, Span::new(0, 2));
        assert_eq!(token.kind(), TokenKind::Fn);
        assert_eq!(token.span().start(), 0);
        assert_eq!(token.span().end(), 2);
    }

    #[test]
    fn token_is() {
        let token = Token::new(TokenKind::Let, Span::new(0, 3));
        assert!(token.is(TokenKind::Let));
        assert!(!token.is(TokenKind::Fn));
    }

    #[test]
    fn token_eof() {
        let token = Token::eof(100);
        assert!(token.is(TokenKind::Eof));
        assert_eq!(token.span().start(), 100);
        assert_eq!(token.span().end(), 100);
    }

    #[test]
    fn token_error() {
        let token = Token::error(Span::new(5, 10));
        assert!(token.is(TokenKind::Error));
        assert_eq!(token.span().start(), 5);
        assert_eq!(token.span().end(), 10);
    }

    // ========================================================================
    // Display
    // ========================================================================

    #[test]
    fn token_kind_display() {
        assert_eq!(format!("{}", TokenKind::Fn), "fn");
        assert_eq!(format!("{}", TokenKind::Plus), "+");
        assert_eq!(format!("{}", TokenKind::EqEq), "==");
        assert_eq!(format!("{}", TokenKind::IntLit), "<int>");
        assert_eq!(format!("{}", TokenKind::Eof), "<eof>");
    }

    // ========================================================================
    // Copy semantics
    // ========================================================================

    #[test]
    fn token_is_copy() {
        let a = Token::new(TokenKind::Fn, Span::new(0, 2));
        let b = a; // Copy
        assert_eq!(a.kind(), b.kind());
        assert_eq!(a.span(), b.span());
    }

    #[test]
    fn token_kind_is_copy() {
        let a = TokenKind::Fn;
        let b = a; // Copy
        assert_eq!(a, b);
    }
}
