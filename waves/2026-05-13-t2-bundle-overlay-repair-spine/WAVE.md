---
wave: t2-bundle-overlay-repair-spine
date_open: 2026-05-13
status: active
source: waves/2026-05-13-t2-game-ops-binding-burndown/CLOSE.md
---

# T2 Bundle Overlay Repair Spine

## Mission

Turn the 15 residual T2 game/ops binding decisions into a repair spine that can
either produce pass-ready bundle overlays or preserve explicit demotion/hold
decisions without laundering unresolved service classes into game, incident,
publication, or upgrade claims.

## Opening Rule

No T2 game/ops binding decision may become `bound` unless the corresponding
`data/game/t2-bundle-overlays.csv` row has a usable service class, a
`US.HWYBUNDLE.*` id, pass validation, and a claim-safe bundle binding status.
Rows that need stop chains, stitched members, terminal stops, pavement debt, or
local-zone treatment remain repair/demotion work rather than silent passes.

## Inputs Inherited

| Input | Source |
|---|---|
| Binding decisions | `data/t2-game-ops-binding-decisions.csv` |
| T2 bundle overlays | `data/game/t2-bundle-overlays.csv` |
| T2 service overlays | `data/game/t2-service-overlays.csv` |
| T2 service diagnostics | `data/t2-service-diagnostic-queue.csv`; `data/beck-t2-diagnostics.csv` |
| Bundle registry | `data/national-segment-bundles.csv`; `data/national-segment-registry.csv` |
| Pavement debt | `data/tier-pavement-source-gaps.csv`; `data/tier-pavement-debt-budget.csv` |
| Optimizer/release manifests | `data/tier-optimizer-runs.csv`; `data/release-manifest.csv` |

## Pulse Status

| Pulse | Status | Evidence |
|---|---|---|
| 01 - Repair target intake | planned | classify 15 held/repair-needed decisions by blocker type |
| 02 - Service-class repair docket | planned | route service-class-held rows to Beck/service/local-zone actions |
| 03 - Bundle readiness disposition | planned | decide stop-chain, stitched-member, terminal-stop, and I37 repair rows |
| 04 - Overlay replay and blocker delta | planned | regenerate game/ops decisions and optimizer manifests |
| 05 - Review and wave close | planned | role review, residual blocker accounting, final gates |

## Done Criteria

- Every residual T2 game/ops binding decision has a repair target row.
- Every repair target has one of: pass candidate, repair-needed, demote, or held.
- Service-class-held rows are separated from bundle-readiness rows.
- I37 `bundle-bound-review` cannot pass until its `needs-stop-chain` status is
  resolved or explicitly demoted.
- Any newly bound row passes the T2 game/ops binding decision gate and carries no
  blocked game, incident, publication, or upgrade claims.
- Optimizer and release manifests register any new repair ledgers.
- `cargo test -p route`, relevant `route ... --gate` commands,
  `route optimizer-constraint-ledger --gate`, `route optimizer-constraint-budget
  --gate`, `route tier-optimize --all-tiers --gate`, `route optimizer-manifest
  --gate`, `route release-manifest --gate`, and
  `scripts/check-mileposts.ps1 -SkipTests` pass before close.

## Non-Goals

- Do not invent new T2 routes or route geometry.
- Do not promote publication readiness from heuristic or held rows.
- Do not use game scenario usefulness as evidence for a bundle binding pass.
- Do not resolve T4 terminal-contact source-needed rows in this wave.
