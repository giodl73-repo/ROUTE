---
wave: t2-game-ops-binding-burndown
date_open: 2026-05-13
status: done
source: waves/2026-05-13-terminal-contact-source-acquisition-spine/CLOSE.md
---

# T2 Game/Ops Binding Burn-Down

## Mission

Turn T2 `game_ops_bundle_binding` constraint-budget blockers into explicit
bundle decisions before game, incident, publication, or upgrade claims consume
T2 service overlays.

## Opening Rule

No T2 service overlay can support game or incident claims unless it is bound to a
`US.HWYBUNDLE.*` id with a known service class and a concrete decision: bound,
repair-needed, demote, or held. Review rows remain visible blockers; they are not
deleted to make optimizer gates pass.

## Inputs Inherited

| Input | Source |
|---|---|
| Constraint budget | `data/optimizer-constraint-budget.csv` |
| T2 bundle overlays | `data/game/t2-bundle-overlays.csv` |
| T2 service overlays | `data/game/t2-service-overlays.csv` |
| T2 scenario hooks | `data/game/t2-scenario-hooks.csv` |
| Bundle registry | `data/national-segment-bundles.csv` |
| Game doctrine | `docs/game/interstate-tycoon-plan.md` |
| Optimizer/release manifests | `data/tier-optimizer-runs.csv`; `data/release-manifest.csv` |

## Pulse Status

| Pulse | Status | Evidence |
|---|---|---|
| 01 - Binding blocker intake | done | `data/t2-game-ops-binding-intake.csv`; 15 T2 game/ops blockers |
| 02 - Binding decision docket | done | `data/t2-game-ops-binding-decisions.csv`; 14 held, 1 repair-needed |
| 03 - Overlay propagation | done | game/scenario readiness remains held for non-bound decisions |
| 04 - Manifest reconciliation | done | optimizer/release manifests register intake and decision ledgers |
| 05 - Wave close | done | `CLOSE.md`; 14 held, 1 repair-needed, final gates |

## Done Criteria

- Every T2 `game_ops_bundle_binding` budget row has an explicit intake row.
- Every intake row has a decision row with a named next artifact and gate effect.
- Bound decisions require bundle id, service class, overlay metadata, and pass
  validation.
- Repair-needed, demote, and held decisions remain visible blockers.
- Optimizer and release manifests register the new decision artifacts.
- `cargo test -p route`, relevant `route ... --gate` commands,
  `route optimizer-manifest --gate`, `route release-manifest --gate`, and
  `scripts/check-mileposts.ps1 -SkipTests` pass before close.

## Non-Goals

- Do not change T2 geometry or author new route bundles.
- Do not promote game publication readiness.
- Do not resolve asset-condition debt in this wave.

