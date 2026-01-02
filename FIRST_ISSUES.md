# First Issues for Contributors

These are clearly defined work items ready for contributors to pick up. Each issue is self-contained and includes everything you need to get started.

## How to Claim Work

1. Comment "I'd like to work on this" on the issue
2. Wait for assignment (usually < 24 hours)
3. Fork, branch, code, PR
4. See [CONTRIBUTING.md](CONTRIBUTING.md) for details

---

## 🟢 Good First Issues (New Contributors)

### Issue #1: Complete Lexer Test Coverage

**What:** Add comprehensive tests for the lexer

**Files:** `bootstrap/src/lexer.rs`

**Requirements:**
- [ ] Test all operator tokens (+, -, *, /, etc.)
- [ ] Test all keywords (fn, let, if, else, etc.)
- [ ] Test number literals (integers, floats, hex, binary)
- [ ] Test string literals with escapes
- [ ] Test edge cases (empty input, only whitespace)
- [ ] Test error cases (unterminated string, invalid char)

**Time:** 2-4 hours

---

### Issue #2: Add Error Recovery to Lexer

**What:** When the lexer hits an error, continue and collect multiple errors

**Files:** `bootstrap/src/lexer.rs`, `bootstrap/src/error.rs`

**Currently:** Lexer stops at first error
**Goal:** Collect all errors, report them all at once

**Time:** 3-5 hours

---

### Issue #3: Improve Error Messages

**What:** Make error messages more helpful

**Files:** `bootstrap/src/error.rs`

**Requirements:**
- [ ] Add hints ("did you mean X?")
- [ ] Add "see also" links to spec
- [ ] Test with various error scenarios
- [ ] Ensure colors work on all terminals

**Time:** 3-5 hours

---

## 🟡 Medium Difficulty

### Issue #4: Implement Struct Parsing

**What:** Parse struct definitions

**Files:** `bootstrap/src/parser.rs`, `bootstrap/src/ast.rs`

**Syntax:**
```nova
struct Point {
    x: f64,
    y: f64,
}
```

**Requirements:**
- [ ] Parse struct keyword
- [ ] Parse struct name
- [ ] Parse fields (name: type)
- [ ] Handle trailing commas
- [ ] Add tests

**Time:** 4-6 hours

---

### Issue #5: Implement Enum Parsing

**What:** Parse enum definitions

**Files:** `bootstrap/src/parser.rs`, `bootstrap/src/ast.rs`

**Syntax:**
```nova
enum Option<T> {
    Some(T),
    None,
}
```

**Requirements:**
- [ ] Parse enum keyword
- [ ] Parse variants (unit, tuple, struct)
- [ ] Handle generics
- [ ] Add tests

**Time:** 4-6 hours

---

### Issue #6: Implement Match Expression Parsing

**What:** Parse match expressions

**Files:** `bootstrap/src/parser.rs`, `bootstrap/src/ast.rs`

**Syntax:**
```nova
match value {
    Pattern1 => expr1,
    Pattern2 if guard => expr2,
    _ => default,
}
```

**Requirements:**
- [ ] Parse match keyword
- [ ] Parse arms (pattern => expr)
- [ ] Parse guards (if condition)
- [ ] Handle complex patterns
- [ ] Add tests

**Time:** 6-8 hours

---

### Issue #7: Implement Generic Parsing

**What:** Parse generic parameters and arguments

**Files:** `bootstrap/src/parser.rs`

**Syntax:**
```nova
fn map<T, U>(items: Vec<T>, f: fn(T) -> U) -> Vec<U>
```

**Requirements:**
- [ ] Parse `<T, U>` after function name
- [ ] Parse `<T>` after type name
- [ ] Handle turbofish `::< >`
- [ ] Add tests

**Time:** 4-6 hours

---

## 🔴 Harder Issues

### Issue #8: Implement Type Inference

**What:** Infer types for expressions without explicit annotations

**Files:** `bootstrap/src/types.rs`

**Currently:** Types must be explicit
**Goal:** Infer from context

```nova
let x = 42         // Should infer i64
let y = x + 1      // Should infer i64
let z = [1, 2, 3]  // Should infer [i64; 3]
```

**Requirements:**
- [ ] Implement unification algorithm
- [ ] Handle type variables
- [ ] Propagate constraints
- [ ] Add tests

**Resources:**
- [Hindley-Milner Type Inference](https://en.wikipedia.org/wiki/Hindley%E2%80%93Milner_type_system)

**Time:** 8-12 hours

---

### Issue #9: Implement Proper Control Flow in Codegen

**What:** Generate correct WASM for if/while/for

**Files:** `bootstrap/src/codegen.rs`, `bootstrap/src/ir.rs`

**Currently:** Control flow is stubbed
**Goal:** Generate proper branch instructions

**Requirements:**
- [ ] Generate `if` with proper block structure
- [ ] Generate `while` as loop with conditional break
- [ ] Handle `break` and `continue`
- [ ] Add tests

**Time:** 8-12 hours

---

### Issue #10: Add WASM Runtime Support

**What:** Create a runtime that supports print, memory, etc.

**Files:** New `runtime/` directory

**Requirements:**
- [ ] Implement print function (WASI or custom)
- [ ] Implement memory allocation
- [ ] Create JavaScript host for browser
- [ ] Create WASI host for CLI
- [ ] Add tests

**Time:** 12-16 hours

---

## 📚 Documentation Issues

### Issue #11: Write Type System Spec

**What:** Document the type system

**Files:** `spec/types.md`

**Requirements:**
- [ ] Document primitive types
- [ ] Document compound types (arrays, tuples, functions)
- [ ] Document generics
- [ ] Document type inference rules
- [ ] Add examples

**Time:** 4-6 hours

---

### Issue #12: Write Standard Library Design

**What:** Design the standard library structure

**Files:** `spec/stdlib.md`

**Requirements:**
- [ ] Define core types (Option, Result, Vec, String)
- [ ] Define I/O traits
- [ ] Define iterator traits
- [ ] Write examples

**Time:** 4-6 hours

---

## 🔧 Tooling Issues

### Issue #13: Create REPL

**What:** Interactive Nova shell

**Files:** New `tools/repl/` directory

**Requirements:**
- [ ] Read input line
- [ ] Lex, parse, type-check, evaluate
- [ ] Print result
- [ ] Handle multi-line input
- [ ] History and readline

**Time:** 8-12 hours

---

### Issue #14: Create Syntax Highlighter

**What:** VS Code / TextMate grammar for Nova

**Files:** New `editors/vscode/` directory

**Requirements:**
- [ ] TextMate grammar (JSON or YAML)
- [ ] Keywords, operators, strings, comments
- [ ] Package for VS Code extension
- [ ] README with install instructions

**Time:** 4-6 hours

---

## Priority Order

If you're not sure where to start:

1. **#1 Lexer Tests** — Best first issue, learn the codebase
2. **#4 Struct Parsing** — Core functionality, well-defined
3. **#11 Type System Spec** — If you prefer writing to coding
4. **#14 Syntax Highlighter** — If you're a VS Code user
5. **#8 Type Inference** — If you want a challenge

---

*Questions? Ask in the issue comments or join our Discord (coming soon).*
