---
wave: t4-terminal-access-evidence-review
date_open: 2026-05-14
status: done
source: waves/2026-05-14-optimizer-residual-blocker-backlog/CLOSE.md
---

# T4 Terminal Access Evidence Review

## Mission

Decide the largest residual blocker family: 69 T4 terminal-access evidence
claim blockers.

## Opening Rule

This wave may classify terminal-access evidence blockers and route them to proof
acquisition, source review, or held-known status. It may not accept terminal
contact proof, mark scenario readiness, or reduce blockers without a non-seed
source artifact.

## Inputs Inherited

| Input | Source |
|---|---|
| Residual blocker backlog | `data/optimizer-residual-blocker-backlog.csv` |
| Terminal contact evidence queue | `data/t4-terminal-contact-evidence.csv` |

## Pulse Status

| Pulse | Status | Evidence |
|---|---|---|
| 01 - Wave card and scope | done | this wave card and pulse plans |
| 02 - Terminal evidence review surface | done | `data/t4-terminal-access-evidence-review.csv`; CLI gate |
| 03 - Review and close | done | `CLOSE.md`; `panels/terminal-access-evidence/review.md`; final gates |

## Done Criteria

- Every T4 terminal contact evidence row has a review row.
- Rows preserve `map;publication;upgrade` blockers unless non-seed proof exists.
- Rows identify the next proof artifact or held-known reason.
- Optimizer and release manifests register the review artifact.
- Final gates pass before close.

## Non-Goals

- Do not accept terminal contact proof from terminal district seed assignment.
- Do not mark scenario readiness.
- Do not mutate terminal access columns or source registries.
