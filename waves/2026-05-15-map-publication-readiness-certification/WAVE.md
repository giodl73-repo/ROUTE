---
wave: map-publication-readiness-certification
date_open: 2026-05-15
status: done
---

# Map Publication Readiness Certification

## Mission

Make the current T1-T4 structural map publication state gateable instead of
implicit in the residual blocker backlog.

## Opening Rule

The readiness artifact may certify map publication only. It must preserve and
name held non-publication claims: source evidence, terminal-access upgrade, and
asset-condition SLA/transit/upgrade obligations.

## Inputs Inherited

- `data/map-atlas.csv`
- `data/map-publication-scope-decision.csv`
- `data/optimizer-residual-blocker-backlog.csv`
- `docs/map-publication-scope.md`

## Pulse Status

| Pulse | Status | Output |
|---|---|---|
| Map publication readiness certification | done | `data/map-publication-readiness.csv`; `route map-publication-readiness --gate` |

## Done Criteria

- `data/map-publication-readiness.csv` reports zero residual `publication`
  blockers.
- The readiness row preserves held `evidence`, `sla`, `transit`, and `upgrade`
  claims.
- The command fails if render gates, scope status, or publication blockers fail.
- The release manifest owns the readiness gate.

## Non-goals

- Do not accept evidence claims.
- Do not clear upgrade, SLA, transit, or repair obligations.
- Do not regenerate map images unless atlas gates fail.
