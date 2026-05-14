---
wave: t2-beck-long-connector-policy
type: review
status: done
---

# Long-Connector Policy Review

## Finding

The three T2 long-connector review routes now have policy rows that define how
Beck rendering should preserve connector service while requiring trunk-interface
labeling and explicit local-service beads. The policy classifies high and
severe long connectors.

## Doctrine Check

- Policy rows preserve blockers until acceptance.
- Rendering treatment is authored, not applied.
- Promotion remains held until accepted policy is replayed through blocker
  relief and the optimizer ledger.

## Residual Holds

All three `map;promotion;publication` blockers remain held pending
`data/t2-beck-long-connector-policy-acceptance.csv`.
