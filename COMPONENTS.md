# Nova Component Architecture

Each component is a self-contained **lego piece** with strict requirements and QA criteria. Components can be developed in parallel and must pass all criteria before integration.

## Component Map

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                           NOVA COMPONENT ARCHITECTURE                        │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                              │
│  TIER 1: FOUNDATION (No dependencies)                                       │
│  ════════════════════════════════════                                        │
│  ┌─────────┐ ┌─────────┐ ┌─────────┐ ┌─────────┐                           │
│  │ NOVA-01 │ │ NOVA-02 │ │ NOVA-03 │ │ NOVA-04 │                           │
│  │  Span   │ │  Token  │ │  Error  │ │  Source │                           │
│  └─────────┘ └─────────┘ └─────────┘ └─────────┘                           │
│                                                                              │
│  TIER 2: LEXING (Depends on Tier 1)                                         │
│  ══════════════════════════════════                                          │
│  ┌─────────┐ ┌─────────┐ ┌─────────┐                                        │
│  │ NOVA-05 │ │ NOVA-06 │ │ NOVA-07 │                                        │
│  │  Lexer  │ │ Keywords│ │Literals │                                        │
│  │  Core   │ │         │ │         │                                        │
│  └─────────┘ └─────────┘ └─────────┘                                        │
│                                                                              │
│  TIER 3: PARSING (Depends on Tier 2)                                        │
│  ═══════════════════════════════════                                         │
│  ┌─────────┐ ┌─────────┐ ┌─────────┐ ┌─────────┐ ┌─────────┐               │
│  │ NOVA-08 │ │ NOVA-09 │ │ NOVA-10 │ │ NOVA-11 │ │ NOVA-12 │               │
│  │  AST    │ │ Expr    │ │  Stmt   │ │  Types  │ │ Pattern │               │
│  │  Core   │ │ Parser  │ │ Parser  │ │ Parser  │ │ Parser  │               │
│  └─────────┘ └─────────┘ └─────────┘ └─────────┘ └─────────┘               │
│                                                                              │
│  TIER 4: SEMANTICS (Depends on Tier 3)                                      │
│  ═════════════════════════════════════                                       │
│  ┌─────────┐ ┌─────────┐ ┌─────────┐ ┌─────────┐                           │
│  │ NOVA-13 │ │ NOVA-14 │ │ NOVA-15 │ │ NOVA-16 │                           │
│  │  Scope  │ │  Type   │ │  Type   │ │  Trait  │                           │
│  │Resolver │ │ Checker │ │Inference│ │ Solver  │                           │
│  └─────────┘ └─────────┘ └─────────┘ └─────────┘                           │
│                                                                              │
│  TIER 5: IR (Depends on Tier 4)                                             │
│  ══════════════════════════════                                              │
│  ┌─────────┐ ┌─────────┐ ┌─────────┐ ┌─────────┐                           │
│  │ NOVA-17 │ │ NOVA-18 │ │ NOVA-19 │ │ NOVA-20 │                           │
│  │   IR    │ │   IR    │ │   IR    │ │   SSA   │                           │
│  │  Types  │ │ Builder │ │ Printer │ │  Form   │                           │
│  └─────────┘ └─────────┘ └─────────┘ └─────────┘                           │
│                                                                              │
│  TIER 6: OPTIMIZATION (Depends on Tier 5)                                   │
│  ════════════════════════════════════════                                    │
│  ┌─────────┐ ┌─────────┐ ┌─────────┐ ┌─────────┐ ┌─────────┐               │
│  │ NOVA-21 │ │ NOVA-22 │ │ NOVA-23 │ │ NOVA-24 │ │ NOVA-25 │               │
│  │  Pass   │ │   DCE   │ │   CSE   │ │ Inline  │ │  Const  │               │
│  │ Manager │ │         │ │         │ │         │ │  Fold   │               │
│  └─────────┘ └─────────┘ └─────────┘ └─────────┘ └─────────┘               │
│                                                                              │
│  TIER 7: CODEGEN (Depends on Tier 5)                                        │
│  ═══════════════════════════════════                                         │
│  ┌─────────┐ ┌─────────┐ ┌─────────┐ ┌─────────┐                           │
│  │ NOVA-26 │ │ NOVA-27 │ │ NOVA-28 │ │ NOVA-29 │                           │
│  │  WASM   │ │  WASM   │ │  LLVM   │ │   ABI   │                           │
│  │  Types  │ │  Emit   │ │ Backend │ │         │                           │
│  └─────────┘ └─────────┘ └─────────┘ └─────────┘                           │
│                                                                              │
│  TIER 8: RUNTIME (Independent)                                              │
│  ═════════════════════════════                                               │
│  ┌─────────┐ ┌─────────┐ ┌─────────┐ ┌─────────┐                           │
│  │ NOVA-30 │ │ NOVA-31 │ │ NOVA-32 │ │ NOVA-33 │                           │
│  │ Memory  │ │  Panic  │ │  Print  │ │   FFI   │                           │
│  │  Alloc  │ │ Handler │ │ Runtime │ │         │                           │
│  └─────────┘ └─────────┘ └─────────┘ └─────────┘                           │
│                                                                              │
│  TIER 9: STDLIB (Depends on Tier 7+8)                                       │
│  ════════════════════════════════════                                        │
│  ┌─────────┐ ┌─────────┐ ┌─────────┐ ┌─────────┐ ┌─────────┐               │
│  │ NOVA-34 │ │ NOVA-35 │ │ NOVA-36 │ │ NOVA-37 │ │ NOVA-38 │               │
│  │  Core   │ │   Vec   │ │ String  │ │ HashMap │ │   I/O   │               │
│  │ Types   │ │         │ │         │ │         │ │         │               │
│  └─────────┘ └─────────┘ └─────────┘ └─────────┘ └─────────┘               │
│                                                                              │
│  TIER 10: TOOLING (Depends on various)                                      │
│  ═════════════════════════════════════                                       │
│  ┌─────────┐ ┌─────────┐ ┌─────────┐ ┌─────────┐ ┌─────────┐               │
│  │ NOVA-39 │ │ NOVA-40 │ │ NOVA-41 │ │ NOVA-42 │ │ NOVA-43 │               │
│  │   CLI   │ │   LSP   │ │Formatter│ │  REPL   │ │  Tests  │               │
│  │         │ │         │ │         │ │         │ │ Runner  │               │
│  └─────────┘ └─────────┘ └─────────┘ └─────────┘ └─────────┘               │
│                                                                              │
└─────────────────────────────────────────────────────────────────────────────┘
```

---

## Tier 1: Foundation Components

### NOVA-01: Span Library

**Branch:** `component/nova-01-span`

**Purpose:** Source location tracking for error reporting

**Files:**
```
nova-span/
├── Cargo.toml
├── src/
│   └── lib.rs
└── tests/
    └── span_tests.rs
