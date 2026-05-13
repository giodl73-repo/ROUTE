---
name: I-84 T1 Exception Counterfactual
slug: pulse-01-i84-counterfactual
type: review
status: reviewed
rubric_version: v1.0
author: route-pulse
created: 2026-05-13
updated: 2026-05-13
sources:
  - data/t1-score-exceptions.csv
  - data/t1-line-selector.csv
  - docs/sla-promise-portfolio.md
---

# I-84 T1 Exception Counterfactual

## Question

Should I-84 remain a T1 score-backbone exception, or should the selector demote
it to T2 and backfill the T1 route budget?

## Counterfactual Result

| Case | I-84 result | Replacement result | Finding |
|---|---|---|---|
| Keep as national-relay exception | selected as explicit score exception with Portland, Boise, and Salt Lake City stops | I-64 remains outside the current route budget | Keeps the 11-line T1 spine and records that I-84 is not a formal 48h/36h promise corridor. |
| Demote to T2 | rejected by score-exception demotion | I-64 still fails the current stop budget as the next candidate | Demotion does not produce a clean T1 replacement without recutting route/stop budgets or the promise portfolio. |

## Decision

Keep I-84 as an explicit T1 national-relay exception.

This is not a score-only promotion. The exception is bounded to I-84's current
role as the Portland-Boise-Salt Lake City connector into the I-80/I-5 western
spine. SLA publication claims remain limited to selected 48h/36h promise pairs
until the promise portfolio is amended.

## Follow-Up

- If a future wave wants to remove I-84 from T1, it must recut the T1 route and
  stop budgets instead of assuming the next score-ranked route can backfill it.
- If Portland-Boise-Salt Lake City becomes a formal promise corridor, amend
  `data/t1-sla-candidate-universe.csv` and regenerate the candidate-pair
  portfolio rather than treating this exception as a hidden promise.

