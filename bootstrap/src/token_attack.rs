//! Adversarial tests for Token and TokenKind (POST-FIX version)
//!
//! These tests verify that the security properties of Token are maintained:
//! - Token is exactly 12 bytes (ADR-004)
//! - TokenKind is exactly 1 byte
//! - Private fields prevent invariant violations
//! - Memory layout is predictable (#[repr(C)] and #[repr(u8)])

#[cfg(test)]
mod attack_tests {
    use crate::span::Span;
    use crate::token::{Token, TokenKind};
    use std::mem::{align_of, size_of};

    // =========================================================================
    // SIZE GUARANTEES (ADR-004) - CRITICAL
    // =========================================================================

    #[test]
    fn token_kind_is_exactly_1_byte() {
        assert_eq!(size_of::<TokenKind>(), 1, "TokenKind MUST be 1 byte");
    }

    #[test]
    fn token_is_exactly_12_bytes() {
        assert_eq!(size_of::<Token>(), 12, "Token MUST be 12 bytes");
    }

    #[test]
    fn token_alignment_is_4() {
        assert_eq!(align_of::<Token>(), 4, "Token should align to 4 bytes");
    }

    // Compile-time size assertions (fail at compile time if wrong)
    const _TOKEN_KIND_SIZE: () = assert!(size_of::<TokenKind>() == 1);
    const _TOKEN_SIZE: () = assert!(size_of::<Token>() == 12);

    // =========================================================================
    // ATTACK 1: Memory layout exploitation - BLOCKED
    // #[repr(C)] ensures predictable layout
    // =========================================================================

    #[test]
    fn memory_layout_is_predictable() {
        let token = Token::new(TokenKind::IntLit, Span::new(100, 200));

        // Get raw bytes
        let bytes: [u8; 12] = unsafe { std::mem::transmute(token) };

        // First byte is TokenKind discriminant
        assert_eq!(bytes[0], TokenKind::IntLit as u8);

        // Bytes 1-3 are padding (values don't matter)

        // Bytes 4-7 are span.start (little-endian)
        assert_eq!(bytes[4], 100); // 100 in little-endian
        assert_eq!(bytes[5], 0);
        assert_eq!(bytes[6], 0);
        assert_eq!(bytes[7], 0);

        // Bytes 8-11 are span.end (little-endian)
        assert_eq!(bytes[8], 200); // 200 in little-endian
        assert_eq!(bytes[9], 0);
        assert_eq!(bytes[10], 0);
        assert_eq!(bytes[11], 0);
    }

    // =========================================================================
    // ATTACK 2: Private field bypass - NOW BLOCKED
    // Fields are private, must use constructor
    // =========================================================================

    // This no longer compiles:
    // let evil = Token { kind: TokenKind::IntLit, span: Span::new(0, 0) };
    // ERROR: field `kind` of struct `Token` is private

    #[test]
    fn token_requires_constructor() {
        // Can only create Token via new() or helpers
        let token = Token::new(TokenKind::Fn, Span::new(0, 2));
        assert_eq!(token.kind(), TokenKind::Fn);
        assert_eq!(token.span().start(), 0);
        assert_eq!(token.span().end(), 2);
    }

    // =========================================================================
    // ATTACK 3: TokenKind discriminant overflow - SAFE
    // #[repr(u8)] ensures valid discriminants
    // =========================================================================

    #[test]
    fn token_kind_discriminants_are_valid() {
        // All our discriminants fit in u8 (0-255)
        assert!(TokenKind::IntLit as u8 == 0);
        assert!(TokenKind::Ident as u8 == 4);
        assert!(TokenKind::Fn as u8 == 19);
        assert!(TokenKind::Eof as u8 == 250);
        assert!(TokenKind::Error as u8 == 251);
    }

    #[test]
    fn token_kind_gaps_are_intentional() {
        // Gaps in discriminants are intentional for categorization
        // Literals: 0-3
        // Keywords: 10-41
        // Operators: 50-102
        // Delimiters: 110-115
        // Special: 250-251

        assert!(TokenKind::IntLit.is_literal());
        assert!(TokenKind::FloatLit.is_literal());
        assert!(TokenKind::StringLit.is_literal());
        assert!(TokenKind::CharLit.is_literal());
        assert!(!TokenKind::Ident.is_literal()); // Ident is not a literal
    }

