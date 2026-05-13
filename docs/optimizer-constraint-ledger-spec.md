# Optimizer Constraint Ledger Spec

## Purpose

The optimizer constraint ledger is the common contract for every T1/T2/T3/T4
constraint that can block, price, penalize, repair, or annotate a route bundle.

Pavement debt proved the pattern: a selected bundle can be real and useful while
still carrying a payment obligation before it may claim SLA, transit, upgrade,
or publication readiness. That rule is not pavement-specific. Bridges,
clearance, capacity, topology, source gaps, duplicate service, stop spacing,
resilience, and Beck geometry need the same row discipline.

The ledger prevents a return to hand-picked route heuristics. Every constraint
that affects line selection, stop selection, tier promotion, rendering, game
hooks, or release claims must become a typed row before the optimizer compares
candidates.

## Core Rule

Constraints attach to bundle identity first.

Use `segment_bundle_id` when a row describes a service, corridor, selected line,
candidate column, map row, incident, upgrade, or game object. Use
`national_segment_id` only when the row deliberately describes one physical
member segment inside a bundle. Use `stop_id`, `pair_id`, `region_id`, or
`map_id` as secondary scope keys, not replacements for bundle identity when a
bundle exists.

Pavement, bridge, clearance, and condition constraints may originate at member
segment level, but their optimizer-facing effects must roll up to bundle rows so
selectors can compare service options without rejoining by route label.

## Constraint Order

The ledger expands the lexicographic order in `docs/tier-optimizer-design.md`
without changing its principle: lower-order constraints can force changes to
higher-order constraints; higher-order constraints cannot silently override
lower-order ones.

| Order | Constraint class | Meaning |
|---|---|---|
| 0 | `evidence_admissibility` | Source, completion, and exception status required before a candidate can be promoted as real. |
| 1 | `promise_portfolio` | T1/T2/T3/T4 promise horizons that force service coverage: 48/36h, 24/12h, 6h, and 1h. |
| 2 | `identity_and_bundle` | Segment ids, bundle ids, stitch groups, aliases, and graph-member continuity. |
| 3 | `route_budget` | Route, bundle, build-class, and capital program limits. |
| 4 | `stop_budget` | Stop count, endpoint count, transfer count, hub class, and schematic node budget. |
| 5 | `topology_connectivity` | Real graph connectivity, contacts, no near-miss transfers, and no disconnected selected components. |
| 6 | `endpoint_and_transfer_qualification` | Endpoint, hub, terminal, transfer, port, border, and intermodal qualification. |
| 7 | `stop_rhythm_and_spacing` | Maximum gaps, relay rhythm, line split points, and tier stop cadence. |
| 8 | `asset_condition_debt` | Pavement, bridge, clearance, grade, safety, and condition obligations. |
| 9 | `capacity_and_sla_reliability` | Throughput, PTI, relay feasibility, special lane need, and promise reliability. |
| 10 | `resilience_and_alternate_path` | Redundancy, cut vulnerability, detour quality, and recovery paths. |
| 11 | `duplication_and_parallel_service` | Duplicate T2/T3 service, close parallel lines, loops, and redundant market behavior. |
| 12 | `lower_tier_attachment` | T2/T3/T4 attachment to accepted higher-tier contacts and upward pressure witnesses. |
| 13 | `schematic_geometry` | Beck stop order, bends only at selected stops, map spacing, label pressure, and color lineage. |
| 14 | `game_ops_publication_readiness` | Scenario hooks, incidents, upgrades, publication claims, and user-facing overlays. |

Orders 3 and 4 split the older generic budget class because ROUTE now treats
routes and stops as separate scarce design resources. Order 8 generalizes the
current pavement debt ledger into a broader asset-condition debt family.

## Behavior Types

Each ledger row must say how the optimizer should treat the constraint.

| Behavior | Meaning | Optimizer effect |
|---|---|---|
| `identity-blocker` | The service/bundle identity is invalid or cannot be constructed. | Candidate cannot be selected until repaired. |
| `selection-hard` | The candidate exists but violates a hard selector rule. | Reject, demote, or require an explicit exception row. |
| `claim-blocker` | The bundle can exist, but cannot claim one or more services. | Keep addressable; block named claims such as SLA, transit, publication, or game readiness. |
| `budget-debt` | The bundle is valid but carries a priced source, repair, or upgrade obligation. | Add cost to the objective and payment plan. |
| `penalty-soft` | The candidate is worse than an alternative but not forbidden. | Add comparable penalty score. |
| `review` | Human/policy review is needed and the next artifact is known. | Carry visibly; do not promote as resolved. |

A row may block a claim without blocking identity. This distinction is central:
maps, games, incidents, and acquisition plans can still target a bundle while
SLA or publication claims remain held.

## Ledger Row Schema

The normalized optimizer ledger artifact is:

```text
data/optimizer-constraint-ledger.csv
```

Required fields:

