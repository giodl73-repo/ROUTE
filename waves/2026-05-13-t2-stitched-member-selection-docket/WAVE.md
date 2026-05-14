---
wave: t2-stitched-member-selection-docket
date_open: 2026-05-13
status: closed
source: waves/2026-05-13-t2-stitched-member-split-plan/CLOSE.md
---

# T2 Stitched Member Selection Docket

## Mission

Classify I295 and I664 state-scoped stitched-member candidate bundles as
evidence-needed review rows before any registry membership mutation.

## Opening Rule

The selection docket may classify review requirements, but it may not select a
bundle, reject a bundle, or mutate registry membership without a later evidence
artifact. Evidence-needed rows do not repair stitched-member blockers.

## Inputs Inherited

| Input | Source |
|---|---|
| Split plan | `data/t2-stitched-member-split-plan.csv` |
| Decision docket | `data/t2-stitched-member-decision-docket.csv` |

## Pulse Status

| Pulse | Status | Evidence |
|---|---|---|
| 01 - Wave card and scope | done | this wave card and pulse plans |
| 02 - Selection docket surface | done | `data/t2-stitched-member-selection-docket.csv` has 11 evidence-needed rows |
| 03 - Review and close | done | manifests registered, role review written, final gates passed |

## Done Criteria

- Every split-plan row has a selection docket row.
- Selection docket rows remain `evidence-needed` and `review`.
- Rows preserve claim blockers and do not select or reject candidate bundles.
- Optimizer and release manifests register the selection docket.
- Final gates pass before close.

## Non-Goals

- Do not select candidate bundle membership.
- Do not reject candidate bundle membership.
- Do not edit registry or bundle membership.
