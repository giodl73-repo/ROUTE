---
reviewer: Alan McKinnon
persona: Alan McKinnon — Professor of Logistics, Kühne Logistics University, Hamburg; lead author, ITF freight decarbonization reports
round: 1
date: 2026-05-07
score: 3/4
---

> **Note:** This is an AI-generated simulated review, written by Claude in the voice of a named-expert persona. It is not an actual review by the named person and does not reflect their views or endorsement.

## Overall

The paper makes a serious attempt to quantify the annual freight cost of interstate corridor closures using a structured expected-cost model, and the Donner finding — that corridor isolation amplifies closure costs dramatically relative to urban interchanges — is the paper's most important result. The B1 penalty multiplier is an elegant and defensible design choice. My main concern is with the ATRI unit cost ($225/hr per truck) and the degree to which the $6.2B annual aggregate estimate rests on that single parameter without sensitivity analysis.

Score: 3/4 — publishable with revision; the ATRI unit cost justification and uncertainty quantification are the key outstanding issues.

## What Works

The expected-cost formula E[cost] = frequency × ∫ duration × volume × max(wait_cost, reroute_cost) × f(d) dd is well-structured. The max() term — taking the higher of wait cost and reroute cost — correctly captures the shipper's decision logic: a shipper routes around the closure only if the detour cost is lower than the wait cost, and the corridor's closure cost is the minimum of these options. This is the right economic framing.

The B1 penalty multiplier (1 + detour_miles/100) is the paper's most original contribution. It formalizes the operational reality that isolated corridors are not just costly to serve during closures — they are categorically more exposed because the wait option is typically worse (no parallel route) and the reroute option is disproportionately long. Donner's B1=8.3 vs. Dallas Interchange's B1=5.9 explains the cost ratio of 4.2× better than any volume-based framing.

The redundancy value concept — Donner's cost drops from $2.4B to $0.5B with an I-70W alternate, implying $1.9B/yr redundancy value — is both intuitive and policy-relevant. It provides a direct NPV foundation for evaluating the capital cost of a Donner alternate route investment.

## What Doesn't Work

The ATRI Operational Costs of Trucking report gives $225/hr as the 2022 carrier operating cost per truck. This is the right data source, but two questions remain unanswered:

First, is $225/hr the marginal cost or the total cost? Carrier fixed costs (lease, insurance, driver salary components) continue during a closure whether or not the truck is moving. The economically relevant figure for closure cost is the marginal cost of delay — the out-of-pocket cost that increases per hour of stoppage. If $225/hr is a fully-loaded average cost including fixed components, the model overstates the per-hour burden of delay (vs. the alternative of diverting the driver to another route). The ATRI report distinguishes between fuel cost, driver pay, and fixed components; the paper should specify which are included.

Second, is the 2022 ATRI rate appropriate for the analysis period? Trucking costs have been highly volatile (COVID-era rate spikes 2020–2022, normalization in 2023–2025). A rate that is ±20% of $225/hr changes the top-line $6.2B estimate by roughly $1.2B. This should be a named sensitivity variable, not a fixed parameter.

The lognormal assumption for closure duration distribution (Neumark's concern, with which I concur) also affects the expected-cost integral. If the duration distribution has heavier tails than lognormal — as disaster-induced closures often do (post-Ida, post-atmospheric river) — the expected value of the integral is understated.

## The Question I'd Push On

The paper presents Donner closure cost at $2.4B/yr as if this is a steady-state annual cost. But Donner closure frequency is highly variable: in atmospheric river years (2016–17, 2022–23), closure days are 3–5× the long-run average; in dry La Niña years, closures may be minimal. What is the interquartile range of the annual Donner closure cost estimate, and how does it compare to the point estimate of $2.4B?

If the distribution is something like P25=$0.6B, P50=$1.5B, P75=$4.2B, then "Donner $2.4B/yr" is not the median — it may be above the median and is clearly not a steady-state estimate. For a *Transportation Research Part E* paper, the expected value is the right central estimate, but the variance matters for policy: a corridor whose closure cost is $1.5B in a typical year but $8B in an atmospheric river year has a very different risk profile from one with a stable $1.5B.
