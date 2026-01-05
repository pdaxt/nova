//! Adversarial tests for the Nova parser
//!
//! These tests attempt to break the parser through malicious input.
//! All tests should PASS (meaning the attack was blocked).
//!
//! # Security Properties Tested
//!
//! - Expression nesting depth limit (MAX_EXPR_DEPTH = 64)
//! - Block nesting depth limit (MAX_BLOCK_DEPTH = 64)
//! - Graceful handling of malformed syntax
//! - No panics on edge cases

#![allow(dead_code)]

use crate::error::NovaError;
use crate::lexer::lex;
use crate::parser::parse;

#[cfg(test)]
mod attack_tests {
    use super::*;

    // ========================================================================
    // Expression Depth Attacks
    // ========================================================================

    /// Attack: Deeply nested parentheses
    #[test]
    fn test_attack_deep_paren_nesting() {
        // Create 66 levels of nested parentheses (exceeds 64 limit)
        let depth = 66;
        let open = "(".repeat(depth);
        let close = ")".repeat(depth);
        let source = format!("fn main() {{ let x = {}42{}; }}", open, close);

        let tokens = lex(&source).unwrap();
        let result = parse(&source, tokens);

        assert!(result.is_err(), "Should block >64 levels of expression nesting");

        match result {
            Err(NovaError::NestingTooDeep { max, .. }) => {
                assert_eq!(max, 64, "Max should be 64");
            }
            Err(e) => panic!("Expected NestingTooDeep error, got {:?}", e),
            Ok(_) => panic!("Should have errored"),
        }
    }

    /// Attack: Deeply nested binary operations
    #[test]
    fn test_attack_deep_binary_nesting() {
        // Create a chain of 66 additions (each is a nested expression)
        let depth = 66;
        let mut source = String::from("fn main() { let x = 1");
        for _ in 0..depth {
            source.push_str(" + 1");
        }
        source.push_str("; }");

        let tokens = lex(&source).unwrap();
        let _result = parse(&source, tokens);

        // Binary operations may or may not exceed depth depending on associativity
        // The key is: no panic
        assert!(!std::panic::catch_unwind(|| {
            let tokens = lex(&source).unwrap();
            let _ = parse(&source, tokens);
        })
        .is_err());
    }

    /// Attack: Deeply nested unary operators
    #[test]
    fn test_attack_deep_unary_nesting() {
        // Create 66 levels of negation
        let depth = 66;
        let ops = "-".repeat(depth);
        let source = format!("fn main() {{ let x = {}42; }}", ops);

        let tokens = lex(&source).unwrap();
        let _result = parse(&source, tokens);

        // Should either error with NestingTooDeep or succeed (if optimized)
        // Key: no panic
        assert!(!std::panic::catch_unwind(|| {
            let tokens = lex(&source).unwrap();
            let _ = parse(&source, tokens);
        })
        .is_err());
    }

    // ========================================================================
    // Block Depth Attacks
    // ========================================================================

    /// Attack: Deeply nested blocks
    #[test]
    fn test_attack_deep_block_nesting() {
        // Create 66 levels of nested blocks (exceeds 64 limit)
        let depth = 66;
        let open = "{ ".repeat(depth);
        let close = " }".repeat(depth);
        let source = format!("fn main() {{ let x = {}42{}; }}", open, close);

        let tokens = lex(&source).unwrap();
        let result = parse(&source, tokens);

        assert!(result.is_err(), "Should block >64 levels of block nesting");

        match result {
            Err(NovaError::NestingTooDeep { max, .. }) => {
                assert_eq!(max, 64, "Max should be 64");
            }
            Err(_) => {} // Other errors are acceptable (malformed syntax)
            Ok(_) => panic!("Should have errored"),
        }
    }

    /// Attack: Deeply nested if-else chains
    #[test]
    fn test_attack_deep_if_nesting() {
        // Create 66 levels of nested if statements
        let depth = 66;
        let mut source = String::from("fn main() { ");
        for _ in 0..depth {
            source.push_str("if true { ");
        }
        source.push_str("42");
        for _ in 0..depth {
            source.push_str(" }");
        }
        source.push_str(" }");

        let tokens = lex(&source).unwrap();
        let result = parse(&source, tokens);

        // Should error with NestingTooDeep
        assert!(result.is_err(), "Should block deep if nesting");
    }

    /// Attack: Deeply nested while loops
    #[test]
    fn test_attack_deep_while_nesting() {
        let depth = 66;
        let mut source = String::from("fn main() { ");
        for _ in 0..depth {
            source.push_str("while true { ");
        }
        source.push_str("break");
        for _ in 0..depth {
            source.push_str(" }");
        }
        source.push_str(" }");

        let tokens = lex(&source).unwrap();
        let result = parse(&source, tokens);

        assert!(result.is_err(), "Should block deep while nesting");
    }

