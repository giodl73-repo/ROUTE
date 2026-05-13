# Tier Optimizer Design

## Goal

Build one principled optimizer for tier lines, stops, contacts, and schematic layout.
The optimizer should replace the current pattern of selecting routes first and then
repairing stops/map geometry by hand.

The optimizer's decision object is a segment bundle, not a route label. A
bundle may currently contain one physical segment, but it is still the service
object the optimizer selects, draws, upgrades, repairs, and simulates.
`route` remains useful display/input shorthand, and `national_segment_id`
remains the member-level physical key, but selected columns, repair witnesses,
map rows, game hooks, and promotion dockets should join through the bundle
grammar in
`docs/route-architecture.md` and `docs/national-segment-identity-spec.md`.

The desired outcome is:

```text
SLA promises -> bundle columns -> tier lines -> stop set -> contact validity -> schematic layout -> T2/T3 attachments
```

with feedback loops when a later constraint proves an earlier choice infeasible.
This is not a one-way waterfall. The optimizer must visit T1 through T4 and
then roll evidence back upward:

```text
T1 national promise spine
  -> T2 regional connectors and relief
  -> T3 zone feeders and regional access
  -> T4 local access / last-hour obligations
  -> upward pressure on T3/T2/T1 when lower-tier access cannot attach cleanly
```

The final accepted plan is the fixed point where each tier has a role, every
lower tier can attach to real higher-tier contacts, and any unmet obligation is
recorded as a repair or source-needed witness.

## Algorithm Pattern

ROUTE should reuse the algorithm vocabulary from the apportionment Algorithm
Atlas (`C:\src\apportionment\docs\algorithm-atlas`) rather than inventing a
bespoke selector. The direct translations are:

| Atlas pattern | ROUTE translation |
|---|---|
| ApportionRegions reusable spine | T1 national spine that remains comparable across budgets and scenarios |
| Capacity-constrained clustering | Stop and route budgets with explicit capacity/status lineage |
| Flow construction | Assign lower-tier demand, city pairs, and access obligations to eligible tier corridors |
| Branch-and-price columns | Candidate bundle/stop columns selected by a master problem |
| Local search | Validity-preserving repairs after an initial tier plan exists |
| Pareto frontier | Compare competing national designs under SLA, budget, resilience, and access objectives |
| Audit certificate fixed point | One verifier contract for selected lines, stops, contacts, exceptions, and generated maps |

The first ROUTE implementation should be deterministic and auditable, not an
opaque optimizer. It should produce candidate bundles, selected bundles,
rejected bundles, repair actions, and infeasibility witnesses.

## Recursive Regionalization

The optimizer is closer to bisection/regionalization than to a flat route ranker.
It divides the country into service regions, gives each region a full lower-tier
treatment, and then subdivides again.

```text
National problem
  -> choose reusable T1 spine
  -> split into T1-bounded service regions
  -> solve full T2 treatment inside each region
  -> split each T2 region into T3 zones
  -> solve T4 local access inside each T3 zone
  -> roll infeasible lower-tier witnesses back up
```

The T1 spine is therefore both a route set and a partition boundary. T2 should
not be selected as one national list of thin lines. Each T1-bounded region gets
its own T2 treatment: regional connectors, relief loops, terminals, duplicate
checks, and stop rhythm. The same pattern repeats for T3 and T4.

This changes the decision object from:

```text
route id -> selected/rejected
```

to:

```text
region workload -> candidate segment/bundle/stop columns -> selected regional treatment
```

The bisection analogy is useful but not literal. ROUTE regions do not need equal
population or area. They need complete service treatment under the promise
horizon for that tier.

## Implementation Steals From Apportionment

The nearby `C:\src\apportionment` workspace has concrete Rust patterns worth
copying into ROUTE crates:

| Source crate | What to borrow | ROUTE target |
|---|---|---|
| `bisect-apportion` | deterministic workload/spine tree | `route-network` regional workload tree |
| `bisect-clustering` | `Valid` / `NeedsRepair` / `InfeasibleCapacity` status and repair lineage | `route-network` tier-region status |
| `bisect-flow` | supply-to-capacity assignment plus infeasibility witness | `route-network` access/SLA demand assignment |
| `bisect-column` | column generation, master problem, selected column ids | new `route-network` or `route-optimizer` selector module |
| `bisect-local-search` | validity-preserving repair over an accepted plan | topology/contact repair pass |
| `bisect-pareto` | frontier rows with validity status and selected-frontier lineage | multi-objective design comparison |
| `rplan-audit` | shared certificate/lineage discipline | ROUTE optimizer run manifest and verifier |