```

**Requirements:**
- [ ] `Span` struct with `start: usize` and `end: usize`
- [ ] `Span::new(start, end)` constructor
- [ ] `Span::merge(self, other)` to combine spans
- [ ] `Span::len()` returns span length
- [ ] `Span::contains(offset)` checks if offset is in span
- [ ] Implement `Clone`, `Copy`, `Debug`, `PartialEq`, `Eq`, `Hash`
- [ ] Zero dependencies (only std)

**QA Criteria:**
| Criterion | Requirement |
|-----------|-------------|
| Test coverage | ≥95% line coverage |
| Tests | ≥10 unit tests |
| Docs | 100% public items documented |
| Clippy | Zero warnings |
| Benchmarks | merge() < 10ns |

**Acceptance Test:**
```rust
#[test]
fn test_span_merge() {
    let a = Span::new(0, 5);
    let b = Span::new(3, 10);
    let merged = a.merge(b);
    assert_eq!(merged, Span::new(0, 10));
}
```

---

### NOVA-02: Token Library

**Branch:** `component/nova-02-token`

**Purpose:** Token type definitions

**Dependencies:** `nova-span`

**Files:**
```
nova-token/
├── Cargo.toml
├── src/
│   ├── lib.rs
│   ├── kind.rs      # TokenKind enum
│   └── token.rs     # Token struct
└── tests/
    └── token_tests.rs
```

**Requirements:**
- [ ] `TokenKind` enum with all token types (≥50 variants)
- [ ] `Token` struct with `kind: TokenKind`, `span: Span`
- [ ] `TokenKind::is_keyword()` method
- [ ] `TokenKind::is_operator()` method
- [ ] `TokenKind::is_literal()` method
- [ ] `TokenKind::precedence()` for operators
- [ ] Display impl for all token kinds
- [ ] No heap allocations for token creation

**QA Criteria:**
| Criterion | Requirement |
|-----------|-------------|
| Test coverage | ≥90% line coverage |
| Tests | ≥30 unit tests (cover all variants) |
| Docs | 100% public items documented |
| Clippy | Zero warnings |
| Size | `Token` ≤ 24 bytes |

**Acceptance Test:**
```rust
#[test]
fn test_token_kind_completeness() {
    // Ensure all keywords are recognized
    assert!(TokenKind::from_str("fn").unwrap().is_keyword());
    assert!(TokenKind::from_str("let").unwrap().is_keyword());
    // ... test all 20+ keywords
}
```

---

### NOVA-03: Error Library

**Branch:** `component/nova-03-error`

**Purpose:** Error types and reporting

**Dependencies:** `nova-span`

**Files:**
```
nova-error/
├── Cargo.toml
├── src/
│   ├── lib.rs
│   ├── diagnostic.rs  # Diagnostic struct
│   ├── reporter.rs    # Error reporter
│   └── codes.rs       # Error codes
└── tests/
    └── error_tests.rs
