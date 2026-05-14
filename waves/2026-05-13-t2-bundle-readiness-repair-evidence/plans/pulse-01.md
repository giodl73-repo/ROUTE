---
wave: t2-bundle-readiness-repair-evidence
pulse: 01
date: 2026-05-13
status: done
depends_on: []
governing_roles:
  - optimization-methodologist
  - traffic-engineer
  - scope-keeper
---

# Pulse 01 - Evidence Probe Surface

## Mission

Create a gateable evidence probe for the four T2 bundle-readiness repair tasks.

## Scope Inventory

| Surface | Current artifact | Pulse target |
|---|---|---|
| Repair docket | `data/t2-bundle-readiness-repair-docket.csv` | Four repair tasks |
| Registry evidence | `data/national-segment-registry.csv` | stop-chain candidates |
| Candidate evidence | `data/tier-segment-candidates.csv` | stitched-member candidates |
| Service evidence | `data/t2-service-selection.csv` | terminal-stop candidates |

## Deliverables

- [x] Add `data/t2-bundle-readiness-repair-evidence.csv`.
- [x] Gate that every repair task has an evidence probe row.
- [x] Preserve game, incident, publication, and upgrade blockers.
- [x] Add tests for no repair completion from evidence probing.

## Evidence

- `cargo test -p route`
- `route t2-bundle-readiness-repair-evidence --gate`
- `data/t2-bundle-readiness-repair-evidence.csv`

## Expected Gates

- `route t2-bundle-readiness-repair-evidence --gate`
- `route t2-bundle-readiness-repair-docket --gate`
- `cargo test -p route`

## Non-Goals

- Do not update bundle readiness status in this pulse.