Do not import redistricting semantics directly. Borrow the machinery shape:
stable candidate ids, status enums, column/master separation, repair summaries,
and witness artifacts.

## ROUTE Compositor Stack

BISECT already separates compositor choice into independent layers:

```text
SplitStrategy     = what structure/tree of splits?
WeightSpec        = what edge and vertex signals matter?
SeedCompositor    = how is the candidate/search space explored?
AlgorithmConfig   = preset + user overrides + manifest mode name
```

ROUTE should copy that shape almost directly:

```text
RegionStrategy    = what service-region tree?
ServiceWeightSpec = what demand, SLA, topology, and evidence signals matter?
SearchCompositor  = single, multi, convergence, percentile, frontier
TierOptimizerConfig = preset + overrides + manifest mode name
```

Proposed ROUTE enums:

```rust
pub enum RegionStrategy {
    StandardBisection,
    PrimeFactorSpine,
    CapacityClustering,
    FlowConstruction,
    Spectral,
    Regionalization,
}

pub struct ServiceWeightSpec {
    pub sla_promise: bool,
    pub freight_market: bool,
    pub top_city_pairs: bool,
    pub intermodal_access: bool,
    pub resilience: bool,
    pub stop_spacing: bool,
    pub evidence_penalty: bool,
    pub duplicate_penalty: bool,
}

pub enum SearchCompositor {
    Single,
    Multi { candidates: usize },
    ConvergenceSweep { threshold: u32 },
    Percentile { percentile_bps: u16, candidates: usize },
    ParetoFrontier { selected_index: usize },
}
```

This gives us the same extensibility rule as BISECT:

- changing regional structure should touch only `RegionStrategy`;
- adding a new evidence/signal dimension should touch only `ServiceWeightSpec`;
- changing how many alternatives we explore should touch only `SearchCompositor`;
- CLI/config presets should build a `TierOptimizerConfig` and preserve the mode
  name in artifacts.

Linear route stop selection follows the same rule. METIS is always the splitter,
but the input weights decide what a good split means:

```text
EqualStops       -> unit stop weights
EqualDistance    -> stop or segment-mile weights
EqualFreight     -> truck/freight demand weights
EqualPopulation  -> market/population catchment weights
HybridService    -> weighted blend for SLA + freight + spacing
```

For a linear route, the engine receives a path graph with one vertex per
candidate stop. A k-way METIS split yields k contiguous stop regions when the
partition is valid, and the useful stop-selection output is the k - 1 boundary
pairs between adjacent regions.

## Primal And Dual Graphs

The optimizer must declare which graph it is splitting.

```text
Primal stop graph:
  vertices = cities, stops, interchanges, terminals
  edges    = route segments between stops
  use      = route stop spacing, SLA path validity, bend/transfer truth

Dual route graph:
  vertices = routes or route/service columns
  edges    = shared stops, transfer contacts, overlaps, or relief relationships
  use      = T1/T2/T3 regionalization, duplicate service review, parent-trunk grouping
```

Both graphs are valid METIS inputs. The difference is what the vertex and edge
weights mean. A T1 line may be evaluated on the primal graph for stop selection
and on the dual graph for region membership. A T2 treatment should usually start
on the dual route graph, then expand into primal stop graphs inside each region.

## Porting Order

1. Copy the pure split-schedule idea from `bisect-core::bisection` into ROUTE as
   a `RegionTree`. It should produce stable region ids, depths, paths, and
   target treatment counts.
2. Copy the `AlgorithmConfig` separation from `bisect-cli::runner` as
   `TierOptimizerConfig`.
3. Copy the status vocabulary from `bisect-clustering` and `bisect-flow`:
   `valid`, `needs-repair`, `infeasible-capacity`, `invalid`.
4. Copy the candidate-column/master split from `bisect-column`: generate
   route/stop/service columns first, then select a compatible covering set.
