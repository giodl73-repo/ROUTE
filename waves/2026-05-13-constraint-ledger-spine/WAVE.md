---
wave: constraint-ledger-spine
date_open: 2026-05-13
status: active
source: optimizer-constraint-ledger-spec
---

# Constraint Ledger Spine

## Mission

Make the optimizer constraint ledger the single spine for ROUTE selection,
rendering, publication, game, and source-readiness decisions. A route, stop,
bundle, map, or game object should not carry hidden blockers in a side report;
it should expose typed pressure through `data/optimizer-constraint-ledger.csv`
and selector-facing rollups through `data/optimizer-constraint-budget.csv`.

## Opening Rule

If a claim can block selection, promotion, SLA readiness, map publication, game
use, source acquisition, or payment, it belongs in the ledger before a selector
or renderer treats it as resolved.

## Inputs Inherited

| Input | Source |
|---|---|
| Optimizer doctrine | `docs/tier-optimizer-design.md` |
| Constraint ledger spec | `docs/optimizer-constraint-ledger-spec.md` |
| Constraint ledger role review | `docs/reviews/optimizer-constraint-ledger-review.md` |
| Bundle architecture | `docs/route-architecture.md`; `docs/bundle-registry-spec.md` |
| Beck contract | `docs/beck-renderer-contract.md` |
| Significant moments | `data/significant-moments.csv` |

## Pulse Status

| Pulse | Status | Evidence |
|---|---|---|
| 01 - Pavement debt becomes optimizer debt | done | commits `1594ace`, `be36060`, `18b656c`, `9acae00` |
| 02 - Bundle and T2 service blockers join the selector path | done | commits `7be4813`, `4e4f370`, `c319aa3`, `e8e2bc7`, `5a208af`, `85200e4` |
| 03 - Constraint ledger and budget commands | done | commits `838e64d`, `49c9988`, `317b501`, `b74fd4c` |
| 04 - Selector adoption across T1/T2/T3/T4 | done | commits `9f455e1`, `b37a8b8`, `943b25d`, `955b36a` |
| 05 - Beck diagnostics enter the ledger | done | commit `487eec2` |
| 06 - Game/source rows enter the ledger | planned | next pulse |
| 07 - Wave close and doctrine cleanup | planned | close after pulse 06 gates |

## Done Criteria

- `route optimizer-constraint-ledger --gate` passes and names every migrated
  source family.
- `route optimizer-constraint-budget --gate` passes and all selectors consume
  the rollup instead of source-specific shortcuts.
- T1/T2/T3/T4 selector artifacts carry generalized constraint-budget fields.
- Beck diagnostics that block maps or publication enter the same ledger.
- Game/source rows that block scenarios, incidents, upgrades, publication, or
  evidence fetches enter the same ledger.
- `route tier-optimize --all-tiers --gate`, `route optimizer-manifest --gate`,
  `route release-manifest --gate`, and `scripts/check-mileposts.ps1 -SkipTests`
  pass.

## Non-Goals

- Do not claim mathematical optimality.
- Do not erase held rows to make gates green.
- Do not make every constraint hard; claim blockers, budget debt, penalties,
  and review rows are valid outputs when visible.
