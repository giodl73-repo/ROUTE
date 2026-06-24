---
wave: milestone-10-t2-service-repair-qualification-compatibility
pulse: 01
date: 2026-06-23
status: done
---

# Pulse 01 - Service Repair Docket Accepts Qualification Effects

## Deliverables

- Add `qualification_effects` to `T2ServiceClassRepairDocketRow`.
- Copy `qualification_effects` from `T2BundleOverlayRepairTargetRow` when docket
  rows are generated.
- Default the field during deserialization so existing docket CSVs remain
  readable.
- Carry qualification effects into the docket optimizer effect when present.

## Gates

- `cargo test -q -p route --bin route t2_service_class_repair_docket_routes_local_zone_holds`
- `npm run check:l2`

## Non-goals

- Do not regenerate `data/t2-service-class-repair-docket.csv`; the current
  selector budget has no active unrelieved intake/decision rows.
- Do not change service repair classification or routing.
- Do not resolve unrelated `data/t1-design-review.csv` edits.

## Result

Done. Future service-class repair docket rows preserve qualification effects from
repair targets, while existing docket CSVs remain compatible.
