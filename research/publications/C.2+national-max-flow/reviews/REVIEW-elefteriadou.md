---
reviewer: Lily Elefteriadou
persona: Lily Elefteriadou, Director, McTrans Center; Professor of Civil Engineering, University of Florida
round: 1
date: 2026-05-07
score: 3/4
---

> **Note:** This is an AI-generated simulated review, written by Claude in the voice of a named-expert persona. It is not an actual review by the named person and does not reflect their views or endorsement.

## Overall

The paper uses V/C-based capacity values as edge capacities in the max-flow model. This is a reasonable first approximation for a national-scale analysis, but the paper does not clearly state the source of the capacity values used, whether they reflect operational capacity or design capacity, and how they handle the significant variation in capacity across lanes, interchange configurations, and terrain. For a paper whose central claim is that specific arcs are "binding bottlenecks," the capacity values are the foundation of that claim — they need to be better documented. Score: 3/4.

## What Works

The 227-corridor, 48,000-edge graph is a reasonable specification for a national-scale max-flow model. Using HPMS as the capacity overlay is the correct source: HPMS provides segment-level annual average daily traffic (AADT), lane counts, and functional class, from which hourly capacity estimates can be derived using HCM 7th Edition basic freeway segment procedures. If the paper used this methodology, it is defensible.

The three binding bottleneck arcs — I-95 Baltimore-Washington (V/C 2.1+), Donner Pass (V/C 0.82 geographic), and Dallas interchange (V/C 1.9+) — are consistent with independent traffic data sources (ATRI, NPMRDS, HPMS) and with the C.1 paper's corridor analysis. The paper is correctly characterizing where the network is stressed.

The I-70W scenario (+4.7% NE→Pacific at baseline; Donner closure with I-70W → flow loss 23%→9%) is a useful counterfactual. The result quantifies I-70W's role as a relief valve for Donner — a finding that is not obvious from corridor-level analysis and demonstrates the value of the network-level max-flow approach.

## What Doesn't Work

The paper does not state clearly whether the edge capacities represent: (a) design capacity (from HCM lane capacity tables), (b) operational capacity (observed maximum throughput from HPMS), or (c) a V/C-adjusted effective capacity (design capacity × (1 - peak hour factor)). These three choices produce substantially different capacity values for high-volume urban segments. For Dallas interchange (V/C 1.9+), the difference between design capacity and operational capacity can be 30–40% on complex multilevel interchanges where auxiliary lanes, weave sections, and ramp metering affect throughput.

The paper also does not address directional capacity asymmetry. I-95 Baltimore-Washington carries very different peak-direction volumes northbound vs. southbound by time of day. A max-flow model using undirected or symmetrically bidirectional capacities would systematically overstate the available capacity in the non-peak direction and understate it in the peak direction.

The Donner Pass capacity value — "V/C 0.82" — implies the corridor is not yet at capacity. But Donner Pass operates at V/C 0.82 on average daily traffic; on peak summer days, Donner Pass approaches V/C 1.0 or higher. The max-flow model using average daily capacities will understate the severity of the Donner constraint on peak-demand days, which are exactly the days when the freight network is most stressed.

## The Question I'd Push On

What is the source and derivation methodology for the edge capacity values? Specifically: (a) what is the assumed peak-hour factor? (b) are capacity values from HPMS observed throughput or HCM design tables? (c) for urban interchanges (Dallas, Baltimore-Washington), are capacity values segment-specific or corridor-average? A capacity appendix showing the derivation for the three binding bottleneck arcs would resolve this concern.
