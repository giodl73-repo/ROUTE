---
wave: tx-i220-pavement-route-state-exclusion
date_open: 2026-05-15
status: done
---

# TX I-220 Pavement Route-State Exclusion

## Mission

Remove the TX / I-220 pavement repair debt row from T2 asset-condition debt only
if an official route source proves the route-state pairing is unsupported.

## Opening Rule

Do not clear pavement repair debt from lack of funding, convenience, or route
score usefulness. A route-state exclusion needs a source-backed route log or DOT
authority showing the route is not valid in the affected state, and the
exclusion must flow through generated pavement debt and optimizer artifacts.

## Inputs Inherited

- `data/tier-pavement-source-gaps.csv`
- `data/tier-pavement-debt-budget.csv`
- `data/tier-pavement-repair-debt-review.csv`
- `data/optimizer-constraint-ledger.csv`
- `data/optimizer-residual-blocker-backlog.csv`
- [FHWA Interstate Route Log and Finders List, Table 2](https://www.fhwa.dot.gov/planning/national_highway_system/interstate_highway_system/routefinder/table02.cfm)

## Pulse Status

| Pulse | Status | Output |
|---|---|---|
| TX I-220 route-state exclusion | done | `data/tier-pavement-route-state-exclusions.csv`; generated debt and optimizer replay |

## Done Criteria

- The exclusion source names I-220 state coverage and does not list Texas.
- The exclusion is represented as a governed input artifact rather than a manual
  edit to generated pavement debt rows.
- T2 asset-condition debt decreases by exactly the excluded TX / I-220 bundle.
- Downstream pavement funding, optimizer, publication, optimizer-manifest, and
  release-manifest gates pass.

## Non-goals

- Do not clear Louisiana I-220 or any I-110 repair debt.
- Do not infer accepted repair funding from state programs or generic pavement
  preservation budgets.
- Do not change T4 terminal-access or T1 snapshot evidence holds.
