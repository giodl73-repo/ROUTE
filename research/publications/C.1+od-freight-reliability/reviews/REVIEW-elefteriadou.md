---
reviewer: Lily Elefteriadou
persona: Lily Elefteriadou, Director, McTrans Center; Professor of Civil Engineering, University of Florida
round: 1
date: 2026-05-07
score: 2/4
---

> **Note:** This is an AI-generated simulated review, written by Claude in the voice of a named-expert persona. It is not an actual review by the named person and does not reflect their views or endorsement.

## Overall

The paper's core PTI calculation relies on a BPR-based estimate applied at V/C ratios where the BPR function is not calibrated. This is not a minor methodological caveat — it is a fundamental validity problem for the paper's most important claim. The Bay Area segment operates at V/C 1.86, and the BPR function was empirically calibrated on facilities with V/C < 1.0; its behavior above 1.0 is an extrapolation with no empirical basis. The PTI estimate of 1.86 at the Bay Area segment may significantly understate or overstate actual conditions, and either error produces an unreliable shipper commitment window calculation. I cannot recommend this paper for publication in its current form. Score: 2/4.

## What Works

The corridor selection and binding constraint identification are well-executed. Donner Pass as a geographic chokepoint and the Bay Area urban network as a capacity chokepoint are both correctly identified and supported by HPMS data. The I-69 completion analysis is the strongest section of the paper: the 290-mile shortening and the elimination of the Dallas and St. Louis interchange nodes from the routing are clearly motivated, and the HOU-CHI transit time reduction from 2.1 days to 1.5 days is a credible estimate given the shorter distance and avoidance of the V/C 1.9+ Dallas bottleneck.

The SLA framework (PTI × mean trip time = 95th-percentile shipper commitment window) is a useful translation device. The target PTI of 1.15 for managed lanes is consistent with published performance targets for value-pricing managed lane facilities in peer literature.

## What Doesn't Work

The BPR function (V/C delay formula) was calibrated on facilities operating below capacity. The standard BPR form — t = t₀ × (1 + α(V/C)^β), with α=0.15 and β=4 — has been shown in the literature (e.g., Spiess 1990, Branston 1976) to underpredict delay at V/C > 0.9 and to produce indeterminate estimates well above V/C = 1.0. At V/C 1.86, the BPR function is operating nearly two capacity-units above its calibrated range. This is not a small extrapolation: the delay exponent term (V/C)^4 at V/C = 1.86 equals approximately 12.0 versus 1.0 at design capacity — a 12x amplification of the base delay factor.

The paper derives PTI = 1 + 0.15 × (V/C × 1.15)^4 — a modified BPR with a PTI-specific scaling factor. This modification is not cited to any peer-reviewed source, and the basis for the 1.15 scaling term is not explained. If this is a calibration adjustment, the paper must state what data it was calibrated on. If it is an assumption, the paper must state that explicitly and provide a sensitivity analysis.

The correct tool for PTI estimation at high V/C conditions is either: (a) direct measurement from NPMRDS travel time data, which provides 5-minute interval speeds and enables direct PTI computation from percentile speed distributions; or (b) a queuing-theory model (e.g., D/D/1 or M/D/1) which correctly handles oversaturated conditions. The paper should use NPMRDS data if available for the Bay Area and Donner Pass segments; this data exists through FHWA's Freight Performance Measures program and would directly validate or replace the BPR-based estimate.

## The Question I'd Push On

What is the PTI for the Bay Area I-80 segment based on actual NPMRDS travel time data, and how does it compare to the BPR-derived estimate of 1.86? NPMRDS provides 5-minute interval travel time data going back to 2016, which is more than sufficient to compute a robust 95th-percentile PTI. If NPMRDS PTI for the Bay Area I-80 segment is, say, 2.4 or 1.5, the entire reliability cost calculation and SLA window claim need to be revised. The BPR estimate should be a back-of-envelope check at best; direct measurement should be the primary method.
