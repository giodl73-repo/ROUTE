---
wave: map-publication-inventory-gate
date_open: 2026-05-15
status: done
---

# Map Publication Inventory Gate

## Mission

Make the structural map publication inventory fail-fast if it drifts from the
map atlas or map publication readiness artifact.

## Opening Rule

The gate certifies packaging consistency only. It must not promote held evidence,
SLA, transit, upgrade, or repair claims.

## Inputs Inherited

- `data/map-publication-inventory.csv`
- `data/map-atlas.csv`
- `data/map-publication-readiness.csv`
- `docs/map-publication-scope.md`

## Pulse Status

| Pulse | Status | Output |
|---|---|---|
| Map publication inventory gate | done | `route map-publication-inventory --gate` |

## Done Criteria

- Inventory row count must match map atlas row count.
- Every inventory map id, path, and type must match the atlas.
- Every row must point to passing map publication readiness.
- Every row must preserve held claims and forbidden non-publication claims.
- Release manifest uses the gate command.

## Non-goals

- Do not regenerate map images.
- Do not accept evidence, SLA, transit, upgrade, or repair claims.
- Do not add source-needed proof scaffolding.
