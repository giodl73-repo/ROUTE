---
reviewer: Alan McKinnon
persona: Alan McKinnon, Professor of Logistics, Kühne Logistics University, Hamburg; Emeritus Professor, Heriot-Watt University
round: 1
date: 2026-05-07
score: 3/4
---

> **Note:** This is an AI-generated simulated review, written by Claude in the voice of a named-expert persona. It is not an actual review by the named person and does not reflect their views or endorsement.

## Overall

The use of FAF5 demand data as the basis for O-D cluster demands is the right choice for a national freight network model, and the max-flow framework is well-suited to identifying binding capacity constraints. My concerns are about the single-commodity formulation from an economic standpoint — not just a modeling standpoint — and about whether the max-flow → investment recommendation causal chain is as robust as the paper implies. Score: 3/4.

## What Works

FAF5 (Freight Analysis Framework, version 5) is the appropriate national freight demand dataset for a US interstate network study. It provides commodity-specific O-D flows by mode at the FAF zone level, and the FAF5 zone-to-FAF5 cluster aggregation to 8 O-D clusters is a reasonable simplification for a national-scale model. The paper should state which FAF5 commodity codes are included and what baseline year is used — but the choice of FAF5 as the demand source is correct.

The bottleneck identification results are internally consistent with established freight research. The Dallas interchange appearing in both the max-flow model and the ATRI Top Truck Bottleneck report provides cross-validation. The I-35 Oklahoma closure dropping Gulf-Midwest max-flow by 31% is a striking result that motivates the I-69 investment case.

The compound failure scenario (Donner + I-35 simultaneous → I-40 V/C 1.11) is the paper's most important resilience finding. It quantifies the fragility of the network under correlated disruptions — the kind of compound event that extreme weather increasingly produces.

## What Doesn't Work

Single-commodity max-flow treats all freight as having equal value, equal time-sensitivity, and equal path preferences. In FAF5, commodity 7 (gasoline and aviation turbine fuel) and commodity 13 (electronics) have very different economic characteristics. A max-flow that finds an optimal routing for undifferentiated freight may route high-value electronics through low-cost paths while bulk petroleum stays on direct routes — or vice versa. The binding constraint in a real multi-commodity flow (where electronics and petroleum compete for the same arc capacity) may differ from the binding constraint in the single-commodity model.

The paper's investment recommendations — I-70W (+4.7% NE-Pacific flow), I-69 completion (+18% Gulf-Chicago), Donner alternatives — are stated as conclusions of the max-flow analysis. But max-flow optimization identifies where additional capacity would increase total flow; it does not identify where additional capacity generates the most economic value. A bottleneck arc that constrains the flow of $10B of high-value electronics should receive higher investment priority than one that constrains an equal volume of lower-value bulk commodity — but the max-flow model cannot make this distinction.

The I-69 NPV is noted as marginally negative at 7% discount rate, yet the paper includes I-69 completion in its recommendations. This creates a tension: the max-flow analysis says I-69 improves flow (+18%), but the NPV analysis says the investment is not economically justified at standard discount rates. The paper should engage with this tension directly rather than presenting both results without reconciliation.

## The Question I'd Push On

For each of the three investment recommendations (I-70W, I-69, Donner hardening), what is the commodity composition of the incremental flow gain — and does the economic value of that incremental flow justify the investment at a standard social discount rate? The paper presents max-flow gains in vehicle-equivalents, but the investment case requires freight-ton-miles weighted by commodity value. FAF5 provides the commodity-level O-D data needed to make this calculation; the paper should use it.
