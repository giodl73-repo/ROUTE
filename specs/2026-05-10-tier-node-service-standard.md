# Tier and Node Service Standard

Status: draft

## Purpose

The national schematic should behave like a metro and rail diagram: route classes define service hierarchy, and node classes define which places deserve named stops, terminals, or transfer emphasis. The map is an output of this standard, not the source of truth.

## Route Tier Semantics

### T1 Trunk

T1 routes are national trunk services. They must connect major national regions, ports, border gateways, or cross-country freight axes.

Minimum map standard:
- Named major node roughly every 250-400 miles.
- Every T1/T1 junction is a named major interchange hub.
- Terminals are named when they anchor a national region, border, port, or coast endpoint.
- Intermediate stops are included only when they change relay operations, resilience, routing, or national freight geography.

### T2 Connector

T2 routes are inter-trunk or national relief services.

Default qualification:
- Touch at least two T1 interchange nodes, and
- Touch at least two distinct T1 trunks.

Allowed exception:
- A one-ended T2 may be retained only when the free end is a terminal-worthy national endpoint: major port, border crossing, military/logistics complex, terrain/weather resilience outlet, or documented T1 failure-relief path.

Routes that only touch one T1 node and have no terminal-worthy exception are T3/T4 candidates.

### T3 Regional Feeder

T3 routes connect a T1/T2 node to a regional production zone, metro, port district, or resilience alternative.

Map standard:
- Usually omitted from the national Beck schematic.
- Named on regional maps.
- Must connect at least one T1/T2 node to a meaningful regional endpoint.

### T4 Local Spur

T4 routes are local loops, short spurs, and access connectors.

Map standard:
- Omitted from the national schematic unless used as a local inset.
- Urban beltways and short bypasses default here unless they prove T2 national relief value.

## Node Classes

Node classes are investment/service classes, not just cartographic symbols. A named stop on the map should imply a service package and a reason for public/private investment.

### Terminal

End of a line or nationally meaningful endpoint. Examples: coast endpoint, border gateway, major port, or remote strategic terminus.

### Major Interchange Hub

Named T1/T1 or high-value T1/T2 transfer with national freight meaning. These are the boldest stations on the schematic.

### Transfer Station

Two-route connection, typically T1/T2 or T2/T2. Named when it changes routing or relay operation.

### Service Stop

Intermediate node on T1/T2 service. Named when it is a meaningful relay, logistics, metro, terrain, or resilience point.

### Local Stop

T3/T4 access node. Shown on regional maps, not the national schematic.

### Future Stop

Proposed or candidate node. Must be visually distinct and should carry an evidence status.

## Stop Investment Classes

### S1 National Terminal

An S1 stop is a line endpoint or terminal market worthy of national investment. It can anchor T1 or exceptional T2 service.

Typical package:
- Intermodal freight access or port/border interface.
- Relay driver base or staffed operations center.
- Truck charging/fueling at national standard.
- Incident recovery staging and emergency operations access.
- Passenger/worker transfer where relevant.

Minimum evidence:
- National endpoint role, plus at least one freight/logistics/military/port/border source.

### S2 Major Interchange Hub

An S2 stop is a major T1/T1 or T1/T2 interchange where transfers are nationally meaningful.

Typical package:
- High-capacity relay hub.
- T1/T2 managed transfer design or diamond redundancy.
- Queue/incident management.
- Charging/fueling and service bays.
- Intermodal or logistics-zone access when present.

Minimum evidence:
- Two or more priority routes, material freight volume or resilience role, and a node failure consequence.

### S3 Transfer Stop

An S3 stop is a T2/T1, T2/T2, or regionally important transfer. It may be worth targeted investment, but not a full national hub.

Typical package:
- Medium relay/charging site.
- Truck parking and safe handoff area.
- Local incident diversion and staging.
- Regional bus/worker transfer where useful.

Minimum evidence:
- At least one priority-route connection and a regional freight/logistics role.

### S4 Service Stop

An S4 stop is an intermediate service location on a T1/T2 route.

Typical package:
- Charging/fueling.
- Truck parking/rest.
- Weather/incident support.
- Basic relay support only if spacing requires it.

Minimum evidence:
- Stop spacing gap, safety/rest need, charging gap, or terrain/weather constraint.

### S5 Local Access Stop

An S5 stop is local access: urban loop, port gate, industrial district, or short bypass access. It is normally not shown on the national schematic.

Typical package:
- Local connector operations.
- Safety/access upgrades.
- Targeted freight gate improvements.

Minimum evidence:
- Local or regional access need. National-tier promotion requires an exception.

## Stop Spacing and Service Rules

Stops should not be every city. They should be placed where the system needs a service decision.

T1 stop rhythm:
- S1/S2 roughly every 250-400 miles where geography allows.
- S4 service coverage every 100-150 miles for charging, rest, and incident support.
- T1/T1 nodes are S2 by default unless evidence says the interchange is operationally minor.

