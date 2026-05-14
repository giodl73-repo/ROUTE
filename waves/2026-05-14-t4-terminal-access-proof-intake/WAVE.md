---
wave: t4-terminal-access-proof-intake
date_open: 2026-05-14
status: done
source: waves/2026-05-14-t4-terminal-access-source-access/CLOSE.md
---

# T4 Terminal Access Proof Intake

## Mission

Define proof-intake artifact requirements for the 69 source-needed T4
terminal-access source-access rows.

## Opening Rule

This wave may specify the manual or cached artifact fields needed for future
proof capture. It may not fetch sources, attach source artifacts, accept proof,
mark scenario readiness, or reduce `map;publication;upgrade` blockers.

## Inputs Inherited

| Input | Source |
|---|---|
| Terminal-access source-access policy | `data/t4-terminal-access-source-access.csv` |

## Pulse Status

| Pulse | Status | Evidence |
|---|---|---|
| 01 - Wave card and scope | done | this wave card and pulse plans |
| 02 - Proof-intake surface | done | `data/t4-terminal-access-proof-intake.csv`; CLI gate |
| 03 - Review and close | done | `CLOSE.md`; `panels/proof-intake/review.md`; final gates |

## Done Criteria

- Every source-needed terminal-access source-access row has one proof-intake row.
- Rows preserve `map;publication;upgrade` and `claim_blocker_delta = 0`.
- Rows keep proof artifact and proof status `source-needed`.
- Optimizer and release manifests register the proof-intake artifact.
- Final gates pass before close.

## Non-Goals

- Do not fetch or attach live source artifacts.
- Do not accept proof.
- Do not reduce terminal-access blockers.
