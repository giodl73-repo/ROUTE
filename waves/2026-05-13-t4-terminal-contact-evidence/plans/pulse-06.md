---
wave: t4-terminal-contact-evidence
pulse: 06
date: 2026-05-13
status: done
depends_on: [pulse-05]
governing_roles:
  - optimization-methodologist
  - numeracy-checker
  - scope-keeper
---

# Pulse 06 - Wave Close

## Mission

Close the T4 terminal contact evidence wave by reconciling contact decisions,
residual source holds, blocker counts, manifests, and gates.

## Deliverables

- [x] Write `waves/2026-05-13-t4-terminal-contact-evidence/CLOSE.md`.
- [x] Update `waves/PHASES.md` and this wave card.
- [x] Summarize before/after terminal evidence blocker counts by zone and
  decision class.
- [x] Update `docs/SPEC_INDEX.md` if new artifacts or specs were created.
- [x] Run final gates and commit.

## Expected Gates

- `route optimizer-constraint-ledger --gate`
- `route optimizer-constraint-budget --gate`
- `route tier-optimize --all-tiers --gate`
- `route optimizer-manifest --gate`
- `route release-manifest --gate`
- `cargo test -p route`
- `scripts/check-mileposts.ps1 -SkipTests`

## Evidence

- Closeout: `waves/2026-05-13-t4-terminal-contact-evidence/CLOSE.md`.
- Final state: 69 terminal-contact rows remain source-needed; 0 source-backed
  rows; 0 scenario-ready rows; 142 ledger rows; 137 budget rows; 117 claim
  blockers.
- Gates passed: `cargo test -p route`, `route optimizer-constraint-ledger
  --gate`, `route optimizer-constraint-budget --gate`, `route tier-optimize
  --all-tiers --gate`, `route optimizer-manifest --gate`, `route
  release-manifest --gate`, and `scripts/check-mileposts.ps1 -SkipTests`.

## Non-Goals

- Do not require all terminal source holds to close.
- Do not close without naming the residual terminal-contact backlog.
