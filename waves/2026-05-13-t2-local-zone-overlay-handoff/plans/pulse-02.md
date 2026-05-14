---
wave: t2-local-zone-overlay-handoff
pulse: 02
date: 2026-05-13
status: done
depends_on: [pulse-01]
governing_roles:
  - optimization-methodologist
  - citation-auditor
  - numeracy-checker
---

# Pulse 02 - Manifest and Blocker Replay

## Mission

Register the local-zone handoff artifact and replay gates without reducing
residual game/ops blockers.

## Deliverables

- [x] Add optimizer manifest row for `data/t2-local-zone-overlay-handoff.csv`.
- [x] Add release-manifest row with held-public status.
- [x] Regenerate `data/tier-optimizer-runs.csv`.
- [x] Prove T2 game/ops binding decisions remain held/repair-needed.

## Evidence

- `route tier-optimize --all-tiers --gate`
- `route t2-game-ops-binding-decisions --gate`
- `route optimizer-manifest --gate`
- `route release-manifest --gate`

## Expected Gates

- `route t2-local-zone-overlay-handoff --gate`
- `route t2-game-ops-binding-decisions --gate`
- `route tier-optimize --all-tiers --gate`
- `route optimizer-manifest --gate`
- `route release-manifest --gate`
- `cargo test -p route`

## Non-Goals

- Do not change release status to public pass.
