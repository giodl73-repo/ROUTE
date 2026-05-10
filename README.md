# ROUTE — Interstate 2.0

The United States chose highways. Not rail, not canals, not airships — highways. The interstate system is the country's circulatory system: 48,800 miles of concrete and asphalt that move 72% of the nation's freight by value and connect every major metro to every other. It was built in 30 years. It has been maintained poorly for 50.

ROUTE is a research and design project for what comes next.

See [docs/SYSTEM_PLAN.md](docs/SYSTEM_PLAN.md) for the living milepost roadmap that connects the specs, Rust implementation, research tracks, and `.roles` review process. See [docs/SPEC_INDEX.md](docs/SPEC_INDEX.md) for spec ownership.

ROUTE also has a game-facing path: [Interstate Tycoon](docs/INTERSTATE_TYCOON.md), a highway strategy game concept that puts the simulation engine under a playable infrastructure loop.

---

## The argument

The interstate system as built optimized for political feasibility, not national efficiency. Corridors were added because senators wanted them, not because the network needed them. Corridors that were strategically obvious were skipped because they crossed the wrong states or had the wrong congressional champions. The result is a network with genuine strengths — coast-to-coast trunks, access to major ports — and systematic gaps: missing redundancy on critical corridors, under-served agricultural regions, freight bottlenecks that cost the economy billions annually, and entire swaths of rural America with no interstate within 50 miles.

The same pattern that produced TIGRIS — score a corpus on enough dimensions and the design space tells you its own gaps — applies here. Score enough existing interstates on enough dimensions (throughput, freight intensity, redundancy, rural connectivity, climate resilience, multimodal integration) and the gaps become visible, not argued. Then design into them on purpose.

---

## The method

ROUTE uses a Milepost plan: each phase is a visible marker from raw data to defensible investment proposal.

**Milepost 0 — Ground Survey**: Establish the repo, specs, data inventory, roles, and CLI scaffold.

**Milepost 1 — Instrument**: Score existing interstate corridors against the 16-dimension pool. Every score is cited; every estimate is labeled. The rubric evolves from what actually differentiates real corridors.

**Milepost 2 — Atlas**: Re-run the corpus and calibration ledger until the national tier map is reproducible.

**Milepost 3 — Fault Lines**: Project the scored corpus into dimension space. Find missing links, bottlenecks, resilience holes, port connector gaps, and coverage gaps. Translate them back to geography.

**Milepost 4 — Pressure Test**: Run flow, incident, SLA, relay, and investment simulations so the gaps have operational and economic meaning.

**Milepost 5 — The Forum**: Run high-stakes claims through Parliament, stakeholder lenses, editorial gates, and panel review. The experts plant incompatible stakes. The argument record is the output; consensus is not the goal.

**Milepost 6 — Blueprint**: Specify which Interstate 2.0 features apply: managed freight lanes, shared transit facilities, intermodal hubs, EV charging corridors, resilience hardening, enhanced rest areas, rural connectivity spurs, relay hubs, and diamond intersections.

**Milepost 7 — Program**: Make the corpus, maps, papers, and design claims reproducible enough to publish.

---

## The corpus

Existing US interstate corridors, scored and cited:

| Corridor | Status |
|---|---|
| I-80 (New York → San Francisco) | ⏳ anchor — first to run |
| ... | ... |

Proposed corridors under analysis:

| Corridor | Gap type targeted | Status |
|---|---|---|
| ... | ... | ... |

---

## The dimension pool

16 dimensions across 4 bands. Each scored 0–10 per corridor. The pool is a candidate until the corpus calibration pass validates which dimensions actually differentiate.

| Band | Dimensions |
|---|---|
| A — Flow | Throughput Gap · Freight Intensity · Speed Reliability · International Trade Corridor · Safety Record |
| B — Network | Redundancy · Network Centrality · Port/Border Access · Military/Strategic |
| C — People | Population Reach · Rural Connectivity · Equity Access · Agricultural Export Access |
| D — Future | Climate Resilience · Multimodal Integration · Infrastructure Vintage |

---

## The parliament

Seven expert voices. Adversarial by design.

**General Eisenhower** — national defense and economic unity.
**Robert Moses** — throughput and deliverability at scale.
**Anthony Foxx** — equity and access.
**Freight Economist** — NPV and commodity flow.
**Traffic Engineer** — capacity, geometry, and safety.
**Climate Resilience Engineer** — 2050 exposure and hardening.
**Rural Advocate** — agricultural access and rural connectivity.

No voice is skipped. A corridor that survives all seven has a real case.

---

## Research

Research papers will be published here as the project progresses.

*(None yet — corpus scoring in progress.)*

---

## Data

Key sources: FHWA Highway Statistics, ATRI Freight Bottleneck Reports, BTS Freight Facts and Figures, FHWA National Bridge Inventory, Census TIGER/Line. Full source catalog in `data/sources.md`.

GIS data: ArcGIS project mapping potential corridors provides the spatial foundation for gap analysis.

---

## License

[MIT](LICENSE) — © 2026 Gio Della-Libera.
