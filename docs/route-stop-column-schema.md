# Route And Stop Column Schema

## Purpose

ROUTE is moving from hand-picked route ids to auditable candidate columns.

A column is a bundled proposal that can be selected, rejected, repaired, or held
by the optimizer. It is not just a line name. It carries graph context, promise
lineage, stop obligations, contact evidence, budget cost, and the next artifact
needed to resolve it.

This document owns the common schema vocabulary for T1/T2/T3/T4 route, stop,
service, and repair artifacts.

The core column abstraction is the bundle. A route column, service column,
repair witness, map hook, incident row, or game overlay should identify the
service/corridor with `segment_bundle_id` unless it is deliberately describing
one physical member segment. Stop columns may attach to a bundle, a member
segment, or both, depending on whether the stop belongs to the service or to a
particular physical extent.

Implementation note: Rust producers and consumers should use the shared
`route-network` bundle types instead of local rollup structs. The CLI may
serialize those bundles to CSV, but the bundle logic belongs in
`route_network::build_segment_bundles`. Downstream crate entrypoints should be
bundle-facing: `route_score::score_bundle`, `route_map::build_bundle_svg`,
`route_sim::BundleIncidentSpec`, and
`route_report::write_bundle_corpus_entry`. `route bundle-architecture --gate`
checks those entrypoints and writes `data/bundle-architecture.csv`.
For efficient lookup, consumers should use `route_network::BundleRegistry`
rather than building ad hoc route-label maps.

Column artifacts are not allowed to treat a route label as stable identity once
they describe a physical segment or service. New service/corridor columns
should carry `segment_bundle_id` and join through
`data/national-segment-bundles.csv`; lower-level physical segment rows should
carry `national_segment_id` or point to `data/national-segment-registry.csv`.

## Column Types

| Column type | Meaning | Current examples |
|---|---|---|
| Promise-pair column | Timed freight promise that can force route/stop candidates | `data/t1-sla-candidate-pairs.csv` |
| Route column | Candidate route spine or connector with score, tier, and SLA lineage | `data/t1-line-selector.csv`, `data/tier-candidate-columns.csv` |
| Stop column | Ordered stop or boundary proposal along a route | `data/t1-stop-selector.csv` |
| Service column | Regional service treatment for a selected or reviewed route | `data/t2-service-selection.csv` |
| Repair witness | Structured infeasibility or repair action produced by a later gate | `data/t1-topology-repairs.csv`, `data/t1-feedback-docket.csv` |
| Debt budget column | Payment/capital debt attached to a selected bundle without blocking identity | `data/tier-pavement-debt-budget.csv` |
| Manifest row | Run-level certificate tying commands to artifacts and gate status | `data/tier-optimizer-runs.csv` |

Future T2/T3/T4 artifacts should reuse these categories rather than inventing
new decision vocabularies.

## Required Lineage Fields

Every optimizer column artifact should expose these fields or an obvious
equivalent:

| Field | Meaning |
|---|---|
| `tier` | T1, T2, T3, or T4 scope |
| `segment_bundle_id` | Stable bundle identity when the row describes a service, corridor, route column, map row, incident, upgrade, or game overlay |
| `national_segment_id` | Stable physical member identity when the row deliberately describes one segment inside a bundle |
| `column_id` or stable natural key | Stable identifier for diffing and references |
| `candidate_type` | Promise pair, route, stop, service, repair, or manifest |
| `graph_kind` | Primal stop graph, dual route graph, path graph, or none |
| `split_objective` | METIS or selector objective, such as hybrid service or route-mile workload |
| `source_artifact` | Artifact that produced the row when not implicit from command |
| `required_artifact` or `next_artifact` | Next artifact needed for proof or repair |
| `evidence_status` | Accepted, heuristic, source-needed, policy-action, or held |
| `validation_status` | Pass, review, held, or fail |

Older artifacts may not yet include every field. New artifacts should.

## Decision Vocabulary

Selectors and repair dockets should prefer these decision words:

| Decision | Meaning |
|---|---|
| `selected` / `select` | Accepted into the current optimizer plan |
| `review` | Still plausible but requires another artifact or human policy decision |
| `blocked` | Cannot proceed without source, contact, endpoint, or graph proof |
| `demote` | Valid work, but belongs to a lower tier or local treatment |
| `replace` | Higher-priority constraint says this candidate should be swapped out |
| `held-known` | Known blocker carried intentionally in a manifest |
| `pass` | Gate accepts the row as complete for this stage |

