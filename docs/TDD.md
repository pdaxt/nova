# Test-Driven Development in Nova

Nova enforces a **test-first approach**. Code cannot be merged without passing all tests.

## Enforcement Layers

### Layer 1: Local Git Hooks

```bash
# Install hooks (run once after cloning)
./scripts/setup-hooks.sh
```

**What happens:**

| Hook | Trigger | Checks |
|------|---------|--------|
| `pre-commit` | `git commit` | fmt, clippy, build, tests |
| `pre-push` | `git push` | all tests + security tests + size guarantees |

**If tests fail, your commit/push is blocked.**

### Layer 2: GitHub CI

Every PR runs:
- Tests on 3 OS (Linux, macOS, Windows)
- Tests on 2 Rust versions (stable, beta)
- Security tests (adversarial test suites)
- Size guarantee verification
- Code quality (clippy pedantic, rustdoc)

### Layer 3: Branch Protection

The `main` branch is protected:
- PRs required (no direct push)
- All CI checks must pass
- BDFL approval required

## Writing Tests First

### 1. Define the Behavior

Before writing code, write a test that defines what it should do:

```rust
#[test]
fn parse_struct_definition() {
    let source = "struct Point { x: i32, y: i32 }";
    let tokens = lex(source).unwrap();
    let ast = parse(source, tokens).unwrap();

    // Assert the expected structure
    match &ast.items[0] {
        Item::Struct(s) => {
            assert_eq!(s.name.name, "Point");
            assert_eq!(s.fields.len(), 2);
        }
        _ => panic!("Expected struct"),
    }
}
```

### 2. Watch It Fail

Run the test:
```bash
cargo test parse_struct_definition
```

It should fail because `parse_struct` isn't implemented.

### 3. Write Minimal Code

Implement just enough to make the test pass:

```rust
fn parse_struct(&mut self) -> Result<StructDef, NovaError> {
    // Minimal implementation
}
```

### 4. Refactor

Once tests pass, improve the code while keeping tests green.

## Security Testing

For any struct with invariants, write adversarial tests:

```rust
// In *_attack.rs
#[test]
#[should_panic(expected = "start must be <= end")]
fn attack_invalid_span_panics() {
    let _ = Span::new(100, 50);  // Must panic
}

#[test]
fn attack_fields_are_private() {
    let span = Span::new(0, 10);
    // This should not compile:
    // span.start = 999;  // ERROR: field is private
}
```

## Size Guarantee Tests

For memory-critical structs, verify sizes at compile time:

```rust
// Compile-time assertion
const _: () = assert!(std::mem::size_of::<Token>() == 12);

// Runtime test for CI
#[test]
fn token_is_12_bytes() {
    assert_eq!(std::mem::size_of::<Token>(), 12);
}
```

## Quick Commands

```bash
# Run all tests
cargo test

# Run specific module tests
cargo test lexer
cargo test parser

# Run security tests only
cargo test span_attack
cargo test token_attack

# Run full CI-equivalent locally
./scripts/test-all.sh
```

## Skipping Hooks (Emergency Only)

```bash
git commit --no-verify  # Skip pre-commit
git push --no-verify    # Skip pre-push
```

**Warning:** CI will still catch failures. Only skip for WIP commits to branches.

## Test Categories

| Category | Files | Purpose |
|----------|-------|---------|
| Unit tests | `mod tests` in each file | Normal behavior |
| Adversarial tests | `*_attack.rs` | Security invariants |
| Integration tests | `tests/*.rs` | End-to-end |
| Doc tests | `///` comments | Example code works |

---

**Remember:** Tests are not optional. They're the specification.
