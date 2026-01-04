# Component: Nova Lexer

## Overview

The Lexer transforms Nova source code into a stream of `Token` values. It operates in a single pass with O(n) time complexity and O(1) memory overhead per token.

## Requirements

- **REQ-001**: Single-pass O(n) lexing
- **REQ-002**: Produce 12-byte Token structs (per ADR-004)
- **REQ-003**: Track positions via 8-byte Span structs (per existing span.rs)
- **REQ-004**: Support all 85 token kinds (per existing token.rs)
- **REQ-005**: No panics on any input
- **REQ-006**: Handle Unicode identifiers (UAX #31 compliant)
- **REQ-007**: Graceful error recovery (continue after errors)
- **REQ-008**: Resource limits (protect against DoS)

## Design

### Core Types

```rust
/// The lexer that tokenizes Nova source code.
pub struct Lexer<'src> {
    /// Source code being lexed
    source: &'src str,
    /// Current byte position
    pos: u32,
    /// Accumulated errors
    errors: Vec<LexError>,
}

/// A lexer error with source location.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LexError {
    span: Span,
    kind: LexErrorKind,
}

/// Types of lexer errors.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LexErrorKind {
    /// Unexpected character
    UnexpectedChar(char),
    /// Unterminated string literal
    UnterminatedString,
    /// Unterminated block comment
    UnterminatedComment,
    /// Invalid escape sequence
    InvalidEscape(char),
    /// Invalid number literal
    InvalidNumber(String),
    /// Nesting too deep (security limit)
    NestingTooDeep,
    /// Source too large
    SourceTooLarge,
}
```

### API

```rust
impl<'src> Lexer<'src> {
    /// Creates a new lexer for the given source.
    ///
    /// Returns `Err` if source exceeds size limit (4GB).
    pub fn new(source: &'src str) -> Result<Self, LexError>;

    /// Lexes all tokens, returning them with any errors.
    ///
    /// Always returns at least one token (EOF).
    /// Errors are accumulated, not fatal.
    pub fn lex_all(self) -> (Vec<Token>, Vec<LexError>);

    /// Returns the next token.
    ///
    /// Returns `None` after EOF has been returned.
    pub fn next_token(&mut self) -> Option<Token>;

    /// Peeks at the next token without consuming it.
    pub fn peek(&self) -> Option<Token>;

    /// Returns accumulated errors so far.
    pub fn errors(&self) -> &[LexError];
}

impl Iterator for Lexer<'_> {
    type Item = Token;
    fn next(&mut self) -> Option<Token>;
}
```

### Literal Storage

Per ADR-005, literals are NOT stored in tokens. Instead:

1. Token stores only the `Span` pointing to source
2. Caller extracts literal value using span: `&source[span.start()..span.end()]`
3. For string literals, the span includes quotes
4. For numbers, the span is the raw text

```rust
// Extracting a literal value
let token = lexer.next_token()?;
if token.kind() == TokenKind::StringLit {
    let span = token.span();
    let raw = &source[span.start() as usize..span.end() as usize];
    let value = parse_string_literal(raw)?; // Handles escapes
}
```

### State Machine

The lexer uses a simple state machine:

```
START → (whitespace) → START
      → (letter/_)   → IDENT
      → (digit)      → NUMBER
      → (")          → STRING
      → (/)          → SLASH (could be comment or divide)
      → (operator)   → OPERATOR
      → (EOF)        → EOF
```

### Token Recognition

| Pattern | Token Kind | Notes |
|---------|------------|-------|
| `let` | `Let` | Keyword |
| `fn` | `Fn` | Keyword |
| `[a-zA-Z_][a-zA-Z0-9_]*` | `Ident` | Identifier |
| `[0-9]+` | `IntLit` | Integer |
| `[0-9]+.[0-9]+` | `FloatLit` | Float |
| `"..."` | `StringLit` | String (with escapes) |
| `//...` | (skip) | Line comment |
| `/*...*/` | (skip) | Block comment (nestable) |
| `+` | `Plus` | Operator |
| ... | ... | See token.rs for full list |

### Unicode Handling

- Identifiers: UAX #31 (XID_Start, XID_Continue)
- Strings: Full UTF-8 support
- Operators: ASCII only
- Normalization: NFC on identifiers

```rust
fn is_ident_start(c: char) -> bool {
    c == '_' || unicode_xid::UnicodeXID::is_xid_start(c)
}

fn is_ident_continue(c: char) -> bool {
    unicode_xid::UnicodeXID::is_xid_continue(c)
}
```

### Error Recovery

On error, the lexer:
1. Records the error with span
2. Skips to a recovery point (next whitespace or known token start)
3. Continues lexing
4. Returns all errors at the end

```rust
// Error recovery example
fn recover(&mut self) {
    // Skip until we find a safe restart point
    while let Some(c) = self.peek_char() {
        if c.is_whitespace() || is_token_start(c) {
            break;
        }
        self.advance();
    }
}
```

## Invariants

1. **Token spans are always valid**: `span.start() < span.end() <= source.len()`
2. **Token spans never overlap**: Each byte belongs to at most one token
3. **Whitespace/comments are skipped**: Not represented as tokens
4. **EOF is always last**: Exactly one EOF token, always at the end
5. **Errors preserve position**: Error spans point to the problematic location

## Security Considerations

### Attack Vectors
1. **Memory exhaustion**: Huge source files
2. **Stack overflow**: Deeply nested block comments
3. **CPU exhaustion**: Pathological patterns
4. **Unicode attacks**: Normalization bombs

### Mitigations

```rust
// Resource limits
const MAX_SOURCE_SIZE: usize = 4 * 1024 * 1024 * 1024; // 4GB
const MAX_NESTING_DEPTH: usize = 256; // Block comment nesting
const MAX_TOKEN_COUNT: usize = 100_000_000; // Sanity limit

// Enforced at construction
pub fn new(source: &str) -> Result<Self, LexError> {
    if source.len() > MAX_SOURCE_SIZE {
        return Err(LexError::source_too_large());
    }
    Ok(Self { ... })
}

// Enforced during lexing
fn lex_block_comment(&mut self, depth: usize) -> Result<(), LexError> {
    if depth > MAX_NESTING_DEPTH {
        return Err(LexError::nesting_too_deep(self.span()));
    }
    ...
}
```

## Testing Strategy

### Unit Tests
- Each token kind recognized correctly
- Edge cases (empty string, single char, max length)
- Keyword vs identifier distinction
- Number formats (int, float, hex, binary, octal)
- String escapes (\n, \t, \", \\, \uXXXX)

### Error Tests
- Unterminated strings
- Unterminated comments
- Invalid escapes
- Invalid numbers
- Unexpected characters

### Adversarial Tests
- `test_attack_huge_source` - Source at size limit
- `test_attack_deep_nesting` - 256+ nested comments
- `test_attack_unicode_bomb` - Normalization edge cases
- `test_attack_long_line` - Single line gigabytes long
- `test_attack_many_tokens` - Token count at limit

### Property Tests
- Roundtrip: tokens can reconstruct source
- Coverage: every byte covered by exactly one token or skip
- Validity: all spans are valid

## Performance Requirements

| Metric | Target | Maximum |
|--------|--------|---------|
| Throughput | > 100 MB/s | - |
| Memory per token | 0 bytes | 0 bytes (iterator) |
| Latency to first token | < 1µs | 10µs |

## Dependencies

- `unicode-xid` (0.2) - For UAX #31 identifier rules
- Internal: `span.rs`, `token.rs`

## Implementation Order

1. Basic structure (Lexer, LexError, LexErrorKind)
2. Single-char tokens (+, -, *, etc.)
3. Multi-char operators (==, !=, >=, etc.)
4. Keywords and identifiers
5. Number literals
6. String literals with escapes
7. Comments (line and block)
8. Error recovery
9. Security limits
10. Unicode support
