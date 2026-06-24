---
wave: milestone-10-t2-regionalizer-qualification-compatibility
date_open: 2026-06-23
date_close: 2026-06-23
status: done
source: goal-resume
---

# Milestone 10 T2 Regionalizer Qualification Compatibility

## Mission

Prepare tier candidate columns and the T2 regionalizer to preserve
selector-facing qualification effects when future budget rows carry them, without
breaking existing candidate or regionalizer CSVs that predate the column.

## Pulses

| Pulse | Status | Evidence |
|---|---|---|
| 01 - Regionalizer accepts qualification effects | done | `TierCandidateColumnRow`; `T2RegionalizerRow`; focused regionalizer/service-selection tests; `npm run check:l2` |

## Close Evidence

`TierCandidateColumnRow` and `T2RegionalizerRow` now have defaulted
`qualification_effects` fields. Candidate generation copies the value from
`OptimizerConstraintBudgetRow`, and regionalizer generation copies it from
candidate rows while historical CSVs remain readable.
