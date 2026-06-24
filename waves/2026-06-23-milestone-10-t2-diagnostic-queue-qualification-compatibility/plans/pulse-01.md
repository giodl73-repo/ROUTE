---
wave: milestone-10-t2-diagnostic-queue-qualification-compatibility
pulse: 01
date: 2026-06-23
status: done
---

# Pulse 01 - Diagnostic Queue Accepts Qualification Effects

## Deliverables

- Add `qualification_effects` to `T2ServiceDiagnosticQueueRow`.
- Copy `qualification_effects` from `T2ServiceSelectionRow` when diagnostic queue
  rows are generated.
- Default the field during deserialization so existing diagnostic queue CSVs
  remain readable.
- Carry qualification effects into the diagnostic optimizer effect when present.

## Gates

- `cargo test -q -p route --bin route t2_service_diagnostic_queue`
- `npm run check:l2`

## Non-goals

- Do not regenerate `data/t2-service-diagnostic-queue.csv`; the current selector
  budget has no active unrelieved qualification-bearing T2 diagnostic rows.
- Do not change diagnostic queue selection or routing.
- Do not resolve unrelated `data/t1-design-review.csv` edits.

## Result

Done. Future T2 service diagnostic queue rows preserve qualification effects from
service selection, while existing CSVs remain compatible.
