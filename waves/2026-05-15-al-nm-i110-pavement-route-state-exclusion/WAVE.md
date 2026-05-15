---
wave: al-nm-i110-pavement-route-state-exclusion
date_open: 2026-05-15
status: done
---

# AL/NM I-110 Pavement Route-State Exclusion

## Mission

Remove the AL / I-110 and NM / I-110 pavement repair debt rows from T2
asset-condition debt only because an official route source proves those
route-state pairings are unsupported.

## Opening Rule

Do not treat missing repair funding as relief. Only route-state pairs absent
from an official route log may enter the route-state exclusion overlay, and the
effect must replay through generated pavement debt and optimizer artifacts.

## Inputs Inherited

- `data/tier-pavement-route-state-exclusions.csv`
- `data/tier-pavement-debt-budget.csv`
- `data/tier-pavement-repair-debt-review.csv`
- `data/optimizer-constraint-ledger.csv`
- `data/optimizer-residual-blocker-backlog.csv`
- [FHWA Interstate Route Log and Finders List, Table 2](https://www.fhwa.dot.gov/planning/national_highway_system/interstate_highway_system/routefinder/table02.cfm)

## Pulse Status

| Pulse | Status | Output |
|---|---|---|
| AL/NM I-110 route-state exclusion | done | `data/tier-pavement-route-state-exclusions.csv`; generated debt and optimizer replay |

## Done Criteria

- The exclusion source lists I-110 state coverage and does not list Alabama or
  New Mexico.
- Both exclusions are represented in the governed route-state exclusion overlay.
- T2 asset-condition debt decreases by exactly the two excluded I-110 bundles.
- Pavement, optimizer, publication, optimizer-manifest, and release-manifest
  gates pass.

## Non-goals

- Do not clear California or Louisiana I-110 repair debt.
- Do not clear Louisiana I-220 repair debt.
- Do not change T4 terminal-access or T1 snapshot evidence holds.
