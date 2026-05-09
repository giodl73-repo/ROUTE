# Interstate 2.0 Standards Evaluation

This document is the proof plan for Milepost 4, Pressure Test. The tier standards are treated as hypotheses, not slogans: every standard must earn its place by improving a tested service outcome under a named adverse condition.

The machine-readable ledger is `data/standards-proof-ledger.csv`.

The L2 scenario catalog is `data/pressure-test-scenarios.csv`.

The T1/T1 failure-rate and reroute evidence ledger is `data/t1-intersection-failures.csv`.

The source acquisition plan for that ledger is `data/t1-failure-source-plan.csv`.

## Evaluation Rule

A standard is ready for Blueprint only when it has:

| Field | Meaning |
|---|---|
| Outcome | The SLA, throughput, resilience, safety, access, energy, or equity result it protects |
| Mechanism | Why the standard should improve that outcome |
| Stressor | The adverse condition used to test the claim |
| Acceptance gate | The measured condition that must pass |
| Evidence level | Implemented, Heuristic, Stub, Planned, or Deprecated |
| Artifact | Command, data file, paper, or review record supporting the claim |
| Blocking gap | The reason the claim cannot be promoted yet |

If a standard cannot name these fields, it can stay in the design spec, but it cannot be used as a proven investment claim.

## Proof Levels

| Level | What It Proves | Examples |
|---|---|---|
| L0 | Primitive arithmetic and invariants are correct | k-connectivity fixtures, PTI calculation, capacity arithmetic, recovery ratio |
| L1 | Generated artifacts are reproducible from stable inputs | standards proof ledger, coverage gaps, T1/T1 intersection ledger, scenario summaries |
| L2 | Representative scenarios produce bounded outputs | Des Moines interchange closure, Donner closure, Houston surge, NY-LA SLA, Houston-Chicago SLA |
| L3 | The claim survives adversarial review and source validation | NPMRDS PTI, FAF5 sensitivity, ramp geometry review, climate exposure review |

Milepost 4 is not finished until active standards have at least L1 evidence, and the highest-stakes T1 standards have L2 scenario coverage or are explicitly downgraded.

## Central Blocking Issue: T1/T1 Interchanges

T1/T1 intersections are the biggest single system issue because they combine high national freight dependence with single-node physical vulnerability. A closure at a T1/T1 core can disrupt two primary arteries at once, exactly where alternate transfer capacity is most valuable.

The T1/T1 standard has two different jobs:

| Component | Job | Failure Mode It Addresses |
|---|---|---|
| Diamond k-connectivity | Ensure at least three independent transfer paths inside the 50-mile zone | Complete or partial closure of the core interchange |
| Express freight flyovers | Preserve freight-only transfer capacity even when connector roads fill with local traffic | Local traffic capture and GP congestion on connector roads |

The acceptance gate is stricter than graph connectivity alone:

1. Baseline and post-build k-connectivity must be reported for each T1/T1 site.
2. Top priority sites must be manually validated against geometry or source imagery, because the B.4 panel flagged TIGER snapping as a blocking risk.
3. Post-build design must achieve k >= 3 in the 50-mile zone.
4. A single interchange or connector failure must not collapse T1/T1 transfer capacity.
5. The T1 recovery target must be tested: at least 80% of baseline throughput restored within 4 hours, or the claim is labeled unproven.

Current status: Heuristic. `route diamond --at all`, `data/t1-intersections.md`, and the B.4 paper establish the right framing, but manual validation and usable-throughput scenarios are still needed before this becomes a Blueprint-grade claim.

The failure-rate evidence is currently the weakest piece. `data/t1-intersection-failures.csv` separates modeled scenario outputs from empirical evidence. Most rows are `source_needed`; the Des Moines row is only `modeled`, because it comes from `route sim scenario des-moines-interchange --intervention` rather than observed closure history. `data/t1-failure-source-plan.csv` lists the first source targets for replacing those blanks with observed incident, work-zone, travel-time, and reroute evidence. `data/t1-failure-events.csv` is the normalized raw observation table; `route t1-failure-events` turns those observations into per-site annual rates, annual probabilities, and p50/p95 duration estimates. With `--write-ledger`, those event summaries can update the T1/T1 failure ledger while preserving existing modeled throughput and reroute fields.

