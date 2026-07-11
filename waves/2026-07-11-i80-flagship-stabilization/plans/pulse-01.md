---
wave: i80-flagship-stabilization
pulse: 01
date: 2026-07-11
status: done
depends_on: []
governing_roles:
  - scope-keeper
  - citation-auditor
  - numeracy-checker
---

# Pulse 01 - Anchor Contract and Baseline

## Mission

Replace the ambiguous post-Milestone 10 state with one active I-80 flagship
execution rail and a factual baseline of what is complete, draft, held, or
missing.

## Scope Inventory

- `GOAL.md`
- `TRACKER.md`
- `waves/PHASES.md`
- `corpus/existing/i80.md`
- `data/scores-all.csv`
- `gaps/`
- `reviews/`
- current CI and dependency configuration

## Deliverables

- [x] Create the active flagship wave.
- [x] Freeze expansion outside the anchor.
- [x] Reconcile the current goal and tracker focus.
- [x] Record the current I-80 evidence and product baseline.
- [x] Define the remaining flagship pulses and exit gates.

## Gates

- `git diff --check`
- Planning documents agree on the active wave.
- The baseline does not promote draft, heuristic, or held claims.
- Existing unrelated worktree changes are not included.

## Non-Goals

- Complete the I-80 narrative.
- Select an investment treatment.
- Generate new maps or data.
- Modify Rust code.
- Resolve unrelated `data/t1-design-review.csv` worktree state.

## Result

The I-80 flagship is now the sole active execution focus. The baseline records
the difference between ROUTE's broad internal capability and the incomplete
anchor deliverable, and the next pulse is bounded to completing the existing
corridor record.
