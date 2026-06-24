---
wave: milestone-10-t2-repair-target-qualification-compatibility
pulse: 01
date: 2026-06-23
status: done
---

# Pulse 01 - Repair Targets Accept Qualification Effects

## Deliverables

- Add `qualification_effects` to `T2BundleOverlayRepairTargetRow`.
- Copy `qualification_effects` from `T2GameOpsBindingDecisionRow` when repair
  target rows are generated.
- Default the field during deserialization so existing repair-target CSVs remain
  readable.
- Add gate coverage so future repair targets with qualification effects cannot
  drop their qualification gate/game-use contract.

## Gates

- `cargo test -q -p route --bin route t2_bundle_overlay_repair_targets_classify_residual_decisions`
- `npm run check:l2`

## Non-goals

- Do not regenerate `data/t2-bundle-overlay-repair-targets.csv`; the current
  selector budget has no active unrelieved intake/decision rows.
- Do not change repair classification or target status policy.
- Do not resolve unrelated `data/t1-design-review.csv` edits.

## Result

Done. Future bundle-overlay repair targets preserve qualification effects from
binding decisions, while existing repair-target CSVs remain compatible.
