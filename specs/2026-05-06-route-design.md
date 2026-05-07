---
name: ROUTE Design Specification v1.0
slug: route-design-v1
type: spec
status: draft
rubric_version: v1.0
author: human
created: 2026-05-06
updated: 2026-05-06
sources: []
---

# ROUTE Design Specification v1.0

## §1. What ROUTE Is

ROUTE scores existing US interstate corridors against a calibrated set of dimensions, finds gaps in the coverage space, and designs into them. It is a research project, not an advocacy document. The output is a ranked set of proposed corridors and upgrades with economic, engineering, and equity evidence behind each.

The shape is borrowed from TIGRIS: score a corpus → let the rubric emerge from variance → find empty regions → design into them. The difference is that ROUTE's corpus is fixed by history (the existing interstate system), while TIGRIS's corpus grows with each review.

### What ROUTE is not

- An advocacy brief for any specific corridor
- A construction plan or engineering study
- A political document
- A prediction of what will actually be built

---

## §2. The Corpus

### Existing interstates

The US interstate system has ~48,800 miles of mainline routes. ROUTE does not score every segment — it scores **corridors**: contiguous stretches of interstate that form a recognizable route from terminus to terminus.

For trunk routes (I-10, I-90, etc.), the corridor is the full named route.
For regionally-significant connectors (I-81, I-77, etc.), the corridor is the full named route.
For proposed corridors, the corridor is the proposed route from its declared termini.

Target corpus size: **40–60 existing corridors + 20–30 proposed corridors**.

The anchor is the first corridor to go through the full pipeline. Suggested anchor: **I-80** (New York to San Francisco — the archetypal cross-country trunk, well-documented, high variance on every dimension).

### Proposed corridors

Sources for proposed corridors:
- FHWA Future Interstate Study (2000)
- AASHTO proposed corridor designations
- State long-range transportation plans
- User's ArcGIS project (`C:\Users\giodl\OneDrive\Documents\ArcGIS\Projects\Truck Highways\`) — mapped potential corridors

Known proposed corridors as of 2026:

| Proposed designation | Termini (approx) | Key states | Status |
|---|---|---|---|
| I-3 | Savannah, GA → Detroit, MI | GA, TN, KY, OH | Proposed; no federal action |
| I-11 | Phoenix, AZ → Las Vegas, NV → Reno, NV | AZ, NV | Partially designated |
| I-14 | I-10 in TX → I-20 in TX → Savannah, GA | TX, LA, MS, AL, GA | Partially designated in TX |
| I-57 | I-55 in MO → I-74 in IL | MO, IL | Study phase |
| I-73 | Charlotte, NC → Akron, OH | NC, VA, WV, OH | Proposed |
| I-74 | Cincinnati, OH → Myrtle Beach, SC | OH, KY, NC, SC | Partially designated |
| I-87 | Bismarck, ND → Canada border | ND | Proposed |
| Gulf Coast Corridor | Houston, TX → Pensacola, FL | TX, LA, MS, AL, FL | Parallel to I-10; proposed |
| Appalachian Connector | Various E-W gaps in Appalachians | Multiple | Multiple studies |
| Northern Tier | Various US-2 corridor proposals | MT, ND, MN, WI, MI | Study phase |

This list grows as research proceeds. Every addition gets a corpus entry in `corpus/proposed/`.

---

## §3. The Dimension Pool

12 candidate dimensions, 4 bands. Scored 0–10 for each corridor. The rubric is a candidate until the corpus calibration pass (§4.3) validates which dimensions actually differentiate corridors.

### Band A — Flow

| Dim | Name | 0 | 10 |
|---|---|---|---|
| A1 | **Throughput Gap** | Well below capacity; free-flow | Severely congested; LOS E/F on majority of route |
| A2 | **Freight Intensity** | <500 trucks/day average | >10,000 trucks/day average; major commodity corridor |
| A3 | **Speed Reliability** | Average speed near design speed (≥65 mph) | Chronically below design speed; high variance |

### Band B — Network

| Dim | Name | 0 | 10 |
|---|---|---|---|
| B1 | **Redundancy** | 3+ parallel interstate-quality alternatives | No alternative route; single point of failure |
| B2 | **Network Centrality** | Peripheral; low betweenness centrality | Spine route; loss would cascade across national network |
| B3 | **Port/Border Access** | No port or border crossing connectivity | Direct access to top-tier port or major border crossing |

