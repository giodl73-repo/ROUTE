---
wave: milestone-10-t2-parallel-service-effects-compatibility
pulse: 01
date: 2026-06-24
status: done
---

# Pulse 01 - Parallel Service Queue Preserves Effects

## Deliverables

- Add `qualification_effects` to `T2ParallelServiceQueueRow`.
- Copy service-selection qualification effects into close-parallel queue rows.
- Include qualification effects in queue optimizer-effect text.
- Add positive close-parallel queue coverage.

## Gates

- `cargo test -q -p route --bin route t2_parallel_service_queue`
- `cargo test -q -p route --bin route t2_service_diagnostic_queue`
- `cargo test -q -p route --bin route optimizer_constraint_ledger`
- `npm run check:l2`

## Non-goals

- Do not regenerate `data/t2-parallel-service-queue.csv`; current data does not
  need a semantic row update for this compatibility assertion.
- Do not change close-parallel review routing.
- Do not resolve unrelated `data/t1-design-review.csv` edits.

## Result

Done. Close-parallel service review rows now preserve service-selection
qualification effects.
