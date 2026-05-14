---
wave: t2-beck-transfer-complexity-policy
date_open: 2026-05-14
status: done
source: data/t2-beck-transfer-complexity-review.csv
---

# T2 Beck Transfer Complexity Policy

## Mission

Author transfer-complexity policy rows for the reviewed T2 Beck routes before
any blocker relief or final Beck replacement publication.

## Opening Rule

This wave may author policy treatment for transfer complexity. It may not accept
the policy, reduce blockers, mutate Beck diagnostics, or change selector output.

## Inputs Inherited

| Input | Source |
|---|---|
| T2 Beck transfer-complexity review | `data/t2-beck-transfer-complexity-review.csv` |
| Beck T2 diagnostics | `data/beck-t2-diagnostics.csv` |

## Pulse Status

| Pulse | Status | Evidence |
|---|---|---|
| 01 - Wave card and scope | done | this wave card and pulse plans |
| 02 - Transfer policy surface | done | `data/t2-beck-transfer-complexity-policy.csv`; CLI gate |
| 03 - Review and close | done | `CLOSE.md`; `panels/policy/review.md`; final gates |

## Done Criteria

- Every transfer-complexity review row has one policy row.
- Policy rows preserve `map;promotion;publication` blockers with
  `claim_blocker_delta = 0`.
- Rows route to policy acceptance before any blocker relief.
- Optimizer and release manifests register the artifact.
- Final gates pass before close.

## Non-Goals

- Do not accept transfer-complexity policy in this wave.
- Do not reduce T2 Beck transfer-complexity blockers.
- Do not publish final Beck replacement geometry.
