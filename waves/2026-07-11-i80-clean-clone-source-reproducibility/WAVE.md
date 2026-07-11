---
wave: i80-clean-clone-source-reproducibility
date_open: 2026-07-11
status: active
source: user-selected-next-trigger
---

# I-80 Clean-Clone Source Reproducibility

## Mission

Make the reviewed I-80 measurement report reproducible from a clean clone
without committing raw source data or silently substituting partial inputs.

## Opening Rule

Source acquisition is not evidence acceptance. Every source must pass download,
parse, coverage, year, and claim-use gates before it can regenerate the reviewed
corpus.

## Pulses

| Pulse | Status | Evidence |
|---|---|---|
| 01 - Source acquisition inventory | done | `research/i80-clean-clone-source-reproducibility.md`; `data/i80-report-source-contract.csv` |
| 02 - No-credential acquisition path | planned | TIGER, Gazetteer, HPMS, and FEMA orchestration with parse gates |
| 03 - Credential and adapter sources | planned | ACS key support plus RUCC, AFDC, NBI, and FARS adapter decisions |
| 04 - Clean-clone report regeneration | planned | One command, source manifest, reviewed-output comparison, and CI proof |
| 05 - Closeout | planned | Source posture, remaining holds, and next trigger |

## Done Criteria

- Every required cache has an authoritative source and explicit access mode.
- One command acquires or explicitly blocks every required input.
- Missing credentials or manual sources produce actionable failures.
- Source years and artifact names are not misleading.
- A clean checkout can regenerate the I-80 report or produce a complete,
  machine-readable blocker record without overwriting the reviewed anchor.

## Non-Goals

- Commit raw transportation datasets.
- Fabricate unavailable source rows.
- Treat download success as claim validation.
- Expand beyond the I-80 report source bundle.
- Resolve unrelated `data/t1-design-review.csv` worktree state.
