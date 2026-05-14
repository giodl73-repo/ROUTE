---
wave: t2-stitched-member-proof-intake
date_open: 2026-05-14
status: closed
source: waves/2026-05-14-t2-stitched-member-source-access-policy/CLOSE.md
---

# T2 Stitched Member Proof Intake

## Mission

Define the manual/cached evidence artifact fields required for I295 and I664
stitched-member source-access rows before any evidence collection begins.

## Opening Rule

The proof-intake docket may name required artifact fields and review blockers,
but it may not attach source artifacts, accept proof, classify candidates, or
mutate membership.

## Inputs Inherited

| Input | Source |
|---|---|
| Source-access policy | `data/t2-stitched-member-source-access-policy.csv` |
| Evidence acquisition | `data/t2-stitched-member-evidence-acquisition.csv` |

## Pulse Status

| Pulse | Status | Evidence |
|---|---|---|
| 01 - Wave card and scope | done | this wave card and pulse plans |
| 02 - Proof intake surface | done | `data/t2-stitched-member-proof-intake.csv` and CLI gate |
| 03 - Review and close | done | manifests, review, gates, closeout |

## Done Criteria

- Every source-needed access-policy row has a proof-intake row.
- Proof-intake rows name required artifact fields and proof blocker.
- Proof-intake rows remain source-needed and review.
- Optimizer and release manifests register the proof-intake artifact.
- Final gates pass before close.

## Non-Goals

- Do not collect source evidence.
- Do not accept proof.
- Do not classify candidates in scope or rejected.
- Do not edit registry or bundle membership.