```

**Requirements:**
- [ ] `Diagnostic` struct with severity, message, span, hints
- [ ] `Severity` enum: Error, Warning, Info, Hint
- [ ] `ErrorCode` for each error type (E0001, E0002, etc.)
- [ ] `DiagnosticReporter` trait
- [ ] `TerminalReporter` impl with colors
- [ ] `JsonReporter` impl for tooling
- [ ] Source snippet display with line numbers
- [ ] Multi-span support (primary + secondary)
- [ ] Suggestion/fix hints

**QA Criteria:**
| Criterion | Requirement |
|-----------|-------------|
| Test coverage | ≥85% line coverage |
| Tests | ≥20 unit tests |
| Docs | 100% public items documented |
| Clippy | Zero warnings |
| Output | Matches rustc/ariadne style |

**Acceptance Test:**
```rust
#[test]
fn test_error_display() {
    let diag = Diagnostic::error("E0001", "type mismatch")
        .with_span(span)
        .with_label("expected i32")
        .with_hint("try adding a type annotation");

    let output = TerminalReporter::render(&diag, &source);
    assert!(output.contains("error[E0001]"));
    assert!(output.contains("type mismatch"));
}
```

---

### NOVA-04: Source Library

**Branch:** `component/nova-04-source`

**Purpose:** Source file management

**Dependencies:** `nova-span`

**Files:**
```
nova-source/
├── Cargo.toml
├── src/
│   ├── lib.rs
│   ├── source.rs     # Source struct
│   ├── file.rs       # SourceFile
│   └── map.rs        # SourceMap
└── tests/
    └── source_tests.rs
```

**Requirements:**
- [ ] `Source` struct holding source text
- [ ] `SourceFile` with path, contents, line offsets
- [ ] `SourceMap` for multiple files
- [ ] `span_to_location(span)` → (line, column)
- [ ] `location_to_offset(line, col)` → offset
- [ ] `get_line(line_number)` → &str
- [ ] `get_snippet(span)` → &str
- [ ] Lazy line offset computation
- [ ] Memory-mapped file support (optional)

**QA Criteria:**
| Criterion | Requirement |
|-----------|-------------|
| Test coverage | ≥90% line coverage |
| Tests | ≥25 unit tests |
| Docs | 100% public items documented |
| Clippy | Zero warnings |
| Perf | Line lookup O(log n) |

**Acceptance Test:**
```rust
#[test]
fn test_line_lookup() {
    let source = Source::new("line1\nline2\nline3");
    let (line, col) = source.span_to_location(Span::new(6, 7));
    assert_eq!(line, 2);
    assert_eq!(col, 1);
}
```

---

## Tier 2: Lexing Components

### NOVA-05: Lexer Core

**Branch:** `component/nova-05-lexer-core`

**Purpose:** Core lexing logic

**Dependencies:** `nova-span`, `nova-token`, `nova-error`, `nova-source`

**Files:**
```
nova-lexer/
├── Cargo.toml
├── src/
│   ├── lib.rs
│   ├── lexer.rs      # Main lexer
│   ├── cursor.rs     # Character cursor
│   └── tests.rs
└── tests/
    ├── lexer_tests.rs
    └── snapshots/    # Insta snapshots
```

**Requirements:**
- [ ] `Lexer::new(source: &Source)` constructor
- [ ] `Lexer::next_token()` → Token
- [ ] `Lexer::lex_all()` → Vec<Token>
- [ ] Cursor with peek(n), advance(), is_eof()
- [ ] Whitespace and comment skipping
- [ ] Error recovery: continue after bad token
- [ ] Collect all errors, don't stop at first
- [ ] Unicode identifier support (XID_Start, XID_Continue)

**QA Criteria:**
| Criterion | Requirement |
|-----------|-------------|
| Test coverage | ≥95% line coverage |
| Tests | ≥50 unit tests |
| Snapshot tests | ≥20 snapshot tests |
| Docs | 100% public items documented |
| Clippy | Zero warnings |
| Perf | ≥10MB/s throughput |
| Fuzz | 1 hour without crashes |

**Acceptance Test:**
```rust
#[test]
fn test_lex_function() {
    let source = Source::new("fn foo(x: i32) -> bool { true }");
    let tokens = Lexer::new(&source).lex_all().unwrap();
    insta::assert_debug_snapshot!(tokens);
}
```

---

### NOVA-06: Keyword Recognition

**Branch:** `component/nova-06-keywords`

**Purpose:** Keyword and reserved word handling

**Dependencies:** `nova-token`

**Files:**
```
nova-keywords/
├── Cargo.toml
├── src/
│   ├── lib.rs
│   └── keywords.rs
├── build.rs          # Generate perfect hash
└── tests/
    └── keyword_tests.rs
```

**Requirements:**
- [ ] Perfect hash table for keyword lookup
- [ ] `is_keyword(s: &str)` → bool
- [ ] `keyword_to_token(s: &str)` → Option<TokenKind>
- [ ] Reserved words (future keywords)
- [ ] Raw identifier support (`r#type`)
- [ ] Case-sensitive matching
- [ ] O(1) lookup time