5. Copy the local-search summary shape from `bisect-local-search` for topology
   repair: moves evaluated, moves accepted, status, before/after score.
6. Copy the selected-frontier lineage from `bisect-pareto` once multiple viable
   national designs exist.

The first code slice should avoid METIS or heavy graph partitioning. Start with
the pure scheduler/config/status pieces because they are small, deterministic,
and immediately useful for T1/T2/T3/T4 artifacts.

## Constraint Order

The optimizer uses lexicographic constraint ordering. A lower-numbered class can
force changes to a higher-numbered class; a higher-numbered class cannot silently
override a lower-numbered class.

| Order | Constraint class | Meaning | Can force |
|---|---|---|---|
| 0 | Evidence admissibility | Do not promote incomplete, unbuilt, or source-gated corridors without an explicit exception row | Reject/demote route candidates |
| 1 | Promise portfolio | T1 must satisfy the designated 48h/36h national SLA pairs under the route/stop budget | Required T1 line candidates |
| 2 | Budget | Route count, stop count, build class, and score-exception caps | Cutline and replacement choices |
| 3 | Network topology | Selected routes must form a connected, auditable graph with real contacts, not near-misses | Add transfer stops, reject dangling lines |
| 4 | Stop qualification | Endpoints and major transfer nodes must qualify for the tier they serve | Promote/demote stops or replace route |
| 5 | Stop rhythm | Max gap and service spacing must satisfy tier stop standards | Add service stops or mark a gap |
| 6 | Redundancy and duplication | Parallel lines must provide distinct service, resilience, or market coverage | Demote duplicate T2/T3 lines |
| 7 | Schematic geometry | Beck layout must preserve stop order, contacts, and bends only at selected stops | Move stops/layout; cannot invent connectivity |
| 8 | T2/T3 attachment | Lower tiers attach to the accepted higher-tier graph and inherit contact rules | Promote/demote connectors |
| 9 | Game/use-case overlays | Incidents, upgrades, throughput, and special lanes attach only to validated tier assets | Hold scenario claims |

## Optimizer Loop

The optimizer should run in passes, but each pass writes a reasoned artifact.

1. Build candidate route universe from current tier table, scored corpus, SLA pair
   shortest paths, known completed interstates, and exception ledgers.
2. Generate candidate route/stop columns. A column is a whole proposal such as
   `I84 as T1 with POR-BOI-SLC stops`, not only an isolated route id.
3. Solve T1 line selection against Class 0-2 constraints.
4. Generate required T1 stops from route endpoints, route intersections, top-city
   promise anchors, spacing gaps, relay hubs, and source-backed terminals.
5. Validate T1 topology against Class 3-5 constraints.
6. If topology or stops fail, return a structured repair action:
   `add_stop`, `replace_route`, `demote_route`, `add_exception`, or `source_needed`.
7. Freeze the accepted T1 graph for this run.
8. Solve T2 against the frozen T1 graph: every T2 must connect to real T1/T2
   contacts, avoid duplicate service unless it has unique markets, and meet its
   24h/12h promise horizon where applicable.
9. Solve T3 zone feeders and T4 local access obligations against the accepted
   T1/T2 graph.
10. Roll lower-tier failures upward. If T3/T4 demand repeatedly fails because no
   valid attachment exists, emit a candidate T2 upgrade or T1 repair witness.
11. Run validity-preserving local repair: add a stop, move a connector to a real
   contact, swap a duplicate route, or demote an infeasible line.
12. Generate a Pareto frontier of feasible or repaired plans when there are
   multiple viable trade-offs.
13. Select one frontier entry by declared policy and package its lineage.
14. Generate Beck layout from selected stops and contacts. Layout may distort
   geography, but it may not create bends between stops or pretend a near miss is
   a transfer.
15. Emit review dockets for unresolved policy choices.

## Tier Roles

