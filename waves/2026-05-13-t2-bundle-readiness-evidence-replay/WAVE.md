---
wave: t2-bundle-readiness-evidence-replay
date_open: 2026-05-13
status: closed
source: waves/2026-05-13-t2-bundle-readiness-repair-evidence/CLOSE.md
---

# T2 Bundle Readiness Evidence Replay

## Mission

Convert the four T2 bundle-readiness evidence probes into explicit replay
decisions so candidate evidence cannot silently promote unresolved game/ops
claims.

## Opening Rule

Candidate evidence may only move a row to a replay decision surface. It cannot
change `data/t2-game-ops-binding-decisions.csv` or clear claim blockers unless a
later replay gate proves the bundle status changed.

## Inputs Inherited

| Input | Source |
|---|---|
| Readiness evidence | `data/t2-bundle-readiness-repair-evidence.csv` |
| Repair delta | `data/t2-bundle-overlay-repair-delta.csv` |
| Game/ops decisions | `data/t2-game-ops-binding-decisions.csv` |
| Optimizer/release manifests | `data/tier-optimizer-runs.csv`; `data/release-manifest.csv` |

## Pulse Status

| Pulse | Status | Evidence |
|---|---|---|
| 01 - Replay decision surface | done | `data/t2-bundle-readiness-replay-decisions.csv` has four held rows |
| 02 - Manifest and blocker replay | done | optimizer/release rows register replay artifact |
| 03 - Review and wave close | done | `CLOSE.md` and panel preserve residual blocker handoff |

## Done Criteria

- Every readiness evidence row has a replay decision.
- Replay decisions preserve claim blockers and stay out of `bound` status.
- Optimizer and release manifests register the replay artifact.
- `cargo test -p route`, relevant `route ... --gate` commands,
  `route optimizer-manifest --gate`, `route release-manifest --gate`, and
  `scripts/check-mileposts.ps1 -SkipTests` pass before close.

## Non-Goals

- Do not mutate bundle readiness or game/ops decisions.
- Do not claim candidate evidence as repaired readiness.
- Do not resolve service-overlay or local-zone rows.
