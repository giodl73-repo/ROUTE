---
wave: map-publication-inventory-package
date_open: 2026-05-15
status: done
---

# Map Publication Inventory Package

## Mission

Package the current structural T1-T4 map set into a release-facing inventory
that names each publishable map and its required held-claim label.

## Opening Rule

The inventory may package maps for publication only as structural held-claim
surfaces. It must not claim evidence validity, SLA validity, transit readiness,
upgrade readiness, or asset-condition repair completion.

## Inputs Inherited

- `data/map-atlas.csv`
- `data/map-publication-readiness.csv`
- `data/map-publication-scope-decision.csv`
- `docs/map-publication-scope.md`

## Pulse Status

| Pulse | Status | Output |
|---|---|---|
| Map publication inventory package | done | `data/map-publication-inventory.csv` |

## Done Criteria

- Inventory has all 17 current map atlas artifacts.
- Every row is publication-ready only with held claims.
- Every row names forbidden non-publication claims.
- Release manifest and spec index own the inventory artifact.

## Non-goals

- Do not regenerate map images.
- Do not accept evidence, SLA, transit, upgrade, or repair claims.
- Do not create placeholder proof ledgers.
