---
wave: milestone-10-t2-local-zone-handoff-effects-compatibility
pulse: 01
date: 2026-06-25
status: done
---

# Pulse 01 - Local-Zone Handoff Preserves Effects

## Deliverables

- Add `qualification_effects` to `T2LocalZoneOverlayHandoffRow`.
- Copy service repair docket qualification effects into local-zone handoff rows.
- Add focused local-zone handoff coverage.

## Gates

- `cargo test -q -p route --bin route t2_local_zone_overlay_handoff`
- `cargo test -q -p route --bin route t2_service_overlay_diagnostic_decisions`
- `npm run check:l2`

## Non-goals

- Do not regenerate `data/t2-local-zone-overlay-handoff.csv`; current data does
  not need a semantic row update for this compatibility assertion.
- Do not change local-zone handoff routing.
- Do not resolve unrelated `data/t1-design-review.csv` edits.

## Result

Done. Local-zone overlay handoff rows now preserve qualification effects from
service repair dockets.
