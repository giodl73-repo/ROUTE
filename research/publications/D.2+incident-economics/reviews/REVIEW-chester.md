---
reviewer: Mikhail Chester
persona: Mikhail Chester — Professor of Civil, Environmental, and Sustainable Engineering, Arizona State University; Director, Metis Center for Infrastructure and Sustainable Engineering
round: 1
date: 2026-05-07
score: 3/4
---

> **Note:** This is an AI-generated simulated review, written by Claude in the voice of a named-expert persona. It is not an actual review by the named person and does not reflect their views or endorsement.

## Overall

The paper's closure cost model is a useful advance over the fragmented estimates in the existing literature, and the B1 penalty multiplier for corridor isolation is the paper's most original methodological contribution. My focus is on whether the model captures the full range of failure modes it claims to address — specifically, whether it handles cascading failures across adjacent corridors and compound exposure events (simultaneous closures triggered by the same climate event). These are not marginal refinements; they affect whether the $6.2B estimate is a lower or upper bound.

Score: 3/4 — publishable with revision after addressing the cascading failure gap and clarifying the relationship between this model and the D.1 climate exposure results.

## What Works

The expected-cost model is theoretically sound and appropriately specified for a first-order analysis. The max(wait_cost, reroute_cost) operator correctly models shipper decision logic under closure. The lognormal duration distribution — while contested by Neumark — is at least a named assumption that can be tested against empirical data in revision.

The B1 penalty multiplier is the paper's most valuable methodological contribution to the literature. No existing closure cost model that I am aware of explicitly formalizes the isolation penalty as a corridor-level property. The result — that Donner costs 4.2× more per closure event than Dallas despite lower peak V/C — is a concrete, falsifiable finding that should be prominent in the abstract.

The redundancy value calculation ($1.9B/yr for Donner with I-70W) is well-executed and provides the most direct policy lever the paper offers. If a proposed redundancy route can be built for $X billion with a design life of Y years, the present value of the avoided closure cost provides a minimum-viable NPV floor. The paper should make this arithmetic explicit.

## What Doesn't Work

The model treats each closure event as independent, but the most consequential closure events are not independent. An atmospheric river that closes I-80 Donner also typically closes CA-50 (the primary alternate) and may disrupt I-5 (the coastal bypass). When the event causing the closure also degrades the reroute options, the B1 multiplier — which assumes the reroute is available at its baseline cost — understates the true closure cost.

The paper should at minimum acknowledge that the B1 multiplier is calibrated for single-corridor closures with functioning alternates. For compound exposure events (which are precisely the high-cost tail events that drive the expected value calculation), B1 underestimates the isolation penalty. A simple sensitivity analysis — "if reroute capacity is degraded to 60% during the closure event, what is the effective B1?" — would quantify this gap.

The connection to D.1 (climate exposure scores) is noted but not developed. The paper is part of a research program that has already scored corridors on climate exposure (D1 dimension). A reader of both papers will immediately want to know: is the correlation between D1 and annual closure cost the expected positive correlation? Does I-10 Gulf Coast LA, which scores highest on D1 (8.4), also produce the largest projected closure cost increase under 2050 climate scenarios? This natural cross-paper finding is not reported.

The FHWA incident database undercount issue (acknowledged in the paper's limitations) is more serious than the paper treats it. If rural incident undercounting is systematic — which the literature suggests it is, because rural incidents involve smaller agencies with lower reporting compliance — then the Donner and other rural corridor estimates are lower bounds. The paper should estimate the direction and approximate magnitude of this bias using the available literature on FHWA reporting completeness rather than simply flagging it as a limitation.

## The Question I'd Push On

The paper's redundancy value calculation assumes that an I-70W alternate (or equivalent) would reduce Donner closure cost from $2.4B to $0.5B. This $0.5B residual is described as the cost attributable to closures that would affect both I-80 Donner and the alternate simultaneously — compound closure events.

My question: what fraction of current Donner closure events would be eliminated by an I-70W alternate, and what fraction are compound events (atmospheric rivers that close both routes)? If atmospheric river events account for 60% of Donner's closure cost but also close any Sierra alternate, then the redundancy value is much lower than $1.9B — perhaps $0.8–0.9B. This is not a small difference; it changes the NPV case for the redundancy investment substantially. Has the paper modeled the historical correlation between Donner closures and alternate route availability?