| Field | Meaning |
|---|---|
| `constraint_id` | Stable row id for diffing, reviews, and repair references. |
| `optimizer_run_id` | Run or manifest id that produced the row. |
| `tier` | `T1`, `T2`, `T3`, `T4`, or `all`. |
| `region_id` | Service region, zone, or `national` when applicable. |
| `constraint_order` | Numeric order from this spec. |
| `constraint_class` | One of the typed classes above. |
| `behavior_type` | One of the behavior types above. |
| `constraint_scope` | `bundle`, `segment`, `route`, `stop`, `pair`, `region`, `map`, `game`, or `run`. |
| `subject_id` | Primary scoped id for the row. |
| `segment_bundle_id` | Bundle id when the row affects a service/corridor. |
| `national_segment_id` | Member segment id when the row is member-specific. |
| `stitch_group_id` | Continuity group when the row concerns stitched service. |
| `route` | Display/input route label, never the primary key by itself. |
| `stop_id` | Stop, transfer, endpoint, or bend node when relevant. |
| `pair_id` | Promise pair when relevant. |
| `map_id` | Map or schematic artifact when relevant. |
| `source_artifact` | Artifact that produced the row. |
| `source_row_id` | Stable source-row key when available. |
| `evidence_status` | `accepted`, `source-needed`, `heuristic`, `planned`, `exception`, or `missing`. |
| `constraint_status` | `pass`, `review`, `blocked`, `debt`, `penalty`, `held`, or `fail`. |
| `blocks_claims` | Pipe-delimited claim families blocked by this row, such as `sla|transit|publication`. |
| `budget_cost_m` | Cost in millions when the row creates capital/source/payment debt. |
| `budget_units` | Optional non-dollar budget units such as stops, lanes, crews, or route slots. |
| `penalty_score` | Comparable soft penalty, with higher meaning worse. |
| `repair_action` | Deterministic repair action such as `add_stop`, `replace_route`, `source_needed`, `pay_debt`, `split_bundle`, or `demote`. |
| `payment_action` | Acquisition, repair, upgrade, or funding action when debt exists. |
| `next_artifact` | Artifact or command expected to close, review, or carry the row. |
| `optimizer_effect` | Short explanation of how this row affects selection. |
| `validation_status` | `pass`, `review`, `held`, `blocked`, or `fail`. |

Optional producers may include richer source-specific columns, but candidate
selectors and manifests should rely only on the normalized fields above.

## Aggregation Contract

Source ledgers can stay specialized. The normalized ledger is the join surface.

Examples of source ledgers:

| Source family | Current or expected source |
|---|---|
| Pavement and ride quality | `data/tier-pavement-debt-budget.csv` |
| Segment and bundle identity | `data/national-segment-registry.csv`, `data/national-segment-bundles.csv` |
| T1 promise portfolio | `data/t1-sla-pairs.csv`, `data/t1-sla-candidate-pairs.csv` |
| T1 line and stop selection | `data/t1-line-selector.csv`, `data/t1-stop-selector.csv` |
| Topology repair | `data/t1-topology-repairs.csv`, T2 contact witnesses |
| T2 service duplication | `data/t2-parallel-service-queue.csv`, `data/t2-service-selection.csv` |
| Lower-tier access pressure | `data/t3-t4-access-gaps.csv`, `data/t1-feedback-docket.csv` |
| Beck rendering | `data/beck-t1-diagnostics.csv`, `data/t3-zone-map-diagnostics.csv` |
| Game and operations hooks | `data/game/t2-bundle-overlays.csv`, scenario hook ledgers |

Future CLI:

```text
route optimizer-constraint-ledger --gate
route optimizer-constraint-budget --gate
```

`optimizer-constraint-ledger` normalizes source rows into
`data/optimizer-constraint-ledger.csv`. `optimizer-constraint-budget` rolls those
rows up by candidate, bundle, tier, and region so selectors can consume one
budget/penalty vector instead of joining every source ledger independently.

## Candidate Summary Fields

Every new route, stop, service, map, or game-facing candidate artifact should
expose these summary fields once the normalized ledger exists:

| Field | Meaning |
|---|---|
| `hard_blocker_count` | Count of unresolved `identity-blocker` and `selection-hard` rows. |
| `claim_blocker_count` | Count of unresolved claim blockers. |
| `constraint_debt_cost_m` | Sum of budget debt rows affecting the candidate. |
| `constraint_penalty_score` | Sum or declared aggregate of soft penalties. |
| `top_constraint_classes` | Pipe-delimited highest-impact unresolved classes. |
| `constraint_ledger_artifact` | Usually `data/optimizer-constraint-ledger.csv`. |

Existing pavement fields can remain during migration, but they should become one
asset-condition slice of `constraint_debt_cost_m`, not a permanent special case.

## Gate Rules

The normalized ledger gate should fail when:

- a selected candidate has an unresolved `identity-blocker`;
- a selected candidate has an unresolved `selection-hard` row and no exception;
- a constraint row lacks a stable subject id;
- a segment-bearing row has neither `segment_bundle_id` nor
  `national_segment_id`;
- a blocker, debt, or review row lacks `next_artifact` or `repair_action`;
- a budget-debt row lacks `budget_cost_m` or `payment_action`;
- a claim-blocker row does not name `blocks_claims`;
- a source-specific artifact invents a new constraint class not listed here.

The gate may pass with claim blockers, budget debt, penalties, or review rows
when those rows are visible, priced or scored when appropriate, and attached to
their next artifact.

## Implementation Path

1. Inventory current blocker/debt/diagnostic fields and map them to the
   constraint classes in this spec.
2. Add `route optimizer-constraint-ledger --gate` as a normalizer over current
   source ledgers, starting with pavement debt, topology repairs, T2 parallel
   service review, T3/T4 access gaps, and Beck diagnostics.
3. Add `route optimizer-constraint-budget --gate` to roll up selected rows by
   `segment_bundle_id`, candidate id, tier, and region.
4. Update T1/T2 selectors to consume the aggregate constraint budget instead of
   pavement-only columns.
5. Expand T3/T4 selectors to emit their access and attachment blockers into the
   same ledger.
6. Add manifest rows so the all-tier optimizer bundle proves the ledger and
   aggregate budget were generated for the same run.

The target is not to make every constraint hard. The target is to make every
constraint typed, priced or blocked when appropriate, and impossible to hide.
