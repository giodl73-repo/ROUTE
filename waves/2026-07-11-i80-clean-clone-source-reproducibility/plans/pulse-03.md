---
wave: i80-clean-clone-source-reproducibility
pulse: 03
date: 2026-07-11
status: done
depends_on:
  - pulse-02
governing_roles:
  - citation-auditor
  - numeracy-checker
  - scope-keeper
---

# Pulse 03 - No-Credential Blocker Repair

## Mission

Resolve the Indiana HPMS blocker and either repair or formally disposition the
I-80 FEMA source path.

## Deliverables

- [x] Inspect the Indiana HPMS service schema and route rows.
- [x] Add `route_name` fallback when `route_signing` is null.
- [x] Re-fetch Indiana and pass all 11 I-80 state coverage.
- [x] Test a corridor-scale I-80 FEMA tile plan.
- [x] Record FEMA endpoint and coverage failure without partial promotion.
- [x] Exclude the legacy FEMA path pending a bounded replacement adapter.

## Gates

- `cargo test -q --locked -p route-data`
- `cargo test -q --locked -p route --bin route`
- `npm run test:i80:sources`
- `npm run gate:i80:sources:no-credential`
- HPMS reports 11/11 I-80 states.
- `git diff --check`

## Non-Goals

- Invent an Indiana shared-route alias.
- Treat failed FEMA requests as zero exposure.
- Select a replacement flood source without research.
- Add credentialed sources.
- Regenerate the reviewed I-80 report.
- Resolve unrelated `data/t1-design-review.csv` worktree state.

## Result

Indiana was a parser defect: the service publishes `route_name = "I 80"` while
`route_signing` is null. The fallback now preserves 42,970 I-80 rows across all
11 states.

The FEMA path was dispositioned rather than forced through. A 49-tile I-80
attempt exceeded the bounded run window and returned pervasive timeout or
non-JSON failures before an atomic output could be written. The existing legacy
tile set is not I-80 coverage. FEMA remains blocked and excluded from reviewed
regeneration until a replacement adapter is selected.
