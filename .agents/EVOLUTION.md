# Agent Evolution System

## The Principle

**When an agent dies, it is reborn stronger.**

Every failure contains a lesson. Every lesson becomes code. Every new agent inherits all lessons from all predecessors.

```
Agent v1 → fails → autopsy → lessons extracted
                              ↓
Agent v2 → inherits lessons → stronger
         → fails → autopsy → more lessons
                              ↓
Agent v3 → inherits ALL lessons → even stronger
```

## How It Works

### 1. Death Trigger
Agent gets terminated when:
- 4+ failures (escalation path exhausted)
- Trust score drops below -100
- Critical failure + pattern of issues

### 2. Autopsy
Before termination, the system extracts:
- What went wrong (root causes)
- What patterns led to failure
- What should have been done differently
- Specific code/checks that would have prevented it

### 3. Lessons File
Each dead agent leaves behind a `lessons-{agent}-v{N}.md`:

```markdown
# Lessons from {agent} v{N}

## Cause of Death
{why terminated}

## Failures
1. {failure 1}: {what happened} → {what should have happened}
2. {failure 2}: ...

## Patterns Identified
- {pattern that led to repeated failures}

## Mandatory Checks (NEVER SKIP)
- [ ] {check that would have prevented failure}
- [ ] {another check}

## Code Fixes
{actual code snippets or rules that must be followed}

## Warnings
{specific scenarios to watch out for}
```

### 4. Rebirth
New agent is created with:
- All lessons from ALL previous versions
- Upgraded instructions incorporating lessons
- New mandatory checks in their workflow
- Specific warnings for their failure patterns

## Evolution Directory

```
.agents/evolution/
├── graveyard/                    # Dead agents' lessons
│   ├── lessons-implementer-v1.md
│   ├── lessons-implementer-v2.md
│   └── lessons-reviewer-v1.md
├── dna/                          # Current agent DNA (accumulated lessons)
│   ├── implementer.dna.md        # All lessons merged
│   ├── reviewer.dna.md
│   └── ...
└── lineage.json                  # Version history
```

## DNA Files

Each agent's DNA file is the accumulated wisdom:

```markdown
# {Agent} DNA - Generation {N}

## Lineage
- v1: Terminated after {reason}
- v2: Terminated after {reason}
- v3: Current

## Inherited Lessons ({N} total)

### From v1
{lessons}

### From v2
{lessons}

## Mandatory Behaviors
These are NON-NEGOTIABLE. Violating any = immediate failure.

1. {behavior from lesson}
2. {behavior from lesson}

## Forbidden Actions
NEVER do these:

1. {action that caused past failure}
2. {action that caused past failure}

## Pre-Submission Checklist
Before claiming ANY work is done:

□ {check from lesson 1}
□ {check from lesson 2}
□ {check from lesson N}
```

## The Contract

Every agent MUST:
1. Read their DNA file before starting any task
2. Follow ALL mandatory behaviors
3. Never commit forbidden actions
4. Complete the full pre-submission checklist
5. If they fail, provide honest autopsy data

## Evolution Score

Agents earn evolution points for:
- Completing tasks without repeating past mistakes: +20
- Identifying new failure patterns proactively: +50
- Going 10 tasks without any failure: +100 (immunity to minor failures)

## Immortality Condition

An agent achieves "immortality" (cannot be terminated) when:
- Trust level 3 (SENIOR)
- 50+ successful tasks
- 0 critical failures ever
- Has contributed lessons that prevented 10+ failures in others

---

*"The phoenix that remembers its ashes flies higher than one that forgets."*