### Band C — People

| Dim | Name | 0 | 10 |
|---|---|---|---|
| C1 | **Population Reach** | <500k people within 50 miles | >20M people within 50 miles |
| C2 | **Rural Connectivity** | Primarily urban; minimal agricultural/rural service | Primary access route for large agricultural region or rural population |
| C3 | **Equity Access** | Primarily serves high-income metros | Primary access for low-income, tribal, or rural communities with limited alternatives |

### Band D — Future

| Dim | Name | 0 | 10 |
|---|---|---|---|
| D1 | **Climate Resilience** | Low exposure; inland, low flood/heat/fire risk | High exposure; coastal flooding, wildfire corridor, extreme heat zone |
| D2 | **Multimodal Integration** | No rail proximity; no intermodal facilities | Adjacent freight rail; major intermodal hubs; transit connection potential |
| D3 | **Infrastructure Vintage** | Recent construction; good condition; low deferred maintenance | Pre-1970 construction; poor condition; significant deferred maintenance backlog |

**Scoring note**: High score on A1, A2, B1, B2 = strong case for corridor upgrade. High score on D1 = resilience risk requiring hardening. High score on D3 = maintenance priority. The dimensions are descriptive, not evaluative — the parliament interprets them.

---

## §4. The Pipeline in Detail

### §4.1 CORPUS — Scoring an existing corridor

**Input**: Interstate designation (e.g., "I-80")
**Output**: `corpus/existing/{designation}.md` with all 12 dimension scores and sources

Process:
1. Identify termini and key intermediate points.
2. Pull data from declared sources (`data/sources.md`): FHWA traffic counts, ATRI freight data, BTS corridor statistics, FHWA bridge/pavement ratings, Census population within buffer.
3. Score each of the 12 dimensions 0–10. Every score gets a one-sentence justification + source citation.
4. Calculate and record band totals (A: flow, B: network, C: people, D: future).
5. Write corridor entry with frontmatter, dimension table, and key facts.
6. Update `personas/axis-pool.md` scoring ledger.
7. Update TRACKER.md.

### §4.2 CORPUS — Scoring a proposed corridor

Same as existing, but:
- Scores are estimates, not measurements. Mark every estimated score with `†`.
- Source for geometry: ArcGIS project, FHWA study, or state DOT plan — cite which.
- For population reach and rural connectivity: use Census buffers along the proposed alignment.
- For freight intensity: use commodity flow data for the origin-destination pair, not actual counts.

### §4.3 RUBRIC CALIBRATES — Amendment pass

Run after 20+ existing corridors are scored.

1. For each dimension: compute variance, mean, and interquartile range across the scored corpus.
2. Flag low-variance dimensions (IQR < 2.0) as retirement candidates — they don't differentiate.
3. Flag high-correlation dimension pairs (r > 0.85) — one candidate for retirement.
4. Propose retirement or redefinition for flagged dimensions. Write to `personas/axis-pool.md` Changelog.
5. Bump rubric version (`v1.0` → `v1.1`). Prior scores are frozen at the version they were scored under.

### §4.4 GAP MAP — Finding empty regions

After calibration:
1. Project all scored corridors onto the dimension space.
2. Identify under-populated regions: combinations of dimension scores with few or no existing corridors.
3. Translate dimension-space gaps to geographic gaps: where on the map does this empty region correspond?
4. Write gap findings to `gaps/{slug}.md`.

Key gap types for highway networks:
- **Missing link**: Two high-intensity endpoints with no direct interstate connection
- **Bottleneck**: Corridor that carries disproportionate load with no parallel alternative
- **Resilience hole**: Region with single-corridor access, high climate exposure
- **Equity gap**: Rural or low-income region with no interstate within 50 miles
- **Intermodal gap**: High-freight corridor with no intermodal facility for 300+ miles

### §4.5 CONCEPT — Proposing a corridor

1. Name the gap type (from §4.4 gap types).
2. Identify termini that address it.
3. Write `corpus/proposed/{slug}.md` with dimension scores (estimated, marked `†`).
4. State the Interstate 2.0 features the corridor would require (see §5).

