---
name: Pulse 04 Game Bundle Holds
slug: pulse-04-game-bundle-holds
type: review
status: reviewed
rubric_version: v1.0
author: route-pulse
created: 2026-05-13
updated: 2026-05-13
sources:
  - data/game/t2-bundle-overlays.csv
  - data/game/t2-service-overlays.csv
  - data/game/t2-scenario-hooks.csv
  - data/national-segment-bundles.csv
  - data/optimizer-constraint-budget.csv
---

# Pulse 04 Game Bundle Holds

## Decision

Pulse 04 carries unresolved T2 game blockers rather than creating new game
scenarios or redefining bundle identity for convenience. The bundle registry is
the service join surface.

## Audit Findings

| Finding | Count | Decision |
|---|---:|---|
| `bundle-bound` | 24 | Valid bundle identities with service-class overlays may feed scenario hooks. |
| `service-class-held-known` | 15 | Bundle ids exist, but service class is unclassified; keep off playable scenario rails until the service class is authored. |
| `bundle-bound-review` | 1 | `I37` has a bundle id and compact-service overlay, but bundle validation still points to stop-chain work. |
| `game_ops_publication_readiness` hooks | 3 | Scenario hooks remain publication-held until evidence rows close. |

## Review Notes

- No stale bundle id repair was available in this pulse; the current registry
  already resolves each overlay route to a bundle id.
- `service-class-held-known` prevents unclassified rows from looking like missing
  game overlay data while still blocking game, incident, upgrade, and publication
  claims.
- `I37` stays with `data/national-segment-bundles.csv` because the unresolved
  issue is bundle validation, not game overlay vocabulary.
