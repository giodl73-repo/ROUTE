---
wave: t2-stitched-member-evidence-contract
date_open: 2026-05-13
status: closed
source: waves/2026-05-13-t2-stitched-member-selection-docket/CLOSE.md
---

# T2 Stitched Member Evidence Contract

## Mission

Define the proof contract required before I295 and I664 state-scoped
stitched-member candidates can move from evidence-needed to in-scope or
rejected.

## Opening Rule

The evidence contract may define required proof fields and next artifacts, but
it may not satisfy those fields or change candidate, registry, bundle, or
game/ops status.

## Inputs Inherited

| Input | Source |
|---|---|
| Selection docket | `data/t2-stitched-member-selection-docket.csv` |
| Split plan | `data/t2-stitched-member-split-plan.csv` |

## Pulse Status

| Pulse | Status | Evidence |
|---|---|---|
| 01 - Wave card and scope | done | this wave card and pulse plans |
| 02 - Evidence contract surface | done | `data/t2-stitched-member-evidence-contract.csv` has 11 source-needed rows |
| 03 - Review and close | done | manifests registered, role review written, final gates passed |

## Done Criteria

- Every evidence-needed selection row has an evidence contract row.
- Contract rows name required continuity, state-scope, and source-proof fields.
- Contract rows remain `source-needed` and `review`.
- Optimizer and release manifests register the evidence contract.
- Final gates pass before close.

## Non-Goals

- Do not collect evidence.
- Do not mark candidates in scope or rejected.
- Do not edit registry or bundle membership.
