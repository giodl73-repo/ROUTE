---
module: route
title: "Interstate 2.0: Data-Driven Analysis and Design of the US Highway Network"
tracks: [A, B, C, D, E, F]
papers: 18
panel_score: 7.9/10
panel_tier: A-
panel_round: 2
panel_round_1_score: 7.4/10
panel_round_1_tier: B+
rubric_version: v1.4
created: 2026-05-06
updated: 2026-05-08
---

# ROUTE — Research Module

## Theme

The US chose highways. ROUTE answers: which ones matter, what's missing, and what does the next version look like?

The module scores 227 existing interstate corridors to calibrate a measurement instrument (Track A), identifies structural gaps in the national network (Track B), quantifies the freight economics of those gaps (Track C), maps climate and incident exposure (Track D), synthesizes the findings into a design specification for Interstate 2.0 (Track E), and shows how the highway investment unlocks a passenger transit layer at near-zero incremental cost (Track F).

---

## Three Properties Check

| Property | Status | Notes |
|---|---|---|
| Causal chain | ✓ Designed | Each paper names its dependency — see chain sentences below |
| No weak links | Designed | All papers have quantification contracts; A.1 and C.1 intro/conclusion written |
| Actionable numbers | Designed | Each paper has a primary number and experiment |

---

## Tracks

### Track A — Corpus & Scoring

**Panel score (R2)**: 8.0/10 | **Chain**: Strong ✓ | **PP1**: PP1.1 (CI for ρ=0.81)

**Theme**: Calibrate the measurement instrument from the existing interstate corpus.

**Chain**: A.1 establishes 4-tier arterial hierarchy → A.2 requires A.1's tier labels to interpret calibration findings → both required by all downstream papers that cite dimension scores.

**Papers**:
| Paper | Title | Stage | Score |
|---|---|---|---|
| A.1 | Interstate Arterials: Tiering the National Highway Network | ready | — |
| A.2 | Rubric Calibration: v1.0→v1.4 Calibration with 16 Dimensions | ready | — |

**Track arc**: Scoring 227 interstate corridors against 16 validated dimensions (v1.4) reveals a four-tier natural hierarchy. Eight Primary Arteries — less than 12% of route miles — carry more than 50% of national truck freight ton-miles and occupy betweenness centrality scores at least 3× higher than the next tier. The calibration pass (A.2) validates all 16 dimensions against real federal data; A5 Safety (FARS fatality rates) emerges as a genuine differentiator. External validation: ρ=0.81 against STRAHNET classification (CI pending — PP1.1).

---

### Track B — Gap Analysis

**Panel score (R2)**: 7.5/10 | **Chain**: Strong ✓ | **PP2**: PP2.2 (K_port calibration)

**Theme**: Find what's missing — links, capacity, and resilience.

