---
wave: constraint-ledger-blocker-burndown
pulse: 06
date: 2026-05-13
status: done
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

- [x] Write `waves/2026-05-13-constraint-ledger-blocker-burndown/CLOSE.md`.
- [x] Update `waves/PHASES.md` and this wave card.
- [x] Summarize before/after blocker counts by tier and constraint class.
- [x] Update `docs/SPEC_INDEX.md` if new specs or action ledgers were created.
- [x] Run final gates and commit.

## Results

- Closeout written to
  `waves/2026-05-13-constraint-ledger-blocker-burndown/CLOSE.md`.
- Final normalized state: 142 ledger rows, 137 budget rows, 0 hard blockers, 117
  claim blockers.
- `zone_assignment_gap` is 0; T4 work is now carried as 69
  `terminal_access_evidence_gap` rows.
- `docs/SPEC_INDEX.md`, `docs/wave-execution.md`, and `waves/PHASES.md` updated
  to mark this wave closed.

## Gate Results

- `cargo test -p route`: pass
- `route optimizer-constraint-ledger --gate`: pass
- `route optimizer-constraint-budget --gate`: pass
- `route tier-optimize --all-tiers --gate`: pass
- `route optimizer-manifest --gate`: pass
- `route release-manifest --gate`: pass
- `scripts/check-mileposts.ps1 -SkipTests`: pass

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

