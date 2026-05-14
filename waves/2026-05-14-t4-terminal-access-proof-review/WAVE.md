---
wave: t4-terminal-access-proof-review
date_open: 2026-05-14
status: done
source: waves/2026-05-14-t4-terminal-access-proof-artifacts/CLOSE.md
---

# T4 Terminal Access Proof Review

## Mission

Review the 69 unattached T4 terminal-access proof artifact placeholders and
return unresolved rows to optimizer held-known status.

## Opening Rule

This wave may review placeholder rows and record that proof cannot be accepted
without attached non-seed source artifacts. It may not fetch sources, attach
source artifacts, accept proof, mark scenario readiness, or reduce
`map;publication;upgrade` blockers.

## Inputs Inherited

| Input | Source |
|---|---|
| Terminal-access proof artifact placeholders | `data/t4-terminal-access-proof-artifacts.csv` |

## Pulse Status

| Pulse | Status | Evidence |
|---|---|---|
| 01 - Wave card and scope | done | this wave card and pulse plans |
| 02 - Proof review surface | done | `data/t4-terminal-access-proof-review.csv`; CLI gate |
| 03 - Review and close | done | `CLOSE.md`; `panels/proof-review/review.md`; final gates |

## Done Criteria

- Every source-needed terminal-access proof artifact row has one proof review
  row.
- Rows preserve `map;publication;upgrade` and `claim_blocker_delta = 0`.
- Rows remain `held-no-source-artifact`, `not-accepted`, and return to optimizer
  held-known status.
- Optimizer and release manifests register the proof review artifact.
- Final gates pass before close.

## Non-Goals

- Do not fetch or attach live source artifacts.
- Do not accept proof.
- Do not reduce terminal-access blockers.
