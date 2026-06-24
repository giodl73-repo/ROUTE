---
wave: milestone-10-t2-service-selection-qualification-compatibility
pulse: 01
date: 2026-06-23
status: done
---

# Pulse 01 - Service Selection Accepts Qualification Effects

## Deliverables

- Add `qualification_effects` to `T2ServiceSelectionRow`.
- Copy `qualification_effects` from `T2RegionalizerRow` when service selection
  rows are generated.
- Default the field during deserialization so existing service-selection CSVs
  remain readable.

## Gates

- `cargo test -q -p route --bin route t2_regionalizer_includes_selected_and_review_columns`
- `cargo test -q -p route --bin route t2_service_selection_joins_regionalizer_to_beck_diagnostics`
- `npm run check:l2`

## Non-goals

- Do not regenerate `data/t2-service-selection.csv`; the current selector budget
  has no active unrelieved qualification-bearing T2 regionalizer rows.
- Do not change service selection policy.
- Do not resolve unrelated `data/t1-design-review.csv` edits.

## Result

Done. Future T2 service selection rows preserve qualification effects from
regionalizer rows, while existing CSVs remain compatible.
