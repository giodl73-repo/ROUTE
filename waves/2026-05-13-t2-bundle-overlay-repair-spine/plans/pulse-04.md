---
wave: t2-bundle-overlay-repair-spine
pulse: 04
date: 2026-05-13
status: planned
depends_on: [pulse-03]
governing_roles:
  - optimization-methodologist
  - scope-keeper
  - citation-auditor
---

# Pulse 04 - Overlay Replay and Blocker Delta

## Mission

Replay T2 game/ops binding decisions after repair dockets exist, then report the
blocker delta without hiding residual holds.

## Scope Inventory

| Surface | Current artifact | Pulse target |
|---|---|---|
| Bundle overlays | `data/game/t2-bundle-overlays.csv` | Updated or explicitly held binding states |
| Binding decisions | `data/t2-game-ops-binding-decisions.csv` | Replayed decisions and delta |
| Constraint ledger/budget | `data/optimizer-constraint-ledger.csv`; `data/optimizer-constraint-budget.csv` | Updated residual blocker counts |
| Manifests | `data/tier-optimizer-runs.csv`; `data/release-manifest.csv` | Registered repair artifacts |

## Deliverables

- [ ] Regenerate T2 overlay/binding artifacts after repair disposition.
- [ ] Add `data/t2-bundle-overlay-repair-delta.csv`.
- [ ] Register repair artifacts in optimizer and release manifests.
- [ ] Prove no non-pass row loses blocked claims.

## Expected Gates

- `route t2-bundle-overlay-repair-delta --gate`
- `route t2-game-ops-binding-decisions --gate`
- `route optimizer-constraint-ledger --gate`
- `route optimizer-constraint-budget --gate`
- `route tier-optimize --all-tiers --gate`
- `route optimizer-manifest --gate`
- `route release-manifest --gate`
- `cargo test -p route`

## Non-Goals

- Do not close unrelated T4 terminal-contact evidence gaps.
