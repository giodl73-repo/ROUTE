---
wave: t2-stitched-member-proof-source-capture
date_open: 2026-05-14
status: closed
source: waves/2026-05-14-t2-stitched-member-proof-intake/CLOSE.md
---

# T2 Stitched Member Proof Source Capture

## Mission

Record source-capture placeholders for I295 and I664 stitched-member
proof-intake rows before any source artifact is attached, reviewed, or accepted.

## Opening Rule

The source-capture docket may define the capture slot for manual or cached DOT
route-geometry artifacts, but it may not attach evidence, accept proof, select
candidate bundles, or mutate registry/bundle membership.

## Inputs Inherited

| Input | Source |
|---|---|
| Proof intake | `data/t2-stitched-member-proof-intake.csv` |
| Source access policy | `data/t2-stitched-member-source-access-policy.csv` |

## Pulse Status

| Pulse | Status | Evidence |
|---|---|---|
| 01 - Wave card and scope | done | this wave card and pulse plans |
| 02 - Source capture surface | done | `data/t2-stitched-member-proof-source-capture.csv` and CLI gate |
| 03 - Review and close | done | manifests, review, gates, closeout |

## Done Criteria

- Every source-needed proof-intake row has a source-capture row.
- Source-capture rows keep source artifact references `source-needed`.
- Source-capture rows remain review-only and preserve claim blockers.
- Optimizer and release manifests register the source-capture artifact.
- Final gates pass before close.

## Non-Goals

- Do not fetch, cache, or attach source artifacts.
- Do not accept proof.
- Do not classify candidates in scope or rejected.
- Do not edit registry or bundle membership.
