---
wave: t2-service-overlay-diagnostic-binding
date_closed: 2026-05-13
status: done
source: waves/2026-05-13-t2-bundle-overlay-repair-spine/CLOSE.md
---

# T2 Service Overlay Diagnostic Binding Closeout

## Decision

The wave is closed with seven T2 service-overlay repair rows converted into a
gateable diagnostic decision surface. All seven rows remain `held`; no
unclassified service overlay was promoted into game, incident, publication, or
upgrade claims.

## Artifacts

| Artifact | Rows | Status | Role |
|---|---:|---|---|
| `data/t2-service-overlay-diagnostic-decisions.csv` | 7 | held | Explicit Beck diagnostic decision surface for service-overlay repair rows |
| `data/tier-optimizer-runs.csv` | 65 | mixed | Registers the diagnostic decision surface as `held-known` with seven blockers |
| `data/release-manifest.csv` | 97 | mixed | Registers the diagnostic decision surface as `held_public` |

## Residual Blockers

| Class | Rows | Disposition | Next Artifact |
|---|---:|---|---|
| service-overlay | 7 | held behind missing Beck T2 diagnostics | `data/beck-t2-diagnostics.csv` |
| local-zone | 7 | held below national game overlay | future local/zone role artifact |
| bundle-readiness | 10 | held or repair-needed behind stop-chain, stitched-member, or terminal-stop repair | `data/national-segment-bundles.csv` |

The seven service-overlay routes are `I195`, `I220`, `I270`, `I275`, `I295`,
`I635`, and `US2`. Each row points to `data/beck-t2-diagnostics.csv` before any
service class can pass.

## Doctrine Preserved

- A row with `current_service_class=unclassified` cannot become `bound`.
- Diagnostic decisions are not a service-class authoring surface.
- Blocked claim families remain `game;incident;publication;upgrade`.
- The release status remains held-public; no public pass is implied.

## Gates

- `cargo test -p route`
- `route t2-service-overlay-diagnostic-decisions --gate`
- `route t2-service-class-repair-docket --gate`
- `route t2-game-ops-binding-decisions --gate`
- `route tier-optimize --all-tiers --gate`
- `route optimizer-manifest --gate`
- `route release-manifest --gate`
- `scripts/check-mileposts.ps1 -SkipTests`
