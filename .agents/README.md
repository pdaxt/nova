# Nova Multi-Agent Development System

A file-based orchestration system for developing Nova using specialized Claude Code agents.

## Quick Start

```bash
# Check agent status
./.agents/coordinator.sh status

# Run an architect task
./.agents/run.sh architect "Design the type inference engine"

# Run from inbox
./.agents/run.sh implementer --from-inbox

# Continue previous session
./.agents/run.sh reviewer --continue

# Watch mode (continuous)
./.agents/coordinator.sh watch
```

## Agents

| Agent | Role | Input | Output |
|-------|------|-------|--------|
| **architect** | Design components, write specs | Issues, requirements | Specs, ADRs, tasks |
| **implementer** | Write code from specs | Specs, tasks | Code, tests |
| **reviewer** | Review code quality | Code, specs | Review report |
| **tester** | Write and run tests | Code | Test report, coverage |
| **security** | Security audit | Code | Security report |
| **perf** | Performance analysis | Code | Benchmarks, profile |
| **docs** | Write documentation | Code, specs | Docs, guides |
| **refactor** | Improve code structure | Code smells | Refactored code |
| **release** | Prepare releases | All artifacts | Release package |

## Pipeline

```
Design → Implement → Review → Test → Security → Perf → Docs → Release
  ↑         ↓          ↓        ↓
  └─────────┴──────────┴────────┘  (feedback loops)
```

## Directory Structure

```
.agents/
├── agents/              # Agent persona definitions
│   ├── architect.md
│   ├── implementer.md
│   ├── reviewer.md
│   ├── tester.md
│   ├── security.md
│   ├── perf.md
│   ├── docs.md
│   ├── refactor.md
│   └── release.md
├── shared/              # Shared context
│   ├── context/         # Project context
│   ├── decisions/       # ADRs
│   └── specs/           # Specifications
├── inbox/               # Pending work per agent
│   └── {agent}/
├── outbox/              # Completed work per agent
│   └── {agent}/
├── examples/            # Example task files
├── logs/                # Execution logs
├── coordinator.sh       # Main orchestrator
├── run.sh              # Agent runner
└── README.md           # This file
```

## Communication Protocol

Agents communicate via JSON files:

### Task File (input)
```json
{
  "id": "TASK-003",
  "type": "implement",
  "component": "lexer",
  "spec": "specs/lexer.md",
  "priority": "high"
}
```

### Result File (output)
```json
{
  "agent": "implementer",
  "task_id": "TASK-003",
  "status": "success",
  "artifacts": ["src/lexer.rs"],
  "next_agent": "reviewer"
}
```

## Private vs Public

- **Private workspace**: `~/.nova-agents/` - Intermediate work
- **Public repo**: `./` - Only approved, reviewed code

Agents work in private space. Only code that passes the full pipeline gets pushed to the public repo.

## Adding Custom Agents

1. Create `.agents/agents/myagent.md` with role definition
2. Add inbox/outbox directories
3. Update routing in `coordinator.sh`
4. Document in this README

## Examples

See `.agents/examples/` for:
- `task-implement-lexer.json` - Sample task file
- `result-implement-lexer.json` - Sample result file
- `review-feedback.json` - Sample review feedback