**QA Criteria:**
| Criterion | Requirement |
|-----------|-------------|
| Test coverage | 100% line coverage |
| Tests | Test all keywords |
| Docs | 100% public items documented |
| Perf | Lookup < 20ns |

**Keyword List:**
```
as, async, await, break, const, continue, crate, dyn,
else, enum, extern, false, fn, for, if, impl, in,
let, loop, match, mod, move, mut, pub, ref, return,
self, Self, static, struct, super, trait, true, type,
unsafe, use, where, while
```

---

### NOVA-07: Literal Parsing

**Branch:** `component/nova-07-literals`

**Purpose:** Parse literal values (numbers, strings, etc.)

**Dependencies:** `nova-span`, `nova-error`

**Files:**
```
nova-literals/
├── Cargo.toml
├── src/
│   ├── lib.rs
│   ├── number.rs     # Integer and float parsing
│   ├── string.rs     # String and char parsing
│   └── escape.rs     # Escape sequence handling
└── tests/
    ├── number_tests.rs
    └── string_tests.rs
```

**Requirements:**
- [ ] Integer parsing: decimal, hex (0x), binary (0b), octal (0o)
- [ ] Float parsing: decimal, scientific notation
- [ ] Underscores in numbers (1_000_000)
- [ ] String parsing with escape sequences
- [ ] Escape sequences: \n, \r, \t, \\, \", \', \0, \xNN, \u{NNNN}
- [ ] Raw strings: r"...", r#"..."#
- [ ] Character literals
- [ ] Byte strings: b"..."
- [ ] Error messages for invalid literals

**QA Criteria:**
| Criterion | Requirement |
|-----------|-------------|
| Test coverage | ≥95% line coverage |
| Tests | ≥40 unit tests |
| Edge cases | All escape sequences tested |
| Overflow | Proper handling of overflow |
| Unicode | Full Unicode string support |

**Acceptance Test:**
```rust
#[test]
fn test_number_parsing() {
    assert_eq!(parse_int("0xFF"), Ok(255));
    assert_eq!(parse_int("0b1010"), Ok(10));
    assert_eq!(parse_int("1_000_000"), Ok(1000000));
    assert_eq!(parse_float("3.14e-2"), Ok(0.0314));
}

#[test]
fn test_string_escapes() {
    assert_eq!(parse_string(r#""hello\nworld""#), Ok("hello\nworld"));
    assert_eq!(parse_string(r#""\u{1F600}""#), Ok("😀"));
}
```

---

## Tier 3: Parsing Components

### NOVA-08: AST Core

**Branch:** `component/nova-08-ast-core`

**Purpose:** AST node definitions

**Dependencies:** `nova-span`, `nova-token`

**Files:**
```
nova-ast/
├── Cargo.toml
├── src/
│   ├── lib.rs
│   ├── expr.rs       # Expression nodes
│   ├── stmt.rs       # Statement nodes
│   ├── item.rs       # Item nodes (fn, struct, etc.)
│   ├── ty.rs         # Type nodes
│   ├── pat.rs        # Pattern nodes
│   ├── visitor.rs    # Visitor trait
│   └── printer.rs    # AST pretty printer
└── tests/
    └── ast_tests.rs
```

**Requirements:**
- [ ] All expression types (≥25 variants)
- [ ] All statement types (≥10 variants)
- [ ] All item types (≥10 variants)
- [ ] All type syntax nodes
- [ ] All pattern nodes
- [ ] Every node has Span
- [ ] Visitor trait for traversal
- [ ] MutVisitor for transformation
- [ ] Pretty printer for debugging

**QA Criteria:**
| Criterion | Requirement |
|-----------|-------------|
| Test coverage | ≥80% line coverage |
| Completeness | All syntax covered |
| Docs | 100% public items documented |
| Visitor | Visitor covers all nodes |
| Memory | Minimal Box usage |

---

### NOVA-09: Expression Parser

**Branch:** `component/nova-09-expr-parser`

**Purpose:** Parse expressions

**Dependencies:** `nova-ast`, `nova-lexer`, `nova-error`

**Files:**
```
nova-parser/
├── src/
│   └── expr.rs       # Add to nova-parser
└── tests/
    ├── expr_tests.rs
    └── snapshots/
```

**Requirements:**
- [ ] Pratt parser for precedence
- [ ] All binary operators with correct precedence
- [ ] All unary operators
- [ ] Parenthesized expressions
- [ ] Function calls: `f(a, b)`
- [ ] Method calls: `x.method()`
- [ ] Field access: `x.field`
- [ ] Index: `arr[i]`
- [ ] If expressions: `if cond { } else { }`
- [ ] Match expressions
- [ ] Block expressions
- [ ] Closures: `|x| x + 1`
- [ ] Ranges: `0..10`, `0..=10`
- [ ] Try: `expr?`
- [ ] Await: `expr.await`

