---
wave: map-publication-scope-decision
date_open: 2026-05-15
status: done
---

# Map Publication Scope Decision

## Mission

Stop placeholder churn and separate render-valid maps from fully
publication-valid T1-T4 maps.

## Opening Rule

A map render gate is not evidence acceptance. Full map publication can only pass
after optimizer blockers are relieved by accepted evidence or explicitly scoped
out by a downgrade/exclusion decision.

## Inputs Inherited

- `data/map-atlas.csv`
- `data/beck-t1-diagnostics.csv`
- `data/beck-t2-diagnostics.csv`
- `data/t3-zone-map-diagnostics.csv`
- `data/optimizer-residual-blocker-backlog.csv`
- `data/optimizer-constraint-budget.csv`

## Pulse Status

| Pulse | Status | Output |
|---|---|---|
| Map publication scope decision | done | `data/map-publication-scope-decision.csv`; `docs/map-publication-scope.md` |

## Done Criteria

- Render validity and publication validity are explicitly separated.
- Full T1-T4 publication remains blocked while T4 evidence gaps, source snapshot
  guard, and T2 asset debt remain unresolved.
- Structural maps may only be used as held-claim work-in-progress surfaces.
- The rail forbids further placeholder-only source-needed ledgers for map
  publication.

## Non-goals

- Do not attach, accept, or replay terminal-access evidence.
- Do not declare full T1-T4 maps publication-valid.
- Do not create more placeholder-only proof ledgers.
