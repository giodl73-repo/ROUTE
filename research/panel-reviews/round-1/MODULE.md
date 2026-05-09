---
module: route
title: "Interstate 2.0: Data-Driven Analysis and Design of the US Highway Network"
tracks: [A, B, C, D, E, F]
papers: 13
panel_score: ~
panel_round: 0
created: 2026-05-06
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

**Theme**: Calibrate the measurement instrument from the existing interstate corpus.

**Chain**: A.1 establishes 4-tier arterial hierarchy → A.2 requires A.1's tier labels to interpret calibration findings → both required by all downstream papers that cite dimension scores.

**Papers**:
| Paper | Title | Stage | Score |
|---|---|---|---|
| A.1 | Interstate Arterials: Tiering the National Highway Network | draft | — |
| A.2 | Rubric Calibration: Which 12 Dimensions Actually Differentiate Corridors | planned | — |

**Track arc**: Scoring 227 interstate corridors against 12 candidate dimensions reveals a four-tier natural hierarchy. Eight Primary Arteries — less than 12% of route miles — carry more than 50% of national truck freight ton-miles and occupy betweenness centrality scores at least 3× higher than the next tier. The calibration pass (A.2) reduces the 12-dimension candidate pool to 9 validated dimensions, retiring at least two correlated pairs.

---

### Track B — Gap Analysis

**Theme**: Find what's missing — links, capacity, and resilience.

**Chain**: B.1 maps missing links using calibrated rubric → B.2 identifies bottlenecks (requires B.1's network map to distinguish bottleneck from gap) → B.3 identifies resilience holes (compound failures requiring both B.1 and B.2).

**Papers**:
| Paper | Title | Stage | Score |
|---|---|---|---|
| B.1 | Missing Links: Gap Analysis of the US Interstate Network | planned | — |
| B.2 | Freight Bottlenecks: Where the System Exceeds Capacity | planned | — |
| B.3 | Resilience Holes: Compound Exposure in the National Highway Network | planned | — |

**Track arc**: The calibrated instrument identifies K missing links scoring above the corpus 75th percentile. M existing corridors operate at V/C > 0.85 during peak periods, accounting for $X billion in annual congestion costs. N compound-exposure corridors — where a capacity constraint and access gap occur simultaneously — affect X% of the rural population with no viable alternate.

---

### Track C — Freight & Throughput

**Theme**: Quantify the economic cost of network gaps on real O-D flows.

**Chain**: C.1 characterizes NY→LA and HOU→CHI corridors with capacity, PTI, transit time → C.2 runs national max-flow using C.1's corridor characterization to interpret flow patterns at national scale.

**Papers**:
| Paper | Title | Stage | Score |
|---|---|---|---|
| C.1 | Freight Reliability on the NY–LA and Houston–Chicago Corridors | draft | — |
| C.2 | National Max-Flow: Capacity and Bottleneck Analysis of the Interstate Network | planned | — |

**Track arc**: The NY→LA northern route has a binding capacity constraint at Donner Pass (~91k vpd). Current PTI values of 1.8–2.2 require 80–100 hour shipper commitment windows. Interstate 2.0 managed lanes (PTI ≤ 1.15) narrow the window to 48 hours and reduce transit time by ~20%. I-69 completion reduces the HOU→CHI distance by 120 miles, eliminating two high-variance interchange nodes.

---

### Track D — Resilience

**Theme**: Map the climate and incident exposure and price the risk.

**Chain**: D.1 maps climate exposure across the corpus → D.2 models incident economics using D.1's exposure map to price the risk correctly.

**Papers**:
| Paper | Title | Stage | Score |
|---|---|---|---|
| D.1 | Climate Exposure in the Interstate System: FEMA Flood Zones and 2050 Projections | planned | — |
| D.2 | The Economics of Corridor Closures: Freight Cost and Redundancy Value | planned | — |

**Track arc**: X corridor-miles lie within FEMA SFHA flood zones; I-10 through coastal Louisiana has Y consecutive miles of exposure. The five highest-risk corridors account for a disproportionate share of flood-related closure frequency; each closure day costs the freight economy ~$X million.

---

### Track E — Interstate 2.0 Design

**Theme**: Synthesize all findings into a design specification and investment case.

**Chain**: E.1 makes the economic/engineering case for managed freight lanes using C.1 and C.2 baseline → E.2 assembles the full I2.0 specification using E.1's managed-lane model plus B.3's resilience holes plus D.2's incident economics.

**Papers**:
| Paper | Title | Stage | Score |
|---|---|---|---|
| E.1 | Managed Freight Lanes: Throughput, Transit Time, and NPV | planned | — |
| E.2 | Interstate 2.0: A Design Framework for Throughput, Resilience, and Shared Transit | planned | — |

**Track arc**: Managed freight lanes increase corridor throughput by 50% and reduce transcontinental transit by ~1 day, yielding NPV of $X million per corridor-mile at 7% discount. The full I2.0 portfolio — managed lanes (Tier 1), intermodal integration (Tier 2), EV charging (Tier 1–2), resilience hardening (exposure points) — costs $X trillion with $Y trillion NPV benefit and reduces national freight reliability variance by W%.

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
| F.1 | N million transit-dependent travelers within X miles of T1/T2 hub; Y% of standalone transit cost | Hub coverage analysis + transit-dependent pop Census join | Whether hub investment justifies transit layer | Hub locations don't align with population |
| F.2 | I2.0 bus corridor travel time vs. current best alternative on each T1; PTI benefit for bus passengers | PTI model applied to bus service; O-D pair analysis | Whether I2.0 bus is competitive with rail alternatives | Congestion still too high on GP lanes |

---

## Track F — Transit Integration

**Theme**: The I2.0 highway investment unlocks a national passenger transit layer at near-zero incremental cost.

**Chain**: F.1 establishes which T1/T1 hubs serve transit-dependent populations and quantifies the coverage gain → F.2 requires F.1's hub locations to compute realistic bus corridor travel times and compare them to existing alternatives.

**Papers**:
| Paper | Title | Stage | Score |
|---|---|---|---|
| F.1 | T1/T1 as Transit Nodes: The Interstate 2.0 Passenger Layer | planned | — |
| F.2 | Intercity Bus Corridors: Travel Time, Coverage, and Equity on the T1 Network | planned | — |

**Track arc**: Nine T1/T1 diamond hubs and ~50 T1/T2 regional stops create a transit network serving N million transit-dependent travelers — at a marginal hub investment of $2B on a $209B highway program. I2.0 intercity bus travel times are Y% faster than current equivalents on every T1 corridor, competitive with Amtrak on corridors where Amtrak exists and available on corridors where it does not.

---

## Self-Score (estimated, pre-review)

| Property | Score | Rationale |
|---|---|---|
| Causal chain | 8.5 | All chains specified; E.2 depends on A, B, C, D all completing — long dependency |
| No weak links | 7.5 | A.2 and D.2 have data availability risks (calibration requires N≥20 scored, closure frequency data scarce) |
| Actionable numbers | 8.0 | Every contract has a specific number; C.1 has PTI fallback risk |
| **Estimated panel score** | **8.0** | Strong design; actualize contracts before review |
