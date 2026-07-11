---
name: I-80 Des Moines Transfer Resilience Validation Plan
slug: i80-des-moines-transfer-resilience-validation
type: plan
status: reviewed
rubric_version: v1.4
author: copilot
created: 2026-07-11
updated: 2026-07-11
sources:
  - gaps/i80-flagship.md
  - data/t1-intersection-failures.csv
  - data/t1-evidence-windows.csv
  - data/pressure-test-scenarios.csv
  - data/throughput-proof-matrix.csv
  - crates/route-sim/src/scenarios/des-moines-interchange.toml
  - docs/STANDARDS_EVALUATION.md
  - research/publications/B.4+t1-intersection-resilience/sections/03-k-connectivity.tex
---

# I-80 Des Moines Transfer Resilience Validation Plan

## Parliament Decision

**Hold and narrow.**

Des Moines remains a falsifiable I-80 review hypothesis. It is not yet a design
proposal because no node-accurate alignment, loaded demand fixture, comparable
topology result, right-of-way screen, or community-impact analysis exists.

## Hypothesis

The I-35/I-80 node may concentrate national east-west and north-south transfer
through too few physically independent freight-capable paths.

The hypothesis is rejected if current topology errors explain the apparent gap,
calibrated demand does not produce a binding transfer failure, or operations
without capital construction perform as well as connector concepts.

## Validation Workstreams

### 1. Topology Provenance

- Record graph version, source date, node pair, zone boundary, route filters,
  freight eligibility rules, and snapping parameters for every k-value.
- Reproduce current and historical k-values on the same definition.
- Treat k=0 and `three connectors needed` as invalid for design use until this
  gate passes.

### 2. Geometry And Constructability

- Inventory real limited-access and freight-capable transfer paths.
- Replace legacy Omaha/Nebraska pseudo-alignments with Des Moines-area geometry.
- Screen at least one node-accurate conceptual option for tie-ins, structures,
  utilities, right-of-way, merge/diverge operations, and work-zone sequencing.
- Do not call the result a design proposal until the geometry screen exists.

### 3. Demand And Operations

- Restore a nonzero, source-labeled freight demand fixture.
- Model explicit connector nodes and edges rather than adding capacity to
  incident edges.
- Test upstream diversion before local diversion.
- Bound I-235 and local roads by truck class, clearance, bridge condition,
  capacity, safety, and community exposure.

### 4. Evidence Window

- Keep annual failure probability, throughput retention, reroute time, and
  recovery metrics null while the evidence window is snapshot-only.
- Continue Iowa 511 polling or obtain archive history.
- Join direct PTI or travel-time evidence for normal, closure, diversion, and
  recovery windows.

### 5. Equity And Community Health

- Map any conceptual path and diversion corridor against population,
  environmental-justice, and social-vulnerability surfaces.
- Prefer existing public right-of-way and brownfield transport land.
- Record residential, business, farm, and community-facility displacement.
- Prohibit unrestricted heavy-freight diversion through the I-235 urban core.
- Preserve pedestrian, bicycle, transit, and non-driving access.

### 6. Rural And Agricultural Access

- Measure farm-to-market and agricultural freight use near the node.
- Identify emergency detour burdens on peri-urban and rural access roads.
- Require a net community-benefit statement before a connector option advances.

### 7. Climate And Shared Failure

- Map present and future flood, extreme-precipitation, heat, and winter hazards.
- Reject alternates that share the same floodplain, drainage basin, power, or
  operations failure zone as the primary interchange.
- Treat the internal 80%-within-four-hours recovery target as a heuristic test
  threshold, not an external standard.

### 8. Intermodal Alternative

- Evaluate whether rail, terminal operations, or other multimodal measures can
  absorb eligible freight during a closure.
- Compare operations-only and intermodal packages against connector concepts.

## Promotion Gates

| Gate | Required result |
|---|---|
| Topology comparability | k-values reproduce on the same documented graph definition |
| Physical baseline | Actual interchange connectivity is at least k=1 and manually validated |
| Loaded demand | Nonzero, source-labeled demand produces a bounded closure effect |
| Intervention realism | New paths are explicit graph edges with merge/diverge constraints |
| Constructability | A node-accurate option has ROW, structure, utility, and staging screens |
| Evidence | Snapshot observations are not annualized |
| Safety | Diversion and connector geometry pass truck and local-road safety review |
| Equity | Displacement, health, and non-driving impacts are bounded and reviewed |
| Rural access | Agricultural and emergency-access effects are source-backed |
| Climate | Proposed paths do not share the governing hazard failure zone |
| Alternatives | Operations-only and intermodal options are compared |
| Null result | Rejection criteria remain acceptable outcomes |

## Explicit Holds

- No physical alignment selected.
- No connector count selected.
- No cost, benefit-cost ratio, or positive NPV.
- No construction schedule or procurement plan.
- No guaranteed throughput, PTI, SLA, or recovery outcome.
- No Iowa DOT, FHWA, MPO, carrier, or community endorsement.

## Next Decision

After the validation gates produce comparable topology, nonzero demand, and a
community-bounded conceptual option, Parliament may decide whether ROUTE should
create an actual design proposal or reject the Des Moines hypothesis.
