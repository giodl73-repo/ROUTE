---
reviewer: Alan McKinnon
persona: Alan McKinnon, Professor of Logistics, Kühne Logistics University, Hamburg; Emeritus Professor, Heriot-Watt University
round: 1
date: 2026-05-07
score: 3/4
---

> **Note:** This is an AI-generated simulated review, written by Claude in the voice of a named-expert persona. It is not an actual review by the named person and does not reflect their views or endorsement.

## Overall

The NPV analysis supporting the diamond interchange zone proposal is credibly structured and the B2_product priority metric is a reasonable proxy for junction economic criticality. My concern is that the NPV model does not make its freight volume and value-of-time assumptions fully transparent, and the B2_product metric — while intuitively appealing — is not benchmarked against a revealed-preference measure of junction economic importance such as FAF5 flow data. The paper makes a good case for the portfolio concept; it needs to work harder to justify the individual site rankings. Score: 3/4.

## What Works

The B2_product metric (betweenness centrality product of intersecting corridors) is well-motivated as a screening tool. For a national portfolio prioritization, a graph-theoretic metric is more tractable than individual traffic counts and captures the network-level impact of a junction failure that a local V/C analysis would miss. The three priority sites — Atlanta, Jacksonville, Toledo — are plausible choices on economic grounds independent of the metric: these are major freight interchange nodes with high daily throughput and significant VMT on both intersecting corridors.

The portfolio framing ($4.5B investment, $12.4B NPV, 2.76:1 B/C) is the right analytical unit. Evaluating each intersection independently would understate the case for the overall program. The phasing logic — prioritize by B2_product × (2 - k), giving highest urgency to high-betweenness k=1 sites — is sensible.

## What Doesn't Work

The NPV model uses betweenness centrality as a proxy for freight economic value, but betweenness centrality in an unweighted graph is not the same as freight-ton-miles or freight value-at-risk. A junction can have high betweenness in the topological graph while carrying relatively low freight value if the intersecting corridors are high-mileage but low-intensity (e.g., rural Western corridors). Conversely, a junction with moderate betweenness but very high freight value concentration (perishables, high-value manufactured goods) would be underweighted.

The paper should cross-reference the B2_product rankings against FAF5 flow data at the junction level. For Atlanta I-75/I-85, FAF5 data exists at sufficient geographic resolution to validate that the junction carries the freight value implied by the NPV model. Without this validation, the $2.8B NPV for Atlanta is an estimate built on an unvalidated proxy.

The value-of-time assumption for freight is not stated. Standard FHWA guidance (2023 USDOT BCA Guidance) recommends $35–$42/hour for truck freight. The reliability premium — the marginal shipper benefit of reducing junction failure probability — is particularly sensitive to this assumption. A 10% change in VoT shifts the NPV by approximately 15–18% in typical freight reliability models.

## The Question I'd Push On

What is the annual probability of a full junction closure event at a k=1 T1/T1 intersection — from incident, maintenance, or extreme weather — and how was that probability estimated for the NPV calculation? The entire NPV model rests on the product of (closure probability × freight diversion cost × duration). If the paper assumes, say, one full closure per decade at 72 hours average duration, that should be stated explicitly and compared to FHWA incident data for high-volume interchange nodes. The reliability benefit — the dominant term in the NPV — needs an empirical anchor, not just a betweenness-based approximation.
