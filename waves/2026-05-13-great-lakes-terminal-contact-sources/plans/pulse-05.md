---
wave: great-lakes-terminal-contact-sources
pulse: 05
date: 2026-05-13
status: done
depends_on: [pulse-04]
governing_roles:
  - optimization-methodologist
  - numeracy-checker
  - scope-keeper
---

# Pulse 05 - Wave Close

## Mission

Close the Great Lakes terminal contact source wave by reconciling source tasks,
residual source holds, blocker counts, manifests, and gates.

## Deliverables

- [x] Write `waves/2026-05-13-great-lakes-terminal-contact-sources/CLOSE.md`.
- [x] Update `waves/PHASES.md`, `WAVE.md`, and pulse statuses.
- [x] Summarize source-needed rows by terminal district and source family.
- [x] Update docs indexes if new artifacts were created.
- [x] Run final gates and commit.

## Expected Gates

- `route t4-terminal-contact-source-plan --gate`
- `route optimizer-constraint-ledger --gate`
- `route optimizer-constraint-budget --gate`
- `route tier-optimize --all-tiers --gate`
- `route optimizer-manifest --gate`
- `route release-manifest --gate`
- `cargo test -p route`
- `scripts/check-mileposts.ps1 -SkipTests`

## Non-Goals

- Do not close without naming the residual proof backlog.
- Do not require live source access to close.