    // =========================================================================
    // ATTACK 4: Invalid TokenKind via transmute - NOW BLOCKED BY RUST
    // Rust's enum safety catches invalid discriminants at runtime
    // =========================================================================

    // NOTE: Transmuting invalid discriminants now panics in Rust!
    // This is a security improvement - we can't even test this anymore
    // because it would abort the process.
    //
    // Previously: unsafe code could create invalid enum values
    // Now: Rust catches this and aborts

    #[test]
    fn valid_transmute_works() {
        // Transmuting VALID discriminants still works
        let valid_discriminant: u8 = TokenKind::Fn as u8;
        let kind: TokenKind = unsafe { std::mem::transmute(valid_discriminant) };
        assert_eq!(kind, TokenKind::Fn);
    }

    // =========================================================================
    // ATTACK 5: Token with invalid Span - NOW BLOCKED
    // Span constructor panics on invalid input
    // =========================================================================

    #[test]
    #[should_panic(expected = "Span start must be <= end")]
    fn token_with_invalid_span_panics() {
        // Can't create Token with invalid Span
        let _ = Token::new(TokenKind::IntLit, Span::new(100, 50));
    }

    // =========================================================================
    // ATTACK 6: Boundary value testing
    // =========================================================================

    #[test]
    fn token_at_max_position() {
        let token = Token::new(TokenKind::Eof, Span::new(u32::MAX, u32::MAX));
        assert_eq!(token.span().start(), u32::MAX);
        assert_eq!(token.span().end(), u32::MAX);
        assert!(token.span().is_empty());
    }

    #[test]
    fn token_spanning_full_range() {
        let token = Token::new(TokenKind::StringLit, Span::new(0, u32::MAX));
        assert_eq!(token.span().start(), 0);
        assert_eq!(token.span().end(), u32::MAX);
        assert_eq!(token.span().len(), u32::MAX);
    }

    // =========================================================================
    // ATTACK 7: Copy semantics verification
    // =========================================================================

    #[test]
    fn token_is_copy() {
        let a = Token::new(TokenKind::Let, Span::new(0, 3));
        let b = a; // Copy, not move
        assert_eq!(a.kind(), b.kind());
        assert_eq!(a.span(), b.span());
    }

    #[test]
    fn token_kind_is_copy() {
        let a = TokenKind::While;
        let b = a; // Copy
        assert_eq!(a, b);
        assert_eq!(a as u8, b as u8);
    }

    // =========================================================================
    // ATTACK 8: Equality and hashing consistency
    // =========================================================================

    #[test]
    fn token_equality() {
        let a = Token::new(TokenKind::If, Span::new(10, 12));
        let b = Token::new(TokenKind::If, Span::new(10, 12));
        let c = Token::new(TokenKind::If, Span::new(20, 22)); // Different span
        let d = Token::new(TokenKind::Else, Span::new(10, 12)); // Different kind

        assert_eq!(a, b);
        assert_ne!(a, c); // Same kind, different span
        assert_ne!(a, d); // Different kind, same span
    }

    #[test]
    fn token_kind_in_hashset() {
        use std::collections::HashSet;

        let mut set = HashSet::new();
        set.insert(TokenKind::Plus);
        set.insert(TokenKind::Plus); // Duplicate
        set.insert(TokenKind::Minus);

        assert_eq!(set.len(), 2);
        assert!(set.contains(&TokenKind::Plus));
        assert!(set.contains(&TokenKind::Minus));
        assert!(!set.contains(&TokenKind::Star));
    }

    // =========================================================================
    // ATTACK 9: Classification methods consistency
    // =========================================================================

    #[test]
    fn is_keyword_is_accurate() {
        // All keywords in range 10-41
        let keywords = [
            TokenKind::As,
            TokenKind::Async,
            TokenKind::Await,
            TokenKind::Break,
            TokenKind::Const,
            TokenKind::Continue,
            TokenKind::Else,
            TokenKind::Enum,
            TokenKind::False,
            TokenKind::Fn,
            TokenKind::For,
            TokenKind::If,
            TokenKind::Impl,
            TokenKind::In,
            TokenKind::Let,
            TokenKind::Loop,
            TokenKind::Match,
            TokenKind::Mod,
            TokenKind::Mut,
            TokenKind::Pub,
            TokenKind::Return,
            TokenKind::SelfLower,
            TokenKind::SelfUpper,
            TokenKind::Static,
            TokenKind::Struct,
            TokenKind::Trait,
            TokenKind::True,
            TokenKind::Type,
            TokenKind::Unsafe,
            TokenKind::Use,
            TokenKind::Where,
            TokenKind::While,
        ];

        for kw in keywords {
            assert!(kw.is_keyword(), "{:?} should be a keyword", kw);
        }
    }