Do not use geometry words such as `draw`, `bend`, or `nearby` as final
decisions. Geometry can diagnose a defect, but the decision must resolve to a
selection, repair, demotion, or hold.

## Promise-Pair Columns

Promise-pair columns answer: which service promises are important enough to
shape the tier graph?

Required fields:

- `pair_id`
- `origin_id`
- `dest_id`
- `target_hours`
- `market_class`
- score inputs and `total_score`
- `portfolio_selected`
- `selected_budget`
- `cutline_status`
- `cutline_reason`
- `covered_by_selected_pair`
- `required_routes`
- `required_stops`
- `evidence_basis`
- `validation_status`

The selected promise-pair portfolio is allowed to force T1 routes. A dropped
promise pair may still matter as T2/T3 pressure, but it cannot silently shape
T1 without becoming a selected promise or repair witness.

## Route Columns

Route columns answer: should this route or route segment be in the tier plan?

Required fields:

- `tier`
- `route`
- `score` or route value proxy
- `rank`
- `selected` or `column_decision`
- `sla_pairs` or promise lineage when applicable
- `selected_stops` or stop obligations when applicable
- `budget_cost`
- `pavement_debt_cost_m`, `pavement_debt_class`, and `pavement_debt_basis` when
  the candidate carries pavement evidence or repair debt
- `reason` or `repair_basis`
- `validation_status`

T1 route columns are national spine candidates. T2 route columns are regional
service candidates inside T1-bounded regions. T3/T4 route columns should be
zone or local-access candidates rather than national overlays.

## Stop Columns

Stop columns answer: where does service actually stop, transfer, bend, or split?

Required fields:

- `route`
- `stop_sequence`
- `stop_id`
- `stop_name`
- `requested_class`
- `selector_weight`
- `split_objective`
- `target_regions`
- `metis_region`
- `boundary_after`
- `evidence_status`
- `validation_status`

For Beck rendering, a bend is legal only if it is attached to a selected stop,
transfer, or explicit schematic bend stop. The stop column is therefore a map
constraint, not a label list.

## Service Columns

Service columns answer: what regional treatment does a lower-tier route provide?

Required fields:

- `tier`
- `region_id`
- `route`
- `parent_trunks`
- `column_decision`
- `treatment_status`
- contact or diagnostic evidence
- duplicate/parallel service diagnostics
- pavement debt cost/class/basis copied from the candidate or regionalizer row
- `selection_action`
- `selection_basis`
- `validation_status`

T2 service columns must distinguish parent-trunk lineage from visual color.
Color follows service lineage; it does not create service lineage.

## Repair Witnesses

Repair witnesses answer: what later constraint failed, and what can fix it?

Required fields:

- affected route, stop, pair, or component id
- source artifact
- blocker or witness type
- repair action
- repair basis
- next artifact
- optimizer effect
- validation status

Repair witnesses are the only way lower-tier pressure should move upward. A
score-only lower-tier row may be visible, but it cannot become a T1 candidate
without a named SLA, stop, topology, or exception witness.

## Graph Contracts

ROUTE uses two graph views:

| Graph | Vertices | Edges | Column use |
|---|---|---|---|
| Primal stop graph | stops, terminals, interchanges, cities | route segments between stops | stop order, spacing, transfers, bends |
| Dual route graph | routes or service columns | shared stops, overlaps, parent links, relief relationships | regionalization, duplicate review, service grouping |
| Path graph | ordered stops on one route | adjacent stop segments | linear route splitting with METIS |

Every graph-backed column should declare which graph it used and what the
weights meant.

## Gate Requirements

A column gate should fail when:

- selected rows exceed route, stop, or service budget;
- selected rows lack required promise, stop, or contact lineage;
- dropped rows lack a cut-line or repair reason;
- a repair row lacks a next artifact;
- a map-facing row requires a bend or transfer at an unselected node;
- a manifest marks a stage pass while its artifact is missing or empty.

Review rows are allowed. Unnamed blockers are not.

## Current Gaps

- `data/tier-candidate-columns.csv` has strong T2 route-column lineage but does
  not yet share one typed schema with T1 route columns.
- `data/t1-line-selector.csv` predates `candidate_type` and `graph_kind`.
- T3/T4 pressure artifacts are still intake/review surfaces, not full regional
  route/stop/service columns.
- Map diagnostics are not yet unified with repair witnesses.

These are compatibility gaps, not doctrine gaps. New artifacts should follow
this schema, and older artifacts can migrate incrementally.
