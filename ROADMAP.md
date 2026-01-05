# Nova Roadmap

This document outlines the development phases for Nova. Our goal is to self-host as quickly as possible, then build out the full language.

## Overview

```
┌─────────────────────────────────────────────────────────────────┐
│                         NOVA ROADMAP                             │
├─────────────────────────────────────────────────────────────────┤
│                                                                  │
│  STAGE 0        Bootstrap                         [IN PROGRESS] │
│  ─────────────────────────────────────────────────────────────  │
│  Minimal Rust compiler. Just enough to compile Stage 1.         │
│                                                                  │
│  STAGE 1        Self-Hosting                      [NOT STARTED] │
│  ─────────────────────────────────────────────────────────────  │
│  Nova compiler written in Nova. Delete the Rust code.           │
│                                                                  │
│  STAGE 2        Full Language                     [NOT STARTED] │
│  ─────────────────────────────────────────────────────────────  │
│  Complete type system, optimizations, full features.            │
│                                                                  │
│  STAGE 3        Ecosystem                         [NOT STARTED] │
│  ─────────────────────────────────────────────────────────────  │
│  Package manager, LSP, tools, community.                        │
│                                                                  │
└─────────────────────────────────────────────────────────────────┘
```

---

## Stage 0: Bootstrap

**Goal:** A working Rust compiler that can compile Nova code.

**Status:** 🟡 In Progress

### Milestone 0.1: Lexer ✅ → 🟡
| Task | Status | Issue | Owner |
|------|--------|-------|-------|
| Token types definition | 🟡 In Progress | #1 | - |
| Whitespace/comments | ⬜ Not Started | #1 | - |
| Identifiers/keywords | ⬜ Not Started | #1 | - |
| Number literals | ⬜ Not Started | #1 | - |
| String literals | ⬜ Not Started | #1 | - |
| Operators | ⬜ Not Started | #1 | - |
| Error recovery | ⬜ Not Started | #1 | - |

### Milestone 0.2: Parser ⬜
| Task | Status | Issue | Owner |
|------|--------|-------|-------|
| AST type definitions | ⬜ Not Started | #2 | - |
| Expression parsing | ⬜ Not Started | #2 | - |
| Statement parsing | ⬜ Not Started | #2 | - |
| Function parsing | ⬜ Not Started | #2 | - |
| Type annotation parsing | ⬜ Not Started | #2 | - |
| Pattern parsing | ⬜ Not Started | #2 | - |
| Error recovery | ⬜ Not Started | #2 | - |

### Milestone 0.3: Type Checker ⬜
| Task | Status | Issue | Owner |
|------|--------|-------|-------|
| Type representation | ⬜ Not Started | #4 | - |
| Name resolution | ⬜ Not Started | #4 | - |
| Basic type checking | ⬜ Not Started | #4 | - |
| Type inference (basic) | ⬜ Not Started | #4 | - |
| Error reporting | ⬜ Not Started | #4 | - |

### Milestone 0.4: Code Generation ⬜
| Task | Status | Issue | Owner |
|------|--------|-------|-------|
| IR definition | ⬜ Not Started | #5 | - |
| AST → IR lowering | ⬜ Not Started | #5 | - |
| WASM output | ⬜ Not Started | #5 | - |
| Runtime support | ⬜ Not Started | #5 | - |

### Milestone 0.5: Hello World ⬜
| Task | Status | Issue | Owner |
|------|--------|-------|-------|
| End-to-end compilation | ⬜ Not Started | #6 | - |
| Print function | ⬜ Not Started | #6 | - |
| CLI interface | ⬜ Not Started | #6 | - |
| Basic test suite | ⬜ Not Started | #6 | - |

**Stage 0 Complete when:** `examples/hello.nova` compiles and runs.

---

## Stage 1: Self-Hosting

**Goal:** Rewrite the compiler in Nova. Delete all Rust code.

**Status:** ⬜ Not Started

**Prerequisites:** Stage 0 complete

### Milestone 1.1: Core Library ⬜
| Task | Status | Issue | Owner |
|------|--------|-------|-------|
| Option type | ⬜ Not Started | - | - |
| Result type | ⬜ Not Started | - | - |
| String type | ⬜ Not Started | - | - |
| Vec type | ⬜ Not Started | - | - |
| HashMap type | ⬜ Not Started | - | - |
| File I/O | ⬜ Not Started | - | - |

