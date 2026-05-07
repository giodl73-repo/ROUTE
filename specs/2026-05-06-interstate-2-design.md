---
name: Interstate 2.0 — Design Specification v1.0
slug: interstate-2-design
type: spec
status: draft
rubric_version: v1.0
author: human
created: 2026-05-06
updated: 2026-05-06
sources:
  - "ROUTE design spec (specs/2026-05-06-route-design.md)"
  - "ROUTE Rust architecture spec (specs/2026-05-06-route-rust-architecture.md)"
  - "ROUTE research module (research/MODULE.md)"
  - "T1/T1 intersection analysis (data/t1-intersections.md)"
  - "FHWA Highway Statistics 2023"
  - "ATRI Top 100 Truck Bottleneck Report 2024"
---

# Interstate 2.0 — Design Specification

## §1. The Premise

The United States built the interstate highway system in 30 years (1956–1985). It has maintained it poorly for 40. The system was designed for the traffic volumes and vehicle mix of the mid-20th century, the geopolitical assumptions of the Cold War, and the political feasibility constraints of the Eisenhower era. It was not designed for:

- 72% of national freight value moving by truck (2024)
- Electric vehicles requiring charging infrastructure every 50 miles
- Climate events closing coastal and mountain corridors with increasing frequency
- 330 million people concentrated in metro areas that postdate the highway design
- Just-in-time logistics requiring sub-48-hour transit time guarantees
- A single national freight network expected to serve both local distribution and 3,000-mile transcontinental hauls on the same lanes

Interstate 2.0 is not a new interstate system. It is a targeted upgrade to the existing system — organized by evidence, prioritized by tier, designed for the next 50 years.

**The country chose highways. Interstate 2.0 is the evidence for which ones to invest in next, in what order, and why.**

---

## §2. The Tier System

The 227 US interstate corridors are not equally important. Scoring all 227 against 12 dimensions (throughput gap, freight intensity, network centrality, redundancy, population reach, rural connectivity, economic opportunity, climate resilience, multimodal integration, infrastructure vintage) and applying centrality-adjusted natural break analysis produces four tiers.

**Centrality-adjusted classification** — B2 (betweenness centrality) weighted at 0.65, aggregate score at 0.35. This prevents congested urban beltways (high A1) from outscoring structurally irreplaceable trunk lines (high B2).

### Tier 1 — Primary Arteries (8 corridors, ~18,000 miles)

The load-bearing spines. These corridors carry the majority of national freight ton-miles and occupy the highest betweenness centrality positions. Loss of any T1 corridor cascades nationally.

| Corridor | Span | Miles | Peak AADT | Peak V/C | Priority gap |
|---|---|---|---|---|---|
| I-5 | San Diego → Blaine WA | 1,381 | 333,100 | 1.13 | Managed freight lanes CA |
| I-10 | Santa Monica → Jacksonville | 2,460 | 311,685 | **2.11** | Critical — 3 managed lane pairs |
| I-35 | Laredo → Duluth | 1,568 | 146,000 | 0.99 | 1 managed lane pair |
| I-40 | Barstow → Wilmington NC | 2,554 | 125,000 | 0.84 | **At target — maintenance only** |
| I-75 | Miami → Sault Ste. Marie | 1,786 | 409,000 | **1.84** | Critical — 3–4 managed lane pairs |
| I-80 | Teaneck → San Francisco | 2,909 | 276,000 | **1.86** | Donner bypass + managed lanes |
| I-90 | Boston → Seattle | 3,085 | 327,200 | **2.21** | Critical — 3 pairs + Snoqualmie |
| I-95 | Miami → Houlton ME | 1,919 | 336,500 | 1.52 | 2–3 managed lane pairs |

**Total T1 managed lane investment: ~$209B** at $15M/mile for 2 added managed lanes.

### Tier 2 — Major Connectors (25 corridors)

Regional corridors linking T1 nodes to secondary metros. Investment in T2 is the most cost-effective way to relieve T1 congestion — diverting regional traffic removes 10–30% of T1 load at 5–10× lower cost per vpd than adding T1 lanes.

