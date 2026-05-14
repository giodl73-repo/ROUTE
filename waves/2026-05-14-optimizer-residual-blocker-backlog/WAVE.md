---
wave: optimizer-residual-blocker-backlog
date_open: 2026-05-14
status: done
source: waves/2026-05-14-t2-overlay-p3-local-zone-overlay-review/CLOSE.md
---

# Optimizer Residual Blocker Backlog

## Mission

Re-rank remaining constraint-budget blockers after the T2 overlay P1/P2/P3
priority reviews produced no blocker relief.

## Opening Rule

This wave may group existing constraint-budget blockers into next optimizer wave
families. It may not reduce blockers, promote claims, or mutate selector,
registry, game, map, source, or bundle artifacts.

## Inputs Inherited

| Input | Source |
|---|---|
| Constraint budget | `data/optimizer-constraint-budget.csv` |
| Tier optimizer manifest | `data/tier-optimizer-runs.csv` |
| Overlay P3 closeout | `waves/2026-05-14-t2-overlay-p3-local-zone-overlay-review/CLOSE.md` |

## Pulse Status

| Pulse | Status | Evidence |
|---|---|---|
| 01 - Wave card and scope | done | this wave card and pulse plans |
| 02 - Residual backlog surface | done | `data/optimizer-residual-blocker-backlog.csv`; CLI gate |
| 03 - Review and close | done | `CLOSE.md`; `panels/residual-backlog/review.md`; final gates |

## Done Criteria

- Remaining non-pass constraint-budget rows are grouped by blocker family.
- Each group has a priority class, blocker count, representative routes, and
  next optimizer wave label.
- Rows are explicitly triage-only and preserve blocker counts.
- Optimizer and release manifests register the backlog artifact.
- Final gates pass before close.

## Non-Goals

- Do not resolve any blocker.
- Do not change selector decisions.
- Do not reopen the T2 overlay priority review inside this wave.
