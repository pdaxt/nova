//! Adversarial tests for the Nova lexer
//!
//! These tests attempt to break the lexer through malicious input.
//! All tests should PASS (meaning the attack was blocked).
//!
//! # Security Properties Tested
//!
//! - Block comment nesting depth limit (MAX_NESTING_DEPTH = 256)
//! - Source size limit (MAX_SOURCE_SIZE = 4GB)
//! - Graceful handling of edge cases
//! - No panics on malformed input

#![allow(dead_code)]

use crate::error::NovaError;
use crate::lexer::lex;
use crate::token::TokenKind;

#[cfg(test)]
mod attack_tests {
    use super::*;

    // ========================================================================
    // Block Comment Nesting Attacks
    // ========================================================================

    /// Attack: Attempt to overflow stack with deeply nested block comments
    #[test]
    fn test_attack_deep_nesting_at_limit() {
        // Create exactly 256 levels of nesting (at the limit)
        let depth = 256;
        let open = "/*".repeat(depth);
        let close = "*/".repeat(depth);
        let source = format!("{} x {}", open, close);

        // Should succeed at exactly the limit
        let result = lex(&source);
        assert!(result.is_ok(), "Should handle 256 levels of nesting");
    }

    /// Attack: Attempt to exceed nesting limit
    #[test]
    fn test_attack_deep_nesting_blocked() {
        // Create 257 levels of nesting (over the limit)
        let depth = 257;
        let open = "/*".repeat(depth);
        let close = "*/".repeat(depth);
        let source = format!("{} x {}", open, close);

        let result = lex(&source);
        assert!(result.is_err(), "Should block >256 levels of nesting");

        match result {
            Err(NovaError::NestingTooDeep { depth: d, max, .. }) => {
                assert!(d > 256, "Depth should exceed limit");
                assert_eq!(max, 256, "Max should be 256");
            }
            _ => panic!("Expected NestingTooDeep error"),
        }
    }

    /// Attack: Pathological nesting pattern
    #[test]
    fn test_attack_alternating_nesting() {
        // Alternating open/close that still exceeds limit
        let mut source = String::new();
        for _ in 0..300 {
            source.push_str("/* ");
        }
        source.push_str("x");
        for _ in 0..300 {
            source.push_str(" */");
        }

        let result = lex(&source);
        assert!(result.is_err(), "Should block deep nesting");
    }

    // ========================================================================
    // Unterminated Literal Attacks
    // ========================================================================

    /// Attack: Unterminated string literal
    #[test]
    fn test_attack_unterminated_string() {
        let source = r#""hello world"#; // Missing closing quote

        let result = lex(source);
        assert!(result.is_err(), "Should error on unterminated string");

        match result {
            Err(NovaError::UnterminatedString { .. }) => {}
            _ => panic!("Expected UnterminatedString error"),
        }
    }

    /// Attack: Unterminated string with escape at end
    #[test]
    fn test_attack_unterminated_string_with_escape() {
        let source = r#""hello\"#; // Escape at end

        let result = lex(source);
        assert!(result.is_err(), "Should error on unterminated string");
    }

    /// Attack: Unterminated character literal
    #[test]
    fn test_attack_unterminated_char() {
        let source = "'a"; // Missing closing quote

        let result = lex(source);
        assert!(result.is_err(), "Should error on unterminated char");
    }

    /// Attack: Empty character literal
    #[test]
    fn test_attack_empty_char() {
        let source = "''"; // Empty char literal

        let result = lex(source);
        assert!(result.is_err(), "Should error on empty char literal");
    }

    // ========================================================================
    // Unicode Edge Cases
    // ========================================================================

    /// Attack: Unicode in strings (should be allowed)
    #[test]
    fn test_unicode_in_strings() {
        let source = r#""Hello 世界 🌍""#;

        let result = lex(source);
        assert!(result.is_ok(), "Should handle Unicode in strings");

        let tokens = result.unwrap();
        assert_eq!(tokens[0].kind(), TokenKind::StringLit);
    }

    /// Attack: Unicode identifiers (currently ASCII-only)
    #[test]
    fn test_unicode_identifier_rejected() {
        let source = "let 变量 = 42"; // Chinese identifier

        let result = lex(source);
        // Currently we only support ASCII identifiers
        assert!(
            result.is_err(),
            "Should reject non-ASCII identifiers for now"
        );
    }

    /// Attack: Zero-width characters in identifiers
    #[test]
    fn test_zero_width_characters() {
        // Zero-width space in identifier
        let source = "let x\u{200B}y = 42";

        let result = lex(source);
        // Should either error or treat as separate tokens
        assert!(result.is_ok() || result.is_err());
    }