Key T2 corridors (selected by T1 relief potential):
- I-94 (Chicago–Detroit–Minneapolis): relieves I-90/I-75 junction load
- I-15 (LA–Las Vegas–Salt Lake City): relieves I-5/I-10 junction, provides I-80 southern alternate
- I-25 (Albuquerque–Denver): relieves I-40/I-70 transfer load
- I-65 (Mobile–Chicago): east of I-35, second N-S connector for freight
- I-85 (Montgomery–Petersburg VA): Piedmont corridor, relieves I-75/I-95 southeastern bottlenecks
- I-70 (Baltimore–Cove Fort UT): relieves I-80/I-90 Midwest transfer load; alternative to I-80 for Denver routing
- I-81 (Knoxville–Harrisburg PA): Appalachian spine; currently the most understressed major connector

### Tier 3 — Regional Feeders (61 corridors)

State-level connectors. Investment focus: maintenance, access, rural connectivity spurs. No managed lanes.

### Tier 4 — Local Access (133 corridors)

Urban distribution and short-distance connectors. Investment focus: maintenance only. Some T4 corridors serve isolated rural populations with no alternative — these qualify for rural connectivity spur investment from the C3 (Economic Opportunity) dimension.

### Missing T1 Corridors (Upgrade Candidates)

The current T1 network leaves three geographic zones without T1 coverage within 300+ miles:

| Zone | Gap | US highway candidate | Upgrade type |
|---|---|---|---|
| Northern Tier | No T1 north of I-90 across MT/ND/MN/WI/MI/NE | US-2 (2,600 mi, Houlton ME → Everett WA) | New Interstate designation + grade separation |
| Gulf-to-Midwest | No direct T1 from Houston to Chicago | I-69 alignment (partial, needs completion TX→IN) | Complete partial construction |
| Appalachian E-W | No T1 between I-40 (Knoxville) and I-70 (Wheeling) | US-60 / proposed Appalachian connector | New Interstate designation |

US-2's B1 score of 10.0 in the ROUTE corpus confirms: it has zero redundancy because it IS the only route through the northern tier. A Northern Tier interstate is the most structurally justified missing T1 in the national network.

---

## §3. T1 Corridor Anatomy

A T1 corridor in its current form tries to serve three incompatible functions on the same lanes simultaneously: long-haul freight, regional travel, and urban distribution. All three are served poorly.

**Interstate 2.0 T1 anatomy separates these functions physically:**

```
┌─────────────────────────────────────────────────────────────────────┐
│  EXPRESS FREIGHT           │  GENERAL PURPOSE    │  COLLECTOR-DIST  │
│  (truck-only, separated)   │  (cars + local del) │  (urban segments) │
│                            │                     │                   │
│  ████ ████                 │  ████ ████          │  ████             │
│  2 lanes per direction     │  2 lanes per dir    │  1 lane per side  │
│  barrier-separated         │  existing lanes     │  local access     │
│                            │                     │                   │
│  65 mph sustained          │  LOS C-D target     │  ramps every 1 mi │
│  platooning-ready          │  standard ramps     │  urban access     │
│  weigh bypass (PrePass)    │  every 2-5 mi       │                   │
│  toll-priced               │  free               │  free             │
│                            │                     │                   │
│  ACCESS: freight terminals │  ACCESS: standard   │  ACCESS: all      │
│  intermodal hubs only      │  highway ramps      │  local ramps      │
└─────────────────────────────────────────────────────────────────────┘
```

### Express Freight Lanes

- **Truck-only**: CDL-A commercial vehicles only. No passenger cars.
- **Physical separation**: concrete barrier from GP lanes. Not just paint.
- **Speed**: 65 mph sustained. Trucks on GP lanes currently average 57 mph due to interference.
- **Platooning-ready**: consistent geometry, 12-foot lanes, no merging weave zones. Enables 2–5 truck platoons at 20–35% fuel savings.
- **Access control**: interchange only at freight terminals and intermodal hubs — not every standard ramp. This is what keeps the express freight lane express.
- **Pricing**: dynamic tolls. Revenue funds operations and cross-subsidizes rural access improvements.
- **Weigh station bypass**: WIM (weigh-in-motion) technology at lane entry. No stops for compliant operators.

**Effect on GP lanes**: removing 25–35% of lane occupancy (trucks) from GP lanes immediately improves their level of service — equivalent to adding 1 GP lane without building one. The express freight investment buys two improvements for the price of one.

### Realistic Lane Limits

