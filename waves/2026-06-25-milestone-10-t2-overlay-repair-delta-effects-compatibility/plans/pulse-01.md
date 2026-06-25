---
wave: milestone-10-t2-overlay-repair-delta-effects-compatibility
pulse: 01
date: 2026-06-25
status: done
---

# Pulse 01 - Repair Deltas Preserve Effects

## Deliverables

- Add `qualification_effects` to `T2BundleOverlayRepairDeltaRow`.
- Merge decision and repair-target effects into repair deltas.
- Add focused repair-delta coverage.

## Gates

- `cargo test -q -p route --bin route t2_bundle_overlay_repair_delta`
- `cargo test -q -p route --bin route t2_overlay_optimizer_action_docket`
- `npm run check:l2`

## Non-goals

- Do not regenerate `data/t2-bundle-overlay-repair-delta.csv`; current data does
  not need a semantic row update for this compatibility assertion.
- Do not change replay decisions or blocker preservation.
- Do not resolve unrelated `data/t1-design-review.csv` edits.

## Result

Done. Bundle overlay repair deltas now preserve qualification effects from
decision and repair-target inputs.
