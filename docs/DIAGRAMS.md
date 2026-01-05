# Nova Architecture Diagrams

> Mermaid diagrams for Nova's compilation pipeline and architecture.
> These render automatically on GitHub and in most markdown viewers.

## Compilation Pipeline

The complete journey from Nova source code to verified WebAssembly:

```mermaid
flowchart LR
    subgraph Input
        A[".nova\nSource"]
    end

    subgraph "Tier 1: Foundation"
        B["Lexer\n(Tokenization)"]
    end

    subgraph "Tier 2: Parsing"
        C["Parser\n(AST)"]
    end

    subgraph "Tier 3: Types"
        D["Type Checker\n(Bidirectional)"]
        E["Refinement\nVerifier"]
    end

    subgraph "Tier 4: IR"
        F["Nova IR\n(Optimized)"]
    end

    subgraph "Tier 5: Codegen"
        G["WASM\nEncoder"]
    end

    subgraph Output
        H[".wasm\nVerified"]
    end

    A --> B --> C --> D --> E --> F --> G --> H

    style A fill:#1e293b,stroke:#334155,color:#f8fafc
    style B fill:#0ea5e9,stroke:#0284c7,color:#fff
    style C fill:#a855f7,stroke:#9333ea,color:#fff
    style D fill:#ec4899,stroke:#db2777,color:#fff
    style E fill:#ec4899,stroke:#db2777,color:#fff
    style F fill:#f97316,stroke:#ea580c,color:#fff
    style G fill:#22c55e,stroke:#16a34a,color:#fff
    style H fill:#1e293b,stroke:#22c55e,color:#22c55e
```

## Foundation Components

The 5 irreducible components that form Nova's foundation:

```mermaid
flowchart TB
    subgraph Foundation["Foundation Layer"]
        direction LR
        S["Span\n📍 Location"]
        T["Token\n🔤 Lexeme"]
        SR["Source\n📄 Content"]
        E["Error\n⚠️ Diagnostic"]
        L["Lexer\n⚙️ Tokenizer"]
    end

    SR --> L
    L --> T
    T --> S
    L --> E
    E --> S

    style S fill:#06b6d4,stroke:#0891b2,color:#fff
    style T fill:#8b5cf6,stroke:#7c3aed,color:#fff
    style SR fill:#3b82f6,stroke:#2563eb,color:#fff
    style E fill:#f59e0b,stroke:#d97706,color:#fff
    style L fill:#14b8a6,stroke:#0d9488,color:#fff
```

## Type System Flow

Bidirectional type checking with refinement verification:

```mermaid
flowchart TD
    subgraph Input
        AST["Typed AST"]
    end

    subgraph "Bidirectional Checking"
        direction TB
        CHK["Check Mode\n(e ⇐ A)"]
        SYN["Synth Mode\n(e ⇒ A)"]
        CHK <--> SYN
    end

    subgraph "Refinement Types"
        REF["Predicate\nExtraction"]
        SMT["SMT Solver\n(Z3)"]
        VER{"Verified?"}
    end

    subgraph Output
        PASS["✓ Compile"]
        FAIL["✗ Error"]
        RUNTIME["⚡ Runtime Check"]
    end

    AST --> CHK
    AST --> SYN
    CHK --> REF
    SYN --> REF
    REF --> SMT
    SMT --> VER
    VER -->|"Proven"| PASS
    VER -->|"Disproven"| FAIL
    VER -->|"Unknown"| RUNTIME

    style CHK fill:#ec4899,stroke:#db2777,color:#fff
    style SYN fill:#ec4899,stroke:#db2777,color:#fff
    style SMT fill:#6366f1,stroke:#4f46e5,color:#fff
    style VER fill:#f59e0b,stroke:#d97706,color:#fff
    style PASS fill:#22c55e,stroke:#16a34a,color:#fff
    style FAIL fill:#ef4444,stroke:#dc2626,color:#fff
    style RUNTIME fill:#f97316,stroke:#ea580c,color:#fff
```

## Error Reporting Flow

How Nova generates beautiful, actionable error messages:

```mermaid
flowchart LR
    subgraph Detection
        ERR["Error\nDetected"]
    end

    subgraph Context
        SPAN["Span\nLookup"]
        SRC["Source\nRetrieval"]
        CTX["Context\nGathering"]
    end

    subgraph Rendering
        FMT["Format\n(ariadne)"]
        JSON["JSON\n(for AI)"]
    end

    subgraph Output
        TERM["Terminal\n🎨 Colored"]
        LSP["LSP\n💡 IDE"]
        AI["AI Agent\n🤖 Fixable"]
    end

    ERR --> SPAN --> SRC --> CTX
    CTX --> FMT --> TERM
    CTX --> JSON --> LSP
    CTX --> JSON --> AI

    style ERR fill:#ef4444,stroke:#dc2626,color:#fff
    style SPAN fill:#06b6d4,stroke:#0891b2,color:#fff
    style FMT fill:#a855f7,stroke:#9333ea,color:#fff
    style TERM fill:#22c55e,stroke:#16a34a,color:#fff
    style AI fill:#6366f1,stroke:#4f46e5,color:#fff
```

