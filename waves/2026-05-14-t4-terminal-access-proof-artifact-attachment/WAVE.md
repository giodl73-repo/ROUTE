---
wave: t4-terminal-access-proof-artifact-attachment
date_open: 2026-05-14
status: done
source: waves/2026-05-14-t4-terminal-access-proof-source-capture/CLOSE.md
---

# T4 Terminal Access Proof Artifact Attachment

## Mission

Record artifact-attachment placeholders for the 69 source-needed T4
terminal-access proof source-capture rows.

## Opening Rule

This wave may record that manual or cached non-seed terminal-access proof
artifacts still need to be attached. It may not fetch sources, accept proof,
mark scenario readiness, or reduce `map;publication;upgrade` blockers.

## Inputs Inherited

| Input | Source |
|---|---|
| Terminal-access proof source capture | `data/t4-terminal-access-proof-source-capture.csv` |

## Pulse Status

| Pulse | Status | Evidence |
|---|---|---|
| 01 - Wave card and scope | done | this wave card and pulse plans |
| 02 - Artifact-attachment surface | done | `data/t4-terminal-access-proof-artifact-attachment.csv`; CLI gate |
| 03 - Review and close | done | `CLOSE.md`; `panels/artifact-attachment/review.md`; final gates |

## Done Criteria

- Every source-needed terminal-access source-capture row has one
  artifact-attachment row.
- Rows preserve `map;publication;upgrade` and `claim_blocker_delta = 0`.
- Rows keep source artifact `source-needed`, attachment status
  `source-needed`, evidence review `not-reviewed`, and proof acceptance
  `not-accepted`.
- Optimizer and release manifests register the artifact-attachment artifact.
- Final gates pass before close.

## Non-Goals

- Do not fetch or attach live source artifacts.
- Do not accept proof.
- Do not reduce terminal-access blockers.
