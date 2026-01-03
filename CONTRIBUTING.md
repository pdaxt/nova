# Contributing to Nova

First off, thank you for considering contributing to Nova! This is an ambitious project and we need all the help we can get.

> **Important:** All pull requests require approval from @pdaxt (BDFL) before merge. See [GOVERNANCE.md](GOVERNANCE.md) for details.

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [How Can I Contribute?](#how-can-i-contribute)
- [Project Architecture](#project-architecture)
- [Development Setup](#development-setup)
- [Contribution Workflow](#contribution-workflow)
- [Style Guidelines](#style-guidelines)
- [Picking Up Work](#picking-up-work)

## Code of Conduct

This project follows our [Code of Conduct](CODE_OF_CONDUCT.md). By participating, you agree to uphold this code.

**TL;DR:** Be kind. Be constructive. We're all here to build something great.

## How Can I Contribute?

### 🐛 Report Bugs
- Check if the bug was already reported in [Issues](../../issues)
- If not, open a new issue with the `bug` label
- Include: Nova version, OS, minimal reproduction, expected vs actual behavior

### 💡 Suggest Features
- Open an issue with the `enhancement` label
- Describe the problem you're trying to solve
- Propose a solution (if you have one)

### 📖 Improve Documentation
- Docs are in `spec/` and `docs/`
- Even small fixes (typos, clarifications) are welcome
- No issue needed for docs-only PRs

### 🔧 Write Code
- See [Picking Up Work](#picking-up-work) below
- All code contributions need tests
- All code contributions need documentation

## Project Architecture

```
nova/
│
├── bootstrap/              # Rust bootstrap compiler
│   ├── src/
│   │   ├── lexer.rs       # Tokenization
│   │   ├── parser.rs      # Parsing → AST
│   │   ├── ast.rs         # AST definitions
│   │   ├── types.rs       # Type checking
│   │   ├── ir.rs          # Intermediate representation
│   │   ├── codegen.rs     # Code generation
│   │   └── main.rs        # CLI entry point
│   └── tests/
│
├── stage1/                 # Self-hosted compiler (in Nova)
│   └── (same structure, but in Nova)
│
├── std/                    # Standard library
│   ├── core/              # Core types (no dependencies)
│   ├── alloc/             # Allocation
│   └── std/               # Full standard library
│
├── spec/                   # Language specification
│   ├── syntax.md          # Syntax grammar
│   ├── types.md           # Type system
│   ├── semantics.md       # Operational semantics
│   └── capabilities.md    # Capability system
│
├── tools/                  # Developer tools
│   ├── lsp/               # Language server
│   ├── fmt/               # Formatter
│   └── test/              # Test runner
│
└── examples/               # Example programs
```

## Test-First Development

**Nova enforces test-first development.** Code that breaks tests cannot be committed.

### Setup Automated Testing

```bash
# After cloning, run this once:
./scripts/setup-hooks.sh
```

This installs git hooks that:
- **pre-commit**: Runs fmt, clippy, build, and tests
- **pre-push**: Runs security tests and size verification

If any test fails, your commit/push is blocked.

### Quick Test Commands

```bash
# Run all tests
cd bootstrap && cargo test

# Run full CI-equivalent locally
./scripts/test-all.sh

# Run security tests only
cargo test span_attack token_attack
```

See [docs/TDD.md](docs/TDD.md) for the full test-driven development guide.

## Development Setup

### Prerequisites

```bash
# Rust (for bootstrap)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup default stable

# LLVM (for native codegen) - optional, WASM works without
# macOS
brew install llvm@17

# Ubuntu
sudo apt install llvm-17-dev

# Windows
# Download from https://llvm.org/builds/
```

### Build

```bash
# Clone the repo
git clone https://github.com/nova-lang/nova.git
cd nova

# Build bootstrap compiler
cd bootstrap
cargo build

# Run tests
cargo test

# Build in release mode
cargo build --release
```

### Verify Setup

```bash
# Compile hello world
./target/debug/nova compile ../examples/hello.nova -o hello.wasm

# Run it (needs wasmtime or similar)
wasmtime hello.wasm
```

## Contribution Workflow

### 1. Find or Create an Issue

- Browse [good first issues](../../labels/good%20first%20issue)
- Or create an issue describing what you want to work on
- Comment "I'd like to work on this" to claim it

### 2. Fork and Branch

```bash
# Fork on GitHub, then:
git clone https://github.com/YOUR_USERNAME/nova.git
cd nova
git remote add upstream https://github.com/nova-lang/nova.git

# Create a branch
git checkout -b feature/your-feature-name
# or
git checkout -b fix/your-bug-fix
```

### 3. Make Your Changes

- Write code
- Write tests
- Run `cargo fmt` and `cargo clippy`
- Update docs if needed

### 4. Commit

We use [Conventional Commits](https://www.conventionalcommits.org/):

```
feat(lexer): add string literal tokenization
fix(parser): handle trailing commas in arrays
docs(spec): clarify type inference rules
test(types): add unit tests for generics
refactor(ir): simplify basic block structure
```

### 5. Push and PR

```bash
git push origin feature/your-feature-name
```

Then open a PR on GitHub. Fill out the template.

### 6. Review

- **Only @pdaxt (BDFL) can approve and merge PRs**
- Other contributors may comment and provide feedback (advisory)
- Address feedback with new commits
- Once @pdaxt approves, the PR will be merged
- Be patient - review may take a few days

## Style Guidelines

### Rust (Bootstrap)

- Run `cargo fmt` before committing
- Run `cargo clippy` and fix warnings
- Follow [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)

```rust
// Good: descriptive names, clear structure
pub fn parse_expression(&mut self) -> Result<Expr, ParseError> {
    let left = self.parse_primary()?;
    self.parse_binary_rhs(left, 0)
}

// Bad: cryptic names, unclear flow
pub fn p_e(&mut self) -> Result<Expr, ParseError> {
    let l = self.p_p()?; self.p_b(l, 0)
}
```

### Nova (Stage 1+)

- Style guide TBD as language develops
- Generally: clarity over brevity

### Documentation

- Use Markdown
- Include code examples
- Keep explanations concise but complete

## Picking Up Work

### Ready-to-Work Issues

Issues labeled `ready` have clear requirements and are ready to be picked up:

| Area | Label | Description |
|------|-------|-------------|
| Bootstrap | `bootstrap` | Rust compiler work |
| Spec | `spec` | Language design |
| Std | `std` | Standard library |
| Tools | `tools` | LSP, formatter, etc. |
| Docs | `docs` | Documentation |

### Difficulty Levels

| Label | Experience Needed |
|-------|------------------|
| `good first issue` | New to project |
| `medium` | Familiar with codebase |
| `hard` | Deep understanding required |

### Current Priorities (Stage 0)

These are the most impactful areas right now:

1. **Lexer completion** — Finish all token types
2. **Parser basics** — Expressions and statements
3. **Type checker** — Basic inference
4. **WASM codegen** — Get hello world running
5. **Spec writing** — Document the language

### How to Claim Work

1. Find an issue you want to work on
2. Comment: "I'd like to work on this"
3. Wait for a maintainer to assign it
4. If no response in 24h, start anyway and mention in PR

### Stuck?

- Ask in the issue comments
- Ask on Discord (coming soon)
- Open a Draft PR with your progress and questions

## Recognition

Contributors are recognized in:
- [CONTRIBUTORS.md](CONTRIBUTORS.md)
- Release notes
- The website (coming soon)

---

**Questions?** Open a discussion or reach out to maintainers.

Thank you for helping build Nova! 🚀