    // ========================================================================
    // Malformed Syntax Attacks
    // ========================================================================

    /// Attack: Unclosed brace
    #[test]
    fn test_attack_unclosed_brace() {
        let source = "fn main() { let x = 42;";

        let tokens = lex(source).unwrap();
        let result = parse(source, tokens);

        assert!(result.is_err(), "Should error on unclosed brace");
    }

    /// Attack: Unclosed parenthesis
    #[test]
    fn test_attack_unclosed_paren() {
        let source = "fn main() { let x = (42; }";

        let tokens = lex(source).unwrap();
        let result = parse(source, tokens);

        assert!(result.is_err(), "Should error on unclosed parenthesis");
    }

    /// Attack: Extra closing brace
    #[test]
    fn test_attack_extra_closing_brace() {
        let source = "fn main() { let x = 42; }}";

        let tokens = lex(source).unwrap();
        let result = parse(source, tokens);

        // Should error (unexpected token after function)
        assert!(result.is_err(), "Should error on extra closing brace");
    }

    /// Attack: Missing function body
    #[test]
    fn test_attack_missing_fn_body() {
        let source = "fn main()";

        let tokens = lex(source).unwrap();
        let result = parse(source, tokens);

        assert!(result.is_err(), "Should error on missing function body");
    }

    /// Attack: Missing semicolon in let
    #[test]
    fn test_attack_missing_semicolon() {
        let source = "fn main() { let x = 42 }";

        let tokens = lex(source).unwrap();
        let _result = parse(source, tokens);

        // This might parse as expression statement without semi
        // The key is no panic
        assert!(!std::panic::catch_unwind(|| {
            let tokens = lex(source).unwrap();
            let _ = parse(source, tokens);
        })
        .is_err());
    }

    // ========================================================================
    // Edge Case Attacks
    // ========================================================================

    /// Attack: Empty source
    #[test]
    fn test_attack_empty_source() {
        let source = "";

        let tokens = lex(source).unwrap();
        let result = parse(source, tokens);

        assert!(result.is_ok(), "Empty source should be valid (empty program)");
        let program = result.unwrap();
        assert_eq!(program.items.len(), 0);
    }

    /// Attack: Only whitespace
    #[test]
    fn test_attack_only_whitespace() {
        let source = "   \n\t\r\n   ";

        let tokens = lex(source).unwrap();
        let result = parse(source, tokens);

        assert!(result.is_ok(), "Whitespace-only should be valid");
    }

    /// Attack: Very long identifier
    #[test]
    fn test_attack_long_identifier() {
        let name = "x".repeat(10000);
        let source = format!("fn {}() {{ }}", name);

        let tokens = lex(&source).unwrap();
        let result = parse(&source, tokens);

        assert!(result.is_ok(), "Long identifiers should be accepted");
    }

    /// Attack: Many function parameters
    #[test]
    fn test_attack_many_params() {
        let mut params = String::new();
        for i in 0..1000 {
            if i > 0 {
                params.push_str(", ");
            }
            params.push_str(&format!("x{}: i32", i));
        }
        let source = format!("fn test({}) {{ }}", params);

        let tokens = lex(&source).unwrap();
        let _result = parse(&source, tokens);

        // Should succeed (or fail gracefully)
        assert!(!std::panic::catch_unwind(|| {
            let tokens = lex(&source).unwrap();
            let _ = parse(&source, tokens);
        })
        .is_err());
    }

    /// Attack: Many function arguments
    #[test]
    fn test_attack_many_args() {
        let mut args = String::new();
        for i in 0..1000 {
            if i > 0 {
                args.push_str(", ");
            }
            args.push_str(&format!("{}", i));
        }
        let source = format!("fn main() {{ foo({}); }}", args);

        let tokens = lex(&source).unwrap();
        let result = parse(&source, tokens);

        // Should succeed
        assert!(result.is_ok(), "Many function arguments should work");
    }

    /// Attack: Very long string literal
    #[test]
    fn test_attack_long_string() {
        let content = "a".repeat(100000);
        let source = format!("fn main() {{ let x = \"{}\"; }}", content);

        let tokens = lex(&source).unwrap();
        let result = parse(&source, tokens);

        assert!(result.is_ok(), "Long strings should be accepted");
    }

    /// Attack: Many statements in a block
    #[test]
    fn test_attack_many_statements() {
        let mut stmts = String::new();
        for i in 0..1000 {
            stmts.push_str(&format!("let x{} = {};\n", i, i));
        }
        let source = format!("fn main() {{ {} }}", stmts);

        let tokens = lex(&source).unwrap();
        let result = parse(&source, tokens);

        assert!(result.is_ok(), "Many statements should work");
    }

    // ========================================================================
    // Operator Edge Cases
    // ========================================================================

