---
reviewer: Robert Puentes
persona: Robert Puentes, President and CEO, Eno Center for Transportation; former Fellow, Brookings Institution Metropolitan Policy Program
round: 1
date: 2026-05-07
score: 3/4
---

> **Note:** This is an AI-generated simulated review, written by Claude in the voice of a named-expert persona. It is not an actual review by the named person and does not reflect their views or endorsement.

## Overall

This paper presents a technically sophisticated analysis of national freight network capacity. My concern is primarily about the policy implications of the I-69 recommendation: the max-flow analysis shows a meaningful flow gain, but the NPV is marginally negative at 7% discount rate, and the paper's recommendation to proceed is not adequately grounded in either economic theory or implementation reality. A marginally negative NPV at 7% and a positive NPV at 5% is a close call — the right response is to characterize this as a policy judgment, not a max-flow finding. Score: 3/4.

## What Works

The national graph construction is the paper's core contribution, and it is a substantial methodological achievement. Building a 227-corridor, 48,000-edge directed graph with HPMS capacity overlays and FAF5 demand data creates an asset that can support a range of future analyses beyond max-flow — minimum cut analysis, vulnerability ranking, scenario simulation. The Edmonds-Karp implementation at this scale is computationally non-trivial and deserves recognition.

The closure simulation results provide actionable intelligence for emergency planning. The Donner + I-35 compound failure producing I-40 overcapacity (V/C 1.11) is exactly the kind of result that FHWA's Emergency Relief program and FEMA's Transportation Emergency Response planning need. Quantifying the compound failure effect is a genuine contribution to resilience policy.

The I-70W result (+4.7% NE→Pacific at baseline; limits Donner closure impact from 23% to 9%) is well-specified and internally consistent. Even if the investment case for I-70W in normal conditions is modest, the Donner closure scenario makes a resilience argument for I-70W that is independent of baseline flow gains.

## What Doesn't Work

The I-69 NPV problem requires direct engagement. The paper states the NPV is marginally negative at 7% discount rate. At 5% discount rate (consistent with the Biden-era USDOT BCA guidance for surface transportation), a marginally-negative-at-7% result would likely be positive. The paper should compute the NPV at both 5% and 7% discount rates (and ideally 3%, consistent with OMB Circular A-4 for intergenerational projects) and present the breakeven discount rate. A project that is NPV-positive at 5% and NPV-negative at 7% is a legitimate policy debate — the recommendation should acknowledge the discount rate sensitivity rather than presenting a single-rate result.

The paper also does not engage with the distributional question for I-69. The primary beneficiaries of I-69 completion are Gulf Coast export shippers (petrochemicals, agricultural products) and upper Midwest manufacturers (auto parts, machinery). These are concentrated industrial interests. If the NPV is marginal in the aggregate, the distributional question — who captures the benefit, who bears the construction disruption and land acquisition impact — becomes central to the policy case. The paper does not address this.

The IIJA funding pathway for I-69 completion is more complex than the paper acknowledges. I-69 qualifies for NHFP, INFRA, and RAISE grants — but with 400 miles of unconstructed corridor and multiple state EIS processes at different stages, the federal programming challenge is not primarily about funding availability. It is about coordinating five or six state DOTs through NEPA processes that are largely independent of each other. Federal can fund but cannot accelerate state NEPA.

## The Question I'd Push On

At what social discount rate does I-69's NPV turn positive, and what is the sensitivity of that breakeven rate to the freight demand growth assumption? If freight demand grows at 1.5%/year (low scenario) vs. 2.5%/year (high scenario), the breakeven discount rate shifts materially. A demand-scenario × discount-rate sensitivity table for the I-69 NPV would allow policymakers to make an informed judgment about the investment rather than confronting a single "marginally negative" conclusion.
