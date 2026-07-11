---
name: I-80 No-Credential Blocker Repair 001
slug: i80-no-credential-blocker-repair-001
type: review
status: reviewed
rubric_version: v1.4
author: copilot
created: 2026-07-11
updated: 2026-07-11
sources:
  - crates/route-data/src/hpms_fetch.rs
  - data/cache/hpms_in.csv
  - data/cache/hpms_2018.csv
  - crates/route-cli/src/main.rs
  - data/i80-report-source-contract.csv
---

# I-80 No-Credential Blocker Repair 001

## Indiana HPMS

The official Indiana 2018 HPMS service contains I-80 rows with:

- `route_number = 80`
- `route_name = "I 80"`
- `f_system = 1`
- `route_signing = null`

ROUTE previously required `route_signing`, skipped those rows, and left only
unrelated signed rows in the Indiana cache. The parser now uses normalized
`route_name` when signing is absent.

After re-fetch:

| Measure | Result |
|---|---:|
| Indiana normalized rows | 19,938 |
| Indiana I-80 rows | 2,055 |
| I-80 rows across corridor cache | 42,970 |
| I-80 states covered | 11 / 11 |

Decision: **HPMS ready at the explicit 2018 vintage.**

## FEMA

A corridor-scale 49-tile I-80 attempt against the legacy NFHL ArcGIS endpoint
produced repeated timeouts, non-JSON responses, and an execution window longer
than the bounded source run. The command was stopped before replacing the prior
cache.

The existing `fetch-fema-d1` tile set targets Gulf, Atlantic, and Mississippi
Valley areas and cannot be described as I-80 coverage.

Decision: **endpoint blocked and excluded.**

This is not evidence that I-80 has zero SFHA exposure. ROUTE must select a
bounded replacement source or downloadable coverage adapter before restoring
FEMA to the reviewed report bundle.