- Standard interstate ROW: 300–400 feet
- At 12-foot lanes + shoulders: physical maximum ~12 travel lanes within existing ROW
- Urban constraint beyond 8–10 total lanes: highway becomes an uncrossable community barrier
- **Practical I2.0 ceiling: 4 express freight lanes (2+2) + 4 GP lanes (2+2) = 8 total**, within existing ROW in most corridors
- I-40 (already at V/C target): **0 express freight lanes needed**
- I-35 and I-5: **2 express freight lanes** (1+1 per direction)
- I-75, I-90, I-10: **4 express freight lanes** (2+2)
- Above 4 express lanes + 4 GP = right-of-way acquisition required — exceptional cases only

### Collector-Distributor Roads (Urban Segments)

In every metro area over 500,000 population on a T1 corridor: a parallel C-D road system running alongside the T1 mainline for the metro span. C-D roads:
- Collect traffic from local ramps and feed it to the GP express lanes
- Distribute GP express traffic to local ramps at the other end
- Keep the T1 mainline running express through the metro without stopping for local access
- Eliminate the weave conflicts that create urban congestion at interchange clusters

C-D roads exist in fragments today (approaches to major interchanges). I2.0 makes them systematic.

---

## §4. T1/T1 Diamond Intersections

**The current design flaw**: where two T1 corridors cross, there is a single complex interchange — one node, one point of failure. An accident, construction, or natural disaster at that node disrupts both T1 corridors simultaneously at exactly the point of maximum national freight dependency.

**The diamond concept**: instead of a single intersection node, create a distributed intersection zone spanning approximately 50 miles in each direction, with multiple independent cross-connection points. Any single connection can fail; the others maintain T1/T1 transfer capacity.

### The 15 T1/T1 Intersections by Priority

| # | Intersection | Node | SPF risk | Diamond needed |
|---|---|---|---|---|
| 1 | I-35 × I-80 | Omaha NE | **HIGH** | Yes — I-680 is only bypass |
| 2 | I-40 × I-75 | Chattanooga TN | **HIGH** | Yes — mountains constrain alternates |
| 3 | I-10 × I-35 | San Antonio TX | **HIGH** | Yes — single spaghetti interchange |
| 4 | I-90 × I-95 | Boston MA | **HIGH** | Yes — Central Artery is single path |
| 5 | I-10 × I-95 | Jacksonville FL | **HIGH** | Yes — I-295 is only ring |
| 6 | I-35 × I-40 | Oklahoma City OK | **HIGH** | Yes — OKC inner loop inadequate |
| 7 | I-75 × I-80 | Toledo OH | **HIGH** | Yes — I-475 at capacity |
| 8 | I-5 × I-80 | Bay Area CA | **MED** | Yes — Bay crossing is single node |
| 9 | I-5 × I-90 | Seattle/Tacoma WA | **MED** | Yes — Puget Sound constrains paths |
| 10 | I-35 × I-90 | Minneapolis MN | **MED** | Partial — Twin Cities ring has 3 paths |
| 11 | I-5 × I-10 | Los Angeles CA | **MED** | Partial — dense urban but nodes cluster |
| 12 | I-40 × I-95 | Wilmington NC | **MED** | Yes — I-40 terminus forces single point |
| 13 | I-75 × I-90 | Detroit MI | **LOW** | No — 4+ paths already |
| 14 | I-5 × I-40 | Barstow CA | **LOW** | No — rural, easy detour via US-395 |
| 15 | I-80 × I-90 | Indiana/Ohio | **STRUCTURE** | Same road for 158 miles — not a node |

**k-connectivity target**: every T1/T1 intersection achieves k ≥ 3 (minimum 3 independent edge-disjoint paths) within the 50-mile zone. Current: 9 of 15 intersections have k=1.

### Diamond Design — I-35 × I-80 at Omaha (example)

```
                    ←— 50 miles —→

              [NORTH CONNECTOR]
              Fremont NE area
              I-80 ←—10mi connector—→ I-35 at Blair NE
                        ↑
                        |  25 miles north of core
                        |
I-35 north ─────────────────────────────────────── I-35 north
                    [OMAHA CORE]
                    (current interchange)
I-35 south ─────────────────────────────────────── I-35 south
                        |
                        | 25 miles south of core
                        ↓
              [SOUTH CONNECTOR]
              Greenwood NE / Nebraska City area
              I-80 ←—connector—→ I-35 near Plattsmouth NE
```

- **Core** (existing): full interchange, maintain and improve
- **North connector** (~$150M): new 10-mile connector from I-80 at Fremont to I-35 at Blair
- **South connector** (~$200M): new connector from I-80 near Greenwood to I-35 near Plattsmouth
- **Result**: Omaha core can close completely — 80% of freight flow maintained via connectors

