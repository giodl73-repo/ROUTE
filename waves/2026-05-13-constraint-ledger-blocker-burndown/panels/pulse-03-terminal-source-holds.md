---
name: Pulse 03 Terminal Source Holds
slug: pulse-03-terminal-source-holds
type: review
status: reviewed
rubric_version: v1.0
author: route-pulse
created: 2026-05-13
updated: 2026-05-13
sources:
  - data/t4-terminal-access-columns.csv
  - data/t3-t4-access-gaps.csv
  - data/intermodal_terminals.csv
  - data/optimizer-constraint-budget.csv
---

# Pulse 03 Terminal Source Holds

## Decision

Pulse 03 carries all 69 `terminal_access_evidence_gap` rows as explicit source
holds. The pulse does not promote any T4 local pressure to scenario-ready,
T3-ready, or publication-ready status because no row yet proves route-to-terminal
contact.

## Hold Queue By Zone

| Zone | Held rows | Named terminal source obligation |
|---|---:|---|
| `t3-great-lakes` | 33 | Chicago Intermodal Complex, Columbus South, Indianapolis Avon, Detroit Livernois, Minneapolis Twin Cities, St. Louis Gateway, Philadelphia Frankford, or New York Fresh Pond |
| `t3-southeast` | 12 | Atlanta Hulsey, Charlotte Intermodal, Savannah Garden City, Miami Hialeah, or New Orleans Gentilly |
| `t3-mid-south` | 11 | Memphis Intermodal, Kansas City Gateway, St. Louis Gateway, New Orleans Gentilly, or Louisville KentuckyOne |
| `t3-mountain-west` | 9 | Denver Logistics Hub, Salt Lake City, Phoenix Sky Harbor area, Portland Albina, Seattle BNSF, Los Angeles/Long Beach, or Kansas City Gateway |
| `t3-texas-border` | 4 | Dallas Alliance, Houston Englewood, San Antonio Kirby, or New Orleans Gentilly |

## Review Notes

- The correct next artifact is still the T3/T4 access diagnostics and terminal
  source queue, not a benefit/cost or broad scenario run.
- `data/intermodal_terminals.csv` supplies named terminal districts, but it does
  not prove that each route has a one-hour contact to a selected T3/T2/T1 column.
- Release/publication status remains held for these claims until a route-level
  terminal contact source is attached.
