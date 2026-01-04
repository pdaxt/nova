# Nova Project Context

## What is Nova?
Nova is a new programming language focused on:
- **Safety**: Memory safety without garbage collection
- **Performance**: Zero-cost abstractions, efficient codegen
- **Simplicity**: Clean syntax, minimal footprint
- **WebAssembly**: First-class WASM compilation target

## Current State
- **Phase**: Bootstrap compiler development
- **Language**: Rust
- **Target**: WASM (future: native)

## Repository Structure
```
nova/
├── bootstrap/           # Bootstrap compiler (Rust)
│   ├── src/
│   │   ├── lib.rs       # Library crate entry
│   │   ├── span.rs      # Source location tracking
│   │   ├── token.rs     # Token types
│   │   ├── lexer.rs     # Lexer (TODO)
│   │   ├── parser.rs    # Parser (TODO)
│   │   ├── ast.rs       # AST types (TODO)
│   │   └── error.rs     # Error handling (TODO)
│   ├── Cargo.toml
│   └── tests/
├── docs/
│   ├── decisions/       # ADRs
│   ├── specs/           # Component specs
│   └── guide/           # User guides
├── .agents/             # Multi-agent system
│   ├── agents/          # Agent definitions
│   ├── shared/          # Shared context
│   ├── inbox/           # Pending work
│   └── outbox/          # Completed work
└── .github/
    └── workflows/       # CI/CD
```

## Completed Work

### Issue #1: Span Struct
- 8-byte struct for source locations
- Compile-time size assertions
- 50+ tests including adversarial

### Issue #2: Token Struct
- 12-byte struct for tokens
- 85 token kinds
- 40+ adversarial tests
- CI automation

## Key Design Decisions

### ADR-004: Token Size (12 bytes)
- TokenKind: 1 byte
- Padding: 3 bytes
- Span: 8 bytes
- Cache-efficient for lexer

### ADR-005: Literal Storage
- Literals stored separately (not in token)
- Token only stores span
- Literal lookup via span

## Code Patterns

### Size Guarantees
```rust
const _: () = assert!(size_of::<Token>() == 12);
```

### Private Fields
```rust
pub struct Span {
    start: u32,  // Private
    end: u32,
}
impl Span {
    pub fn start(&self) -> u32 { self.start }
}
```

### Error Handling
```rust
pub fn new(start: u32, end: u32) -> Result<Self, SpanError> {
    if start > end {
        return Err(SpanError::InvalidRange { start, end });
    }
    Ok(Self { start, end })
}
```

## CI Pipeline
- Ubuntu, macOS, Windows
- Rust stable + beta
- Tests, clippy, fmt, docs, MSRV
- Security audit
- All 10 jobs must pass

## Contributing
1. Pick/create an issue
2. Architect designs spec
3. Implementer writes code
4. Reviewer checks quality
5. Tester verifies
6. Security audits
7. Perf benchmarks
8. Docs writes guides
9. Release ships it
