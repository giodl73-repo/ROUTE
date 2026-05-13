# Optimizer Constraint Ledger Spec Review

Date: 2026-05-13

## Scope

Reviewed:

- `docs/optimizer-constraint-ledger-spec.md`

Roles used:

- `.roles/parliament/optimization-methodologist.md`
- `.roles/parliament/traffic-engineer.md`
- `.roles/parliament/freight-economist.md`
- `.roles/parliament/schematic-cartographer.md`
- `.roles/stakeholders/state-dot.md`
- `.roles/editorial/scope-keeper.md`

## Verdict

Pass as implemented optimizer doctrine for migrated blocker families; hold
future families until they join the same ledger and budget surface.

The spec correctly makes constraints first-class optimizer data instead of
allowing pavement, topology, Beck, source, stop, capacity, and game constraints
to become one-off selector code. It also preserves the core distinction between
bundle identity and service claims: a bundle may remain addressable while SLA,
transit, publication, or game-readiness claims are blocked.

The review required one spec tightening pass before acceptance. The spec now
adds measurement threshold fields, cost-basis fields, lifecycle/delivery fields,
and exception lineage fields so the ledger cannot hide vague blockers behind a
generic `constraint_status`.

## Findings

1. Optimization Methodologist: Pass after exception-lineage patch.
   The ledger separates hard blockers, claim blockers, budget debt, penalties,
   and review rows. The original draft allowed `selection-hard` rows to pass by
   exception without naming the exception artifact. The spec now requires
   `exception_id` and `exception_artifact` for selected candidates that carry a
   hard exception.

2. Traffic Engineer: Pass after measurement patch.
   The constraint classes include pavement, bridge, clearance, capacity, PTI,
   grade, and safety, but physical constraints need observed values, thresholds,
   units, and standards. The spec now requires `standard_artifact`,
   `observed_value`, `threshold_value`, and `measurement_unit` when a row claims
   pass/fail against a physical or operational standard.

3. Freight Economist: Conditional pass after cost-basis patch.
   `budget_cost_m` alone is not enough for capital allocation. The spec now
   adds `cost_category`, `cost_basis`, and `cost_confidence`, and it separates
   lifecycle/maintenance debt from one-time capital debt in candidate summaries.
   Investment-grade ROI still requires future value/benefit artifacts; this
   ledger only owns constraints and debt.

4. Schematic Cartographer: Pass.
   The spec keeps schematic geometry downstream of topology and stop truth.
   Beck rows can block map/publication claims, but they cannot invent contacts,
   bends, stops, or service identity. This is the right relationship between
   optimizer truth and map legibility.

5. State DOT Planner: Conditional pass after delivery patch.
   The first draft priced debt but did not identify who carries it or how it is
   delivered. The spec now includes `owner_jurisdiction`, `funding_program`, and
   `delivery_risk` so state/federal delivery limits can become visible
   constraints rather than prose caveats.

6. Scope Keeper: Pass.
   The artifact stays within a spec role. It defines a normalized artifact
   contract and implementation path; it does not claim the ledger commands are
   already implemented.

## Required Next Artifacts

Implemented:

- `data/optimizer-constraint-ledger.csv`
- `route optimizer-constraint-ledger --gate`
- `data/optimizer-constraint-budget.csv`
- `route optimizer-constraint-budget --gate`

Still required:

- future renderer, game, source, capacity, resilience, and asset-condition
  families to consume the same budget summary when they make new claim,
  upgrade, incident, publication, or payment decisions

## Decision

Accept `docs/optimizer-constraint-ledger-spec.md` as the doctrine for optimizer
constraint rows.

Treat the first ledger and budget slices as implemented for pavement debt, T1
topology repair, and T2 parallel-service rows. The T1 line selector now carries
constraint-adjusted score, blocker counts, debt, penalty, class summary, and
ledger artifact fields. T2 candidate columns, regionalizer rows, and
service-selection rows carry the same generalized constraint budget summary
alongside pavement compatibility fields. T3/T4 access gaps now enter the
normalized ledger, and T3 zone route columns, T4 terminal access columns, and
T3/T4 access gaps carry the resulting budget fields back through the lower-tier
selector cycle. T1/T2 Beck diagnostics now normalize schematic review flags into
claim-blocking ledger rows and roll up through the same budget surface. T2 game
scenario hooks, T2 bundle-bound overlays, and source-fetch policy rows now also
enter the normalized ledger and budget surface. Treat future renderer, game, and
source artifacts as incomplete until they join the same schema.

## Closeout Addendum

Pulse 07 closes the initial implementation wave. The normalized ledger now
contains 143 rows and the budget rollup contains 138 rows. The implemented
families include pavement debt, T1 topology/promise blockers, T2 duplicate
service review, T3/T4 access gaps, Beck diagnostics, source acquisition guards,
T2 scenario hooks, and T2 bundle-bound overlays. The acceptance condition is not
zero blockers; it is visible blockers with named claims, repair actions, and
next artifacts.
