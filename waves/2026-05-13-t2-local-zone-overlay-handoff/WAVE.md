---
wave: t2-local-zone-overlay-handoff
date_open: 2026-05-13
date_closed: 2026-05-13
status: done
source: waves/2026-05-13-t2-service-overlay-diagnostic-binding/CLOSE.md
---

# T2 Local Zone Overlay Handoff

## Mission

Convert the seven T2 `local-zone` repair rows into explicit T3 zone handoff
decisions so local relief treatments cannot re-enter national T2 game overlays
without a named zone role and map treatment.

## Opening Rule

No `local-zone` repair row may become a national game/ops binding pass until it
has a route-specific T3 zone role, visible map treatment, and an explicit
decision that keeps local relief below T2 claims unless a later review promotes
it.

## Inputs Inherited

| Input | Source |
|---|---|
| Service-class repair docket | `data/t2-service-class-repair-docket.csv` |
| T3 zone route columns | `data/t3-zone-route-columns.csv` |
| T3 zone render board | `data/t3-zone-render-board.csv` |
| Repair delta | `data/t2-bundle-overlay-repair-delta.csv` |
| Optimizer/release manifests | `data/tier-optimizer-runs.csv`; `data/release-manifest.csv` |

## Pulse Status

| Pulse | Status | Evidence |
|---|---|---|
| 01 - Local-zone handoff surface | done | `data/t2-local-zone-overlay-handoff.csv` holds seven local-zone rows |
| 02 - Manifest and blocker replay | done | release and optimizer manifests register the held local-zone artifact |
| 03 - Review and wave close | done | residual blocker handoff and gates in `CLOSE.md` |

## Done Criteria

- Every `local-zone` row from `data/t2-service-class-repair-docket.csv` has a
  handoff decision.
- No row is promoted into national T2 game/ops binding.
- Handoff rows name the T3 zone role or mark the row held if zone context is
  missing.
- Optimizer and release manifests register the handoff artifact.
- `cargo test -p route`, relevant `route ... --gate` commands,
  `route optimizer-manifest --gate`, `route release-manifest --gate`, and
  `scripts/check-mileposts.ps1 -SkipTests` pass before close.

## Non-Goals

- Do not change T3 zone selection thresholds.
- Do not author new zone maps or stops.
- Do not resolve service-overlay or bundle-readiness blockers.
