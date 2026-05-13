---
wave: constraint-ledger-blocker-burndown
pulse: 06
date: 2026-05-13
status: planned
depends_on: [pulse-05]
governing_roles:
  - optimization-methodologist
  - scope-keeper
  - numeracy-checker
---

# Pulse 06 - Wave Close

## Mission

Close the blocker-burndown wave by reconciling blocker counts, carried holds,
spec updates, manifests, and release gates.

## Deliverables

- [ ] Write `waves/2026-05-13-constraint-ledger-blocker-burndown/CLOSE.md`.
- [ ] Update `waves/PHASES.md` and this wave card.
- [ ] Summarize before/after blocker counts by tier and constraint class.
- [ ] Update `docs/SPEC_INDEX.md` if new specs or action ledgers were created.
- [ ] Run final gates and commit.

## Expected Gates

- `route optimizer-constraint-ledger --gate`
- `route optimizer-constraint-budget --gate`
- `route tier-optimize --all-tiers --gate`
- `route optimizer-manifest --gate`
- `route release-manifest --gate`
- `cargo test -p route`
- `scripts/check-mileposts.ps1 -SkipTests`

## Non-Goals

- Do not require zero blockers to close.
- Do not close without naming the residual blocker backlog.

