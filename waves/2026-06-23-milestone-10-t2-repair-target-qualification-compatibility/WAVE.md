---
wave: milestone-10-t2-repair-target-qualification-compatibility
date_open: 2026-06-23
date_close: 2026-06-23
status: done
source: goal-resume
---

# Milestone 10 T2 Repair Target Qualification Compatibility

## Mission

Prepare the bundle-overlay repair target surface to preserve selector-facing
qualification effects when future binding decisions carry them, without breaking
existing repair-target CSVs that predate the column.

## Pulses

| Pulse | Status | Evidence |
|---|---|---|
| 01 - Repair targets accept qualification effects | done | `T2BundleOverlayRepairTargetRow`; `cargo test -q -p route --bin route t2_bundle_overlay_repair_targets_classify_residual_decisions`; `npm run check:l2` |

## Close Evidence

`T2BundleOverlayRepairTargetRow` now has a defaulted `qualification_effects`
field, and generated future repair target rows copy it from
`T2GameOpsBindingDecisionRow`. The field is defaulted so historical repair-target
CSVs remain readable.
