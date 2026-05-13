---
wave: columbus-south-terminal-contact-proof
pulse: 05
date: 2026-05-13
status: planned
depends_on: [pulse-04]
governing_roles:
  - optimization-methodologist
  - citation-auditor
  - scope-keeper
---

# Pulse 05 - Wave Close

## Mission

Close the Columbus South proof pilot by reconciling proof outcomes, residual
source-needed rows, blocker counts, manifests, and gates.

## Deliverables

- [ ] Write `waves/2026-05-13-columbus-south-terminal-contact-proof/CLOSE.md`.
- [ ] Update `waves/PHASES.md`, `WAVE.md`, and pulse statuses.
- [ ] Summarize each Columbus route as source-backed, source-needed, blocked, or
  rejected.
- [ ] Name any scenario-ready rows or explicitly record that none exist.
- [ ] Run final gates and commit.

## Expected Gates

- `route t4-terminal-contact-source-plan --gate`
- `route t4-terminal-contact-evidence --gate`
- `route t4-terminal-scenario-readiness --gate`
- `route optimizer-constraint-ledger --gate`
- `route optimizer-constraint-budget --gate`
- `route tier-optimize --all-tiers --gate`
- `route optimizer-manifest --gate`
- `route release-manifest --gate`
- `cargo test -p route`
- `scripts/check-mileposts.ps1 -SkipTests`

## Non-Goals

- Do not close without naming residual proof blockers.
- Do not require live source access to close.
