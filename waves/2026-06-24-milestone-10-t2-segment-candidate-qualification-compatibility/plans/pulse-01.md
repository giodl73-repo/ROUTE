---
wave: milestone-10-t2-segment-candidate-qualification-compatibility
pulse: 01
date: 2026-06-24
status: done
---

# Pulse 01 - Segment Candidates Accept Qualification Effects

## Deliverables

- Add `qualification_effects` to `TierSegmentCandidateRow`.
- Copy `qualification_effects` from `T2ServiceSelectionRow` for future T2
  segment candidate rows.
- Merge route-family split `qualification_effects` when a split row has
  additional route-family qualification effects for the same service.
- Default the field during deserialization so existing candidate CSVs remain
  readable.

## Gates

- `cargo test -q -p route --bin route tier_segment_candidates`
- `npm run check:l2`

## Non-goals

- Do not regenerate `data/tier-segment-candidates.csv`; the current candidate
  data does not need a semantic row update for this compatibility field.
- Do not change segment membership, bundle ids, or stitch groups.
- Do not resolve unrelated `data/t1-design-review.csv` edits.

## Result

Done. Future T2 segment candidate rows preserve qualification effects from
service selection and route-family split inputs, while existing candidate CSVs
remain compatible.