| Tier | Primary promise | Selection unit | Must connect to | Typical repair pressure |
|---|---|---|---|---|
| T1 | 48h/36h national freight promises, top-25 city-pair coverage, national resilience | national spine column and first-level region boundary | other T1 lines at real transfer hubs | add T1 stop, replace route, or record score exception |
| T2 | 24h/12h regional freight, T1 relief, secondary metro attachment | full regional treatment inside a T1-bounded region | at least two system contacts unless terminal-worthy exception | demote duplicate, split by parent trunk, or upgrade stop |
| T3 | 6h regional feeder and zone access | zone treatment inside a T2/T1 service region | T1/T2 contacts and regional hubs | create regional inset, add transfer stop, propose T2 upgrade |
| T4 | 1h local access and last-mile service | local access treatment inside a T3 zone | nearest qualified T3/T2/T1 access point | expose access gap or promote feeder candidate |

This means T4 is not "afterthought local roads." T4 pressure can reveal that a
T3 feeder is missing; repeated T3 failures can reveal a T2 gap; repeated T2
attachment failures can reveal a T1 stop or line-design problem.

## Upward Feedback Rule

Lower-tier pressure is allowed to reopen higher-tier decisions, but only through
the same constraint order that selected the higher tier in the first place.

For T1, that means a T2/T3/T4 route cannot become a national candidate because
it is high-scoring, locally useful, or visually convenient on the Beck map. It
must supply at least one named T1 dependency:

1. a designated 48h/36h SLA pair whose path improves or becomes feasible;
2. a T1 stop, transfer, or topology repair witness that the accepted graph needs;
3. an evidence-backed exception row that explains why the promise portfolio must
   change.

The feedback docket therefore separates score-only pressure from promise-backed
pressure. Score-only rows can be visible review evidence, but they cannot
override the T1 promise portfolio.

## Objective Function

Within the hard constraint order, score candidates by:

```text
total_value =
  SLA_coverage
  + top_city_pair_coverage
  + freight_market_coverage
  + intermodal/port/border access
  + resilience/k-connectivity value
  + stop_spacing_improvement
  - route_budget_cost
  - stop_budget_cost
  - pavement_debt_budget_cost
  - duplicate_service_penalty
  - unbuilt_or_source_gap_penalty
  - schematic_complexity_penalty
```

T1 uses national SLA coverage as the dominant objective. T2 uses regional SLA,
relief, and attachment value. T3/T4 use access coverage and local service value.
Pavement debt is carried as a budget penalty from
`data/tier-pavement-debt-budget.csv`: it can make one otherwise valid bundle
more expensive than another, but it does not erase the bundle identity or hide
the service relationship from maps, games, incidents, or later upgrade planning.

## Stop Generation Rules

T1 stops are not decorative labels. They are generated by this priority order:

1. SLA promise origins/destinations and top-25 city anchors.
2. T1/T1 intersections and required transfer hubs.
3. T1/T2 contacts that create regional attachment value.
4. Ports, borders, intermodal terminals, and military/logistics anchors with
   source-backed evidence.
5. Spacing stops needed to keep the maximum T1 stop gap within the service rhythm.
6. Schematic bend stops needed to keep Beck geometry truthful.

If a route requires a bend or transfer at a place that is not a selected stop,
the optimizer must either select that stop or reroute/demote the line.

## Current Working Example

The I-69/I-84 decision illustrates the ordering:

- I-69 fails Class 0 for T1 because it is not fully connected as the current
  national promise path and has no current 48h/36h promise pair.
- I-84 can enter as a conditional score-backbone exception, but Class 4 and 5
  require real T1 stops: Portland, Boise, Pendleton, Twin Falls, and Salt Lake.
- T2 routes that previously leaned on I-69 or near-Chicago contact nodes must
  reconnect to actual T1 stops or be demoted/reworked.

## Implementation Targets

Near-term artifacts:

```text
data/tier-optimizer-runs.csv
data/tier-region-workloads.csv
data/tier-candidate-columns.csv
data/t1-stop-selector.csv
data/t1-topology-repairs.csv
data/tier-infeasibility-witnesses.csv
```

Near-term CLI:

```text
route tier-optimize --tier T1 --gate
route tier-optimize --all-tiers --gate
route tier-regions --tier T2 --gate
route t1-stop-selector --gate
route t1-topology-repairs --gate
```

The first implementation should not try to be a perfect mathematical solver.
It should be a deterministic constraint engine that emits ranked choices and
repair actions. Once the artifacts are stable, the route and stop budgets can be
swapped for an ILP or min-cost flow solver if needed.
