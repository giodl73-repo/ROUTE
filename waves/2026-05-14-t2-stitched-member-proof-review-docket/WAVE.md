---
wave: t2-stitched-member-proof-review-docket
date_open: 2026-05-14
status: closed
source: waves/2026-05-14-t2-stitched-member-proof-artifact-attachment/CLOSE.md
---

# T2 Stitched Member Proof Review Docket

## Mission

Review I295 and I664 stitched-member artifact-attachment rows before candidate
disposition, while preserving holds because no real source artifacts are
attached. This is the terminal source-chain slice: unresolved rows return to the
optimizer as held-known work instead of spawning another source-placeholder wave.

## Opening Rule

The proof-review docket may record a review decision for each attachment row. It
may not accept proof, classify candidates, or mutate registry/bundle membership
unless a real manual or cached source artifact exists and is reviewed.

## Inputs Inherited

| Input | Source |
|---|---|
| Artifact attachment | `data/t2-stitched-member-proof-artifact-attachment.csv` |
| Source capture | `data/t2-stitched-member-proof-source-capture.csv` |

## Pulse Status

| Pulse | Status | Evidence |
|---|---|---|
| 01 - Wave card and scope | done | this wave card and pulse plans |
| 02 - Proof review surface | done | `data/t2-stitched-member-proof-review-docket.csv` and CLI gate |
| 03 - Review and close | done | manifests, review, gates, closeout |

## Done Criteria

- Every source-needed attachment row has a proof-review row.
- Review rows remain held because source artifact references are source-needed.
- Review rows preserve claim blockers, do not accept proof, and route back to
  `data/tier-optimizer-runs.csv`.
- Optimizer and release manifests register the proof-review artifact.
- Final gates pass before close.

## Non-Goals

- Do not fetch, cache, attach, or invent source artifacts.
- Do not accept proof.
- Do not classify candidates in scope or rejected.
- Do not edit registry or bundle membership.
- Do not create another placeholder-only source acquisition wave from this
  docket.
