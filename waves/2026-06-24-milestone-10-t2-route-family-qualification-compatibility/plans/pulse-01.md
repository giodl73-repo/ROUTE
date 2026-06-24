---
wave: milestone-10-t2-route-family-qualification-compatibility
pulse: 01
date: 2026-06-24
status: done
---

# Pulse 01 - Route-Family Splits Accept Qualification Effects

## Deliverables

- Add `qualification_effects` to `T2RouteFamilySplitRow`.
- Copy `qualification_effects` from `T2ServiceDiagnosticQueueRow` for diagnostic
  driven split rows.
- Default the field during deserialization so existing split CSVs remain
  readable.
- Carry qualification effects into the route-family optimizer effect when
  present.

## Gates

- `cargo test -q -p route --bin route t2_route_family_splits`
- `npm run check:l2`

## Non-goals

- Do not regenerate `data/t2-route-family-splits.csv`; the current selector
  budget has no active unrelieved qualification-bearing route-family rows.
- Do not change route-family split decisions.
- Do not resolve unrelated `data/t1-design-review.csv` edits.

## Result

Done. Future route-family split rows preserve qualification effects from service
diagnostics, while existing split CSVs remain compatible.
