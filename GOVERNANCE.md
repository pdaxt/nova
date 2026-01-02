# Nova Governance

## Project Leadership

**Benevolent Dictator for Life (BDFL):** Pran (@pran)

All final decisions on direction, merges, releases, and project governance rest with the BDFL.

## Decision Making

### What the BDFL Decides
- All PR merges to `main`
- Release timing and versioning
- Roadmap priorities
- Breaking changes
- New maintainer appointments
- License changes
- Project direction

### What Contributors Can Do
- Open issues and PRs
- Discuss in issues and PRs
- Review code (advisory only)
- Propose features and changes
- Write documentation
- Help other contributors

## Pull Request Process

1. **All PRs require BDFL approval** before merge
2. Contributors may review and comment, but only BDFL can approve
3. No PR will be merged without explicit BDFL sign-off
4. Force pushes to `main` are prohibited (except by BDFL)

## Branch Protection

The `main` branch is protected:
- Requires BDFL approval for all merges
- No direct pushes (all changes via PR)
- No force pushes
- Status checks must pass

## Roles

### BDFL (Pran)
- Full admin access
- Sole merge authority
- Final say on all decisions

### Maintainers (Future)
- May be appointed by BDFL
- Can triage issues
- Can review PRs (advisory)
- Cannot merge without BDFL approval

### Contributors
- Can submit PRs
- Can open issues
- Can participate in discussions
- No merge rights

## Forking

This is open source. Anyone can fork and create their own version. However, only the official `nova-lang/nova` repository is governed by this document.

## Changes to Governance

Only the BDFL can modify this governance document.

---

*This governance model is intentionally simple. As the project grows, it may evolve.*