### Diamond Investment Summary

9 priority intersections × average $500M per diamond = **$4.5B total**. This is the highest NPV resilience investment in the national highway network — at $4.5B it provides distributed redundancy at 15 of the highest-criticality nodes. Compare to a single managed-lane segment on I-75 in Atlanta at $27B.

---

## §5. T2 as Relief Valves

For each T1 bottleneck, there is a specific T2 investment that relieves more T1 congestion per dollar than adding T1 lanes. These are not adjacent parallels — they are strategic connectors that let traffic find alternative paths through the network.

| T1 bottleneck | T2 relief corridor | Investment type | Est. T1 relief |
|---|---|---|---|
| I-75 Atlanta (V/C 1.84) | I-285 managed bypass + I-575/GA-400 capacity | Managed lanes on beltway | 40,000 vpd |
| I-90 Boston (V/C 2.21) | I-93 + Route 128 interchange capacity | Interchange improvements | 25,000 vpd |
| I-10 Houston (V/C 2.11) | I-69 north-south diversion | I-69 completion | 50,000 vpd |
| I-10 LA (V/C 2.11) | SR-60 + I-210 Inland Empire capacity | Lane additions on T2 | 35,000 vpd |
| I-80 Bay Area (V/C 1.86) | I-580 + I-680 + SR-24 ring improvements | Interchange + managed lanes | 30,000 vpd |
| I-95 NE Corridor (V/C 1.52) | I-78 + I-81 southern routing improvement | I-81 interchange upgrades | 20,000 vpd |