**QA Criteria:**
| Criterion | Requirement |
|-----------|-------------|
| Test coverage | ≥95% line coverage |
| Tests | ≥60 unit tests |
| Snapshot tests | ≥30 snapshots |
| Precedence | All operators tested |
| Associativity | Left/right correct |
| Error recovery | Continues after errors |

**Precedence Table (must match exactly):**
| Level | Operators | Associativity |
|-------|-----------|---------------|
| 1 | `?` | Postfix |
| 2 | `-` `!` `&` `*` | Prefix |
| 3 | `as` | Left |
| 4 | `*` `/` `%` | Left |
| 5 | `+` `-` | Left |
| 6 | `<<` `>>` | Left |
| 7 | `&` | Left |
| 8 | `^` | Left |
| 9 | `\|` | Left |
| 10 | `==` `!=` `<` `>` `<=` `>=` | Non-assoc |
| 11 | `&&` | Left |
| 12 | `\|\|` | Left |
| 13 | `..` `..=` | Non-assoc |
| 14 | `=` `+=` `-=` etc. | Right |

---

### NOVA-10: Statement Parser

**Branch:** `component/nova-10-stmt-parser`

**Purpose:** Parse statements

**Dependencies:** `nova-ast`, `nova-09-expr-parser`

**Files:**
```
nova-parser/
├── src/
│   └── stmt.rs
└── tests/
    └── stmt_tests.rs
```

**Requirements:**
- [ ] Let statements: `let x = 1;`, `let x: T = 1;`
- [ ] Expression statements: `expr;`
- [ ] Item statements (nested functions, structs)
- [ ] Empty statements: `;`
- [ ] While loops: `while cond { }`
- [ ] For loops: `for x in iter { }`
- [ ] Loop: `loop { }`
- [ ] Break/continue with labels
- [ ] Return statements
- [ ] Semicolon handling (required vs optional)

**QA Criteria:**
| Criterion | Requirement |
|-----------|-------------|
| Test coverage | ≥95% line coverage |
| Tests | ≥30 unit tests |
| Semicolons | Correct insertion rules |

---

### NOVA-11: Type Parser

**Branch:** `component/nova-11-type-parser`

**Purpose:** Parse type annotations

**Dependencies:** `nova-ast`

**Files:**
```
nova-parser/
├── src/
│   └── ty.rs
└── tests/
    └── type_tests.rs
```

**Requirements:**
- [ ] Path types: `Foo`, `std::Vec`
- [ ] Generic types: `Vec<T>`, `HashMap<K, V>`
- [ ] Tuple types: `(A, B, C)`
- [ ] Array types: `[T; N]`
- [ ] Slice types: `[T]`
- [ ] Reference types: `&T`, `&mut T`
- [ ] Pointer types: `*const T`, `*mut T`
- [ ] Function types: `fn(A) -> B`
- [ ] Never type: `!`
- [ ] Infer type: `_`
- [ ] Impl trait: `impl Trait`
- [ ] dyn trait: `dyn Trait`
- [ ] Turbofish: `foo::<T>()`

**QA Criteria:**
| Criterion | Requirement |
|-----------|-------------|
| Test coverage | ≥95% line coverage |
| Tests | All type forms tested |
| Generics | Nested generics work |

---

### NOVA-12: Pattern Parser

**Branch:** `component/nova-12-pattern-parser`

**Purpose:** Parse patterns (for let, match, etc.)

**Dependencies:** `nova-ast`

**Files:**
```
nova-parser/
├── src/
│   └── pat.rs
└── tests/
    └── pattern_tests.rs
```

**Requirements:**
- [ ] Identifier patterns: `x`, `mut x`
- [ ] Wildcard: `_`
- [ ] Literal patterns: `1`, `"hello"`, `true`
- [ ] Tuple patterns: `(a, b, c)`
- [ ] Struct patterns: `Point { x, y }`
- [ ] Enum patterns: `Some(x)`, `None`
- [ ] Slice patterns: `[first, .., last]`
- [ ] Rest patterns: `..`
- [ ] Or patterns: `A | B | C`
- [ ] Guard patterns: `x if x > 0`
- [ ] Reference patterns: `&x`, `&mut x`
- [ ] Range patterns: `0..=10`
- [ ] At patterns: `x @ Some(_)`

**QA Criteria:**
| Criterion | Requirement |
|-----------|-------------|
| Test coverage | ≥95% line coverage |
| Tests | All pattern forms tested |
| Match | Works in match arms |
| Let | Works in let statements |

---

## Tier 4: Semantic Analysis Components

### NOVA-13: Scope Resolver

**Branch:** `component/nova-13-scope-resolver`

**Purpose:** Resolve names to definitions

**Dependencies:** `nova-ast`, `nova-error`

**Files:**
```
nova-resolve/
├── Cargo.toml
├── src/
│   ├── lib.rs
│   ├── scope.rs      # Scope stack
│   ├── resolver.rs   # Name resolution
│   ├── symbol.rs     # Symbol table
│   └── import.rs     # Import resolution
└── tests/
    └── resolve_tests.rs
```