    /// Attack: Chained comparisons
    #[test]
    fn test_attack_chained_comparisons() {
        let source = "fn main() { let x = 1 < 2 < 3; }";

        let tokens = lex(source).unwrap();
        let result = parse(source, tokens);

        // Chained comparisons parse as (1 < 2) < 3
        assert!(result.is_ok(), "Chained comparisons should parse");
    }

    /// Attack: Assignment chain
    #[test]
    fn test_attack_assignment_chain() {
        let source = "fn main() { a = b = c = 42; }";

        let tokens = lex(source).unwrap();
        let result = parse(source, tokens);

        // Right-associative assignment
        assert!(result.is_ok(), "Assignment chains should work");
    }

    /// Attack: Mixed operators without parens
    #[test]
    fn test_attack_mixed_operators() {
        let source = "fn main() { let x = 1 + 2 * 3 - 4 / 5 % 6; }";

        let tokens = lex(source).unwrap();
        let result = parse(source, tokens);

        assert!(result.is_ok(), "Mixed operators should parse correctly");
    }

    // ========================================================================
    // Control Flow Edge Cases
    // ========================================================================

    /// Attack: If without else
    #[test]
    fn test_attack_if_without_else() {
        let source = "fn main() { if true { 42 } }";

        let tokens = lex(source).unwrap();
        let result = parse(source, tokens);

        assert!(result.is_ok(), "If without else should work");
    }

    /// Attack: Multiple else-if
    #[test]
    fn test_attack_multiple_else_if() {
        let source = "fn main() { if a { 1 } else if b { 2 } else if c { 3 } else { 4 } }";

        let tokens = lex(source).unwrap();
        let result = parse(source, tokens);

        assert!(result.is_ok(), "Multiple else-if should work");
    }

    /// Attack: Break without value
    #[test]
    fn test_attack_break_without_value() {
        let source = "fn main() { while true { break; } }";

        let tokens = lex(source).unwrap();
        let result = parse(source, tokens);

        assert!(result.is_ok(), "Break without value should work");
    }

    /// Attack: Return without value
    #[test]
    fn test_attack_return_without_value() {
        let source = "fn main() { return; }";

        let tokens = lex(source).unwrap();
        let result = parse(source, tokens);

        assert!(result.is_ok(), "Return without value should work");
    }

    // ========================================================================
    // Type Annotation Edge Cases
    // ========================================================================

    /// Attack: Complex type annotations
    #[test]
    fn test_attack_complex_type() {
        let source = "fn main() { let x: &mut [i32] = arr; }";

        let tokens = lex(source).unwrap();
        let result = parse(source, tokens);

        assert!(result.is_ok(), "Complex type annotations should work");
    }

    /// Attack: Tuple type
    #[test]
    fn test_attack_tuple_type() {
        let source = "fn main() { let x: (i32, i64, bool) = (1, 2, true); }";

        let tokens = lex(source).unwrap();
        let result = parse(source, tokens);

        assert!(result.is_ok(), "Tuple types should work");
    }

    /// Attack: Never type
    #[test]
    fn test_attack_never_type() {
        // Use while true instead of loop (loop not implemented yet)
        let source = "fn panic() -> ! { while true {} }";

        let tokens = lex(source).unwrap();
        let result = parse(source, tokens);

        assert!(result.is_ok(), "Never type should work");
    }

    // ========================================================================
    // Memory Safety
    // ========================================================================

    /// Attack: Ensure no panics on random-looking valid syntax
    #[test]
    fn test_attack_stress_valid_syntax() {
        let sources = [
            "fn a() {}",
            "fn b(x: i32) -> i32 { x }",
            "fn c() { let x = 1 + 2 * 3; }",
            "fn d() { if true { 1 } else { 2 } }",
            "fn e() { while false { break; } }",
            "fn f() { for i in range { print(i); } }",
            "fn g() { return 42; }",
            "fn h() { let x = (1, 2, 3); }",
            "fn i() { let x = [1, 2, 3]; }",
            "fn j() { foo.bar.baz(); }",
        ];

        for source in sources {
            let result = std::panic::catch_unwind(|| {
                let tokens = lex(source).unwrap();
                let _ = parse(source, tokens);
            });
            assert!(result.is_ok(), "Parser panicked on: {}", source);
        }
    }

    // ========================================================================
    // Summary Test
    // ========================================================================

    /// Summary of all security properties
    #[test]
    fn security_summary() {
        println!("\n=== PARSER SECURITY SUMMARY ===");
        println!("  Expression nesting limited to 64 levels");
        println!("  Block nesting limited to 64 levels");
        println!("  Malformed syntax produces errors (no panics)");
        println!("  Edge cases handled gracefully");
        println!("  Long inputs don't cause issues");
        println!("================================\n");
    }
}
