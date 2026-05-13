---
wave: constraint-ledger-blocker-burndown
pulse: 01
date: 2026-05-13
status: planned
depends_on: []
governing_roles:
  - optimization-methodologist
  - freight-economist
  - traffic-engineer
  - scope-keeper
---

# Pulse 01 - I-84 T1 Hard Blocker Decision

## Mission

Resolve or deliberately carry the single current hard blocker in the constraint
budget: `I84` as a T1 `promise_portfolio` exception.

## Scope Inventory

| Surface | Current artifact | Pulse target |
|---|---|---|
| Hard blocker row | `data/optimizer-constraint-ledger.csv` (`CON-T1TOPO-I84`) | Decide whether I-84 earns a T1 exception or should be demoted/held. |
| Selector exception | `data/t1-score-exceptions.csv` | Record the exception, demotion, or review action. |
| T1 selector | `data/t1-line-selector.csv` | Regenerate and verify the blocker no longer appears as an unresolved hard blocker unless intentionally held. |
| Constraint budget | `data/optimizer-constraint-budget.csv` | Confirm hard-blocker count and top class changes. |

## Deliverables

- [ ] Run a bounded counterfactual: `justify-as-national-relay` vs
  `demote-to-t2`.
- [ ] Update `data/t1-score-exceptions.csv` with the chosen decision and
  evidence basis.
- [ ] Regenerate affected optimizer artifacts.
- [ ] Update docs or review notes if the decision changes T1 exception doctrine.
- [ ] Add or update tests if selector behavior changes.

## Expected Gates

- `route t1-sla-candidate-pairs --gate`
- `route tier-optimize --all-tiers --gate`
- `route optimizer-constraint-ledger --gate`
- `route optimizer-constraint-budget --gate`
- `route optimizer-manifest --gate`
- `route release-manifest --gate`
- `cargo test -p route`

## Non-Goals

- Do not rebalance the whole T1 portfolio unless the counterfactual proves the
  I-84 decision cannot be localized.
- Do not use score alone to keep I-84 in T1.

