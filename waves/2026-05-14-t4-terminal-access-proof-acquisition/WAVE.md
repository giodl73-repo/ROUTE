---
wave: t4-terminal-access-proof-acquisition
date_open: 2026-05-14
status: done
source: waves/2026-05-14-t4-terminal-access-evidence-review/CLOSE.md
---

# T4 Terminal Access Proof Acquisition

## Mission

Turn the 69 held terminal-access evidence review rows into explicit proof
acquisition tasks.

## Opening Rule

This wave may create acquisition tasks and source requirements. It may not
attach proof artifacts, accept proof, mark scenario readiness, or reduce
`map;publication;upgrade` blockers.

## Inputs Inherited

| Input | Source |
|---|---|
| Terminal-access evidence review | `data/t4-terminal-access-evidence-review.csv` |

## Pulse Status

| Pulse | Status | Evidence |
|---|---|---|
| 01 - Wave card and scope | done | this wave card and pulse plans |
| 02 - Acquisition task surface | done | `data/t4-terminal-access-proof-acquisition.csv`; CLI gate |
| 03 - Review and close | done | `CLOSE.md`; `panels/proof-acquisition/review.md`; final gates |

## Done Criteria

- Every held terminal-access review row has one acquisition task.
- Rows preserve `map;publication;upgrade` and `claim_blocker_delta = 0`.
- Rows define required non-seed proof and acquisition status.
- Optimizer and release manifests register the acquisition artifact.
- Final gates pass before close.

## Non-Goals

- Do not fetch or attach live source artifacts.
- Do not accept proof.
- Do not reduce terminal-access blockers.
