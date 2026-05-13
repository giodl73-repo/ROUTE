---
wave: t4-terminal-contact-evidence
pulse: 06
date: 2026-05-13
status: planned
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

- [ ] Write `waves/2026-05-13-t4-terminal-contact-evidence/CLOSE.md`.
- [ ] Update `waves/PHASES.md` and this wave card.
- [ ] Summarize before/after terminal evidence blocker counts by zone and
  decision class.
- [ ] Update `docs/SPEC_INDEX.md` if new artifacts or specs were created.
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

- Do not require all terminal source holds to close.
- Do not close without naming the residual terminal-contact backlog.
