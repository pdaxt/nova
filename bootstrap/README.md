# Nova Bootstrap Compiler

**A minimal Rust compiler that bootstraps the Nova language.**

This compiler's sole purpose is to compile the first version of the Nova compiler written in Nova itself. Once Stage 1 is complete, this code will be archived.

---

## Quick Start

```bash
# Build
cargo build

# Run tests (90 tests, all should pass)
cargo test

# Compile a Nova program
cargo run -- compile ../examples/hello.nova

# See all commands
cargo run -- help
```

## Architecture Overview

```
src/
├── main.rs          # CLI entry point
├── span.rs          # Source location tracking (8 bytes)
├── token.rs         # Token definitions (12 bytes per token)
├── lexer.rs         # Tokenization
├── parser.rs        # Parsing → AST
├── ast.rs           # Abstract Syntax Tree
├── types.rs         # Type checking
├── ir.rs            # Intermediate representation
├── codegen.rs       # WASM code generation
├── error.rs         # Error types and reporting
│
├── span_attack.rs   # Adversarial tests for Span
└── token_attack.rs  # Adversarial tests for Token
```

## Design Decisions (ADRs)

We document architectural decisions formally. See the `docs/adr/` directory:

| ADR | Title | Status |
|-----|-------|--------|
| [ADR-001](../docs/adr/ADR-001-memory-layout.md) | Span 8-byte struct | Implemented |
| [ADR-002](../docs/adr/ADR-002-span-optimization.md) | u32 offsets (4GB limit) | Implemented |
| [ADR-004](../docs/adr/ADR-004-token-size.md) | Token 12-byte struct | Implemented |
| [ADR-005](../docs/adr/ADR-005-literal-values.md) | Literals external to tokens | Implemented |

## Key Principles

### 1. Security by Design
- All structs use private fields with getters
- Invariants enforced at construction time (panics on invalid input)
- Adversarial test suites (`*_attack.rs`) verify security properties
- Compile-time size assertions prevent silent regressions

### 2. Memory Efficiency
- `Span`: exactly 8 bytes (two u32 fields)
- `Token`: exactly 12 bytes (kind + padding + span)
- `TokenKind`: exactly 1 byte (`#[repr(u8)]`)
- No heap allocation for tokens (literals extracted from source)

### 3. Clear API Boundaries
- Public API uses getters, not direct field access
- Constructors validate inputs
- Methods are `const` where possible

## Contributing

### Good First Issues

Look for issues labeled `good first issue` and `bootstrap`:

- **Lexer**: Add new token types, handle edge cases
- **Parser**: Implement struct/enum/match parsing
- **Type checker**: Add type inference
- **Codegen**: Generate more WASM instructions

### Where to Start

1. **Read the tests first** - They document expected behavior
2. **Run `cargo test`** - Make sure everything passes
3. **Pick a TODO** - Search for `TODO:` in the code
4. **Write tests first** - TDD is encouraged

### Code Style

```rust
// Good: Clear, documented, tested
/// Parses an identifier from the token stream.
///
/// # Errors
///
/// Returns `UnexpectedToken` if the current token is not an identifier.
fn parse_ident(&mut self) -> Result<Ident, NovaError> {
    if self.peek().kind() == TokenKind::Ident {
        let token = self.advance();
        let name = self.text(token.span()).to_string();
        Ok(Ident { name, span: token.span() })
    } else {
        Err(NovaError::UnexpectedToken { ... })
    }
}
```

### Testing Philosophy

We write three kinds of tests:

1. **Unit tests** - Normal behavior (`#[test]`)
2. **Edge case tests** - Boundary conditions
3. **Adversarial tests** - Security properties (`*_attack.rs`)

Example adversarial test:
```rust
#[test]
#[should_panic(expected = "Span start must be <= end")]
fn attack_invalid_span_panics() {
    // This MUST panic - we're verifying the security invariant
    let _ = Span::new(100, 50);
}
```

## Module Guide

### `span.rs` - Source Locations

The `Span` type tracks where tokens/AST nodes come from in source code.

```rust
let span = Span::new(10, 20);  // Bytes 10-19 (exclusive end)
assert_eq!(span.start(), 10);
assert_eq!(span.end(), 20);
assert_eq!(span.len(), 10);
```

**Key invariant**: `start <= end` (enforced at construction)

### `token.rs` - Token Definitions

Tokens are the output of lexing. They're designed to be tiny (12 bytes).

```rust
// TokenKind is 1 byte - no data stored
pub enum TokenKind {
    IntLit,      // 0 - value extracted from source via span
    Ident,       // 4 - text extracted from source via span
    Fn,          // 19 - keyword
    Plus,        // 50 - operator
    // ...
}

// Token is 12 bytes: kind (1) + padding (3) + span (8)
pub struct Token {
    kind: TokenKind,
    span: Span,
}
```

**Key principle**: Literal values are NOT stored in tokens. Extract them from source:
```rust
let value = source[token.span().start()..token.span().end()].parse::<i64>()?;
```

### `lexer.rs` - Tokenization

Converts source code into a stream of tokens.

```rust
let tokens = lex("let x = 42;")?;
// [Let, Ident, Eq, IntLit, Semi, Eof]
```

**TODO for contributors**:
- Hex/binary/octal literals with underscores
- Raw strings (`r"..."`, `r#"..."#`)
- Character literals with escapes
- Better error recovery

### `parser.rs` - Parsing

Converts tokens into an AST. Uses Pratt parsing for expressions.

```rust
let source = "fn main() { return 42; }";
let tokens = lex(source)?;
let ast = parse(source, tokens)?;  // Note: needs source for literal extraction
```

**TODO for contributors**:
- Struct definitions
- Enum definitions
- Match expressions
- Generic parameters
- Where clauses

### `ast.rs` - Abstract Syntax Tree

Defines the tree structure produced by parsing.

```rust
pub struct Function {
    pub name: Ident,
    pub params: Vec<Param>,
    pub return_type: Option<Type>,
    pub body: Block,
    pub span: Span,
}
```

### `types.rs` - Type Checking

Validates and infers types for the AST.

```rust
let typed_ast = check(&ast)?;
```

**TODO for contributors**:
- Type inference
- Generic instantiation
- Trait bounds

### `ir.rs` - Intermediate Representation

Lowers the typed AST to a simpler IR for codegen.

```rust
let ir = lower(&typed_ast);
```

### `codegen.rs` - Code Generation

Generates WASM binary from IR.

```rust
let wasm_bytes = generate(&ir);
fs::write("output.wasm", wasm_bytes)?;
```

**TODO for contributors**:
- More WASM instructions
- Function calls
- Memory operations
- Control flow

## Testing

```bash
# Run all tests
cargo test

# Run specific module tests
cargo test token::tests
cargo test span_attack

# Run with output
cargo test -- --nocapture

# Run specific test
cargo test token_is_12_bytes
```

## Debugging

```bash
# Lex a file (see tokens)
cargo run -- lex ../examples/hello.nova

# Parse a file (see AST)
cargo run -- parse ../examples/hello.nova

# Compile with verbose output
RUST_BACKTRACE=1 cargo run -- compile ../examples/hello.nova
```

## Performance

The bootstrap compiler prioritizes correctness over speed. However:

- Tokens are cache-friendly (12 bytes, no heap allocation)
- Spans are copy-friendly (8 bytes)
- Source text is borrowed, not copied

## License

Same as the main Nova project: MIT OR Apache-2.0

---

**Questions?** Open an issue or ask in Discord!
