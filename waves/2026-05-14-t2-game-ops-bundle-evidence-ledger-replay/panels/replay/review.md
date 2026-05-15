---
wave: t2-game-ops-bundle-evidence-ledger-replay
type: review
status: reviewed
rubric_version: v1.0
author: route-wave
created: 2026-05-14
updated: 2026-05-14
sources:
  - data/t2-game-ops-bundle-evidence-blocker-relief.csv
  - data/optimizer-constraint-ledger.csv
  - data/optimizer-constraint-budget.csv
  - data/optimizer-residual-blocker-backlog.csv
---

# Replay Review - T2 Game/Ops Bundle Evidence

## Findings

1. Relief replay is scoped to accepted rows with `blocker_count_after = 0` and
   negative blocker delta.
2. Original `game_ops_bundle_binding` blockers are suppressed only by matching
   `segment_bundle_id`.
3. The I-110 mixed-family row remains represented through asset-condition debt;
   only its game/ops bundle-binding blockers are relieved.
4. Budget rows must be grouped by the same subject key represented by
   `budget_id`. Region-only separation created duplicate bundle budget ids when
   pass relief and debt rows shared a subject.

## Verdict

The replay is accepted. It reduces game/ops bundle-binding claim blockers
without laundering unrelated asset debt or terminal-access evidence gaps.

