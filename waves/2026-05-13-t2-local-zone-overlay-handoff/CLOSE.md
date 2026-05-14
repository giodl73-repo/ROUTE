---
wave: t2-local-zone-overlay-handoff
date_closed: 2026-05-13
status: done
source: waves/2026-05-13-t2-service-overlay-diagnostic-binding/CLOSE.md
---

# T2 Local Zone Overlay Handoff Closeout

## Decision

The wave is closed with seven T2 local-zone repair rows converted into a
gateable handoff surface. All seven rows remain `held-local-zone`; no local
relief treatment was promoted into national T2 game, incident, publication, or
upgrade claims.

## Artifacts

| Artifact | Rows | Status | Role |
|---|---:|---|---|
| `data/t2-local-zone-overlay-handoff.csv` | 7 | held | Explicit T3 zone handoff surface for local-zone repair rows |
| `data/tier-optimizer-runs.csv` | 66 | mixed | Registers the handoff surface as `held-known` with seven blockers |
| `data/release-manifest.csv` | 98 | mixed | Registers the handoff surface as `held_public` |

## Residual Blockers

| Class | Rows | Disposition | Next Artifact |
|---|---:|---|---|
| local-zone | 7 | held below national T2 game overlay with visible T3 zone roles | `data/t3-zone-stop-placement.csv` |
| service-overlay | 7 | held behind missing Beck T2 diagnostics | `data/beck-t2-diagnostics.csv` |
| bundle-readiness | 10 | held or repair-needed behind stop-chain, stitched-member, or terminal-stop repair | `data/national-segment-bundles.csv` |

The seven local-zone routes are `I205`, `I225`, `I240`, `I264`, `I610`,
`I664`, and `I680`. Each row names a T3 zone role and stays below national game
overlay claims.

## Doctrine Preserved

- A local-zone handoff is not a national T2 service-class promotion.
- Handoff decisions remain `review` and `held-local-zone`.
- Blocked claim families remain `game;incident;publication;upgrade`.
- The release status remains held-public; no public pass is implied.

## Gates

- `cargo test -p route`
- `route t2-local-zone-overlay-handoff --gate`
- `route t2-service-class-repair-docket --gate`
- `route t2-game-ops-binding-decisions --gate`
- `route tier-optimize --all-tiers --gate`
- `route optimizer-manifest --gate`
- `route release-manifest --gate`
- `scripts/check-mileposts.ps1 -SkipTests`
