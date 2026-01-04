# Nova Coding Standards

## Rust Style

### Formatting
- Use `cargo fmt` (no exceptions)
- Max line length: 100 characters
- Use trailing commas in multi-line

### Naming
| Type | Convention | Example |
|------|------------|---------|
| Types | PascalCase | `TokenKind`, `ParseError` |
| Functions | snake_case | `parse_expression`, `new` |
| Constants | SCREAMING_SNAKE | `MAX_TOKENS`, `EOF` |
| Modules | snake_case | `token_kind`, `span` |
| Lifetimes | lowercase short | `'a`, `'src` |

### Documentation
```rust
/// Brief one-line description.
///
/// Longer description if needed, explaining behavior,
/// edge cases, and usage patterns.
///
/// # Examples
///
/// ```
/// let token = Token::new(kind, span);
/// ```
///
/// # Panics
///
/// Panics if [condition].
///
/// # Errors
///
/// Returns `Err` if [condition].
pub fn function() {}
```

## Code Quality

### Required
1. **No panics in library code** - Use `Result<T, E>`
2. **No `unwrap()`** - Use `?` or `expect("reason")`
3. **Size assertions** for core types
4. **Private fields** for invariants
5. **`#[repr(C)]`** for layout guarantees

### Encouraged
1. Functions under 50 lines
2. Nesting under 3 levels
3. Prefer iterators over loops
4. Use `const fn` where possible

### Discouraged
1. `clone()` without necessity
2. `unsafe` without justification
3. Premature optimization
4. Over-abstraction

## Error Handling

### Pattern
```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NovaError {
    /// Error during lexing
    Lex {
        span: Span,
        kind: LexErrorKind,
    },
    /// Error during parsing
    Parse {
        span: Span,
        kind: ParseErrorKind,
    },
}

impl std::error::Error for NovaError {}
impl std::fmt::Display for NovaError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Lex { span, kind } => write!(f, "lex error at {}: {}", span, kind),
            Self::Parse { span, kind } => write!(f, "parse error at {}: {}", span, kind),
        }
    }
}
```

### Rules
- Include source span in all errors
- Make error messages actionable
- No internal details in user-facing errors
- Log internal errors for debugging

## Testing

### Structure
```rust
#[cfg(test)]
mod tests {
    use super::*;

    // Happy path tests
    #[test]
    fn test_basic_case() {}

    // Edge cases
    #[test]
    fn test_empty() {}

    #[test]
    fn test_max_value() {}

    // Error cases
    #[test]
    fn test_invalid_returns_error() {}

    // Adversarial (prefix with test_attack_)
    #[test]
    fn test_attack_overflow_blocked() {}
}
```

### Naming
```
test_<function>_<scenario>_<expected>

Examples:
- test_new_valid_returns_token
- test_parse_empty_returns_error
- test_attack_deep_nesting_blocked
```

## Git Commits

### Format
```
<type>(<scope>): <description>

[optional body]

🤖 Generated with [Claude Code](https://claude.com/claude-code)

Co-Authored-By: Claude Opus 4.5 <noreply@anthropic.com>
```

### Types
| Type | Use For |
|------|---------|
| feat | New feature |
| fix | Bug fix |
| docs | Documentation |
| test | Tests |
| refactor | Refactoring |
| perf | Performance |
| chore | Maintenance |

### Examples
```
feat(token): add 12-byte Token struct with 85 token kinds

- Implements ADR-004 token size optimization
- All tokens stored in exactly 12 bytes
- Includes compile-time size assertions

🤖 Generated with [Claude Code](https://claude.com/claude-code)
```

## Security

### Input Validation
- Validate all external input
- Use checked arithmetic
- Limit recursion depth
- Limit input size

### Attack Prevention
```rust
// Size limits
const MAX_SOURCE_SIZE: usize = 4 * 1024 * 1024 * 1024; // 4GB
const MAX_NESTING_DEPTH: usize = 256;
const MAX_TOKENS: usize = 100_000_000;

// Checked arithmetic
let new_pos = pos.checked_add(len).ok_or(Error::Overflow)?;

// Recursion guard
if depth > MAX_NESTING_DEPTH {
    return Err(Error::NestingTooDeep);
}
```

## Performance

### Guidelines
- Profile before optimizing
- Benchmark critical paths
- Document complexity (O-notation)
- No allocations in hot loops

### Targets
| Operation | Target |
|-----------|--------|
| Lex token | < 100ns |
| Parse statement | < 1µs |
| Memory per token | ≤ 12 bytes |
