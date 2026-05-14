---
wave: t2-service-overlay-diagnostic-binding
date_open: 2026-05-13
date_closed: 2026-05-13
status: done
source: waves/2026-05-13-t2-bundle-overlay-repair-spine/CLOSE.md
---

# T2 Service Overlay Diagnostic Binding

## Mission

Convert the seven T2 service-overlay repair rows into explicit Beck diagnostic
decisions so unclassified service overlays cannot loop back into game/ops
binding without a named service class.

## Opening Rule

No `service-overlay` repair row may become a game/ops binding pass until it has
a route-specific Beck T2 diagnostic, a non-`unclassified` service class, and a
matching `data/game/t2-service-overlays.csv` service-class contract.

## Inputs Inherited

| Input | Source |
|---|---|
| Service-class repair docket | `data/t2-service-class-repair-docket.csv` |
| Repair targets | `data/t2-bundle-overlay-repair-targets.csv` |
| Service diagnostics | `data/t2-service-diagnostic-queue.csv`; `data/beck-t2-diagnostics.csv` |
| Service overlay contract | `data/game/t2-service-overlays.csv` |
| Optimizer/release manifests | `data/tier-optimizer-runs.csv`; `data/release-manifest.csv` |

## Pulse Status

| Pulse | Status | Evidence |
|---|---|---|
| 01 - Diagnostic decision surface | done | `data/t2-service-overlay-diagnostic-decisions.csv` holds seven service-overlay rows |
| 02 - Manifest and blocker replay | done | release and optimizer manifests register the held diagnostic artifact |
| 03 - Review and wave close | done | residual blocker handoff and gates in `CLOSE.md` |

## Done Criteria

- Every `service-overlay` row from `data/t2-service-class-repair-docket.csv` has
  a diagnostic binding decision.
- No row is promoted while service class remains `unclassified`.
- Rows with missing Beck diagnostics point to `data/beck-t2-diagnostics.csv`.
- Optimizer and release manifests register the decision artifact.
- `cargo test -p route`, relevant `route ... --gate` commands,
  `route optimizer-manifest --gate`, `route release-manifest --gate`, and
  `scripts/check-mileposts.ps1 -SkipTests` pass before close.

## Non-Goals

- Do not author new Beck geometry or service classes in this wave.
- Do not resolve local-zone rows.
- Do not change T2 bundle readiness or pavement debt status.
