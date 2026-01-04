# Nova Multi-Agent Development System

## Overview

A file-based architecture where specialized Claude Code agents collaborate through a shared filesystem. Each agent reads from its inbox, processes work, and writes to the next agent's inbox.

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                        NOVA AGENT ORCHESTRATION                             │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│   ┌──────────┐    ┌──────────┐    ┌──────────┐    ┌──────────┐             │
│   │ ARCHITECT│───▶│IMPLEMENTER───▶│ REVIEWER │───▶│  TESTER  │             │
│   └──────────┘    └──────────┘    └──────────┘    └──────────┘             │
│        │               │               │               │                    │
│        ▼               ▼               ▼               ▼                    │
│   specs/*.md      src/*.rs       reviews/*.md    tests/*.rs                │
│                                                                             │
│   ┌──────────┐    ┌──────────┐    ┌──────────┐    ┌──────────┐             │
│   │ SECURITY │    │  PERF    │    │   DOCS   │    │ RELEASE  │             │
│   │ AUDITOR  │    │ ANALYST  │    │  WRITER  │    │ MANAGER  │             │
│   └──────────┘    └──────────┘    └──────────┘    └──────────┘             │
│        │               │               │               │                    │
│        ▼               ▼               ▼               ▼                    │
│   audits/*.md    perf/*.json     docs/*.md      releases/*.md              │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

## Directory Structure

```
.agents/
├── ARCHITECTURE.md          # This file
├── coordinator.sh           # Main orchestrator
├── status.json              # Global state
│
├── agents/                  # Agent definitions
│   ├── architect.md
│   ├── implementer.md
│   ├── reviewer.md
│   ├── tester.md
│   ├── security-auditor.md
│   ├── perf-analyst.md
│   ├── documenter.md
│   ├── refactorer.md
│   └── release-manager.md
│
├── inbox/                   # Per-agent inboxes
│   ├── architect/
│   ├── implementer/
│   ├── reviewer/
│   ├── tester/
│   ├── security/
│   ├── perf/
│   ├── docs/
│   └── release/
│
├── outbox/                  # Completed work
│   ├── specs/              # Architecture specs
│   ├── implementations/    # Code ready for review
│   ├── reviews/            # Review feedback
│   ├── tests/              # Test results
│   ├── audits/             # Security audits
│   ├── benchmarks/         # Performance data
│   └── releases/           # Release artifacts
│
├── shared/                  # Shared knowledge
│   ├── decisions/          # ADRs
│   ├── patterns/           # Reusable patterns
│   ├── checklists/         # Quality checklists
│   └── context/            # Project context
│
└── logs/                    # Execution logs
    ├── 2024-01-03/
    └── ...
```

## Agent Types

### 1. ARCHITECT
**Role**: Design system components, make architectural decisions
**Reads**: Requirements, issues, feedback
**Writes**: Specs, ADRs, component designs
**Triggers**: IMPLEMENTER

### 2. IMPLEMENTER
**Role**: Write code based on specifications
**Reads**: Specs from ARCHITECT
**Writes**: Source code, unit tests
**Triggers**: REVIEWER

### 3. REVIEWER
**Role**: Code review for quality, correctness, style
**Reads**: Code from IMPLEMENTER
**Writes**: Review comments, approval/rejection
**Triggers**: TESTER (if approved) or IMPLEMENTER (if rejected)

### 4. TESTER
**Role**: Comprehensive testing
**Reads**: Code + specs
**Writes**: Test results, coverage reports
**Triggers**: SECURITY (if passed) or IMPLEMENTER (if failed)

### 5. SECURITY AUDITOR
**Role**: Security analysis, adversarial testing
**Reads**: Code + tests
**Writes**: Security audit, vulnerability report, attack tests
**Triggers**: PERF (if passed) or IMPLEMENTER (if critical issues)

### 6. PERF ANALYST
**Role**: Performance profiling and optimization suggestions
**Reads**: Code + benchmarks
**Writes**: Performance report, optimization recommendations
**Triggers**: DOCS or REFACTORER

### 7. DOCUMENTER
**Role**: Write documentation
**Reads**: Code + specs + tests
**Writes**: API docs, guides, examples
**Triggers**: RELEASE

### 8. REFACTORER
**Role**: Improve code quality without changing behavior
**Reads**: Code + perf report + code smells
**Writes**: Refactored code
**Triggers**: REVIEWER (for re-review)

### 9. RELEASE MANAGER
**Role**: Prepare releases
**Reads**: All approved artifacts
**Writes**: Changelog, version bumps, release notes
**Triggers**: None (end of pipeline)

## Communication Protocol

### Task File Format
```json
{
  "id": "TASK-001",
  "type": "implement",
  "priority": "high",
  "created": "2024-01-03T12:00:00Z",
  "from": "architect",
  "to": "implementer",
  "spec": "specs/lexer-v2.md",
  "context": {
    "issue": "#42",
    "adr": "ADR-006",
    "dependencies": ["TASK-000"]
  },
  "status": "pending"
}
```

### Result File Format
```json
{
  "task_id": "TASK-001",
  "agent": "implementer",
  "completed": "2024-01-03T14:00:00Z",
  "status": "success",
  "artifacts": [
    "src/lexer_v2.rs",
    "tests/lexer_v2_tests.rs"
  ],
  "next": {
    "agent": "reviewer",
    "task": "TASK-001-review"
  },
  "notes": "Implemented per spec. Added 15 unit tests."
}
```

## Pipeline Stages

```
┌─────────────────────────────────────────────────────────────────┐
│                    DEVELOPMENT PIPELINE                         │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│  STAGE 1: DESIGN                                               │
│  ┌─────────┐                                                   │
│  │ARCHITECT│──▶ spec.md + adr.md                              │
│  └─────────┘                                                   │
│       │                                                         │
│       ▼                                                         │
│  STAGE 2: IMPLEMENT                                            │
│  ┌───────────┐                                                 │
│  │IMPLEMENTER│──▶ src/*.rs + tests/*.rs                       │
│  └───────────┘                                                 │
│       │                                                         │
│       ▼                                                         │
│  STAGE 3: REVIEW (loop until approved)                         │
│  ┌────────┐     ┌───────────┐                                  │
│  │REVIEWER│◀───▶│IMPLEMENTER│                                  │
│  └────────┘     └───────────┘                                  │
│       │                                                         │
│       ▼                                                         │
│  STAGE 4: TEST                                                 │
│  ┌──────┐                                                      │
│  │TESTER│──▶ test_results.json + coverage.json                │
│  └──────┘                                                      │
│       │                                                         │
│       ▼                                                         │
│  STAGE 5: SECURITY                                             │
│  ┌────────┐                                                    │
│  │SECURITY│──▶ audit.md + *_attack.rs                         │
│  └────────┘                                                    │
│       │                                                         │
│       ▼                                                         │
│  STAGE 6: PERFORMANCE                                          │
│  ┌────┐                                                        │
│  │PERF│──▶ benchmarks.json + recommendations.md               │
│  └────┘                                                        │
│       │                                                         │
│       ▼                                                         │
│  STAGE 7: DOCUMENT                                             │
│  ┌────┐                                                        │
│  │DOCS│──▶ api.md + guide.md + examples/                      │
│  └────┘                                                        │
│       │                                                         │
│       ▼                                                         │
│  STAGE 8: RELEASE                                              │
│  ┌───────┐                                                     │
│  │RELEASE│──▶ CHANGELOG.md + version bump + tag               │
│  └───────┘                                                     │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

## Specialized Sub-Agents

For complex components like a programming language, we need domain experts:

### Language Design Agents
- **SYNTAX-DESIGNER**: Designs language syntax, grammar
- **TYPE-THEORIST**: Designs type system, inference rules
- **SEMANTICS-EXPERT**: Defines operational semantics

### Compiler Agents
- **LEXER-EXPERT**: Tokenization specialist
- **PARSER-EXPERT**: Parsing, AST design
- **TYPE-CHECKER**: Type checking, inference
- **IR-DESIGNER**: Intermediate representation
- **OPTIMIZER**: Optimization passes
- **CODEGEN-EXPERT**: Code generation (WASM, LLVM)

### Quality Agents
- **FUZZER**: Generates random inputs to find bugs
- **PROPERTY-TESTER**: QuickCheck-style testing
- **MUTATION-TESTER**: Mutation testing for test quality

## Execution Model

### 1. Sequential Mode
```bash
./agents/run.sh architect specs/new-feature.md
./agents/run.sh implementer --from architect
./agents/run.sh reviewer --from implementer
./agents/run.sh tester --from reviewer
```

### 2. Watch Mode (Continuous)
```bash
./agents/watch.sh  # Watches inboxes, triggers agents automatically
```

### 3. Parallel Mode
```bash
./agents/parallel.sh security perf docs  # Run independent agents in parallel
```

## State Machine

```
                    ┌─────────────────┐
                    │     START       │
                    └────────┬────────┘
                             │
                             ▼
                    ┌─────────────────┐
                    │    DESIGNING    │◀──────────────────┐
                    └────────┬────────┘                   │
                             │                            │
                             ▼                            │
                    ┌─────────────────┐                   │
                    │  IMPLEMENTING   │◀─────────┐        │
                    └────────┬────────┘          │        │
                             │                   │        │
                             ▼                   │        │
                    ┌─────────────────┐          │        │
                    │   REVIEWING     │──────────┘        │
                    └────────┬────────┘  (rejected)       │
                             │ (approved)                 │
                             ▼                            │
                    ┌─────────────────┐                   │
                    │    TESTING      │───────────────────┘
                    └────────┬────────┘  (major issues)
                             │ (passed)
                             ▼
                    ┌─────────────────┐
                    │   AUDITING      │
                    └────────┬────────┘
                             │
                             ▼
                    ┌─────────────────┐
                    │   PROFILING     │
                    └────────┬────────┘
                             │
                             ▼
                    ┌─────────────────┐
                    │  DOCUMENTING    │
                    └────────┬────────┘
                             │
                             ▼
                    ┌─────────────────┐
                    │   RELEASING     │
                    └────────┬────────┘
                             │
                             ▼
                    ┌─────────────────┐
                    │      DONE       │
                    └─────────────────┘
```

## Benefits

1. **Separation of Concerns**: Each agent is an expert in its domain
2. **Audit Trail**: Every decision and change is documented
3. **Parallelization**: Independent agents can run concurrently
4. **Reproducibility**: File-based state enables replay
5. **Human Override**: Humans can inject files at any stage
6. **Cost Efficiency**: Use cheaper models for simpler tasks
7. **Quality Gates**: Each stage must pass before proceeding