T2 stop rhythm:
- S2/S3 at each T1/T2 or T2/T2 transfer.
- S4 every 100-200 miles when the corridor is long.
- Free endpoints must qualify as S1/S3 or the route is a T3 candidate.
- Schematic endpoints must intersect their trunk at a named stop. A T2 endpoint that lands near a T1 trunk but does not touch it is a stop-selection failure, not a drawing issue.
- Any schematic crossing between a T2 connector and T1 trunk must either become a transfer stop or be rerouted so the lines do not imply a transfer.

T3 stop rhythm:
- S3/S4 at regional anchors and service gaps.
- S5 local access stops are acceptable but should not be confused with national stations.
- A T3 schematic chain needs at least two visible stops: one transfer-grade stop (S1/S2/S3) and one regional terminal, service, or access stop (S3/S4/S5).
- T3 endpoints in a regional schematic must connect cleanly to a named T1/T2/T3 stop or be recorded as a free regional terminal exception.
- T3 bends and crossings follow the same map rule as T1/T2: bends happen at visible stops, and crossings imply transfers unless explicitly separated.

T4 stop rhythm:
- Local operational stops only; omitted from national map.

## Stop Evaluation Criteria

A candidate stop earns investment class by scoring evidence across these dimensions:

- `transfer_value`: route transfer count and tier mix.
- `freight_volume`: truck AADT, port/border/intermodal role, or logistics market.
- `spacing_need`: distance to adjacent qualified stops.
- `resilience_value`: incident, weather, terrain, or closure recovery value.
- `energy_service`: charging/fueling gap and grid/station role.
- `land_ops_feasibility`: plausible land, staging, service-bay, or parking envelope.
- `equity/community`: exposure, access, and mitigation requirements.
- `evidence_status`: validated, heuristic, planned, missing source, or demote.

## Endpoint Qualification

Endpoint qualification is a separate measure from route connectivity. A route may touch the right number of higher-tier nodes and still have weak termini, or it may be one-ended but terminate at a place important enough to justify the tier.

### T1 Endpoint

A T1 endpoint must be a national terminal. It should satisfy at least two of these conditions:
- Coast, border, or international gateway.
- Major port, intermodal gateway, or logistics market with national freight role.
- Top national metro or region anchor.
- Military/logistics complex with national mobilization value.
- Terrain, weather, or geography endpoint where no comparable trunk continuation exists.
- Endpoint of a cross-country freight axis with no higher-tier continuation.

T1 endpoints should not be ordinary places where a trunk merely stops because the current map or graph ends.

### T2 Endpoint

A T2 route usually has two endpoint types:
- A higher-tier transfer endpoint: T1/T2 terminal or interchange.
- A connector endpoint: another T1/T2 node, or a terminal-worthy free end.

A T2 free end must satisfy at least one terminal-worthy exception:
- Major port or border gateway.
- Military/logistics complex.
- Major regional production or distribution anchor.
- Documented T1 relief/resilience outlet.
- Future T1/T2 continuation with a clear corridor plan.

If a T2 touches only one T1/T2 node and its free end has no exception, it is a T3 candidate.

### T3 Endpoint

A T3 endpoint may be regional rather than national. It should connect a T1/T2 node to one of:
- Regional metro or production zone.
- Secondary port, airport, rail terminal, or military site.
- Resilience alternate for a T1/T2 closure zone.
- Rural access corridor with explicit service obligation.

### T4 Endpoint

A T4 endpoint is local access. It may terminate at a suburb, local port gate, beltway segment, industrial district, or short bypass. T4 endpoints are not national schematic terminals.

## Endpoint Evidence Status

Every terminal-worthy endpoint should carry an evidence status:

- `validated`: source-backed endpoint function.
- `heuristic`: plausible from route/city role, not source-backed.
- `planned`: intended future endpoint or corridor continuation.
- `missing_graph_data`: graph does not expose the endpoint/contact cleanly.
- `demote`: endpoint is local/regional only for the requested tier.

## Evaluation Gates

The first mechanical gate is connectivity behavior:

```text
route tier-connectivity --tier T2 --gate
```

For each T2 route, the gate counts:
- T1 interchange nodes touched by the route.
- Distinct T1 trunks touched by the route.
- Route mileage.

Rows passing as `trunk_connector` or `relief_loop` may remain in T2 pending evidence review. Rows classified as `one_ended_feeder`, `local_spur`, or `missing_graph_data` require demotion or a terminal-worthy exception record.

The second gate is endpoint worthiness. A one-ended T2 can remain T2 only when the free endpoint has a complete exception record:

```text
data/tier-node-exceptions.csv
route tier-connectivity --tier T2 --exceptions data/tier-node-exceptions.csv --gate
```

Without that record, the route remains a demotion candidate even if it is long or strategically plausible. An endpoint exception does not repair missing graph/contact data; those rows remain blocked until the graph can expose the route's higher-tier contacts or the route is explicitly demoted.

The third gate is stop worthiness:

```text
data/tier-stop-candidates.csv
```

Routes should not create named stops automatically. Each named stop needs a class, service package, evidence status, and next validation step.
