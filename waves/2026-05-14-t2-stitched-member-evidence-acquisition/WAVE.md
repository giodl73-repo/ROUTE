---
wave: t2-stitched-member-evidence-acquisition
date_open: 2026-05-14
status: closed
source: waves/2026-05-13-t2-stitched-member-evidence-contract/CLOSE.md
---

# T2 Stitched Member Evidence Acquisition

## Mission

Turn I295 and I664 source-needed stitched-member evidence contracts into
concrete acquisition targets without satisfying evidence or changing candidate
membership.

## Opening Rule

The acquisition docket may name source owners, source targets, and manual
acquisition actions, but it may not mark evidence collected or move any
candidate in scope, rejected, or into registry membership.

## Inputs Inherited

| Input | Source |
|---|---|
| Evidence contract | `data/t2-stitched-member-evidence-contract.csv` |
| Selection docket | `data/t2-stitched-member-selection-docket.csv` |

## Pulse Status

| Pulse | Status | Evidence |
|---|---|---|
| 01 - Wave card and scope | done | this wave card and pulse plans |
| 02 - Acquisition docket surface | done | `data/t2-stitched-member-evidence-acquisition.csv` has 11 source-needed rows |
| 03 - Review and close | done | manifests registered, role review written, final gates passed |

## Done Criteria

- Every source-needed evidence contract row has an acquisition docket row.
- Acquisition rows name source owner, source target, and manual action.
- Acquisition rows remain `source-needed` and `review`.
- Optimizer and release manifests register the acquisition docket.
- Final gates pass before close.

## Non-Goals

- Do not collect source evidence.
- Do not classify candidates in scope or rejected.
- Do not edit registry or bundle membership.
