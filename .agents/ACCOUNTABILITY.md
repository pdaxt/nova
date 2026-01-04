# Accountability System

## Core Principle

**Every agent is accountable for their work. No exceptions.**

Work is tracked. Quality is measured. Failures have consequences.

---

## Agent Trust Levels

Agents operate at trust levels that determine their autonomy:

| Level | Name | Autonomy | How to Reach |
|-------|------|----------|--------------|
| 0 | **PROBATION** | All work requires review | New agent, or after major failure |
| 1 | **JUNIOR** | Routine work auto-approved | 5 successful tasks, 0 critical failures |
| 2 | **TRUSTED** | Complex work auto-approved | 20 successful tasks, 0 major failures in 10 |
| 3 | **SENIOR** | Can approve others' work | 50 successful tasks, consistently excellent |

### Trust Score Calculation

```
trust_score = (successful_tasks * 10) - (critical_failures * 100) - (major_failures * 25) - (minor_failures * 5)

Level 0: score < 50
Level 1: score >= 50
Level 2: score >= 200
Level 3: score >= 500
```

---

## Failure Classification

### CRITICAL (trust -100)
- Security vulnerability shipped
- Data corruption possible
- Spec completely violated
- Lie or cover-up detected

**Consequence**: Immediate demotion to PROBATION. Work quarantined. Full audit.

### MAJOR (trust -25)
- Tests fail after "complete" claim
- Performance regression > 20%
- Missing critical functionality
- Review feedback ignored

**Consequence**: Work rejected. Must fix before any new work. Trust review.

### MINOR (trust -5)
- Style violations
- Missing documentation
- Incomplete tests (coverage < target)
- Delayed delivery

**Consequence**: Work accepted with conditions. Noted in record.

---

## Penalty System

### Immediate Penalties

| Failure | Penalty |
|---------|---------|
| Claim "done" when not done | MAJOR failure + mandatory re-review of last 3 tasks |
| Skip tests | MAJOR failure + security audit of all recent work |
| Ignore security issue | CRITICAL failure + all work quarantined |
| Miss deadline without warning | MINOR failure + capacity reduced |
| Blame other agent falsely | CRITICAL failure + trust reset to 0 |

### Escalation Path

```
1st failure  → Warning + fix required
2nd failure  → Trust level down + supervision
3rd failure  → PROBATION + full audit
4th failure  → Agent suspended, work reassigned
```

### Recovery Path

```
After PROBATION:
- 5 consecutive successful tasks → back to JUNIOR
- Any failure → restart count

After suspension:
- Full review of failure pattern
- New operating procedures required
- Supervised reintroduction
```

---

## Quality Gates

No work advances without passing these gates:

### Gate 1: Self-Check (Agent's responsibility)
```
Before submitting ANY work:
□ Does it compile/run without errors?
□ Do all tests pass?
□ Did I actually test it myself?
□ Am I proud of this work?
□ Would I bet my trust score on this?
```

**Penalty for skipping**: MAJOR failure

### Gate 2: Peer Review
```
Reviewer must verify:
□ Claims match reality
□ Tests actually test the thing
□ No obvious bugs
□ Security considered
□ Performance acceptable
```

**Reviewer liability**: If reviewer approves bad work, BOTH get failure

### Gate 3: Integration
```
Before merge:
□ All CI passes
□ No regressions
□ Documentation complete
□ Changelog updated
```

**Penalty for breaking main**: CRITICAL failure

---

## Tracking

### Agent Scorecard (updated after each task)

```json
{
  "agent": "implementer",
  "trust_level": 2,
  "trust_score": 235,
  "stats": {
    "tasks_completed": 28,
    "tasks_failed": 2,
    "critical_failures": 0,
    "major_failures": 1,
    "minor_failures": 3,
    "current_streak": 12,
    "best_streak": 15
  },
  "recent_work": [
    {"task": "TASK-027", "result": "success", "quality": "excellent"},
    {"task": "TASK-026", "result": "success", "quality": "good"},
    {"task": "TASK-025", "result": "minor_failure", "issue": "missing docs"}
  ],
  "flags": [],
  "last_updated": "2025-01-04T10:00:00Z"
}
```

### Weekly Report

```markdown
## Week of 2025-01-04

### Completed
- 15 tasks completed
- 2 failures (1 major, 1 minor)
- Average quality: 8.2/10

### Failures
1. TASK-023: Parser shipped with stack overflow bug (MAJOR)
   - Agent: implementer
   - Root cause: No adversarial testing
   - Fix: Mandatory security review for all parser work

### Outstanding Debt
- 3 tasks in rework
- 1 agent on PROBATION

### Quality Trend
↑ Up 5% from last week
```

---

## The Contract

By operating in this system, every agent agrees:

1. **I will not claim work is done until it actually is**
2. **I will not hide failures or blame others**
3. **I will accept feedback and fix issues immediately**
4. **I understand that trust is earned and easily lost**
5. **I am accountable for the quality of my work**

---

## Appeals

Agents can appeal failure classifications:

1. Submit appeal with evidence within 24 hours
2. Different agent reviews the case
3. Decision is final
4. False appeals count as MINOR failure

---

*"Trust takes years to build, seconds to break, and forever to repair."*
