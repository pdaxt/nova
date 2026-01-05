<p align="center">
  <img src="docs/brand/nova-logo.svg" alt="Nova" width="200" height="200">
</p>

<h1 align="center">Nova</h1>

<p align="center">
  <strong>Code that proves itself.</strong>
</p>

[![CI](https://github.com/pdaxt/nova/actions/workflows/ci.yml/badge.svg)](https://github.com/pdaxt/nova/actions/workflows/ci.yml)
[![Security](https://img.shields.io/badge/security-adversarial%20tested-brightgreen)](bootstrap/src/token_attack.rs)
[![Tests](https://img.shields.io/badge/tests-91%20passing-brightgreen)](bootstrap/)
[![Token](https://img.shields.io/badge/Token-12%20bytes-blue)](docs/decisions/ADR-004-token-size.md)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue)](LICENSE-MIT)

**A programming language designed for the AI age.**

*Verified. Efficient. Open.*

---

## Mission

We're building a new programming language from scratch—not because the world needs another language, but because the world needs a *different kind* of language.

Current languages were designed for humans to write and machines to execute. Nova is designed for **AI to write and machines to verify**. The result: code that is mathematically proven correct, dramatically more efficient, and impossible to exploit.

### Why Nova?

| Today's Reality | Nova's Vision |
|-----------------|---------------|
| Runtime errors crash production | Errors are impossible by construction |
| Security vulnerabilities everywhere | Capabilities prevent unauthorized access |
| 10x abstraction overhead | Direct hardware mapping |
| AI generates buggy code | AI generates verified code |
| Humans debug machine output | Machines prove human intent |

## Core Principles

1. **Correctness is not optional.** Every program either proves itself correct or doesn't compile.

2. **Capabilities, not permissions.** Code can only access what it's explicitly given. No ambient authority.

3. **AI-native.** Optimized for LLMs to generate: minimal syntax, zero ambiguity, maximum verifiability.

4. **Self-hosting is day one.** The compiler is written in Nova. We eat our own dogfood immediately.

5. **Radical openness.** Everything is open source. The spec, the compiler, the tools, the governance.

## Quick Example

```nova
// A function that sorts a list
// The compiler PROVES these properties:
//   - Output has same length as input
//   - Output is sorted
//   - Output is a permutation of input

fn sort(input: Vec<i32>) -> Vec<i32>
where
    ensures output.len() == input.len(),
    ensures output.is_sorted(),
    ensures output.is_permutation_of(input),
{
    // Implementation here
    // If it doesn't satisfy the properties, it won't compile
}
```

## Compiler Pipeline

```mermaid
flowchart LR
    A[".nova"] --> B["Lexer"]
    B --> C["Parser"]
    C --> D["Type Checker"]
    D --> E["Verifier"]
    E --> F["IR"]
    F --> G["Codegen"]
    G --> H[".wasm"]

    style A fill:#1e293b,stroke:#334155,color:#f8fafc
    style B fill:#0ea5e9,stroke:#0284c7,color:#fff
    style C fill:#a855f7,stroke:#9333ea,color:#fff
    style D fill:#ec4899,stroke:#db2777,color:#fff
    style E fill:#ec4899,stroke:#db2777,color:#fff
    style F fill:#f97316,stroke:#ea580c,color:#fff
    style G fill:#22c55e,stroke:#16a34a,color:#fff
    style H fill:#1e293b,stroke:#22c55e,color:#22c55e
```

> See [docs/DIAGRAMS.md](docs/DIAGRAMS.md) for more architecture diagrams.

## Project Status

🚧 **Stage 0: Bootstrap** — Building the minimal Rust compiler that will compile the first Nova compiler.

### Bootstrap Compiler Progress

| Component | Status | Tests | Notes |
|-----------|--------|-------|-------|
| **Lexer** | ✅ Complete | 25 | Full tokenization with span tracking |
| **Parser** | ✅ Complete | 30 | Pratt parsing, full expression support |
| **Span** | ✅ Hardened | 15 | 8-byte struct, [adversarial tested](bootstrap/src/span_attack.rs) |
| **Token** | ✅ Hardened | 25 | 12-byte struct, [adversarial tested](bootstrap/src/token_attack.rs) |
| **Types** | 🚧 In Progress | 1 | Basic type checking |
| **IR** | 🚧 In Progress | - | SSA-based intermediate representation |
| **Codegen** | 🚧 In Progress | - | WebAssembly output |

**91 tests passing** • **40 adversarial security tests** • **Compile-time size guarantees**

See [bootstrap/README.md](bootstrap/README.md) for contributor guide.

See [ROADMAP.md](ROADMAP.md) for the full plan.

## Documentation

| Document | Description |
|----------|-------------|
| [ROADMAP.md](ROADMAP.md) | Development stages and milestones |
| [COMPONENTS.md](COMPONENTS.md) | 43-component architecture with QA criteria |
| [FOUNDATION.md](FOUNDATION.md) | The 5 irreducible foundation components |
| [DECISIONS.md](DECISIONS.md) | Architectural decisions with rationale |
| [CONTRIBUTING.md](CONTRIBUTING.md) | How to contribute |
| [GOVERNANCE.md](GOVERNANCE.md) | BDFL model and approval process |
| [docs/DIAGRAMS.md](docs/DIAGRAMS.md) | Architecture diagrams (Mermaid) |
| [docs/brand/BRAND.md](docs/brand/BRAND.md) | Brand guidelines and assets |

## Get Involved

We're looking for contributors! Nova is designed to be built by a community.

### Easy First Contributions
- [ ] [#1 Lexer: Basic tokenization](../../issues/1)
- [ ] [#2 Parser: Expression parsing](../../issues/2)
- [ ] [#3 Docs: Language syntax spec](../../issues/3)

### More Involved
- [ ] [#4 Type system: Inference engine](../../issues/4)
- [ ] [#5 Codegen: WASM output](../../issues/5)

See [CONTRIBUTING.md](CONTRIBUTING.md) for how to get started.

## Architecture

```
nova/
├── bootstrap/          # Rust bootstrap compiler (temporary)
├── stage1/             # Nova compiler written in Nova
├── std/                # Standard library
├── spec/               # Language specification
├── tools/              # LSP, formatter, etc.
└── examples/           # Example programs
```

## Building

### Prerequisites
- Rust 1.75+ (for bootstrap only)
- LLVM 17+ (for native codegen)

### Build the bootstrap compiler
```bash
cd bootstrap
cargo build --release
```

### Compile a Nova program
```bash
./target/release/nova compile examples/hello.nova -o hello.wasm
```

### Run tests
```bash
cargo test
```

## Governance

Nova uses a **BDFL (Benevolent Dictator for Life)** governance model.

- All PRs require approval from @pdaxt before merge
- See [GOVERNANCE.md](GOVERNANCE.md) for details
- Contributors can open issues, PRs, and participate in discussions

## Community

- **Discord**: [Join our Discord](https://discord.gg/nova-lang) (coming soon)
- **Discussions**: [GitHub Discussions](../../discussions)
- **Twitter/X**: [@nova_lang](https://twitter.com/nova_lang) (coming soon)

## License

Nova is dual-licensed under MIT and Apache 2.0. See [LICENSE-MIT](LICENSE-MIT) and [LICENSE-APACHE](LICENSE-APACHE).

## Acknowledgments

Nova stands on the shoulders of giants:
- [Rust](https://rust-lang.org) — Memory safety without GC
- [Lean](https://lean-lang.org) — Dependent types and proofs
- [Zig](https://ziglang.org) — Simplicity and control
- [LLVM](https://llvm.org) — Production-grade codegen

---

**Nova: Code that proves itself.**
