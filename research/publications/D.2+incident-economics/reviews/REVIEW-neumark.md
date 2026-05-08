---
reviewer: David Neumark
persona: David Neumark — Distinguished Professor of Economics, UC Irvine; Director, Economic Self-Sufficiency Policy Research Institute; specialist in labor and regional economics
round: 1
date: 2026-05-07
score: 2/4
---

> **Note:** This is an AI-generated simulated review, written by Claude in the voice of a named-expert persona. It is not an actual review by the named person and does not reflect their views or endorsement.

## Overall

The paper presents a $6.2B annual freight cost estimate for the top-15 closure corridors as if it were a well-established finding. It is not. It is a point estimate produced by a model with multiple key parameters that are assumed, not estimated — and no sensitivity analysis is provided. For a paper targeting *Transportation Research Part E*, this is a serious methodological gap. I cannot recommend acceptance without major revision.

The conceptual framework is sound. The B1 penalty multiplier is novel and valuable. The redundancy value framing is useful. But the paper's credibility rests on its quantitative claims, and those claims are not adequately defended.

Score: 2/4 — reject and revise; structural issue with uncertainty quantification.

## What Works

The conceptual model — closure cost as the product of frequency, duration, volume, and the max(wait, reroute) operator — is the right economic framing. The insight that wait cost and reroute cost create a threshold decision, and that the closure cost is determined by whichever is lower, captures actual shipper behavior correctly.

The B1 multiplier is the paper's strongest original contribution. Formalizing the isolation penalty as (1 + detour_miles/100) creates a corridor-level property that explains why Donner ($2.4B/yr) costs 4.2× more than Dallas Interchange ($0.8B/yr) despite comparable truck volumes. This is publishable; the methodology is transferable to other corridor assessments.

The redundancy value concept ($1.9B/yr for Donner with I-70W alternate) provides a direct capital investment justification framework. It translates a cost model into an NPV argument for redundancy investment, which is the policy implication the paper reaches for.

## What Doesn't Work

The paper reports a $6.2B aggregate annual cost figure with no confidence interval. Given the model's structure, at least five parameters carry substantial uncertainty:

1. **ATRI unit cost ($225/hr)**: ATRI's own report gives this as an industry average, not a corridor-specific estimate. Urban vs. rural, dry van vs. refrigerated, long-haul vs. short-haul carriers have different operating costs. ±20% on this parameter changes the aggregate by ~$1.2B.
2. **Closure frequency**: Sourced from FHWA incident database, which the paper acknowledges undercounts rural incidents vs. urban (a limitation that systematically biases Donner and rural corridor estimates downward, meaning $2.4B may be conservative).
3. **Closure duration distribution (lognormal assumption)**: Not validated against empirical duration data. Lognormal is a convenient choice, not a demonstrated fit. If the tail is heavier (Pareto, log-Cauchy), the expected value of the integral is higher.
4. **Traffic volume during closure**: The model uses average daily volume; closures often happen during conditions (winter, storm) when volume is already reduced. Using average volume may overstate the traffic exposed to delay.
5. **Reroute cost assumption**: The B1 multiplier uses a linear detour-miles function. Actual reroute costs are nonlinear: the first 50 miles of detour uses spare capacity on alternate routes; beyond that, those routes become congested, and cost rises super-linearly. The linear function understates B1 for very isolated corridors.

None of these sensitivities is reported. A two-way sensitivity table — ATRI unit cost (±20%) × closure frequency (±30%) — showing the range of the aggregate estimate would take two paragraphs and substantially improve the paper's credibility. The authors should run it.

The no-build counterfactual is also absent. The paper presents closure costs as a snapshot, but the relevant policy question is: how are these costs trending? If freight volumes on I-80 Donner are growing at 2%/yr and climate change is increasing atmospheric river frequency, the expected annual cost by 2040 may be $4–5B, not $2.4B. Without a baseline trend, the paper cannot make the case that investment is more urgent than a 2026 cross-section implies.

## The Question I'd Push On

The model assumes that shippers choose between waiting and rerouting, and selects the min-cost option. But in practice, a significant fraction of closures — particularly disaster-level events (post-atmospheric river, multi-day winter closure) — do not offer a viable reroute at any reasonable cost. I-80 Donner's alternate (US-50, or I-15/I-70 for long-haul rerouting) adds 150–300 miles depending on origin-destination. For time-sensitive shipments (refrigerated, automotive JIT, hazmat), the wait option may be preferred despite cost, because the reroute physically cannot be completed within shipper constraints.

What fraction of Donner truck volume is time-constrained in a way that makes the reroute option infeasible rather than merely expensive? If 30% of Donner volume is refrigerated or JIT and cannot wait more than 24 hours, then for closures >24 hours the effective cost is not max(wait, reroute) — it is a contract penalty or cargo loss cost that the current model does not capture. If this fraction is material, the $2.4B estimate is a lower bound on a different basis than the FHWA undercount the paper already acknowledges.
