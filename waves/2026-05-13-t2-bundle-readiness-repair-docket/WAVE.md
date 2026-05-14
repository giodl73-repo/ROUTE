---
wave: t2-bundle-readiness-repair-docket
date_open: 2026-05-13
date_closed: 2026-05-13
status: done
source: waves/2026-05-13-t2-local-zone-overlay-handoff/CLOSE.md
---

# T2 Bundle Readiness Repair Docket

## Mission

Turn the four `repair-needed` T2 bundle-readiness rows into explicit repair
tasks for stop-chain, stitched-member, and terminal-stop blockers, while keeping
service-blocked readiness rows held.

## Opening Rule

No bundle-readiness row may become a game/ops binding pass until its repair task
names the missing structural artifact, preserves claim blockers, and remains
under review until the downstream bundle artifact actually changes.

## Inputs Inherited

| Input | Source |
|---|---|
| Readiness disposition | `data/t2-bundle-readiness-disposition.csv` |
| Repair delta | `data/t2-bundle-overlay-repair-delta.csv` |
| Bundle registry | `data/national-segment-bundles.csv` |
| Segment candidates | `data/tier-segment-candidates.csv` |
| Service selection | `data/t2-service-selection.csv` |
| Optimizer/release manifests | `data/tier-optimizer-runs.csv`; `data/release-manifest.csv` |

## Pulse Status

| Pulse | Status | Evidence |
|---|---|---|
| 01 - Readiness repair docket | done | `data/t2-bundle-readiness-repair-docket.csv` dockets four repair-needed readiness rows |
| 02 - Manifest and blocker replay | done | release and optimizer manifests register the held repair docket |
| 03 - Review and wave close | done | residual blocker handoff and gates in `CLOSE.md` |

## Done Criteria

- Every `repair-needed` row from `data/t2-bundle-readiness-disposition.csv` has
  a repair task.
- Held service-blocked readiness rows remain out of the repair docket.
- No row is promoted into national T2 game/ops binding.
- Optimizer and release manifests register the repair docket artifact.
- `cargo test -p route`, relevant `route ... --gate` commands,
  `route optimizer-manifest --gate`, `route release-manifest --gate`, and
  `scripts/check-mileposts.ps1 -SkipTests` pass before close.

## Non-Goals

- Do not author stop chains, stitched members, terminal stops, or new service
  classes in this wave.
- Do not change bundle ids or national segment membership.
- Do not resolve service-overlay or local-zone rows.
