---
wave: milestone-10-t2-blocker-closure-qualification-compatibility
pulse: 01
date: 2026-06-24
status: done
---

# Pulse 01 - Blocker Closures Accept Qualification Effects

## Deliverables

- Add `qualification_effects` to `T2BlockerClosureRow`.
- Copy `qualification_effects` from route-matched `NationalSegmentBundleRow`
  inputs for future blocker closure rows.
- Default the field during deserialization so existing closure CSVs remain
  readable.
- Gate that non-empty closure qualification effects remain attached to a segment
  bundle.

## Gates

- `cargo test -q -p route --bin route t2_blocker_closure`
- `npm run check:l2`

## Non-goals

- Do not regenerate `data/t2-blocker-closure.csv`; the current closure data does
  not need a semantic row update for this compatibility field.
- Do not change blocker class, closure status, or route-family split decisions.
- Do not resolve unrelated `data/t1-design-review.csv` edits.

## Result

Done. Future blocker-closure rows preserve qualification effects from national
segment bundle inputs, while existing closure CSVs remain compatible.
