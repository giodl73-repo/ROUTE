---
wave: constraint-ledger-blocker-burndown
pulse: 01
date: 2026-05-13
status: done
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

- [x] Run a bounded counterfactual: `justify-as-national-relay` vs
  `demote-to-t2`.
- [x] Update `data/t1-score-exceptions.csv` with the chosen decision and
  evidence basis.
- [x] Regenerate affected optimizer artifacts.
- [x] Update docs or review notes if the decision changes T1 exception doctrine.
- [x] Add or update tests if selector behavior changes.

## Result

I-84 is kept as an explicit T1 national-relay exception, not as a hidden
score-only promotion. The bounded counterfactual showed that demoting I-84 to T2
does not produce a clean T1 replacement under the current route and stop budgets:
I-64 remains outside the selected T1 set without recutting budgets or promise
portfolio rows.

The selected I-84 row now uses `score-exception-keep`, the T1 design review marks
the row accepted, and the optimizer constraint ledger no longer emits
`CON-T1TOPO-I84`.

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

