---
wave: milestone-10-t2-regionalizer-qualification-compatibility
pulse: 01
date: 2026-06-23
status: done
---

# Pulse 01 - Regionalizer Accepts Qualification Effects

## Deliverables

- Add `qualification_effects` to `TierCandidateColumnRow`.
- Add `qualification_effects` to `T2RegionalizerRow`.
- Carry the value from optimizer constraint budget into candidate columns, then
  into T2 regionalizer rows.
- Default the field during deserialization so existing candidate and regionalizer
  CSVs remain readable.

## Gates

- `cargo test -q -p route --bin route t2_regionalizer_includes_selected_and_review_columns`
- `cargo test -q -p route --bin route t2_service_selection_joins_regionalizer_to_beck_diagnostics`
- `npm run check:l2`

## Non-goals

- Do not regenerate `data/tier-candidate-columns.csv` or
  `data/t2-regionalizer.csv`; the current selector budget has no active
  unrelieved qualification-bearing T2 regionalizer rows.
- Do not change candidate or regionalizer selection policy.
- Do not resolve unrelated `data/t1-design-review.csv` edits.

## Result

Done. Future T2 candidate/regionalizer rows preserve qualification effects from
budget rollup, while existing CSVs remain compatible.
