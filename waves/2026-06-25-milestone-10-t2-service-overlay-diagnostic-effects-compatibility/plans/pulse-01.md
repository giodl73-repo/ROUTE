---
wave: milestone-10-t2-service-overlay-diagnostic-effects-compatibility
pulse: 01
date: 2026-06-25
status: done
---

# Pulse 01 - Service Overlay Decisions Preserve Effects

## Deliverables

- Add `qualification_effects` to `T2ServiceOverlayDiagnosticDecisionRow`.
- Copy service repair docket qualification effects into service-overlay
  diagnostic decisions.
- Add focused coverage for the service repair to diagnostic decision handoff.

## Gates

- `cargo test -q -p route --bin route t2_service_overlay_diagnostic_decisions`
- `cargo test -q -p route --bin route t2_local_zone_overlay_handoff`
- `npm run check:l2`

## Non-goals

- Do not regenerate `data/t2-service-overlay-diagnostic-decisions.csv`; current
  data does not need a semantic row update for this compatibility assertion.
- Do not change service-overlay decision routing.
- Do not resolve unrelated `data/t1-design-review.csv` edits.

## Result

Done. Service-overlay diagnostic decisions now preserve qualification effects
from service repair dockets.
