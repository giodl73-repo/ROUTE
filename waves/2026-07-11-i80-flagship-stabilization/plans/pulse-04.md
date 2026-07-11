---
wave: i80-flagship-stabilization
pulse: 04
date: 2026-07-11
status: done
depends_on:
  - pulse-03
governing_roles:
  - traffic-engineer
  - freight-economist
  - climate-engineer
  - scope-keeper
  - numeracy-checker
---

# Pulse 04 - I-80 Gap And Treatment Decision

## Mission

Identify the most decision-ready I-80 gap, compare bounded treatments, and
select no more than one treatment for Parliament review without converting
model or source gaps into an investment claim.

## Scope Inventory

- `corpus/existing/i80.md`
- `data/pressure-test-scenarios.csv`
- `data/throughput-proof-matrix.csv`
- `data/t1-intersection-failures.csv`
- `data/t1-evidence-windows.csv`
- `gaps/bottleneck.md`
- `docs/game/des-moines-diamond-g0.md`
- `docs/game/donner-weather-closure-g0.md`
- current Des Moines CLI outputs

## Deliverables

- [x] Separate measured, modeled, source-needed, and contradicted evidence.
- [x] Compare corridor-wide, Chicago, Donner, and Des Moines treatments.
- [x] Name a falsifier for every treatment retained for review.
- [x] Write one corridor-specific gap artifact.
- [x] Write one bounded design-review candidate.
- [x] Preserve construction, ROI, SLA, and agency-endorsement holds.

## Gates

- Current command evidence is recorded even when it contradicts older docs.
- No stale cost, NPV, or benefit claim is promoted.
- The selected treatment has an explicit review status and falsifier.
- `git diff --check`

## Non-Goals

- Approve construction or funding.
- Select a physical alignment.
- Claim the current simulation proves benefit.
- Resolve clean-clone source-cache reproducibility.
- Expand the flagship beyond I-80.
- Resolve unrelated `data/t1-design-review.csv` worktree state.

## Result

Des Moines is selected as the flagship design-review hypothesis because it is
the most bounded and falsifiable current mechanism: a named I-80 transfer node,
a snapshot observation path, a bound but zero-demand scenario, an executable
topology command, and an explicit independent-path question. The capital
decision remains held.

The current command run supersedes stale positive scenario outputs: with the
available cache it produced zero demand pairs and zero throughput, while the
diamond analyzer reported k=0 and three connectors needed. Parliament will
review the independent-transfer-path hypothesis and validation package, not a
claim that benefits, costs, geometry, or ROI have been proven.
