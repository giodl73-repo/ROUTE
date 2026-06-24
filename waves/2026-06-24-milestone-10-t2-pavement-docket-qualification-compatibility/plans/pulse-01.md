---
wave: milestone-10-t2-pavement-docket-qualification-compatibility
pulse: 01
date: 2026-06-24
status: done
---

# Pulse 01 - Pavement Dockets Accept Qualification Effects

## Deliverables

- Add `qualification_effects` to `TierPavementDocketRow`.
- Copy `qualification_effects` from `TierSegmentCandidateRow` for future
  pavement docket rows.
- Default the field during deserialization so existing docket CSVs remain
  readable.
- Gate that non-empty segment candidate qualification effects survive into the
  pavement docket member row.

## Gates

- `cargo test -q -p route --bin route tier_pavement`
- `npm run check:l2`

## Non-goals

- Do not regenerate `data/tier-pavement-docket.csv`; the current docket data does
  not need a semantic row update for this compatibility field.
- Do not change pavement thresholds, repair decisions, or source contracts.
- Do not resolve unrelated `data/t1-design-review.csv` edits.

## Result

Done. Future pavement docket rows preserve qualification effects from segment
candidate inputs, while existing docket CSVs remain compatible.