**Chain**: B.1 maps missing links using calibrated rubric → B.2 identifies bottlenecks (requires B.1's network map to distinguish bottleneck from gap) → B.3 identifies resilience holes (compound failures requiring both B.1 and B.2) → B.4 maps T1/T1 intersection resilience → B.5 extends to port connector gaps.

**Papers**:
| Paper | Title | Stage | Score |
|---|---|---|---|
| B.1 | Missing Links: Gap Analysis of the US Interstate Network | ready | — |
| B.2 | Freight Bottlenecks: Where the System Exceeds Capacity | ready | — |
| B.3 | Resilience Holes: Compound Exposure in the National Highway Network | ready | — |
| B.4 | T1/T1 Intersection Resilience: Diamond Zone Investment Case | ready | — |
| B.5 | The Last Interstate Mile: T1 Port Connections and the Highway-Maritime Interface | ready | — |

**Cross-track dependency**: B.3 requires D.1's ECH100-normalized D1 scores. Citation in place.

**Track arc**: The calibrated instrument identifies structural missing links, capacity bottlenecks ($22.7B annual congestion cost at top-50 ATRI nodes), and compound-exposure corridors (B1>7 AND D1>6) that affect rural populations with no alternate routing. B.5 adds a systemic diagnosis: 7/10 major US port connector segments operate at V/C>1.2 at peak — equivalent to the worst T1 bottlenecks — but are excluded from T1-tier analysis by their short length. Savannah's I-16/I-95 connector becomes the binding East Coast freight constraint within 5 years at current port growth trajectory.

---

### Track C — Freight & Throughput

**Panel score (R2)**: 8.5/10 | **Chain**: Strong ✓ | **PP2**: PP2.1 (C.4 algorithm scalability)

**Theme**: Quantify the economic cost of network gaps on real O-D flows; quantify the economic opportunity unlocked by I2.0 relay.

**Chain**: C.1 characterizes NY→LA and HOU→CHI corridors with capacity, PTI, transit time → C.2 runs national max-flow using C.1's corridor characterization → C.3 quantifies the supply-chain economics of the 48-hour corridor unlocked by managed lanes + relay → C.4 models the load-matching relay hub marketplace and empty-backhaul reduction.

**Papers**:
| Paper | Title | Stage | Score |
|---|---|---|---|
| C.1 | Freight Reliability on the NY–LA and Houston–Chicago Corridors | ready | — |
| C.2 | National Max-Flow: Capacity and Bottleneck Analysis of the Interstate Network | ready | — |
| C.3 | The 48-Hour Corridor: Economic Opportunities Unlocked by I2.0 Transcontinental Freight | ready | — |
| C.4 | Empty Miles and Load Matching: The Relay Marketplace as National Freight Optimizer | ready | — |

**Track arc**: The NY→LA northern route has a binding capacity constraint at Donner Pass (91,200 vpd). Current PTI 1.86 requires 80–100 hour shipper commitment windows. I2.0 managed lanes (PTI ≤ 1.15) plus relay drive transit time to 48 hours — a categorical shift from air-freight economics to truck economics. This unlocks $8.1B/yr in air-to-truck freight substitution (domestic air cargo currently flying coast-to-coast at $4/lb shifts to refrigerated truck at $0.40/lb). Relay hub load-matching reduces national empty backhaul from 35% to 18–22%, capturing $113–135B/yr in freight efficiency at zero additional infrastructure cost. Simulation validated: relay-only captures 65–90% of full I2.0 SLA improvement at 0.03% of managed-lane capital cost.

---

### Track D — Resilience

**Panel score (R2)**: 8.0/10 | **Chain**: Strong ✓ | **No open PP items**

**Theme**: Map the climate and incident exposure and price the risk.

**Chain**: D.1 maps multi-hazard climate exposure across the corpus (v1.4 ECH100 normalization) → D.2 models incident economics using D.1's exposure map to price the risk correctly.

**Papers**:
| Paper | Title | Stage | Score |
|---|---|---|---|
| D.1 | Climate Exposure in the Interstate System: Multi-Hazard, NBI, and 2050 Projections | ready | — |
| D.2 | The Economics of Corridor Closures: Freight Cost and Redundancy Value | ready | — |

**Track arc**: D.1 integrates FEMA NFHL flood zones, NBI bridge condition data (95k bridges), FARS safety, wildfire/tornado/seismic exposure — all normalized to expected annual lane-closure-hours per 100 miles (ECH100). Five highest-risk corridors account for a disproportionate share of closure frequency; $6.2B/yr in freight disruption cost from top-5 closure events. Break-even analysis shows redundancy hardening investment justified at 7% discount for corridors with ECH100 > threshold.

---

### Track E — Interstate 2.0 Design

**Panel score (R2)**: 8.0/10 | **Chain**: Strong ✓ | **No open PP items**

**Theme**: Synthesize all findings into a design specification and investment case with relay sequencing.

**Chain**: E.1 makes the economic/engineering case for managed freight lanes using C.1 and C.2 baseline → E.2 assembles the full I2.0 specification using E.1's managed-lane model plus B.3's resilience holes plus D.2's incident economics, with Phase 0 relay sequencing.

**Papers**:
| Paper | Title | Stage | Score |
|---|---|---|---|
| E.1 | Managed Freight Lanes: Throughput, Transit Time, and NPV | ready | — |
| E.2 | Interstate 2.0: A Design Framework for Throughput, Resilience, and Shared Transit | ready | — |

**Track arc**: Managed freight lanes increase corridor throughput by 50% and reduce transcontinental transit by ~1 day, yielding $121B NPV at 7% discount (2.3:1 B/C). The full I2.0 portfolio ($253B cost, $298B NPV) is sequenced: Phase 0 — relay marketplace ($40M, zero infrastructure, captures 65–90% of SLA gain); Phase 1 — managed lanes T1 ($121B); Phase 2 — resilience hardening and intermodal integration. NPV arithmetic closes: benefit reconciliation table shows how each component contributes to $31.2B/yr discounting to $298B at 7% over 30 years.

---

## Module Arc

The United States built the interstate system in 30 years and has maintained it poorly for 50. ROUTE establishes, from publicly available federal data, that the system has a measurable tier structure (Track A), identifiable structural gaps (Track B), a quantifiable freight reliability deficit (Track C), a climate exposure profile that is worsening (Track D), and a feasible upgrade path (Track E).

The upgrade path — Interstate 2.0 — is not a new interstate system. It is a targeted investment in the 8 corridors that carry half the freight, the 15 missing links that would close the largest gaps, and the resilience hardening that would prevent the most expensive failures.

The country chose highways. This is the evidence for which ones to invest in next.

---

## Quantification Contracts

| Paper | Primary Number | Experiment Design | Decision It Changes | Null Fallback |
|---|---|---|---|---|
| A.1 | Tier 1 carries ≥50% ton-miles; Brandes gap Tier1/Tier2 ≥ 3× | Score 227; natural break cluster | Investment tier priority | Tiers collapse to 2 |
| A.2 | 16 dimensions calibrated with variance, confidence, and review-risk ledgers | Variance + correlation + confidence-risk summary at N≥20 | Which dimensions and source gaps to prioritize downstream | Confidence-risk summary missing |
| B.1 | K missing links with gap score ≥ 7.5; avg nearest-interstate X miles | Score proposed vs corpus distribution | Which proposed corridors to advance | No corridor above corpus 75th |
| B.2 | M corridors at V/C > 0.85; top-10 ATRI cost $X billion | ATRI + HPMS AADT + lane count join | Bottleneck investment sequencing | ATRI ≠ A1 scores |
| B.3 | N compound-exposure corridors; X% rural pop affected | Intersect B1 > 7.0 AND D1 > 6.0 | Resilience investment targeting | No compound corridors |
| B.4 | 9 of 15 T1/T1 intersections k=1; diamond investment $4.5B total; NPV vs managed lanes | k-connectivity analysis on intersection subgraphs | Diamond investment priority | Connectivity already adequate |
| C.1 | PTI on I-80: Z.Z; SLA window narrows from N to M hours | PTI model + managed lane simulation | Whether managed lanes justify cost | PTI data unavailable |
| C.2 | Donner closure drops throughput Z%; I-69 adds W% HOU-CHI | Edmonds-Karp incident simulation | I-69 investment priority | Graph too fragmented |
| D.1 | X corridor-miles SFHA; top-3 by max consecutive miles | FEMA NFHL polygon join | Climate adaptation investment order | Insufficient overlap |
| D.2 | Top-5 closure annual cost $Y billion; break-even $Z billion | Freight value × closure × detour | Redundancy investment case | No closure frequency data |
| E.1 | Managed lanes: transit −20%; PTI 1.8→1.15; NPV $X M/mi | Throughput + PTI simulation | Whether managed lanes are fundable | NPV negative |
| E.2 | I2.0 portfolio: $X trillion cost, $Y trillion NPV; W% reliability gain | Investment LP on full corpus | National I2.0 investment plan | No optimal solution |
| B.5 | 7/10 port connectors at V/C>1.2 peak; $10.8B/yr Laredo delay cost; $20-50B investment unlocks 15% national throughput gain | HPMS V/C + drayage peak model + USDOT NPV | Port connector investment priority; K_port standard adoption | Connectors already adequate |
| C.3 | $8.1B/yr air-to-truck freight substitution; $15-25B/yr total corridor economic value | Seven-sector analysis; air cargo BTS data; modal cost differential | Whether 48h truck is an economic transformation or incremental improvement | No addressable air freight market |
| C.4 | Empty backhaul 35%→18-22%; $113-135B/yr efficiency gain; 45M fewer empty truck-miles/day | Bipartite hub load-matching model; UPS Worldport benchmark calibration | Relay hub investment priority by flow imbalance; load-matching vs. driver-matching value | Hub catchment too thin for match rate |
| F.1 | 12.4M transit-dependent travelers within 30 miles of T1/T1 hub (geographic proximity); $500-800M feeder gap | Hub coverage analysis + ACS B08201 transit-dependent pop join | Whether hub investment justifies transit layer | Hub locations don't align with population |
| F.2 | I2.0 bus 28-45% faster than alternatives; effective ~58 mph with stop penalty; 24M passengers on 12 corridors | PTI model applied to bus service; stop_penalty = N_stops × 8 min | Whether I2.0 bus is competitive with rail alternatives | Congestion still too high on GP lanes |
| F.3 | 27-day hub payback; $68M/yr net margin per hub; relay-only captures 65-90% of full I2.0 SLA at 0.03% of capital | Slot exchange revenue model; sla-matrix simulation (5,000 trips/corridor) | Whether relay marketplace is the correct Phase 0 investment | Regulatory barriers make relay unviable |

---

## Track F — Transit + Relay

**Panel score (R2)**: 7.5/10 | **Chain**: Strong ✓ | **PP2**: PP2.3 (decarbonization stack)

**Theme**: The I2.0 highway investment unlocks a national passenger transit layer at near-zero incremental cost; the relay marketplace is the institutional foundation for both freight and transit operations.

**Chain**: F.1 establishes which T1/T1 hubs serve transit-dependent populations (proximity) → F.2 computes bus corridor travel times using F.1's hub locations → F.3 designs the relay marketplace platform that F.1 and F.2 presupposed as the institutional mechanism.

**Papers**:
| Paper | Title | Stage | Score |
|---|---|---|---|
| F.1 | T1/T1 as Transit Nodes: The Interstate 2.0 Passenger Layer | ready | — |
| F.2 | Intercity Bus Corridors: Travel Time, Coverage, and Equity on the T1 Network | ready | — |
| F.3 | The Relay Marketplace: Platform Design for 48-Hour National Freight | ready | — |

**Track arc**: Nine T1/T1 diamond hubs and ~50 T1/T2 regional stops create a transit network serving 12.4M transit-dependent travelers within 30 miles (geographic proximity; feeder service gap estimated at $500–800M, not included in hub cost). I2.0 intercity bus travel times are 28–45% faster than current alternatives; effective average ~58 mph with stop penalty. F.3's relay marketplace (slot exchange + hub operator model + independent relay driver layer) resolves the coordination failure that prevents cross-carrier relay today: 27-day payback at $200k/day revenue per hub, $68M net margin per hub per year. FMCSA rulemaking pathway (18–24 months) is realistic and does not require statutory change. AV transition: managed freight lanes are designed as the AV operating environment; relay hubs become AV handoff nodes in 10–15 years.

---

## Panel Review History

| Round | Date | Score | Tier | PP1 | PP2 | Panel |
|---|---|---|---|---|---|---|
| 1 | 2026-05-07 | 7.4/10 | B+ | 4 (all resolved) | 6 (4 resolved) | Hanson·Adamic·Puentes·McKinnon·Chester·Schmitt·Walker |
| 2 | 2026-05-08 | 7.9/10 | A- | 1 | 3 | Hanson·Adamic·Puentes·McKinnon·Chester·Neumark·Walker |

---

## Self-Score (updated post Round 2)

| Property | Score | Rationale |
|---|---|---|
| Causal chain | 8.0 | All 18 papers have explicit chain dependencies; no broken links; F.3 closes the F-track institutional gap |
| No weak links | 7.5 | Three new PP2 items from new papers (C.4 algorithm, B.5 calibration, F.3 decarbonization); no PP1 weak links remain except A.2 CI |
| Actionable numbers | 8.5 | C.3 and C.4 add large, vivid, independently verifiable numbers; all 18 contracts honored |
| **Round 2 panel score** | **7.9** | A-; path to A clears 4 PP items across 4 sessions |
