---
name: Constraint Ledger Blocker Burn-Down R1 Consolidated Review
slug: blocker-burndown-r1-consolidated
type: review
status: reviewed
rubric_version: v1.0
author: route-review
created: 2026-05-13
updated: 2026-05-13
sources:
  - waves/2026-05-13-constraint-ledger-blocker-burndown/WAVE.md
  - docs/optimizer-constraint-ledger-spec.md
  - docs/sla-promise-portfolio.md
  - docs/t3-t4-access-optimization.md
---

# R1 - Consolidated Plan Review

## Verdict

Plan approved after one gate-name correction.

The wave has enough specs to start. It should not create a new doctrine spec
before Pulse 01. It should run one bounded I-84 counterfactual before changing
claim status, then classify T4 blockers before running any broad traffic, game,
or investment scenarios.

## Findings

| Severity | Artifact | Finding | Concrete fix |
|---|---|---|---|
| WARN | `plans/pulse-01.md` | The I-84 hard blocker is a real feasibility decision, not clerical cleanup. | Run `justify-as-national-relay` versus `demote-to-t2` as the first pulse deliverable, then update `data/t1-score-exceptions.csv`. |
| WARN | `plans/pulse-02.md` | T4 zone assignment can accidentally imply new zone doctrine. | Use existing `docs/t3-t4-access-optimization.md`; create a new spec only if the pulse introduces a new zone taxonomy or terminal source contract. |
| WARN | `plans/pulse-03.md` | Terminal evidence rows can be over-promoted into design claims. | Keep rows held unless source/terminal obligations are explicit and release claims remain guarded. |
| WARN | `plans/pulse-05.md` | The original generic Beck gate name was not implemented. | Corrected the plan to use `beck-t1-diagnostics`, `beck-t2-diagnostics`, `t1-beck-alignment`, and `map-atlas` gates. |
| NOTE | `WAVE.md` | Broad scenarios are useful only after blocker claims are named. | Defer traffic/game/investment scenarios until after I-84 and T4 blocker classification identifies specific claims. |

## Spec Answer

Existing specs are sufficient for the first two pulses:

- `docs/optimizer-constraint-ledger-spec.md` for blocker semantics.
- `docs/sla-promise-portfolio.md` for I-84 T1 exception/demotion rules.
- `docs/t3-t4-access-optimization.md` for T3/T4 zone and local-access doctrine.
- `docs/route-stop-column-schema.md` for decision vocabulary.

Potential spec work is conditional, not first: add or amend a T4 zone/terminal
source spec only if Pulse 02 cannot classify rows under the current T3/T4 access
doctrine.

## Scenario Answer

Yes, but only one scenario comes first: the bounded I-84 counterfactual inside
Pulse 01. Other scenarios should wait until blocker rows name the claim being
tested. Running broad scenarios before classification would produce attractive
but unauditable outputs.