**Requirements:**
- [ ] `Scope` struct for single scope level
- [ ] `ScopeStack` for nested scopes
- [ ] `SymbolTable` for all definitions
- [ ] `Symbol` with name, kind, span, type
- [ ] Resolve local variables
- [ ] Resolve function references
- [ ] Resolve type references
- [ ] Resolve struct fields
- [ ] Resolve enum variants
- [ ] Resolve imports
- [ ] Detect undefined names
- [ ] Detect duplicate definitions
- [ ] Detect use-before-define

**QA Criteria:**
| Criterion | Requirement |
|-----------|-------------|
| Test coverage | ≥90% line coverage |
| Tests | ≥40 unit tests |
| Errors | All error cases tested |
| Shadowing | Correct shadowing behavior |

---

### NOVA-14: Type Checker

**Branch:** `component/nova-14-type-checker`

**Purpose:** Check type correctness

**Dependencies:** `nova-ast`, `nova-resolve`, `nova-error`

**Files:**
```
nova-types/
├── Cargo.toml
├── src/
│   ├── lib.rs
│   ├── ty.rs         # Type representation
│   ├── checker.rs    # Type checking
│   ├── coerce.rs     # Type coercion
│   └── error.rs      # Type errors
└── tests/
    └── typecheck_tests.rs
```

**Requirements:**
- [ ] `Type` enum for all types
- [ ] Check binary operator types
- [ ] Check function call arguments
- [ ] Check return type matches
- [ ] Check assignment compatibility
- [ ] Check struct field types
- [ ] Check if/match arm types match
- [ ] Integer promotion rules
- [ ] Reference/deref type rules
- [ ] Never type propagation
- [ ] Unit type handling

**QA Criteria:**
| Criterion | Requirement |
|-----------|-------------|
| Test coverage | ≥90% line coverage |
| Tests | ≥50 unit tests |
| Error messages | Clear and helpful |
| Edge cases | Never type, unit type |

---

### NOVA-15: Type Inference

**Branch:** `component/nova-15-type-inference`

**Purpose:** Infer types for expressions

**Dependencies:** `nova-14-type-checker`

**Files:**
```
nova-types/
├── src/
│   ├── infer.rs      # Inference engine
│   ├── unify.rs      # Unification
│   └── constraint.rs # Constraint solving
```

**Requirements:**
- [ ] Type variables for unknowns
- [ ] Constraint generation
- [ ] Unification algorithm
- [ ] Generalization (let-polymorphism)
- [ ] Instantiation
- [ ] Occurs check
- [ ] Error on ambiguous types
- [ ] Infer from usage context

**QA Criteria:**
| Criterion | Requirement |
|-----------|-------------|
| Test coverage | ≥90% line coverage |
| Algorithm | Hindley-Milner correct |
| Polymorphism | Let-poly works |
| Errors | Clear inference errors |

---

### NOVA-16: Trait Solver

**Branch:** `component/nova-16-trait-solver`

**Purpose:** Resolve trait implementations

**Dependencies:** `nova-14-type-checker`

**Files:**
```
nova-types/
├── src/
│   ├── trait.rs      # Trait definitions
│   ├── impl.rs       # Impl blocks
│   └── solver.rs     # Trait resolution
```

**Requirements:**
- [ ] Trait definition representation
- [ ] Impl block registration
- [ ] Method resolution
- [ ] Associated type resolution
- [ ] Trait bounds checking
- [ ] Orphan rule checking
- [ ] Coherence checking
- [ ] Blanket impl handling

**QA Criteria:**
| Criterion | Requirement |
|-----------|-------------|
| Test coverage | ≥85% line coverage |
| Orphan rules | Correctly enforced |
| Method lookup | Correct dispatch |

---

## Tier 5: IR Components

### NOVA-17: IR Types

**Branch:** `component/nova-17-ir-types`

**Purpose:** IR type definitions

**Dependencies:** None (foundation)

**Files:**
```
nova-ir/
├── Cargo.toml
├── src/
│   ├── lib.rs
│   ├── types.rs      # IR types
│   ├── value.rs      # Value representation
│   ├── instr.rs      # Instructions
│   └── block.rs      # Basic blocks
```

**Requirements:**
- [ ] `IrType`: i8, i16, i32, i64, f32, f64, ptr, void, struct, array
- [ ] `Value`: typed SSA value
- [ ] `Instruction`: all operations (≥30 types)
- [ ] `Terminator`: ret, br, condbr, switch, unreachable
- [ ] `BasicBlock`: instructions + terminator
- [ ] `Function`: params, blocks, return type
- [ ] `Module`: functions, globals

**QA Criteria:**
| Criterion | Requirement |
|-----------|-------------|
| Completeness | All WASM ops representable |
| Docs | 100% documented |
| Size | Instruction ≤ 32 bytes |

---

