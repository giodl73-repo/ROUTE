# Tier Node Evaluation Design

## Goal

Build a repeatable process that decides which routes and nodes deserve national schematic prominence. This supports the Beck map, but the same data should also drive tier tables, review dockets, and future regional maps.

## Inputs

- `data/tier-table.csv`: current T1/T2/T3/T4 route assignment.
- Cached `HighwayGraph`: route edges, nodes, interchange flags, and route indices.
- Future evidence ledgers: terminal-worthy exceptions, port/border/military/logistics annotations, observed relief value.

## Route Connectivity Evaluation

Implemented in `route-network::tier`.

For each route in a selected tier:

1. Gather all graph nodes touched by the route.
2. For each route node, inspect incident edges.
3. Count incident T1 routes other than the route being evaluated.
4. Count distinct T1 touch nodes and distinct T1 trunks.
5. Classify:

```text
trunk_connector   >=2 T1 touch nodes and >=2 distinct T1 trunks
relief_loop       >=2 T1 touch nodes on the same T1 trunk
missing_graph_data 0 T1 touch nodes but >=75 route miles
one_ended_feeder  1 T1 touch node and >=75 route miles
local_spur        anything shorter/weaker
missing_graph_data also applies when the route has no graph edges
```

The rule is intentionally strict. It does not mean a route has no value; it means the route needs a different tier or an explicit exception record.

## CLI

```text
route tier-connectivity --tier T2
route tier-connectivity --tier T2 --details
route tier-connectivity --tier T2 --gate
route tier-connectivity --tier T2 --exceptions data/tier-node-exceptions.csv --gate
```

The report prints mileage, T1 touch-node count, distinct T1 trunk count, classification, endpoint exception summary, and optional touch-node coordinates.

## Endpoint Worthiness Evaluation

Endpoint worthiness is the second measure. It answers a different question:

```text
Does this line end at a place important enough for the requested tier?
```

This is especially important for one-ended T2 routes. A one-ended T2 is not automatically wrong, but the free end must be terminal-worthy.

Endpoint classes:

```text
national_terminal      T1-worthy endpoint
t2_terminal_exception  T2-worthy free end
regional_terminal      T3-worthy endpoint
local_access_end       T4/local endpoint
graph_endpoint_gap     endpoint/contact likely hidden by graph geometry
```

Endpoint evidence statuses:

```text
validated
heuristic
planned
missing_graph_data
demote
```

The exception ledger is the first data model:

```text
data/tier-node-exceptions.csv
route,requested_tier,endpoint_name,endpoint_role,exception_type,evidence_level,artifact,next_step
```

Standalone review:

```text
route endpoint-exceptions
route endpoint-exceptions --tier T2 --blockers
route endpoint-exceptions --route I65 --details
route endpoint-exceptions --gate
```

Plain `--gate` checks ledger completeness. `--blockers --gate` is stricter and fails rows that are complete but not terminal-worthy for the requested tier.

The route-connectivity gate joins this ledger:

- `trunk_connector`: passes connectivity; endpoint review still controls map terminal naming.
- `relief_loop`: may pass if relief role is source-backed or intentionally heuristic.
- `one_ended_feeder`: fails unless the free endpoint has a complete T2 terminal exception.
- `local_spur`: demote unless there is a rare source-backed national exception.
- `missing_graph_data`: route needs geometry/contact validation before tier judgment.

Implemented policy:

- A one-ended route can pass the connectivity gate only when the exception row is complete and terminal-worthy.
- A local spur can pass only with a complete terminal-worthy exception at `validated` evidence level.
- A missing-graph route cannot be promoted by endpoint exception alone. The exception is printed for review, but graph/contact evidence must be fixed before the tier can pass.

## Stop Investment Evaluation

Stop evaluation is the station-level process. It decides what should be named on the schematic and what deserves capital/operations investment.

Inputs already available:

- `data/relay-hubs.toml`: confirmed and proposed relay hubs.
- `data/intermodal_terminals.csv`: rail/truck, port/rail/truck, and air/truck terminals.
- `data/ports.csv`: major ports and border gateways.
- `data/atri-bottlenecks.csv`: high-delay interchange evidence.
- `data/tier-table.csv`: current route tier assignment.
- Cached `HighwayGraph`: route contacts and candidate interchange nodes.

Starter ledger:

```text
data/tier-stop-candidates.csv
```

Columns:

```text
stop_id,name,state,lat,lon,requested_class,route_refs,stop_role,
transfer_value,freight_volume,spacing_need,resilience_value,
energy_service,land_ops_feasibility,equity_community,
evidence_status,source_artifact,next_step
```

Evaluation classes:

```text
S1 national_terminal
S2 major_interchange_hub
S3 transfer_stop
S4 service_stop
S5 local_access_stop
```

Near-term CLI shape:

```text
route stop-candidates --class S2 --gate
route stop-candidates --route I65 --details
```

Gate rule:

- S1/S2 require complete `route_refs`, stop role, at least one positive service dimension, source artifact, and next step.
- S3 requires a transfer or regional freight role.
- S4 requires a spacing, energy, rest, weather, or safety need.
- S5 should not appear on national schematic unless explicitly requested as a local inset.

## Next Iteration

The next process should expand endpoint evidence from exception rows into named stop candidates:

```text
route stop-candidates --route I65 --details
```
