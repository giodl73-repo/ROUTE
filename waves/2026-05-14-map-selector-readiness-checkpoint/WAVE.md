---
wave: map-selector-readiness-checkpoint
date_open: 2026-05-14
status: done
source: data/t1-line-selector.csv
---

# Map Selector Readiness Checkpoint

## Mission

Checkpoint selector and map readiness after T2 game/ops bundle relief replay so
the next work rail is explicit: map gates are green for the current T1
selection, but full publication readiness still depends on residual blocker
burn-down.

## Opening Rule

This wave may document selector readiness and rerun map/selector gates. It must
not reduce terminal-access blockers, attach proof artifacts, or refresh
publication images without a changed render input.

## Inputs Inherited

| Input | Source |
|---|---|
| T1 line selector | `data/t1-line-selector.csv` |
| T1 stop selector | `data/t1-stop-selector.csv` |
| Map atlas | `data/map-atlas.csv` |
| Optimizer map hooks | `data/optimizer-map-hooks.csv` |
| Residual blocker backlog | `data/optimizer-residual-blocker-backlog.csv` |

## Pulse Status

| Pulse | Status | Evidence |
|---|---|---|
| 01 - Selector/map checkpoint | done | `CLOSE.md`; map/selector gate record |
| 02 - Review and rail update | done | `panels/readiness/review.md`; `waves/PHASES.md`; `docs/SPEC_INDEX.md` |

## Done Criteria

- Current T1 selection count, stop count, and map atlas count are recorded.
- Map/selector hooks pass against regenerated optimizer outputs.
- Residual blockers that prevent broader publication readiness are named.
- Final gates pass before close.

## Non-Goals

- Do not attach source-needed T4 proof artifacts.
- Do not reduce `terminal_access_evidence_gap` blockers.
- Do not regenerate map PNGs when the atlas/render contracts have no changed
  source input.

