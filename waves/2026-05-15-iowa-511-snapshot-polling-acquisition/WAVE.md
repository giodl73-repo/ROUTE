---
wave: iowa-511-snapshot-polling-acquisition
date_open: 2026-05-15
status: done
---

# Iowa 511 Snapshot Polling Acquisition

## Mission

Run the executable Iowa 511 polling path and record real source-acquisition
progress against the source snapshot evidence hold.

## Opening Rule

This is acquisition, not evidence promotion. New Iowa 511 rows may extend the
source window, but the T1/T1 evidence claim must remain held unless the evidence
window is reviewed as repeated-window or archive evidence.

## Inputs Inherited

- `scripts/poll-t1-iowa511.ps1`
- `data/t1-failure-events.csv`
- `data/t1-evidence-windows.csv`
- `docs/evidence-campaigns/milepost-9-iowa-repeat-window.md`

## Pulse Status

| Pulse | Status | Output |
|---|---|---|
| Iowa 511 snapshot polling acquisition | done | 4 net-new `data/t1-failure-events.csv` rows; updated `data/t1-evidence-windows.csv` |

## Done Criteria

- Polling path completes without source or observation gate failure.
- New normalized observations are accumulated into `data/t1-failure-events.csv`.
- Evidence-window row remains `snapshot_only` and `promotion_eligible=false`.
- The source snapshot evidence hold is preserved.

## Non-goals

- Do not mark T1 evidence as promotion eligible.
- Do not clear the source snapshot guard.
- Do not commit raw cache payloads under `data/cache/`.