**Key insight**: T2 relief is 5–10× more cost-effective per vpd of congestion relief than adding T1 lanes because:
1. T2 improvements leverage existing infrastructure
2. They divert regional trips (which don't need T1 capacity) off T1
3. They don't induce demand the way T1 lane additions do

---

## §6. Resilience Design

### Resilience Spurs (Rural Segments)

Every T1 corridor shall have a resilience egress point every 50 miles on rural segments — not a full interchange, but a directional exit connecting to the US highway network with clear signage to the nearest alternate routing.

Current design: when I-80 closes at Donner Pass, drivers are trapped between Reno and Sacramento with no exit for 30+ miles in either direction. A resilience spur at Truckee (US-267 to SR-89) and at Auburn (SR-49) would provide early egress before the closure zone.

Cost: $2–5M per spur (emergency exit ramp + US highway upgrade + dynamic signage). For all T1 corridors (~600 rural 50-mile segments): **$1.2–3B total**.

### Intermodal Spurs

Every T1 corridor shall have at least one intermodal freight spur per state traversed — a short connector (≤5 miles) from the T1 express freight lanes to the nearest Class I freight rail intermodal terminal.

These spurs enable:
- Truck-to-rail transfer for long-haul freight (reduces T1 truck load by 10–20% per corridor)
- Rail-to-truck transfer at distribution hubs
- Resilience redundancy: when T1 highway is closed, freight can move by rail

Current intermodal hub count on T1 corridors: 0–3 per corridor (too sparse). Target: minimum 1 hub per 300 corridor miles with T1 spur access.

### Donner Pass Bypass (I-80 specific)

The single most critical resilience investment in the national network. Donner Pass on I-80:
- 2 lanes each direction = 91,200 vpd capacity (the binding bottleneck for the entire northern transcontinental route)
- Closes approximately 50 days/year for weather
- No interstate-grade alternate exists
- SR-49/US-50 detour adds 4–6 hours

**Options:**
1. **Second alignment**: a southern bypass of the Sierra Nevada at lower elevation, reconnecting to I-80 east of Reno. ~80 miles of new construction. Estimated $6–10B. Reduces closure frequency.
2. **Managed tunnel**: a dedicated freight tunnel through the Sierra at lower grade. Estimated $3–5B. Eliminates weather closure for freight. Most cost-effective solution for the express freight lanes specifically.
3. **Rail-road hybrid**: expand Amtrak/Union Pacific tunnel capacity to handle truck trailers directly (TOFC — Trailer on Flat Car). $500M–1B. Partial solution, capacity-limited.

Recommendation: managed freight tunnel + second alignment in the long term. The tunnel provides immediate weather resilience for freight; the second alignment provides full capacity and passenger vehicle redundancy.

### Climate Hardening

From the D.1 analysis (pending full computation):
- **Gulf Coast I-10** (Louisiana/Mississippi coast): elevated roadbed in FEMA SFHA zones; storm surge barriers at highest-risk bridges
- **I-95 Miami–Fort Lauderdale**: sea level rise projection exposure by 2050 requires either roadbed elevation or corridor relocation inland
- **I-5 Willamette Valley** (Oregon): Cascadia Subduction Zone earthquake risk — seismic retrofit of all bridges rated poor
- **I-90 Snoqualmie Pass**: same weather closure pattern as Donner, second alignment needed through Cascades

---

## §7. The Simulation Toolkit

Design and investment decisions for a $200B+ infrastructure program cannot rest on static analysis alone. Before committing to managed lane configurations, diamond interchange locations, or T2 relief investments, the system must be tested under adversarial conditions.

**Interstate 2.0 requires a simulation layer** — a software system that:

1. **Encodes the national highway graph** with current and proposed infrastructure (the `route` CLI provides this)
2. **Runs traffic demand models** — not just AADT but time-of-day, seasonal, and commodity-specific flows
3. **Injects chaos** — simulated incidents, weather events, construction closures, demand shocks
4. **Tests proposed interventions** — does this diamond help? does this managed lane actually reduce T1 congestion or just shift it?
5. **Measures outcomes** — PTI, throughput, freight cost, resilience (recovery time after incident)

### Simulation Modules (to build in Rust)

```
route-sim/                  ← new crate in the workspace
├── src/
│   ├── demand.rs           ← traffic demand model (FAF5-derived O-D flows)
│   ├── assignment.rs       ← user equilibrium traffic assignment (Wardrop)
│   ├── incident.rs         ← incident injection: accidents, weather, construction
│   ├── chaos.rs            ← chaos scenarios: compound incidents, cascades
│   ├── intervention.rs     ← model proposed I2.0 interventions (lanes, diamonds)
│   ├── metrics.rs          ← PTI, throughput, freight cost, recovery time
│   └── scenarios/          ← named scenario files
│       ├── donner-closure.toml
│       ├── atlanta-peak.toml
│       ├── omaha-interchange-failure.toml
│       └── hurricane-ian-i75.toml
```

### Key Algorithms

**Traffic assignment** (Wardrop user equilibrium): every driver/truck chooses the path that minimizes their own travel time, given everyone else's choices. This is the standard traffic engineering model. For highway networks: Frank-Wolfe algorithm, implemented in Rust using petgraph shortest-path primitives we already have.

**Chaos injection**: following the chaos engineering pattern (Netflix's Chaos Monkey, etc.):
- Randomly close edges for a specified duration
- Reduce capacity of edges (accidents, lane closures)
- Spike demand on specific O-D pairs (event traffic, supply chain disruption)
- Compound scenarios: two simultaneous incidents + weather + peak demand

**Intervention testing**: take the baseline graph, apply a proposed change (add 2 managed lanes to I-75 in Atlanta), re-run demand assignment, measure PTI improvement. Compare to baseline. Report cost/PTI improvement ratio.

**Resilience measurement**: after injecting a closure, measure:
- Time to 90% throughput recovery (T90)
- Peak freight cost increase during closure
- Number of alternative paths available
- Whether the diamond helps (does the system route around the closed node?)

### CLI Commands (to add)

```
route sim --scenario donner-closure [--duration 48h] [--intervention managed-lanes]
    Run a named scenario. Report: baseline PTI, incident PTI, recovery T90,
    freight cost impact, comparison with/without intervention.

route sim --chaos [--iterations 1000] [--seed 42]
    Monte Carlo chaos: random edge closures, random demand spikes, random
    weather events. Reports: mean PTI degradation, worst-case T90,
    most vulnerable corridors by failure frequency.

route sim --intervention diamond --at I35xI80
    Test a specific diamond design: add connector edges to the graph at
    Omaha, re-run chaos, compare resilience metrics before/after.

route sim --investment-rank --budget 50
    Given a $50B budget and the simulation results, rank all proposed
    interventions (managed lanes, diamonds, T2 relief, spurs) by
    cost/resilience-improvement ratio. Uses chaos Monte Carlo results
    as the resilience measure.
```

### The Toolkit Philosophy

The simulation is not a black box that outputs a number. It is a **transparent, auditable system** that:
- Uses only publicly-available data (FAF5 flows, HPMS volumes, TIGER geometry)
- Runs deterministically from a seed (reproducible chaos)
- Exposes every assumption (demand model parameters, capacity formulas, assignment algorithm)
- Produces named scenario files that can be shared, reviewed, and peer-reviewed
- Generates the quantification for papers C.2 (National Max-Flow), D.2 (Incident Economics), and E.1 (Managed Lane NPV)

The `route sim` commands generate the data that paper authors use. Papers cite named scenarios with seeds. Results are reproducible by any reader.

---

## §8. The Full Interstate 2.0 Feature Set

Organized by where each feature applies in the tier/anatomy framework:

### Express Freight Lanes (T1 only)
- Physical barrier separation from GP lanes
- 65 mph design speed sustained
- Platooning-ready geometry (consistent lane width, no weave zones)
- PrePass/WIM integration at entry points
- Dynamic tolling (revenue → operations + rural access cross-subsidy)
- CDL-A commercial vehicles only

### Intermodal Integration (T1 + T2)
- Intermodal freight spur ≤5 miles to nearest Class I terminal, per state per T1 corridor
- Standardized truck-rail transfer facilities at each hub (minimum 500 trailer capacity)
- Real-time available capacity broadcast to freight dispatchers

### EV Charging Corridor (T1 + T2)
- DC fast charging (≥150kW) at ≤50 mile intervals
- Minimum 4 chargers per station (8 at high-volume locations)
- Separate truck-capable charging (≥350kW) at express freight terminals
- Target: 100% of T1 corridor miles within 50 miles of DC fast charge by 2030

### Enhanced Rest Areas (T1 + T2)
- Full service (showers, food, fuel, parking) at ≤100 mile intervals on T1
- Minimum 50 truck parking spaces per facility (current average: 18)
- Wi-fi, health clinic, EV charging at T1 facilities
- Connected to freight dispatch systems (real-time parking availability)

### Shared Transit Facilities (T1 + selected T2)
- Park-and-ride hub at every major T1 interchange in metro areas >500k
- Intercity bus stop infrastructure (shelter, restroom, real-time info) at select interchanges
- Consistent with I2.0's equity mandate: transit-dependent travelers should be able to use T1 corridor interchanges as transit nodes

### Collector-Distributor Roads (T1 through metros >500k)
- Parallel local-access lane system alongside T1 mainline through metro spans
- Eliminates on-ramp weave conflicts from T1 express lanes
- Enables T1 to run as true expressway through urban areas

### Resilience Spurs (T1 rural segments)
- Emergency egress at 50-mile intervals in rural segments
- Connection to US highway network with directional alternate routing signage
- Dynamic closure messaging integrated with national incident management

### Diamond Interchanges (T1/T1 intersections — 9 priority)
- Three independent edge-disjoint paths (k=3 connectivity) at each T1/T1 intersection
- Two connector roads (north/south or east/west of main interchange) per intersection
- Maintains 80%+ transfer capacity during main interchange closure

### Rural Connectivity Spurs (T3/T4 targeted)
- Short connectors (≤10 miles) from T1/T2 to rural communities with nearest-interstate > 50 miles
- Focus: agricultural shipping access, healthcare access, evacuation routing
- ~500 communities currently qualify (nearest interstate > 50 miles, rural classification)

### Autonomous Vehicle / Platooning Infrastructure
- Express freight lanes designed for AV trucking from day one: consistent geometry, lidar-reflective lane markings, V2X communication infrastructure at all interchanges
- Platooning coordination areas (PCAs) at 100-mile intervals: designated zones where truck platoons form and break before/after urban segments
- Not retrofitted later — built in to express lane specification

---

## §9. Investment Plan

### Priority Tiers

**Phase 1 — $25B (Years 1–5): Resilience and Diamonds**

Highest NPV per dollar. Fixes single points of failure before adding capacity.

| Investment | Cost | What it does |
|---|---|---|
| 9 T1/T1 diamond intersections | $4.5B | Eliminates single points of failure at 9 critical nodes |
| Donner Pass freight tunnel | $4B | Eliminates I-80's binding 91k-vpd constraint for freight |
| Snoqualmie Pass bypass | $3B | I-90's equivalent Donner constraint in Washington |
| Resilience spurs (all T1 rural) | $2B | Emergency egress every 50 miles |
| T2 relief: I-69 completion (key missing segment TX–IN) | $8B | Diverts regional traffic from I-10/I-35 tangle |
| EV charging: T1 full coverage | $1.5B | All T1 within 50 miles of DC fast charge |
| Enhanced rest areas: T1 full | $2B | Full service at ≤100 mi intervals |

**Phase 2 — $84B (Years 5–15): Managed Freight Lanes, Priority Corridors**

Highest-urgency V/C corridors first.

| Corridor | Managed lanes | Estimated cost |
|---|---|---|
| I-75 Atlanta–Detroit segment (worst V/C) | 3–4 lane pairs, 600 miles | $27B |
| I-90 Boston + Midwest segments | 3 lane pairs, key segments | $20B |
| I-10 Houston + LA segments | 3 lane pairs, key segments | $18B |
| I-80 Bay Area + key Midwest | 2–3 lane pairs | $12B |
| I-95 Northeast Corridor | 2–3 lane pairs | $7B |

**Phase 3 — $100B (Years 15–30): Network Completion**

Remaining T1 managed lanes, T2 improvements, new T1 corridors.

| Investment | Cost |
|---|---|
| Remaining T1 managed lanes (I-35, I-5, remainder) | $25B |
| Northern Tier interstate (US-2 alignment, key segments) | $40B |
| I-69 full completion | $15B |
| Appalachian E-W connector | $12B |
| T2 improvements (ring roads, key connectors) | $8B |

**Total: ~$209B over 30 years**

For comparison: the US spent $530B on the original interstate system (2024 dollars). Interstate 2.0 is 40% of the original build cost to bring the system to modern standards for the next 50 years.

---

## §10. Research Program

The ROUTE research module (11 papers across 5 tracks) provides the empirical foundation for Interstate 2.0. Papers generate the numbers; the `route` CLI generates the data; the simulation toolkit (§7) generates the scenario results.

| Track | Papers | Feeds into |
|---|---|---|
| A — Corpus & Scoring | A.1 (arterials), A.2 (calibration) | All downstream tracks |
| B — Gap Analysis | B.1 (missing links), B.2 (bottlenecks), B.3 (resilience holes), **B.4 (T1/T1 intersections)** | E.2 investment plan |
| C — Freight & Throughput | C.1 (O-D reliability), C.2 (national max-flow) | E.1 managed lane NPV |
| D — Resilience | D.1 (climate exposure), D.2 (incident economics) | Phase 1 investment priorities |
| E — Interstate 2.0 Design | E.1 (managed lanes), E.2 (full framework) | This spec + investment plan |

**Note**: B.4 (T1/T1 Intersection Resilience) is added to the module as a result of the diamond intersection analysis. The k-connectivity computation and diamond investment NPV are the primary contributions.

---

## §11. The CLI Toolkit

The `route` CLI is the computational backbone. Current commands:

```
route fetch              # Download FHWA data (TIGER, HPMS, NBI, FAF5, FEMA)
route fetch-hpms         # Download HPMS from geo.dot.gov (no auth required)
route build              # Build HighwayGraph with all attribute joins
route score <corridor>   # Score one corridor against 12 dimensions
route score-all          # Score all corridors; compute Brandes centrality
route flow <corridor>    # Max-flow capacity analysis; identify bottleneck
route invest --budget N  # LP investment allocation across corridors
route gap --type X       # Gap analysis by type
route map <corridor>     # Render corridor map PNG
route report <corridor>  # Write corpus markdown entry
route calibrate          # Rubric calibration pass (after 20+ corridors scored)
```

Planned commands (from §7 simulation + §4 diamond analysis):

```
route sim --scenario <name>             # Run named chaos scenario
route sim --chaos --iterations N        # Monte Carlo chaos testing
route sim --intervention <type>         # Test proposed infrastructure change
route sim --investment-rank --budget N  # Rank all interventions by NPV/resilience
route intersection --t1 <A> --t1 <B>   # Analyze T1/T1 intersection k-connectivity
route diamond --at <intersection>       # Design optimal diamond for intersection
route od-analysis --from X --to Y       # Full O-D analysis: path, PTI, incident sim
```

---

## §12. Spec Amendment Log

| Date | Amendment | Reason |
|---|---|---|
| 2026-05-06 | Initial spec | Synthesizes all design work from ROUTE session |
| — | B.4 paper added to module | T1/T1 diamond intersection analysis warrants own paper |
| — | §7 (Simulation Toolkit) added | Chaos simulation identified as critical missing capability |