### NOVA-18: IR Builder

**Branch:** `component/nova-18-ir-builder`

**Purpose:** Build IR from typed AST

**Dependencies:** `nova-ir-types`, `nova-types`

**Files:**
```
nova-ir/
├── src/
│   ├── builder.rs    # IR construction API
│   └── lower.rs      # AST → IR lowering
```

**Requirements:**
- [ ] `IrBuilder` with builder pattern
- [ ] `build_add(a, b)`, `build_sub(a, b)`, etc.
- [ ] `build_call(func, args)`
- [ ] `build_branch(target)`
- [ ] `build_cond_branch(cond, then, else)`
- [ ] Lower all expression types
- [ ] Lower all statement types
- [ ] Lower all control flow
- [ ] Handle closures (capture environment)

**QA Criteria:**
| Criterion | Requirement |
|-----------|-------------|
| Test coverage | ≥90% line coverage |
| Roundtrip | Print → parse → same |

---

### NOVA-19: IR Printer

**Branch:** `component/nova-19-ir-printer`

**Purpose:** Textual IR representation

**Dependencies:** `nova-ir-types`

**Files:**
```
nova-ir/
├── src/
│   ├── printer.rs    # IR → text
│   └── parser.rs     # text → IR (for testing)
```

**Requirements:**
- [ ] Human-readable format
- [ ] All instructions printable
- [ ] All types printable
- [ ] Round-trip: print → parse → identical
- [ ] Syntax highlighting hints

**Format:**
```
fn @add(i32 %a, i32 %b) -> i32 {
entry:
    %0 = add i32 %a, %b
    ret i32 %0
}
```

---

### NOVA-20: SSA Construction

**Branch:** `component/nova-20-ssa`

**Purpose:** Convert to proper SSA form

**Dependencies:** `nova-ir-types`

**Files:**
```
nova-ir/
├── src/
│   ├── ssa.rs        # SSA construction
│   ├── cfg.rs        # CFG analysis
│   └── dom.rs        # Dominator tree
```

**Requirements:**
- [ ] CFG construction
- [ ] Dominator tree computation
- [ ] Dominance frontier computation
- [ ] Phi node insertion
- [ ] Variable renaming
- [ ] SSA validation

**QA Criteria:**
| Criterion | Requirement |
|-----------|-------------|
| Correctness | SSA properties hold |
| Efficiency | O(n) construction |

---

## Tier 6: Optimization Components

### NOVA-21: Pass Manager

**Branch:** `component/nova-21-pass-manager`

**Purpose:** Optimization pass infrastructure

**Dependencies:** `nova-ir`

**Files:**
```
nova-opt/
├── Cargo.toml
├── src/
│   ├── lib.rs
│   ├── pass.rs       # Pass traits
│   ├── manager.rs    # Pass manager
│   └── analysis.rs   # Analysis framework
```

**Requirements:**
- [ ] `FunctionPass` trait
- [ ] `ModulePass` trait
- [ ] `AnalysisPass` trait
- [ ] Analysis caching
- [ ] Invalidation tracking
- [ ] Pass pipeline construction
- [ ] Debug output for each pass

---

### NOVA-22: Dead Code Elimination

**Branch:** `component/nova-22-dce`

**Purpose:** Remove dead instructions

**Dependencies:** `nova-opt-pass-manager`

**Files:**
```
nova-opt/
├── src/
│   └── dce.rs
```

**Requirements:**
- [ ] Remove unused instructions
- [ ] Remove unreachable blocks
- [ ] Preserve side effects
- [ ] Iterative to fixed point

**QA Criteria:**
| Criterion | Requirement |
|-----------|-------------|
| Test coverage | ≥95% |
| No miscompiles | Extensive testing |

---

### NOVA-23: Common Subexpression Elimination

**Branch:** `component/nova-23-cse`

**Purpose:** Eliminate redundant computations

**Dependencies:** `nova-opt-pass-manager`

**Files:**
```
nova-opt/
├── src/
│   └── cse.rs
```

**Requirements:**
- [ ] Value numbering
- [ ] Replace redundant values
- [ ] Handle commutative ops
- [ ] Respect memory operations

---

### NOVA-24: Inlining

**Branch:** `component/nova-24-inline`

**Purpose:** Inline function calls

**Dependencies:** `nova-opt-pass-manager`

**Files:**
```
nova-opt/
├── src/
│   └── inline.rs
```