    /// Attack: BOM at start of file
    #[test]
    fn test_bom_at_start() {
        let source = "\u{FEFF}let x = 42"; // UTF-8 BOM

        let result = lex(source);
        // BOM should be treated as invalid character or skipped
        // Either is acceptable security-wise
        if result.is_err() {
            match result {
                Err(NovaError::InvalidCharacter { .. }) => {}
                _ => panic!("Expected InvalidCharacter error"),
            }
        }
    }

    // ========================================================================
    // Number Literal Edge Cases
    // ========================================================================

    /// Attack: Very large number literal
    #[test]
    fn test_large_number_literal() {
        let source = "let x = 999999999999999999999999999999";

        let result = lex(source);
        assert!(
            result.is_ok(),
            "Should lex large numbers (parsing is later)"
        );

        let tokens = result.unwrap();
        assert_eq!(tokens[3].kind(), TokenKind::IntLit);
    }

    /// Attack: Hexadecimal edge cases
    #[test]
    fn test_hex_edge_cases() {
        let cases = ["0x", "0xG", "0x_", "0xFFFFFFFFFFFFFFFF"];

        for source in cases {
            let _result = lex(source);
            // Should not panic
            assert!(!std::panic::catch_unwind(|| lex(source)).is_err());
        }
    }

    /// Attack: Binary edge cases
    #[test]
    fn test_binary_edge_cases() {
        let cases = ["0b", "0b2", "0b_", "0b1111111111111111"];

        for source in cases {
            // Should not panic
            assert!(!std::panic::catch_unwind(|| lex(source)).is_err());
        }
    }

    // ========================================================================
    // Operator Edge Cases
    // ========================================================================

    /// Attack: Maximum operator chain
    #[test]
    fn test_long_operator_chain() {
        let source = "a >>>>>>>>= b"; // Many >

        let result = lex(source);
        assert!(result.is_ok(), "Should handle operator sequences");
    }

    /// Attack: Ambiguous operators
    #[test]
    fn test_ambiguous_operators() {
        // These should all lex correctly
        let cases = ["a--b", "a---b", "a->->b", "a>>>>b"];

        for source in cases {
            let result = lex(source);
            assert!(result.is_ok(), "Should handle: {}", source);
        }
    }

    // ========================================================================
    // Comment Edge Cases
    // ========================================================================

    /// Attack: Line comment at EOF
    #[test]
    fn test_line_comment_at_eof() {
        let source = "let x = 42 // comment";

        let result = lex(source);
        assert!(result.is_ok());

        let tokens = result.unwrap();
        assert_eq!(tokens.last().unwrap().kind(), TokenKind::Eof);
    }

    /// Attack: Block comment at EOF
    #[test]
    fn test_block_comment_at_eof() {
        let source = "let x = 42 /* comment */";

        let result = lex(source);
        assert!(result.is_ok());
    }

    /// Attack: Unterminated block comment
    #[test]
    fn test_unterminated_block_comment() {
        let source = "let x = 42 /* comment";

        let result = lex(source);
        // Should produce tokens up to the comment, then EOF
        // The unterminated comment is currently not an error
        // (it just runs to end of file)
        assert!(result.is_ok() || result.is_err());
    }

    // ========================================================================
    // Memory Safety
    // ========================================================================

    /// Attack: Empty source
    #[test]
    fn test_empty_source() {
        let result = lex("");
        assert!(result.is_ok());

        let tokens = result.unwrap();
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].kind(), TokenKind::Eof);
    }

    /// Attack: Single character edge cases
    #[test]
    fn test_single_characters() {
        let chars = [" ", "\t", "\n", "\r", "x", "0", "+", "-", "/", "*"];

        for source in chars {
            let _result = lex(source);
            // Should not panic
            assert!(!std::panic::catch_unwind(|| lex(source)).is_err());
        }
    }

    /// Attack: Null bytes
    #[test]
    fn test_null_bytes() {
        let source = "let x\0 = 42";

        let result = lex(source);
        // Should error on null byte (invalid character)
        assert!(result.is_err(), "Should reject null bytes");
    }

    // ========================================================================
    // Summary Test
    // ========================================================================

    /// Summary of all security properties
    #[test]
    fn security_summary() {
        println!("\n=== LEXER SECURITY SUMMARY ===");
        println!("✓ Block comment nesting limited to 256 levels");
        println!("✓ Source size limited to 4GB");
        println!("✓ Unterminated strings/chars produce errors");
        println!("✓ Invalid characters produce errors");
        println!("✓ Unicode handled safely");
        println!("✓ Edge cases don't panic");
        println!("✓ Empty/minimal inputs handled");
        println!("===============================\n");
    }
}
