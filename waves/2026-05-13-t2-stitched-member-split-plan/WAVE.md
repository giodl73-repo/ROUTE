---
wave: t2-stitched-member-split-plan
date_open: 2026-05-13
status: closed
source: waves/2026-05-13-t2-stitched-member-decision-docket/CLOSE.md
---

# T2 Stitched Member Split Plan

## Mission

Convert I295 and I664 stitched-member split decisions into state-scoped
candidate bundle choices before any registry or bundle membership mutation can
reduce game, incident, publication, or upgrade blockers.

## Opening Rule

The split plan may enumerate candidate bundle choices, but it may not select,
merge, append, or remove bundle members. A split row is a review target, not a
repaired stitched service.

## Inputs Inherited

| Input | Source |
|---|---|
| Decision docket | `data/t2-stitched-member-decision-docket.csv` |
| Candidate segment rows | `data/tier-segment-candidates.csv` |
| Candidate scope review | `data/t2-stitched-member-candidate-scope-review.csv` |

## Pulse Status

| Pulse | Status | Evidence |
|---|---|---|
| 01 - Wave card and scope | done | this wave card and pulse plans |
| 02 - Split plan surface | done | `data/t2-stitched-member-split-plan.csv` has 11 review rows |
| 03 - Review and close | done | manifests registered, role review written, final gates passed |

## Done Criteria

- Every split decision has candidate bundle rows matching its candidate count.
- Split rows name candidate bundle id, stitch group, state scope, member count,
  and member mileage.
- Split rows preserve claim blockers and remain in review status.
- Optimizer and release manifests register the split-plan artifact.
- Final gates pass before close.

## Non-Goals

- Do not edit tier segment candidates.
- Do not edit registry or bundle membership.
- Do not reduce game, incident, publication, or upgrade blockers.
