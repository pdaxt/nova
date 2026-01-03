# Good First Issues - Bootstrap Compiler

Welcome to Nova! This document lists beginner-friendly tasks for the bootstrap compiler.

**Before you start:**
1. Read the [README.md](README.md) to understand the architecture
2. Run `cargo test` to make sure everything works
3. Pick an issue and comment "I'd like to work on this"

---

## Lexer (`src/lexer.rs`)

### Easy

| Task | Description | Skills |
|------|-------------|--------|
| **Raw strings** | Add `r"..."` and `r#"..."#` syntax | Pattern matching |
| **Byte literals** | Add `b'x'` and `b"bytes"` syntax | Character handling |
| **Better errors** | Improve "unterminated string" messages with line numbers | Error handling |

### Medium

| Task | Description | Skills |
|------|-------------|--------|
| **Unicode escapes** | Handle `\u{1F600}` in strings | Unicode, parsing |
| **Nested comments** | Support `/* /* nested */ */` | Stack-based parsing |
| **Doc comments** | Lex `///` and `//!` as special tokens | Token design |

---

## Parser (`src/parser.rs`)

### Easy

| Task | Description | Skills |
|------|-------------|--------|
| **Struct parsing** | Implement `parse_struct()` stub | Recursive descent |
| **Enum parsing** | Implement `parse_enum()` stub | Pattern matching |
| **Match expressions** | Implement `parse_match_expr()` stub | Control flow |

### Medium

| Task | Description | Skills |
|------|-------------|--------|
| **Generic parameters** | Parse `<T, U>` in function signatures | Type systems |
| **Where clauses** | Parse `where T: Clone` | Trait bounds |
| **Error recovery** | Continue parsing after errors | Compiler design |

### Hard

| Task | Description | Skills |
|------|-------------|--------|
| **Macro parsing** | Parse `macro_rules!` definitions | Metaprogramming |
| **Async/await** | Parse async functions and await expressions | Async design |

---

## Type Checker (`src/types.rs`)

### Easy

| Task | Description | Skills |
|------|-------------|--------|
| **More primitives** | Add `u8`, `u16`, `u64`, `f32`, etc. | Type systems |
| **Better errors** | Include source location in type errors | Error reporting |

### Medium

| Task | Description | Skills |
|------|-------------|--------|
| **Type inference** | Infer types for `let x = 42;` | Hindley-Milner |
| **Array types** | Type check `[T; N]` array types | Type systems |

### Hard

| Task | Description | Skills |
|------|-------------|--------|
| **Generics** | Implement generic instantiation | Polymorphism |
| **Trait bounds** | Check trait constraints | Type theory |

---

## Code Generation (`src/codegen.rs`)

### Easy

| Task | Description | Skills |
|------|-------------|--------|
| **More instructions** | Add `i32.mul`, `i32.div`, etc. | WASM basics |
| **Local variables** | Generate `local.get`, `local.set` | Stack machines |

### Medium

| Task | Description | Skills |
|------|-------------|--------|
| **If expressions** | Generate `if-else` control flow | WASM control |
| **While loops** | Generate `loop` and `br_if` | WASM control |
| **Function calls** | Generate `call` instructions | WASM functions |

### Hard

| Task | Description | Skills |
|------|-------------|--------|
| **Memory** | Implement heap allocation | WASM memory |
| **Strings** | Store strings in linear memory | Memory layout |

---

## Testing

### Easy

| Task | Description | Skills |
|------|-------------|--------|
| **More lexer tests** | Test edge cases (empty files, only comments) | Testing |
| **Parser fuzz tests** | Generate random valid syntax | Fuzzing |
| **Error message tests** | Verify error messages are helpful | UX |

### Medium

| Task | Description | Skills |
|------|-------------|--------|
| **Integration tests** | Compile and run full programs | E2E testing |
| **Benchmark suite** | Measure compilation speed | Performance |

---

## Documentation

### Easy

| Task | Description | Skills |
|------|-------------|--------|
| **Code comments** | Add doc comments to public functions | Technical writing |
| **Examples** | Add more example `.nova` files | Language design |
| **Error catalog** | Document all error codes | Documentation |

### Medium

| Task | Description | Skills |
|------|-------------|--------|
| **Architecture docs** | Write detailed module guides | Technical writing |
| **Tutorial** | "Your first Nova program" guide | Education |

---

## How to Claim an Issue

1. Comment "I'd like to work on [task name]" on this file or create a GitHub issue
2. Fork the repository
3. Create a branch: `git checkout -b feat/your-feature`
4. Make your changes with tests
5. Submit a PR

**Need help?** Ask in the issue comments or Discord!

---

## Mentorship Available

These maintainers can help with specific areas:

| Area | Contact | Expertise |
|------|---------|-----------|
| Lexer/Parser | @pdaxt | Compiler frontend |
| Type system | @pdaxt | Type theory |
| WASM codegen | @pdaxt | WebAssembly |

---

**Thank you for contributing to Nova!** Every contribution, no matter how small, helps build something amazing.
