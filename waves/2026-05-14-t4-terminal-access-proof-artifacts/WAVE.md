---
wave: t4-terminal-access-proof-artifacts
date_open: 2026-05-14
status: done
source: waves/2026-05-14-t4-terminal-access-proof-acquisition/CLOSE.md
---

# T4 Terminal Access Proof Artifacts

## Mission

Turn the 69 source-needed T4 terminal-access proof acquisition tasks into
explicit proof artifact attachment placeholders.

## Opening Rule

This wave may create artifact placeholder rows and attachment requirements. It
may not fetch sources, attach source artifacts, accept proof, mark scenario
readiness, or reduce `map;publication;upgrade` blockers.

## Inputs Inherited

| Input | Source |
|---|---|
| Terminal-access proof acquisition tasks | `data/t4-terminal-access-proof-acquisition.csv` |

## Pulse Status

| Pulse | Status | Evidence |
|---|---|---|
| 01 - Wave card and scope | done | this wave card and pulse plans |
| 02 - Proof artifact placeholder surface | done | `data/t4-terminal-access-proof-artifacts.csv`; CLI gate |
| 03 - Review and close | done | `CLOSE.md`; `panels/proof-artifacts/review.md`; final gates |

## Done Criteria

- Every not-attached terminal-access acquisition task has one proof artifact row.
- Rows preserve `map;publication;upgrade` and `claim_blocker_delta = 0`.
- Rows remain `source-needed`, `not-reviewed`, and `not-accepted`.
- Optimizer and release manifests register the proof artifact.
- Final gates pass before close.

## Non-Goals

- Do not fetch or attach live source artifacts.
- Do not accept proof.
- Do not reduce terminal-access blockers.
