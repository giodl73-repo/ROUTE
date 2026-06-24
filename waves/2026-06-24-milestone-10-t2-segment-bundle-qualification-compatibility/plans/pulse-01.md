---
wave: milestone-10-t2-segment-bundle-qualification-compatibility
pulse: 01
date: 2026-06-24
status: done
---

# Pulse 01 - Segment Bundles Accept Qualification Effects

## Deliverables

- Add `qualification_effects` to `NationalSegmentBundleRow`.
- Roll up `qualification_effects` from `NationalSegmentRegistryRow` members for
  future bundle rows.
- Default the field during deserialization so existing bundle CSVs remain
  readable.
- Gate that non-empty registry qualification effects survive bundle generation.

## Gates

- `cargo test -q -p route --bin route national_segment`
- `npm run check:l2`

## Non-goals

- Do not regenerate `data/national-segment-bundles.csv`; the current bundle data
  does not need a semantic row update for this compatibility field.
- Do not change bundle readiness, bundle membership, or next-artifact decisions.
- Do not resolve unrelated `data/t1-design-review.csv` edits.

## Result

Done. Future national segment bundle rows preserve qualification effects from
registry members, while existing bundle CSVs remain compatible.