### Milestone 1.2: Lexer in Nova ⬜
| Task | Status | Issue | Owner |
|------|--------|-------|-------|
| Port lexer to Nova | ⬜ Not Started | - | - |
| Verify same behavior | ⬜ Not Started | - | - |

### Milestone 1.3: Parser in Nova ⬜
| Task | Status | Issue | Owner |
|------|--------|-------|-------|
| Port parser to Nova | ⬜ Not Started | - | - |
| Verify same behavior | ⬜ Not Started | - | - |

### Milestone 1.4: Type Checker in Nova ⬜
| Task | Status | Issue | Owner |
|------|--------|-------|-------|
| Port type checker to Nova | ⬜ Not Started | - | - |
| Verify same behavior | ⬜ Not Started | - | - |

### Milestone 1.5: Codegen in Nova ⬜
| Task | Status | Issue | Owner |
|------|--------|-------|-------|
| Port codegen to Nova | ⬜ Not Started | - | - |
| Verify same behavior | ⬜ Not Started | - | - |

### Milestone 1.6: Bootstrap Moment 🎉 ⬜
| Task | Status | Issue | Owner |
|------|--------|-------|-------|
| Nova compiler compiles itself | ⬜ Not Started | - | - |
| Triple verification | ⬜ Not Started | - | - |
| Archive Rust bootstrap | ⬜ Not Started | - | - |

**Stage 1 Complete when:** Nova compiler compiles itself, output matches.

---

## Stage 2: Full Language

**Goal:** Implement the complete Nova language with all features.

**Status:** ⬜ Not Started

**Prerequisites:** Stage 1 complete

### 2.1 Advanced Types
- [ ] Generics
- [ ] Traits/interfaces
- [ ] Dependent types (basic)
- [ ] Linear types (basic)
- [ ] Effect tracking

### 2.2 Verification
- [ ] Property specifications (`ensures`, `requires`)
- [ ] Automatic proof generation
- [ ] SMT solver integration
- [ ] Proof caching

### 2.3 Optimization
- [ ] Dead code elimination
- [ ] Inlining
- [ ] Constant folding
- [ ] Basic loop optimizations

### 2.4 Additional Backends
- [ ] LLVM backend (native code)
- [ ] GPU backend (compute shaders)
- [ ] JavaScript backend (web)

### 2.5 Capability System
- [ ] Capability types
- [ ] Static capability checking
- [ ] Sandboxed execution

---

## Stage 3: Ecosystem

**Goal:** Build the tools and community for production use.

**Status:** ⬜ Not Started

**Prerequisites:** Stage 2 substantially complete

### 3.1 Package Manager
- [ ] Package format
- [ ] Dependency resolution
- [ ] Registry (packages.nova-lang.org)
- [ ] Publishing workflow

### 3.2 Developer Tools
- [ ] Language server (LSP)
- [ ] VS Code extension
- [ ] Formatter
- [ ] Linter
- [ ] Debugger

### 3.3 Documentation
- [ ] Language reference
- [ ] Standard library docs
- [ ] Tutorials
- [ ] Examples repository

### 3.4 Interop
- [ ] C FFI
- [ ] Rust interop
- [ ] JavaScript interop
- [ ] Python interop

### 3.5 Community
- [ ] Website
- [ ] Playground
- [ ] Discord/forum
- [ ] Blog

---

## Timeline (Estimated)

| Stage | Duration | Target |
|-------|----------|--------|
| Stage 0 | 2-3 months | Q1 2026 |
| Stage 1 | 2-3 months | Q2 2026 |
| Stage 2 | 6-9 months | Q4 2026 |
| Stage 3 | Ongoing | 2026+ |

*Note: These are rough estimates. Open source projects depend on contributor availability.*

---

## How to Help

Each task in the tables above will become a GitHub issue. To contribute:

1. Find a task that interests you
2. Check the linked issue for details
3. Comment to claim it
4. See [CONTRIBUTING.md](CONTRIBUTING.md) for workflow

### Highest Priority Right Now

1. **Lexer** — Finish tokenization
2. **Parser** — Get expressions working
3. **Spec** — Document the syntax
4. **Tests** — Write test cases

---

## Decision Log

Major decisions are recorded here for posterity.

| Date | Decision | Rationale |
|------|----------|-----------|
| 2026-01-03 | Use WASM as first backend | Simpler than native, runs anywhere |
| 2026-01-03 | Self-host before features | Proves the language, reduces technical debt |
| 2026-01-03 | Rust for bootstrap | Familiar, safe, good tooling |

---

*Last updated: 2026-01-03*
