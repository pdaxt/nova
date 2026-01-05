# Nova Architectural Decisions

This document records every significant architectural decision in Nova, including:
- **Context**: Why the decision was needed
- **Decision**: What we chose
- **Alternatives**: What else we considered
- **Rationale**: Why we chose this over alternatives
- **Consequences**: Trade-offs and implications

Each decision is numbered (ADR-XXX) for reference.

---

## Table of Contents

1. [Foundation Architecture](#foundation-architecture)
   - [ADR-001: Five Foundation Components](#adr-001-five-foundation-components)
   - [ADR-002: Span Size (8 bytes)](#adr-002-span-size-8-bytes)
   - [ADR-003: FileId Outside Span](#adr-003-fileid-outside-span)
   - [ADR-004: Token Size Optimization](#adr-004-token-size-optimization)
   - [ADR-005: Literal Values External to Tokens](#adr-005-literal-values-external-to-tokens)
   - [ADR-006: 1-Indexed Line/Column Numbers](#adr-006-1-indexed-linecolumn-numbers)
   - [ADR-007: Binary Search for Line Lookup](#adr-007-binary-search-for-line-lookup)
   - [ADR-008: Error Codes System](#adr-008-error-codes-system)
   - [ADR-009: Nested Block Comments](#adr-009-nested-block-comments)
   - [ADR-010: Lexer Error Recovery Strategy](#adr-010-lexer-error-recovery-strategy)
2. [Type System](#type-system)
   - [ADR-011: Types Separate from Parsing](#adr-011-types-separate-from-parsing)
   - [ADR-012: Syntax Types vs Semantic Types](#adr-012-syntax-types-vs-semantic-types)
3. [Language Design](#language-design)
   - [ADR-013: Keyword Selection](#adr-013-keyword-selection)
   - [ADR-014: Operator Precedence Table](#adr-014-operator-precedence-table)
   - [ADR-015: Assignment as Expression vs Statement](#adr-015-assignment-as-expression-vs-statement)
4. [Build & Tooling](#build--tooling)
   - [ADR-016: Rust for Bootstrap Compiler](#adr-016-rust-for-bootstrap-compiler)
   - [ADR-017: WASM as Primary Target](#adr-017-wasm-as-primary-target)

---

## Foundation Architecture

### ADR-001: Five Foundation Components

**Status**: Accepted
**Date**: 2026-01-03

#### Context

We needed to determine the minimal set of components required before parsing can begin. Too few components leads to tangled responsibilities. Too many leads to unnecessary complexity.

#### Decision

The foundation consists of exactly **5 components**:
1. Span (location tracking)
2. Token (lexical units)
3. Source (file management)
4. Error (diagnostics)
5. Lexer (tokenization)

#### Alternatives Considered

| Alternative | Description | Why Rejected |
|-------------|-------------|--------------|
| **4 components** (merge Span+Source) | Combine location and content management | **Rejected**: Span is a value type (8 bytes, copied everywhere). Source is a reference type (holds file contents). Different memory semantics. Span is used by Tokens, AST nodes, IR nodes - it must be lightweight. Source is used only for error display. |
| **4 components** (merge Token+Span) | Embed span directly in token enum | **Rejected**: Span is also used by AST nodes, error labels, and IR nodes. Tying it to Token would require duplication or a separate Span type anyway. |
| **4 components** (merge Error+Source) | Combine diagnostics with source management | **Rejected**: Source is read-only data. Error is write-only output. Different directions of data flow. Also, Source is needed by LSP/tooling even when no errors exist. |
| **6 components** (add Symbol) | Separate string interning table | **Rejected**: Symbol interning is an optimization, not a requirement. Can be added to Token module later. Many simple compilers work without interning. Premature optimization. |
| **6 components** (add Keywords) | Separate keyword table | **Rejected**: Keywords are just a lookup in the Lexer. No separate data structure needed. A `match` statement or `HashMap` inside Lexer is sufficient. |
| **6 components** (add LineMap) | Separate line offset tracking | **Rejected**: LineMap is tightly coupled to Source - it's computed from file contents and used only for position lookup. No reason to separate. |
| **3 components** (Lexer only) | Single monolithic lexer module | **Rejected**: Violates separation of concerns. Makes testing harder. Makes reuse impossible (e.g., Error is also needed by Parser, Type Checker, etc.). |

#### Evidence

Cross-compiler analysis shows convergence on 5 components:

```
Rust:       rustc_span + token + rustc_errors + source_map + rustc_lexer = 5
Go:         token + scanner + position + errors + source = 5
Swift:      Token + Lexer + SourceLoc + Diagnostics + SourceManager = 5
TypeScript: SyntaxKind + Scanner + TextRange + Diagnostic + SourceFile = 5
Zig:        Token + Tokenizer + Location + Error + Source = 5
```

This convergence across independent implementations suggests this is the natural factoring.

#### Consequences

- **Positive**: Clean separation of concerns
- **Positive**: Each component can be tested independently
- **Positive**: Clear dependency graph (no cycles)
- **Negative**: More files to manage (5 crates vs 1)
- **Negative**: Some boilerplate for inter-crate dependencies

---

### ADR-002: Span Size (8 bytes)

**Status**: Accepted
**Date**: 2026-01-03

#### Context

Span is the most frequently allocated type in a compiler. Every token has a span. Every AST node has a span. Every IR node can have a span. The size of Span directly impacts memory usage and cache performance.

#### Decision

Span is exactly **8 bytes**: two `u32` fields (start and end).

```rust
#[derive(Clone, Copy)]
pub struct Span {
    start: u32,  // 4 bytes
    end: u32,    // 4 bytes
}
// Total: 8 bytes
```

#### Alternatives Considered

| Alternative | Size | Why Rejected |
|-------------|------|--------------|
| **Two `usize`** | 16 bytes on 64-bit | **Rejected**: Doubles memory usage. No file will ever be >4GB. Wastes cache. |
| **Two `u64`** | 16 bytes | **Rejected**: Same as usize. No practical need for files >4GB. |
| **Start + Length** (`u32` + `u16`) | 6 bytes | **Rejected**: Spans can be >65KB (long strings, large functions). 6 bytes has poor alignment. |
| **Single `u64` packed** | 8 bytes | **Rejected**: Bit manipulation for every access. Start/end split is 32/32 anyway. No benefit. |
| **`u32` + `u32` + `FileId`** | 12 bytes | **Rejected**: FileId rarely needed at Span level. Keep it external. See ADR-003. |
| **Interned spans** | 4 bytes (index) | **Rejected**: Adds indirection. Complicates lifetime management. Only saves memory if same span appears multiple times (rare). |

#### Evidence

- Rust's `rustc_span::Span` is 8 bytes (actually 4 bytes via interning, but with complexity)
- Swift's `SourceLoc` is 8 bytes
- Go's `token.Pos` is 4 bytes (single offset into concatenated sources - different model)

#### Consequences

- **Positive**: Fits in a register pair (x86-64)
- **Positive**: Can be `Copy` with no overhead
- **Positive**: Good cache locality in arrays
- **Negative**: Limited to files <4GB (acceptable)
- **Negative**: Need separate FileId tracking

---

### ADR-003: FileId Outside Span

**Status**: Accepted
**Date**: 2026-01-03

#### Context

When compiling multiple files, we need to know which file a span refers to. Should FileId be inside Span or tracked separately?

#### Decision

FileId is stored **outside** Span. The Span struct contains only start/end offsets.

```rust
pub struct Span {
    start: u32,
    end: u32,
    // NO file_id here
}

pub struct FileId(pub u32);
```

FileId is tracked at higher levels (in Token, in AST nodes, or in a separate table).

#### Alternatives Considered

| Alternative | Why Rejected |
|-------------|--------------|
| **FileId inside Span** | Increases Span from 8 to 12 bytes (50% larger). Most operations don't need FileId. Only error reporting needs it. Wastes memory and cache in the common case. |
| **Pack FileId into Span** (e.g., 16-bit FileId + 24-bit offsets) | 6 bytes is awkward alignment. 24-bit offsets limit files to 16MB. Too restrictive. |
| **Global "current file" context** | Works for single-file compilation only. Multi-file requires explicit tracking anyway. |

#### Evidence

- Rust's approach: `Span` is 4 bytes (interned). FileId is resolved through the interner.
- TypeScript: `TextRange` has no file. `Node` has `getSourceFile()` method.
- Go: `token.Pos` encodes file implicitly via offset ranges per file.

All major compilers keep file tracking separate from span storage.

#### Consequences

- **Positive**: Span stays 8 bytes
- **Positive**: Single-file compilation has zero overhead
- **Negative**: Multi-file needs explicit FileId passing
- **Negative**: Can't determine file from Span alone

---

### ADR-004: Token Size Optimization

**Status**: Accepted
**Date**: 2026-01-03

#### Context

Tokens are created for every lexeme in the source. A typical file might have 1,000-10,000 tokens. Token size affects memory usage during parsing.

#### Decision

Token is exactly **12 bytes** (or less):
- TokenKind: 1 byte (enum with <256 variants)
- Padding: 3 bytes
- Span: 8 bytes

```rust
pub struct Token {
    pub kind: TokenKind,  // 1 byte
    // 3 bytes padding
    pub span: Span,       // 8 bytes
}
// Total: 12 bytes
```

#### Alternatives Considered

| Alternative | Size | Why Rejected |
|-------------|------|--------------|
| **Include literal value in Token** | 24+ bytes | **Rejected**: String literals can be large. Would bloat every token. See ADR-005. |
| **Include source text slice** | 32 bytes (with fat pointer) | **Rejected**: Text can be extracted from Span + Source. Redundant storage. |
| **Use `u16` for TokenKind** | 14 bytes | **Rejected**: We won't have >256 token kinds. Wastes 1 byte per token. |
| **Pack TokenKind into Span's unused bits** | 8 bytes | **Rejected**: Span doesn't have unused bits. Would require reducing offset precision. |

#### Evidence

- Most tokens need only kind + location
- Literal values are needed for <10% of tokens
- Extracting text from span is O(1) with Source

#### Consequences

- **Positive**: Minimal memory per token
- **Positive**: Good cache locality
- **Negative**: Need separate storage for literal values
- **Negative**: Text extraction requires Source access

---

### ADR-005: Literal Values External to Tokens

**Status**: Accepted
**Date**: 2026-01-03

#### Context

Tokens like `IntLit`, `StringLit`, `FloatLit` have associated values. Where should these values be stored?

#### Decision

Literal values are stored in a **separate table**, not in the Token struct.

```rust
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
    // NO value field
}

// Parser extracts value from source when needed:
let text = source.span_to_snippet(token.span);
let value: i64 = text.parse()?;
```

#### Alternatives Considered

| Alternative | Why Rejected |
|-------------|--------------|
| **Store value in Token** | Most tokens (keywords, operators, punctuation) have no value. Storing `Option<Value>` or a union type bloats every token. |
| **Token enum with value variants** | `Token::Int(i64, Span)`, `Token::String(String, Span)` - Different token types have different sizes. Can't store in contiguous array. Complex pattern matching. |
| **Intern all values** | Adds complexity. Most literals appear exactly once. Interning only helps with repeated strings (which are rare in source code). |

#### Evidence

- TypeScript: Scanner produces tokens without values. Parser extracts values lazily.
- Rust: Tokens have no values. Literal values parsed during AST construction.
- Go: Scanner returns token type + position. Values extracted from source.

#### Consequences

- **Positive**: Token struct is small and uniform
- **Positive**: Memory efficient for non-literal tokens (majority)
- **Negative**: Requires reparsing literal text when value needed
- **Negative**: Error recovery harder (malformed literals detected later)
- **Mitigation**: Validate literal format during lexing, defer value parsing

---

### ADR-006: 1-Indexed Line/Column Numbers

**Status**: Accepted
**Date**: 2026-01-03

#### Context

Error messages show line and column numbers. Should they be 0-indexed (programmer-friendly) or 1-indexed (human-friendly)?

#### Decision

Line and column numbers are **1-indexed**.

```rust
pub struct Position {
    pub line: u32,   // 1 = first line
    pub column: u32, // 1 = first column
}
```

#### Alternatives Considered

| Alternative | Why Rejected |
|-------------|--------------|
| **0-indexed** | Every major editor shows 1-indexed lines. Users would need to add 1 mentally. Confusing error messages. |
| **0-indexed internally, 1-indexed display** | Conversion at every display point. Easy to forget. Bugs when someone displays without converting. |
| **Configurable** | Unnecessary complexity. No user wants 0-indexed line numbers in error messages. |

#### Evidence

- Every text editor (VS Code, Vim, Emacs, etc.) shows 1-indexed lines
- Every compiler (GCC, Clang, rustc, go, javac) outputs 1-indexed errors
- LSP protocol uses 0-indexed positions (we convert at the boundary)

#### Consequences

- **Positive**: Error messages match what users see in editors
- **Positive**: No mental conversion needed
- **Negative**: Need to subtract 1 for array indexing
- **Negative**: LSP boundary needs conversion
- **Mitigation**: Internal representation is 1-indexed. Convert to 0-indexed only at LSP boundary.

---

### ADR-007: Binary Search for Line Lookup

**Status**: Accepted
**Date**: 2026-01-03

#### Context

Given a byte offset, we need to find the corresponding line number. This is called for every error, for every hover in LSP, for every "go to definition" response.

#### Decision

Use **binary search** on a precomputed line-starts array.

```rust
pub struct SourceFile {
    content: String,
    line_starts: Vec<u32>,  // line_starts[i] = byte offset of line (i+1)
}

impl SourceFile {
    fn offset_to_line(&self, offset: u32) -> u32 {
        match self.line_starts.binary_search(&offset) {
            Ok(exact) => exact as u32 + 1,
            Err(next) => next as u32, // saturating_sub handled
        }
    }
}
```

Time complexity: O(log L) where L = number of lines.

#### Alternatives Considered

| Alternative | Complexity | Why Rejected |
|-------------|------------|--------------|
| **Linear scan** | O(L) | Too slow for large files. 10,000 lines = 10,000 comparisons per lookup. |
| **Cached last lookup** | O(1) amortized, O(L) worst | Only helps for sequential access. Random access (error at line 100, then line 5000) still slow. |
| **Store line number per byte** | O(1) | Requires `content.len()` extra bytes of storage. 1MB file = 1MB overhead. Wasteful. |
| **Segment tree / other structures** | O(log L) | More complex than binary search. Same time complexity. No benefit for this use case. |
| **Line index per fixed interval** | O(1) | E.g., store line number every 1000 bytes. Adds complexity. Binary search is already O(log L) which is <20 for any practical file. |

#### Evidence

- Rust's `SourceMap` uses binary search
- Go's `token.File` uses binary search
- LLVM's `SourceMgr` uses binary search

This is the standard approach across all major compilers.

#### Consequences

- **Positive**: O(log L) lookup - <20 comparisons for any file
- **Positive**: Simple implementation
- **Positive**: Precomputed once, used many times
- **Negative**: O(L) space for line_starts array
- **Negative**: Must recompute if file content changes (fine for compilers, handled for LSP)

---

### ADR-008: Error Codes System

**Status**: Accepted
**Date**: 2026-01-03

#### Context

Should errors have unique codes (like Rust's E0001)? This affects error documentation, searchability, and tooling.

#### Decision

Every error has a unique **error code** in the format `EXXXX`.

```rust
pub struct ErrorCode(pub &'static str);

pub mod codes {
    pub const UNEXPECTED_CHAR: ErrorCode = ErrorCode("E0001");
    pub const UNTERMINATED_STRING: ErrorCode = ErrorCode("E0002");
    // ...
}
```

Codes are organized by phase:
- E0xxx: Lexer errors
- E1xxx: Parser errors
- E2xxx: Type errors
- E3xxx: Borrow checker errors
- E9xxx: Internal compiler errors

#### Alternatives Considered

| Alternative | Why Rejected |
|-------------|--------------|
| **No error codes** | Can't link to documentation. Can't search for specific error. Can't track error statistics. Can't build "explain this error" tooling. |
| **Human-readable error names** | `error[unexpected_token]` - Longer, localization issues, harder to look up. |
| **Numeric only** | `error 1001` - No namespace indication. Easy to collide when adding errors. |
| **UUID per error** | Overkill. No human can remember or type a UUID. |

#### Evidence

- Rust uses E0xxx codes with `rustc --explain E0001`
- TypeScript uses TSxxxx codes
- C# uses CSxxxx codes
- Go has no error codes (often criticized for it)

Error codes are standard practice in mature compilers.

#### Consequences

- **Positive**: Errors are searchable
- **Positive**: Can link to detailed documentation
- **Positive**: Tools can track specific error patterns
- **Positive**: Users can ignore specific codes (warnings)
- **Negative**: Must maintain error code registry
- **Negative**: Backward compatibility concern when removing errors

---

### ADR-009: Nested Block Comments

**Status**: Accepted
**Date**: 2026-01-03

#### Context

Should block comments (`/* */`) nest? In C, `/* /* */ */` is an error. In Rust, it's valid.

#### Decision

Block comments **do nest**.

```nova
/* outer comment
   /* inner comment */
   still in outer
*/
```

#### Alternatives Considered

| Alternative | Why Rejected |
|-------------|--------------|
| **Non-nesting (C-style)** | Can't comment out code that contains comments. Major pain point in C. Forces `#if 0` workarounds. |
| **Only line comments** | Multi-line comments are useful for documentation blocks. |
| **Different syntax for nesting** | Added complexity. `/+ +/` (D language) or similar - unfamiliar to most developers. |

#### Evidence

- Rust: Nested block comments
- Swift: Nested block comments
- Kotlin: Nested block comments
- D: Nested block comments (`/+ +/`)
- C/C++/Java: Non-nesting (widely criticized)

Modern languages overwhelmingly support nesting.

#### Consequences

- **Positive**: Can comment out code containing comments
- **Positive**: No surprise parse errors
- **Negative**: Slightly more complex lexer (must track nesting depth)
- **Negative**: Unmatched `*/` at file end - but this is an error anyway

---

### ADR-010: Lexer Error Recovery Strategy

**Status**: Accepted
**Date**: 2026-01-03

#### Context

When the lexer encounters an invalid character or malformed token, what should it do?

#### Decision

Lexer **records the error and continues**. It produces an `Error` token for the problematic input and attempts to continue lexing.

```rust
fn next_token(&mut self) -> Token {
    // ...
    Some(c) if !is_valid_start(c) => {
        self.errors.push(Diagnostic::error(...));
        self.make_token(start, TokenKind::Error)
    }
}
```

#### Alternatives Considered

| Alternative | Why Rejected |
|-------------|--------------|
| **Panic on first error** | Compiler crashes. User gets only one error per run. Terrible UX. |
| **Return Result<Token>** | Caller must handle error at every call. Complicates parser. Can't produce multiple errors. |
| **Skip invalid characters silently** | User doesn't know something is wrong. Could lead to confusing parse errors later. |
| **Stop lexing on error** | User gets only one error per run. Must fix and recompile for next error. |

#### Evidence

- All modern compilers attempt error recovery
- IDEs require continued lexing despite errors for syntax highlighting
- Users expect to see all errors in a file, not just the first one

#### Consequences

- **Positive**: Multiple errors reported per compilation
- **Positive**: IDE can highlight beyond first error
- **Positive**: User fixes all errors at once
- **Negative**: Error tokens must be handled by parser
- **Negative**: Some cascading errors may be confusing
- **Mitigation**: Parser can limit error cascade by synchronizing on statement boundaries

---

## Type System

### ADR-011: Types Separate from Parsing

**Status**: Accepted
**Date**: 2026-01-03

#### Context

Should the parser produce typed AST or untyped AST? Should type checking happen during parsing or as a separate phase?

#### Decision

Parser produces **untyped AST**. Type checking is a **separate phase** that runs after parsing.

```
Source → Lexer → Tokens → Parser → Untyped AST → Type Checker → Typed AST
```

#### Alternatives Considered

| Alternative | Why Rejected |
|-------------|--------------|
| **Type during parsing** | Requires forward declarations or multiple passes anyway. Type errors would halt parsing. Can't report parse errors if type checking fails. Mixing concerns. |
| **Attribute grammar** | Parse and type simultaneously using inherited/synthesized attributes. Highly complex. Not suitable for languages with inference. |
| **No separate AST** | Go from tokens directly to typed IR. Loses source structure. Makes error recovery harder. Harder to implement incrementally. |

#### Evidence

Every major compiler surveyed (Rust, Go, Swift, TypeScript, Zig) separates parsing from type checking:

| Compiler | Parser Output | Type Checker Input |
|----------|--------------|-------------------|
| Rust | Untyped AST | AST → HIR → Type check |
| Go | Untyped AST | AST → Type check |
| Swift | Untyped AST | AST → Sema (type check) |
| TypeScript | Untyped AST | AST → Binder → Checker |
| Zig | Untyped AST | AST → AstGen → Sema |

Quote from Swift docs: "Semantic analysis is responsible for taking the parsed AST and transforming it into a well-formed, fully-type-checked form."

#### Consequences

- **Positive**: Parsing is fast and context-free
- **Positive**: All parse errors shown before type errors
- **Positive**: IDE can show syntax-only info without type checking
- **Positive**: Easier to implement incrementally
- **Negative**: Two AST representations (untyped and typed)
- **Negative**: Some duplication in node definitions
- **Mitigation**: Use generic AST with type parameter: `Ast<()>` vs `Ast<Type>`

---

### ADR-012: Syntax Types vs Semantic Types

**Status**: Accepted
**Date**: 2026-01-03

#### Context

Type annotations in source code (syntax types) and types in the type system (semantic types) need different representations. How should we handle this?

#### Decision

**Separate types** for syntax and semantics:

```rust
// Syntax type: what the user wrote
pub enum SyntaxType {
    Named(Ident, Vec<SyntaxType>),  // Vec<i32>
    Tuple(Vec<SyntaxType>),          // (i32, bool)
    Function(Vec<SyntaxType>, Box<SyntaxType>), // fn(i32) -> bool
    Infer,                           // _ (placeholder)
}

// Semantic type: fully resolved
pub enum SemanticType {
    Int(IntWidth),
    Float(FloatWidth),
    Bool,
    Char,
    Struct(StructId),
    Enum(EnumId),
    Tuple(Vec<SemanticType>),
    Function(Vec<SemanticType>, Box<SemanticType>),
    Generic(GenericId),
    Error, // For error recovery
}
```

#### Alternatives Considered

| Alternative | Why Rejected |
|-------------|--------------|
| **Single type representation** | Syntax types have unresolved names (`Vec<T>`). Semantic types have resolved IDs (`Struct(42)`). Can't mix. Would need `Option<Resolved>` fields everywhere. |
| **Type resolution in parser** | Parser doesn't have type context. Forward references wouldn't work. Type aliases couldn't be expanded. |
| **Strings for type names everywhere** | String comparison is slow. No structural sharing. Hard to handle generics. |

#### Evidence

- Rust has `rustc_ast::Ty` (syntax) vs `rustc_middle::ty::Ty` (semantic)
- TypeScript has syntax nodes vs `Type` objects
- Swift has AST type nodes vs semantic `Type` class

#### Consequences

- **Positive**: Clean separation of parsing and type resolution
- **Positive**: Syntax types preserve source exactly (for error messages)
- **Positive**: Semantic types can be interned/shared efficiently
- **Negative**: Conversion code between representations
- **Negative**: Two "type" concepts to understand

---

## Language Design

### ADR-013: Keyword Selection

**Status**: Accepted
**Date**: 2026-01-03

#### Context

Which keywords should Nova reserve? Too many keywords limits identifier choices. Too few requires awkward syntax.

#### Decision

Nova reserves **30 keywords** (similar to Rust):

```
// Declarations
fn, let, const, mut, struct, enum, trait, impl, type, mod, use, pub

// Control flow
if, else, match, while, for, loop, break, continue, return

// Other
as, in, where, self, Self, super, crate, true, false
```

#### Alternatives Considered

| Alternative | Why Rejected |
|-------------|--------------|
| **Minimal keywords (<15)** | Would require sigils or contextual keywords. `func` instead of `fn`? Then what for functions that don't return? |
| **C-style keywords (>40)** | `unsigned`, `volatile`, `register` - legacy baggage. Nova doesn't need these. |
| **Contextual keywords** | `async` only keyword in function context. Complicates parsing. Tooling must understand context. |
| **No `mut` keyword** | Everything mutable by default? Bugs. Everything immutable by default? Needs `mut` anyway. |

#### Evidence

| Language | Keyword Count |
|----------|---------------|
| Rust | 36 |
| Go | 25 |
| Swift | 69 (including contextual) |
| TypeScript | 63 (including strict mode) |
| Zig | 46 |

30 keywords is in the reasonable range.

#### Consequences

- **Positive**: Familiar keywords for Rust developers
- **Positive**: Clear syntax without overloaded meanings
- **Negative**: Reserved words can't be used as identifiers
- **Mitigation**: Common names (`type`, `self`) are intuitive as keywords anyway

---

### ADR-014: Operator Precedence Table

**Status**: Accepted
**Date**: 2026-01-03

#### Context

How should operators bind relative to each other? `a + b * c` should parse as `a + (b * c)`, not `(a + b) * c`.

#### Decision

Standard mathematical precedence with 13 levels:

| Level | Operators | Associativity |
|-------|-----------|---------------|
| 1 | `=`, `+=`, `-=`, etc. | Right |
| 2 | `..`, `..=` | Non-associative |
| 3 | `\|\|` | Left |
| 4 | `&&` | Left |
| 5 | `==`, `!=`, `<`, `>`, `<=`, `>=` | Non-associative |
| 6 | `\|` | Left |
| 7 | `^` | Left |
| 8 | `&` | Left |
| 9 | `<<`, `>>` | Left |
| 10 | `+`, `-` | Left |
| 11 | `*`, `/`, `%` | Left |
| 12 | `as` | Left |
| 13 | Unary `-`, `!`, `~` | Prefix |

#### Alternatives Considered

| Alternative | Why Rejected |
|-------------|--------------|
| **C precedence** | `&` higher than `==`. `a & b == c` means `a & (b == c)`. Counter-intuitive. Source of bugs. |
| **All same precedence** | Requires parentheses everywhere. `a + b * c` would be ambiguous. |
| **No precedence (Lisp-style)** | Prefix notation eliminates ambiguity. But Nova uses infix operators. Would be jarring. |
| **Fewer levels** | Collapse bitwise ops? Then `a | b & c` is ambiguous. Would need parentheses. |

#### Evidence

- Rust precedence table (we match it exactly)
- Swift has similar precedence
- Go requires parentheses for comparisons in bitwise ops (avoids the C bug)

#### Consequences

- **Positive**: Mathematical expressions work as expected
- **Positive**: Familiar to Rust developers
- **Positive**: Avoids C's precedence bugs
- **Negative**: 13 levels is a lot to remember
- **Mitigation**: IDE can show parentheses suggestions for unusual combinations

---

### ADR-015: Assignment as Expression vs Statement

**Status**: Accepted
**Date**: 2026-01-03

#### Context

Is assignment (`x = 5`) an expression (returns a value) or a statement (doesn't return)?

#### Decision

Assignment is an **expression** that returns `()` (unit).

```nova
let y = (x = 5);  // y: () - but this is allowed
```

However, assignment cannot appear in most expression contexts because `()` is not useful:

```nova
if x = 5 { }  // Error: expected bool, found ()
```

#### Alternatives Considered

| Alternative | Why Rejected |
|-------------|--------------|
| **Assignment returns assigned value** (C-style) | `if x = 5` bug. `while c = getchar()` - too easy to confuse with `==`. |
| **Assignment is statement only** | Can't chain: `a = b = c`. Can't use in match arms. Less flexible. |
| **Walrus operator** (`:=`) | Python added this late. Extra operator. Rust doesn't need it. |

#### Evidence

- Rust: Assignment returns `()`. `if x = 5` is error (type mismatch).
- Swift: Assignment returns `Void`. Same behavior.
- Go: Assignment is statement. No chaining.
- C: Assignment returns value. Bug-prone.

#### Consequences

- **Positive**: No `if x = 5` bugs
- **Positive**: Assignment chaining works: `a = b = c`
- **Positive**: Consistent with Rust
- **Negative**: Need to explain why `()` is returned

---

## Build & Tooling

### ADR-016: Rust for Bootstrap Compiler

**Status**: Accepted
**Date**: 2026-01-03

#### Context

Nova will eventually be self-hosting (written in Nova). But the first compiler must be written in an existing language. Which one?

#### Decision

The bootstrap compiler is written in **Rust**.

#### Alternatives Considered

| Alternative | Why Rejected |
|-------------|--------------|
| **C** | Manual memory management. No sum types. Error-prone. Slow to develop. |
| **C++** | Complex language. Build system complexity. Manual memory management. |
| **Go** | No sum types (critical for AST). No generics (at the time). GC pauses. |
| **OCaml** | Excellent for compilers. But smaller community. Fewer contributors would be familiar. |
| **Haskell** | Great type system. But lazy evaluation can make performance unpredictable. Steep learning curve. |
| **Python** | Too slow for compiler. Dynamic typing catches errors late. |
| **TypeScript** | Reasonable choice. But Rust's ownership model aligns with Nova's goals. |

#### Evidence

| Compiler | Bootstrap Language |
|----------|-------------------|
| Rust | OCaml → Rust |
| Go | C → Go |
| Swift | C++ |
| Zig | C++ (now Zig) |
| TypeScript | TypeScript |

Languages with systems-level goals (Rust, Go, Zig) use systems languages for bootstrap.

#### Consequences

- **Positive**: Memory safe without GC
- **Positive**: Excellent error handling (Result type)
- **Positive**: Sum types for AST representation
- **Positive**: Strong community, many contributors know Rust
- **Positive**: Aligns with Nova's design philosophy
- **Negative**: Steeper learning curve than some alternatives
- **Negative**: Compile times can be slow for large projects

---

### ADR-017: WASM as Primary Target

**Status**: Accepted
**Date**: 2026-01-03

#### Context

What should be Nova's primary compilation target?

#### Decision

**WebAssembly (WASM)** is the primary target. Native code via LLVM is secondary.

#### Alternatives Considered

| Alternative | Why Rejected |
|-------------|--------------|
| **Native only** | Limits portability. Can't run in browsers. Harder to sandbox. |
| **JVM bytecode** | Java ecosystem dependency. Large runtime. Not suitable for systems programming. |
| **CLR/.NET** | Microsoft ecosystem dependency. Windows-centric historically. |
| **LLVM IR only** | Still need to compile LLVM IR to something. LLVM is a backend, not a target. |
| **JavaScript** | Could work (AssemblyScript does this). But JS semantics leak through. Performance limitations. |
| **Custom VM** | Requires building and maintaining a VM. Huge effort. No ecosystem. |

#### Evidence

- Zig compiles to WASM
- Rust compiles to WASM
- AssemblyScript targets WASM exclusively
- WASM is supported in all major browsers and runtimes (Wasmtime, Wasmer, Node.js)

WASM provides:
1. Portability (runs everywhere)
2. Sandboxing (capabilities-based security aligns with Nova's goals)
3. Near-native performance
4. Growing ecosystem

#### Consequences

- **Positive**: Runs in browsers
- **Positive**: Portable across architectures
- **Positive**: Built-in sandboxing
- **Positive**: Simpler codegen than native
- **Negative**: No direct OS access (need WASI)
- **Negative**: Some performance overhead vs native
- **Negative**: Limited debugging tools (improving)
- **Mitigation**: LLVM backend provides native option when needed

---

## Appendix: Decision Template

For new decisions, use this template:

```markdown
### ADR-XXX: [Decision Title]

**Status**: [Proposed | Accepted | Deprecated | Superseded by ADR-XXX]
**Date**: YYYY-MM-DD

#### Context

[Why is this decision needed? What problem are we solving?]

#### Decision

[What is the decision? Be specific.]

#### Alternatives Considered

| Alternative | Why Rejected |
|-------------|--------------|
| **[Option 1]** | [Reason] |
| **[Option 2]** | [Reason] |

#### Evidence

[What research supports this decision? Other compilers? Academic papers? Benchmarks?]

#### Consequences

- **Positive**: [Good outcome]
- **Negative**: [Trade-off]
- **Mitigation**: [How we address the negative]
```

---

*This document is the authoritative source for Nova's architectural decisions. All contributors should read and understand these decisions before proposing changes.*
