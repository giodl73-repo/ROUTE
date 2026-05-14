---
wave: t2-beck-long-connector-policy-acceptance
pulse: 02
status: done
---

# Pulse 02 - Acceptance Surface

## Deliverable

Generate `data/t2-beck-long-connector-policy-acceptance.csv` from the authored
policy rows.

## Gates

- Every policy row has one acceptance row.
- `claim_blocker_delta = 0` for every row.
- The next artifact is `data/t2-beck-long-connector-blocker-relief.csv`.

## Result

Done by `route t2-beck-long-connector-policy-acceptance --gate`.

