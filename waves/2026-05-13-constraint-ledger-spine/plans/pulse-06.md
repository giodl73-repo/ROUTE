---
wave: constraint-ledger-spine
pulse: 06
date: 2026-05-13
status: planned
depends_on: [pulse-05]
governing_roles:
  - optimization-methodologist
  - freight-economist
  - scope-keeper
  - citation-auditor
---

# Pulse 06 - Game and Source Rows Enter the Ledger

## Mission

Migrate remaining game/source blockers into the optimizer constraint ledger so
scenario hooks, incident/upgrades, publication holds, and evidence-fetch holds
use the same blocker/debt/next-artifact contract as selectors.

## Scope Inventory

| Surface | Current artifact | Pulse target |
|---|---|---|
| Game scenario hooks | `data/game/t2-scenario-hooks.csv` | Claim blockers for scenarios that cannot publish or upgrade because evidence/map/source claims remain held. |
| T2 game overlays | `data/game/t2-bundle-overlays.csv` | Bundle-binding and overlay-pending rows normalize as game/publication readiness constraints. |
| Source fetch policy | `data/source-fetch-policy.csv` | Fetch-cache preservation rows that block evidence claims normalize as source-acquisition constraints. |
| Release holds | `data/release-manifest.csv` | Held-public artifacts remain release metadata, but ledger rows should point to the underlying blocker source where applicable. |

## Deliverables

- [ ] Extend `optimizer_constraint_ledger_rows` with game/source input rows.
- [ ] Add typed constraint classes for source acquisition and game ops readiness
  without collapsing them into schematic geometry.
- [ ] Regenerate ledger, budget, affected game/source artifacts, and optimizer
  manifest.
- [ ] Update `docs/optimizer-constraint-ledger-spec.md` and review notes so
  game/source migration is no longer listed as fully future work.
- [ ] Add or update tests that prove at least one game row and one source row
  normalize into the ledger.

## Expected Gates

- `cargo fmt -p route`
- `cargo test -p route`
- `route source-fetch-policy --gate`
- `route game t2-hooks --gate`
- `route game t2-overlays --gate`
- `route optimizer-constraint-ledger --gate`
- `route optimizer-constraint-budget --gate`
- `route tier-optimize --all-tiers --gate`
- `route optimizer-manifest --gate`
- `route release-manifest --gate`
- `scripts/check-mileposts.ps1 -SkipTests`

## Non-Goals

- Do not solve every held release claim.
- Do not make game rows own optimizer truth; they consume and expose blockers.
- Do not fetch live sources in this pulse.