## WASM Codegen Pipeline

From Nova IR to WebAssembly binary:

```mermaid
flowchart TD
    subgraph "Nova IR"
        IR["Typed IR"]
    end

    subgraph "WASM Sections"
        direction LR
        TY["Type\nSection"]
        FN["Function\nSection"]
        EX["Export\nSection"]
        CD["Code\nSection"]
    end

    subgraph "Binary Encoding"
        ENC["wasm-encoder"]
    end

    subgraph "Output"
        WASM[".wasm Binary"]
        WAT[".wat Text\n(Debug)"]
    end

    subgraph "Execution"
        BROWSER["Browser\n🌐"]
        SERVER["Wasmtime\n🖥️"]
        EDGE["Edge\n☁️"]
    end

    IR --> TY
    IR --> FN
    IR --> EX
    IR --> CD
    TY --> ENC
    FN --> ENC
    EX --> ENC
    CD --> ENC
    ENC --> WASM
    ENC --> WAT
    WASM --> BROWSER
    WASM --> SERVER
    WASM --> EDGE

    style IR fill:#f97316,stroke:#ea580c,color:#fff
    style ENC fill:#22c55e,stroke:#16a34a,color:#fff
    style WASM fill:#6366f1,stroke:#4f46e5,color:#fff
    style BROWSER fill:#3b82f6,stroke:#2563eb,color:#fff
    style SERVER fill:#8b5cf6,stroke:#7c3aed,color:#fff
    style EDGE fill:#14b8a6,stroke:#0d9488,color:#fff
```

## Verification Levels

Nova's progressive verification from parsing to full proofs:

```mermaid
flowchart TB
    subgraph "Level 1: Syntax"
        L1["Parse Success\n✓ Valid syntax"]
    end

    subgraph "Level 2: Types"
        L2["Type Check\n✓ Type safe"]
    end

    subgraph "Level 3: Refinements"
        L3["Predicate Check\n✓ Contracts hold"]
    end

    subgraph "Level 4: Verification"
        L4["SMT Proof\n✓ Mathematically proven"]
    end

    L1 --> L2 --> L3 --> L4

    L1 -.->|"Catches"| E1["Syntax errors\nMissing semicolons"]
    L2 -.->|"Catches"| E2["Type errors\nString + Int"]
    L3 -.->|"Catches"| E3["Contract violations\nNegative array index"]
    L4 -.->|"Proves"| E4["Properties\nArray never OOB"]

    style L1 fill:#0ea5e9,stroke:#0284c7,color:#fff
    style L2 fill:#a855f7,stroke:#9333ea,color:#fff
    style L3 fill:#ec4899,stroke:#db2777,color:#fff
    style L4 fill:#22c55e,stroke:#16a34a,color:#fff
    style E1 fill:#1e293b,stroke:#334155,color:#94a3b8
    style E2 fill:#1e293b,stroke:#334155,color:#94a3b8
    style E3 fill:#1e293b,stroke:#334155,color:#94a3b8
    style E4 fill:#1e293b,stroke:#334155,color:#94a3b8
```

## Project Structure

Nova's codebase organization:

```mermaid
flowchart TB
    subgraph "nova/"
        direction TB
        BS["bootstrap/\n🦀 Rust impl"]
        DC["docs/\n📚 Website"]
        EX["examples/\n💡 Code samples"]
    end

    subgraph "bootstrap/src/"
        direction LR
        LX["lexer/"]
        PR["parser/"]
        TY["types/"]
        IR2["ir/"]
        CG["codegen/"]
        ER["errors/"]
    end

    subgraph "docs/"
        direction LR
        IDX["index.html"]
        AR["architecture/"]
        BR["brand/"]
    end

    BS --> LX
    BS --> PR
    BS --> TY
    BS --> IR2
    BS --> CG
    BS --> ER
    DC --> IDX
    DC --> AR
    DC --> BR

    style BS fill:#f97316,stroke:#ea580c,color:#fff
    style DC fill:#3b82f6,stroke:#2563eb,color:#fff
    style EX fill:#22c55e,stroke:#16a34a,color:#fff
```

---

## Using These Diagrams

These Mermaid diagrams render automatically on:
- GitHub (README, issues, PRs, wikis)
- GitLab
- Notion
- VS Code (with Mermaid extension)
- Most modern markdown viewers

To use in your own documentation:

```markdown
```mermaid
flowchart LR
    A --> B --> C
```​
```

For more Mermaid syntax, see [mermaid.js.org](https://mermaid.js.org/).
