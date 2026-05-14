---
wave: t4-terminal-access-proof-source-capture
date_open: 2026-05-14
status: done
source: waves/2026-05-14-t4-terminal-access-proof-intake/CLOSE.md
---

# T4 Terminal Access Proof Source Capture

## Mission

Record source-capture placeholders for the 69 source-needed T4 terminal-access
proof-intake rows.

## Opening Rule

This wave may record that manual or cached non-seed source artifacts still need
to be captured. It may not fetch sources, attach source artifacts, accept proof,
mark scenario readiness, or reduce `map;publication;upgrade` blockers.

## Inputs Inherited

| Input | Source |
|---|---|
| Terminal-access proof intake | `data/t4-terminal-access-proof-intake.csv` |

## Pulse Status

| Pulse | Status | Evidence |
|---|---|---|
| 01 - Wave card and scope | done | this wave card and pulse plans |
| 02 - Source-capture surface | done | `data/t4-terminal-access-proof-source-capture.csv`; CLI gate |
| 03 - Review and close | done | `CLOSE.md`; `panels/source-capture/review.md`; final gates |

## Done Criteria

- Every source-needed terminal-access proof-intake row has one source-capture
  row.
- Rows preserve `map;publication;upgrade` and `claim_blocker_delta = 0`.
- Rows keep source artifact `source-needed`, capture status `source-needed`,
  and evidence acceptance `not-reviewed`.
- Optimizer and release manifests register the source-capture artifact.
- Final gates pass before close.

## Non-Goals

- Do not fetch or attach live source artifacts.
- Do not accept proof.
- Do not reduce terminal-access blockers.