**Requirements:**
- [ ] Cost model for inlining decisions
- [ ] Inline small functions
- [ ] Handle recursion (don't infinite loop)
- [ ] Update call graph
- [ ] Configurable threshold

---

### NOVA-25: Constant Folding

**Branch:** `component/nova-25-const-fold`

**Purpose:** Evaluate constants at compile time

**Dependencies:** `nova-opt-pass-manager`

**Files:**
```
nova-opt/
├── src/
│   └── const_fold.rs
```

**Requirements:**
- [ ] Fold arithmetic on constants
- [ ] Fold comparisons on constants
- [ ] Fold boolean logic
- [ ] Propagate constants
- [ ] Handle overflow correctly

---

## Tier 7: Code Generation Components

### NOVA-26: WASM Types

**Branch:** `component/nova-26-wasm-types`

**Purpose:** WebAssembly type definitions

**Dependencies:** None

**Files:**
```
nova-wasm/
├── Cargo.toml
├── src/
│   ├── lib.rs
│   ├── types.rs      # WASM types
│   ├── instr.rs      # WASM instructions
│   └── module.rs     # Module structure
```

**Requirements:**
- [ ] All WASM value types
- [ ] All WASM instructions
- [ ] Module, function, type, import, export sections
- [ ] Memory and table sections
- [ ] LEB128 encoding utilities

---

### NOVA-27: WASM Emitter

**Branch:** `component/nova-27-wasm-emit`

**Purpose:** Generate WASM binary

**Dependencies:** `nova-wasm-types`, `nova-ir`

**Files:**
```
nova-wasm/
├── src/
│   ├── emit.rs       # Binary emission
│   ├── lower.rs      # IR → WASM
│   └── validate.rs   # WASM validation
```

**Requirements:**
- [ ] Correct WASM binary format
- [ ] All sections emitted correctly
- [ ] Validate output with wasmparser
- [ ] Debug info (name section)
- [ ] Source maps

**QA Criteria:**
| Criterion | Requirement |
|-----------|-------------|
| Validation | wasmparser accepts output |
| Execution | wasmtime runs output |
| Tests | ≥30 end-to-end tests |

---

### NOVA-28: LLVM Backend

**Branch:** `component/nova-28-llvm`

**Purpose:** Generate native code via LLVM

**Dependencies:** `nova-ir`

**Files:**
```
nova-llvm/
├── Cargo.toml
├── src/
│   ├── lib.rs
│   ├── context.rs    # LLVM context
│   ├── codegen.rs    # IR → LLVM IR
│   └── target.rs     # Target machine
```

**Requirements:**
- [ ] LLVM context management
- [ ] IR → LLVM IR lowering
- [ ] Target triple handling
- [ ] Optimization levels
- [ ] Object file output
- [ ] Executable linking

---

### NOVA-29: ABI

**Branch:** `component/nova-29-abi`

**Purpose:** Calling convention handling

**Dependencies:** None

**Files:**
```
nova-abi/
├── Cargo.toml
├── src/
│   ├── lib.rs
│   ├── sysv.rs       # System V AMD64
│   ├── win64.rs      # Windows x64
│   └── wasm.rs       # WASM calling convention
```

**Requirements:**
- [ ] Argument passing rules
- [ ] Return value handling
- [ ] Stack layout
- [ ] Register allocation for args
- [ ] Struct passing rules

---

## Integration Requirements

### Every Component Must:

1. **Be independently testable**
   - Own test suite
   - No runtime dependencies on other components for tests
   - Mock interfaces for dependencies

2. **Have clear boundaries**
   - Defined public API
   - Internal details hidden
   - Versioned API

3. **Pass CI before merge**
   - All tests pass
   - No clippy warnings
   - Documentation complete
   - Coverage meets threshold

4. **Have benchmark baselines**
   - Performance tests
   - Memory usage tests
   - Regression detection

### Component Checklist Template

```markdown
## Component: NOVA-XX

- [ ] Branch created: `component/nova-XX-name`
- [ ] Cargo.toml with correct dependencies
- [ ] Public API documented
- [ ] Unit tests: ≥N tests
- [ ] Test coverage: ≥X%
- [ ] Snapshot tests (if applicable)
- [ ] Benchmarks (if applicable)
- [ ] Clippy clean
- [ ] cargo fmt applied
- [ ] Integration test with dependent component
- [ ] README.md for component
- [ ] CHANGELOG.md entry
- [ ] PR created
- [ ] @pdaxt approval
- [ ] Merged to main
```

---

## Development Order

```
Phase 1 (Foundation):   NOVA-01 → NOVA-04 (parallel)
Phase 2 (Lexing):       NOVA-05 → NOVA-07 (sequential)
Phase 3 (Parsing):      NOVA-08 → NOVA-12 (NOVA-08 first, rest parallel)
Phase 4 (Semantics):    NOVA-13 → NOVA-16 (sequential)
Phase 5 (IR):           NOVA-17 → NOVA-20 (sequential)
Phase 6 (Optimization): NOVA-21 → NOVA-25 (NOVA-21 first, rest parallel)
Phase 7 (Codegen):      NOVA-26 → NOVA-29 (parallel)
Phase 8 (Runtime):      NOVA-30 → NOVA-33 (parallel)
Phase 9 (Stdlib):       NOVA-34 → NOVA-38 (sequential)
Phase 10 (Tooling):     NOVA-39 → NOVA-43 (parallel)
```

---

*Each component = 1 PR. No mega-PRs. Clear scope. Strict QA.*