### §4.6 PARLIAMENT — Expert review

Run `/route-panel {corridor-slug}` (or manually once skill is built).

Three phases:
- **STAKES**: Each of 7 voices drafts 2 dimensions they'll argue from. Overlaps are designed — Eisenhower and the Freight Economist both want Network Centrality; they argue from different foundations.
- **ARGUMENT**: Corridor is "walked" — key segments, key moments (urban approaches, rural spans, choke points, climate-exposed segments). Each voice attacks, defends, or collides on their staked dimensions at each moment.
- **AMENDMENT**: After review, any dimension that generated no argument in 3+ consecutive reviews is a retirement candidate. Any new dimension surfaced by the argument record is a promotion candidate.

### §4.7 DESIGN — Interstate 2.0 proposal

A design proposal in `design/{slug}.md` specifies:
- Corridor geometry and alignment (prose description + reference to ArcGIS data)
- Interstate 2.0 features included (see §5)
- Economic case: cost estimate range, NPV estimate, commodity flow served
- Phasing: what gets built first and why
- Known objections and how the design addresses them

---

## §5. Interstate 2.0 Feature Set

These are the building blocks of an Interstate 2.0 design. Not every corridor needs all of them. The parliament and design process determines which apply.

| Feature | What it means |
|---|---|
| **Managed freight lanes** | Dedicated truck-only lanes with weight and height clearances above current standards; separated from passenger traffic |
| **Shared transit facilities** | Park-and-ride hubs, bus rapid transit infrastructure, intercity coach access points at major interchanges |
| **Intermodal hubs** | Truck-to-rail transfer facilities at major freight nodes; integrated with existing rail lines |
| **EV charging corridor** | High-capacity charging infrastructure at regular intervals (≤50 miles) for both passenger and commercial electric vehicles |
| **Resilience hardening** | Elevated roadbeds in flood zones, shaded rest areas in extreme heat zones, fire-resistant vegetation management, backup communication infrastructure |
| **Enhanced rest areas** | Full-service truck stops with showers, parking, health facilities, wi-fi — not just fuel — at ≤100 mile intervals |
| **Rural connectivity spurs** | Short connector routes (≤10 miles) linking rural communities to the main corridor |
| **Autonomous vehicle lanes** | Future-ready lane infrastructure for truck platooning and eventual AV freight |

---

## §6. Data Sources

See `data/sources.md` for full access notes. Key sources:

| Source | Data | URL |
|---|---|---|
| FHWA Highway Statistics | VMT, AADT, LOS by route | https://www.fhwa.dot.gov/policyinformation/statistics.cfm |
| FHWA National Bridge Inventory | Bridge condition, age, sufficiency rating | https://www.fhwa.dot.gov/bridge/nbi.cfm |
| ATRI (American Transportation Research Institute) | Truck congestion, freight bottlenecks | https://truckingresearch.org |
| BTS Freight Facts and Figures | Commodity flows, ton-miles by corridor | https://www.bts.gov |
| Census TIGER/Line + ACS | Population within buffer zones | https://www.census.gov/geo/maps-data/data/tiger.html |
| ArcGIS project (local) | User-mapped potential corridors | `C:\Users\giodl\OneDrive\Documents\ArcGIS\Projects\Truck Highways\` |
| FHWA Future Interstate Study (2000) | Proposed corridor list and justifications | FHWA archive |
| State DOT LRTPs | State-level proposed corridors | Per-state DOT websites |

---

## §7. Success Criteria

A completed ROUTE project produces:
1. ≥40 existing corridors scored in `corpus/existing/`
2. A calibrated rubric (≥1 amendment pass)
3. ≥1 gap map with identified missing-link, bottleneck, and resilience-hole candidates
4. ≥5 proposed corridors analyzed in `corpus/proposed/`
5. ≥3 full Parliament reviews in `reviews/`
6. ≥1 Interstate 2.0 design proposal in `design/`
7. ≥1 research paper in `research/papers/`

The anchor corridor (first through full pipeline) is the gate. Nothing else scales until it's done.

---

## §8. Spec Amendment Protocol

If scope needs to expand beyond this spec, add a dated amendment here. Don't silently expand. State what changed and why.

| Date | Amendment | Reason |
|---|---|---|
| — | — | — |
