---
wave: milestone-10-t2-service-diagnostic-effects-compatibility
pulse: 01
date: 2026-06-24
status: done
---

# Pulse 01 - Diagnostics Preserve Service Effects

## Deliverables

- Add positive diagnostic queue coverage for service-selection qualification
  effects.
- Confirm diagnostic optimizer effects include the pipe-delimited qualification
  text.
- Preserve diagnostic status, next-artifact, and route-family split behavior.

## Gates

- `cargo test -q -p route --bin route t2_service_diagnostic_queue`
- `cargo test -q -p route --bin route t2_service_selection_joins_regionalizer_to_beck_diagnostics`
- `cargo test -q -p route --bin route t2_route_family_splits`
- `npm run check:l2`

## Non-goals

- Do not regenerate `data/t2-service-diagnostic-queue.csv`; current data does
  not need a semantic row update for this compatibility assertion.
- Do not change diagnostic routing.
- Do not resolve unrelated `data/t1-design-review.csv` edits.

## Result

Done. Diagnostic queue coverage now locks in preservation of service-selection
qualification effects.