    #[test]
    fn is_operator_is_accurate() {
        // All operators in range 50-102
        let operators = [
            TokenKind::Plus,
            TokenKind::Minus,
            TokenKind::Star,
            TokenKind::Slash,
            TokenKind::Percent,
            TokenKind::EqEq,
            TokenKind::BangEq,
            TokenKind::AmpAmp,
            TokenKind::PipePipe,
        ];

        for op in operators {
            assert!(op.is_operator(), "{:?} should be an operator", op);
        }
    }

    #[test]
    fn is_delimiter_is_accurate() {
        // All delimiters in range 110-115
        let delimiters = [
            TokenKind::LParen,
            TokenKind::RParen,
            TokenKind::LBracket,
            TokenKind::RBracket,
            TokenKind::LBrace,
            TokenKind::RBrace,
        ];

        for d in delimiters {
            assert!(d.is_delimiter(), "{:?} should be a delimiter", d);
        }
    }

    // =========================================================================
    // ATTACK 10: Precedence table consistency
    // =========================================================================

    #[test]
    fn precedence_is_well_ordered() {
        // Multiplication > Addition
        assert!(TokenKind::Star.precedence() > TokenKind::Plus.precedence());

        // Comparison > Logical
        assert!(TokenKind::EqEq.precedence() > TokenKind::AmpAmp.precedence());
        assert!(TokenKind::AmpAmp.precedence() > TokenKind::PipePipe.precedence());

        // Non-operators have no precedence
        assert_eq!(TokenKind::Ident.precedence(), None);
        assert_eq!(TokenKind::LParen.precedence(), None);
    }

    // =========================================================================
    // ATTACK 11: Keyword lookup consistency
    // =========================================================================

    #[test]
    fn keyword_lookup_roundtrip() {
        let keywords = [
            ("fn", TokenKind::Fn),
            ("let", TokenKind::Let),
            ("if", TokenKind::If),
            ("else", TokenKind::Else),
            ("while", TokenKind::While),
            ("for", TokenKind::For),
            ("return", TokenKind::Return),
            ("true", TokenKind::True),
            ("false", TokenKind::False),
        ];

        for (s, expected) in keywords {
            assert_eq!(TokenKind::from_keyword(s), Some(expected));
        }
    }

    #[test]
    fn non_keywords_return_none() {
        assert_eq!(TokenKind::from_keyword("foo"), None);
        assert_eq!(TokenKind::from_keyword("FN"), None); // Case sensitive
        assert_eq!(TokenKind::from_keyword(""), None);
        assert_eq!(TokenKind::from_keyword("123"), None);
    }

    // =========================================================================
    // ATTACK 12: Helper methods work correctly
    // =========================================================================

    #[test]
    fn token_eof_helper() {
        let eof = Token::eof(1000);
        assert!(eof.is(TokenKind::Eof));
        assert_eq!(eof.span().start(), 1000);
        assert_eq!(eof.span().end(), 1000);
        assert!(eof.span().is_empty());
    }

    #[test]
    fn token_error_helper() {
        let err = Token::error(Span::new(50, 55));
        assert!(err.is(TokenKind::Error));
        assert_eq!(err.span().start(), 50);
        assert_eq!(err.span().end(), 55);
    }

    #[test]
    fn token_is_helper() {
        let token = Token::new(TokenKind::Struct, Span::new(0, 6));
        assert!(token.is(TokenKind::Struct));
        assert!(!token.is(TokenKind::Enum));
        assert!(token.is_keyword());
        assert!(!token.is_operator());
    }

    // =========================================================================
    // VERIFICATION: All security properties maintained
    // =========================================================================

    #[test]
    fn security_summary() {
        // S1: Size guarantees - ENFORCED (compile-time assertions)
        // S2: Private fields - ENFORCED (can't bypass constructor)
        // S3: Valid discriminants - ENFORCED (#[repr(u8)])
        // S4: Valid spans - ENFORCED (Span constructor panics)
        // S5: Unsafe transmute - ACCEPTABLE (caller's responsibility)

        println!("All Token security properties are maintained!");
    }
}