## SLA And Throughput Proof

The project should keep three ideas separate:

| Concept | Meaning | Current Risk |
|---|---|---|
| PTI | p95 travel time divided by free-flow time | Some paper claims still rely on modeled PTI where direct NPMRDS PTI is required |
| SLA window | The shipper commitment window implied by p95 travel time, relay rules, and incident buffers | SLA outputs can look precise before data confidence supports them |
| Throughput | Vehicle or freight movement capacity under normal and adverse conditions | Max-flow can identify structural capacity, but not all freight is interchangeable |

The C.1 panel accepted the SLA framework but blocked publication-grade quantitative claims until direct NPMRDS or a better oversaturation model replaces BPR extrapolation. The C.2 panel accepted national max-flow as valuable but blocked investment conclusions until single-commodity sensitivity or multi-commodity analysis is added.

For now, SLA and throughput outputs should be labeled Heuristic unless they are backed by direct observed reliability data and sensitivity ranges.

## Required Scenario Library

| Scenario | Purpose | Existing Seed |
|---|---|---|
| Des Moines T1/T1 closure | Diamond and flyover proof | `crates/route-sim/src/scenarios/des-moines-interchange.toml` |
| Donner closure | Mountain-pass closure and resilience-routing proof | `crates/route-sim/src/scenarios/donner-closure.toml` |
| Atlanta peak | Managed-lane/PTI stress proof | `crates/route-sim/src/scenarios/atlanta-peak.toml` |
| Houston surge | Port/hurricane/evacuation stress proof | `crates/route-sim/src/scenarios/houston-surge.toml` |
| NY-LA SLA | 48-hour freight SLA proof | `route od ny-la`, `route interventions --corridor ny-la` |
| Houston-Chicago SLA | I-69 and Gulf-to-Midwest proof | `route od hou-chi`, `route interventions --corridor hou-chi` |

The scenario library should report confidence labels with the output. A scenario can fail and still be valuable if it tells us which standard does not yet earn its place.

Important current limitation: embedded TOML scenarios now bind stable graph edge IDs, but their intervention and demand models are still heuristic. `route sim list` reports definition readiness, and `route sim scenario ...` prints warnings before execution when a scenario is only a shell. A scenario with no affected edges is not an L2 pressure test; it is a fixture waiting for graph binding. A bound-edge scenario still needs focused demand and acceptance gates before it becomes a proof-grade L2 result; the Donner and Atlanta runs currently show no throughput delta under the synthetic demand proxy.

## Current Conclusions

| Package | Current Claim Level | Reason |
|---|---|---|
| T1/T1 diamond k-connectivity | Heuristic | Correct metric and command exist, but manual geometry validation is still blocking |
| Express freight flyovers | Planned | Concept is specified, but site geometry and throughput scenarios are not yet built |
| T1 PTI/SLA | Heuristic | Simulation commands exist; direct PTI/source validation is still blocking |
| Max-flow throughput | Heuristic | Edmonds-Karp implementation exists; multi-commodity sensitivity is still blocking investment conclusions |
| Coverage gaps | Implemented | `route coverage` and gap classification artifacts exist |
| EV/rest/transit operations | Heuristic/Planned | Useful models exist, but inventories and outage scenarios are incomplete |
| Climate resilience | Heuristic | Hazard seed data exists, but site-specific exposure and frequency need validation |

## Rust Interface

`route standards-proof` reads `data/standards-proof-ledger.csv`, prints the proof status table, and can fail a Blueprint gate when unresolved standards would be promoted.

Useful forms:

```text
route standards-proof
route standards-proof --tier T1 --family resilience
route standards-proof --tier T1 --family resilience --details
route standards-proof --gate-blueprint
route t1-failures
route t1-failures --needs-sources
route t1-failure-sources
route t1-failure-events
route t1-failure-events --write-ledger data/t1-intersection-failures.csv
```

The next build step is to move the ledger parser and gate rules out of the CLI into a small library module once additional commands need to consume the same proof model.

For scenario readiness:

```text
route sim list
route sim scenario des-moines-interchange --intervention
```
