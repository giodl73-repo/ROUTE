---
reviewer: David Neumark
persona: David Neumark — Distinguished Professor of Economics, UC Irvine; Chancellor's Fellow; labor economics, market structure, firm behavior
round: 1
date: 2026-05-08
score: 2/4
---

> **Note:** AI-generated simulated review.

## Overall

The paper identifies a coordination failure in relay freight as the correct economic diagnosis and builds a platform design proposal around it. The intuition is sound and the practical design work is more detailed than most coordination-failure papers bother to produce. However, *Management Science* is an economics and operations research journal, and this paper's economic mechanism is underspecified in ways that will prevent acceptance. The paper cites Rochet and Tirole (2003) as if citing a two-sided market paper is sufficient to establish that the relay marketplace is two-sided; it is not. It cites Williamson (1979) for transaction cost economics but does not apply the framework. And its central quantitative claim — that relay is cheaper than solo — collapses when hub fees are included. These are not presentational gaps; they are analytical gaps that require substantive revision.

## What Works

The coordination failure diagnosis is correct as far as it goes. The four barriers the paper identifies — regulatory uncertainty, first-mover disadvantage, union resistance, industry fragmentation — are real and each maps onto a recognized coordination failure type (uncertain property rights, network externality, collective action, market power respectively). This taxonomy is a contribution, even if the paper does not develop it rigorously.

The asset utilization argument is the paper's strongest quantitative contribution and does not depend on the contested hub-fee omission. Solo truck utilization of 48% vs. relay utilization of 98% is a real efficiency gap, and at $180,000/truck, the fleet capital implications are significant. The paper should lead with this argument, not bury it. A fleet running at 98% utilization needs roughly half as many trucks to move the same volume — at $90,000/truck in avoided fleet cost — as a fleet at 48% utilization. This is a durable competitive advantage that does not depend on any price comparison.

The hub business case (Atlanta, 750 docks) is directionally correct. The numbers are order-of-magnitude plausible, though the queueing analysis is absent (see below).

## What Doesn't Work

**The hub fee omission destroys the central cost comparison.** The paper's headline claim is "relay is cheaper than solo: $1,050 vs. $1,456/trip." But a 2,800-mile NY-LA trip requires approximately 6 relay swaps (at ~470-mile relay segments). At the paper's own stated swap fee of $75–150 (midpoint $112), hub fees total $675/trip. Adding hub fees: relay total = $1,050 + $675 = $1,725 vs. solo $1,456. Relay is now more expensive per trip. The paper's central quantitative claim is false on its own numbers once hub fees are included. The resolution — which the paper needs to adopt explicitly — is that the correct comparison is not per-trip but per-ton-mile, accounting for: (a) the relay trip moves the same freight faster (better SLA, higher revenue); (b) asset utilization advantage reduces fleet capital cost; (c) driver shortage premium for long-haul solo drivers is currently $5,000–$15,000 in signing bonuses (ATRI 2024) vs. zero for regional relay drivers. On a fully-loaded per-ton-mile basis, relay is likely still competitive or superior — but the paper must demonstrate this explicitly rather than relying on a comparison that excludes the marketplace's own revenue source.

**Two-sided market framing is not rigorously applied.** The paper cites Rochet and Tirole (2003) to frame the relay marketplace as a two-sided platform but does not apply the framework. In a canonical two-sided market, two distinct groups of agents are brought together by a platform, and the platform's pricing structure captures cross-side network externalities. In the relay marketplace, carriers are on both sides — as providers of trucks arriving at hubs and as demanders of relay drivers. This is a one-sided matching market with platform intermediation, not a two-sided market in the Rochet-Tirole sense. The distinction matters for pricing strategy: a two-sided market requires getting the relative price between the two sides right (the Rochet-Tirole "price structure" insight); a one-sided matching market with platform intermediation is better modeled as a matching market (Roth, Sotomayor 1990) or a mechanism design problem. The paper should either (a) correctly characterize the market structure and apply the appropriate model, or (b) drop the Rochet-Tirole citation and describe the relay marketplace as a neutral matching platform without claiming two-sided market properties it doesn't have.

**Transaction cost framework is invoked but not applied.** Williamson (1979) is cited for transaction cost economics, but the paper does not apply the framework. Transaction cost economics would predict: given the specificity of relay driver assets (CDL, drug test currency, HOS reset status, familiarity with hub protocol), make-or-buy analysis favors vertical integration (hub operator employing W-2 drivers) over market contracting (IC gig drivers) when asset specificity is high and uncertainty is high. This prediction exactly matches the paper's Mode 1 vs. Mode 2 recommendation — but the paper arrives at it by assertion rather than by applying the framework it cites. This is an easy revision: apply Williamson's asset-specificity analysis formally to the Mode 1 vs. Mode 2 choice.

## The Question I'd Push On

Is the relay marketplace actually a coordination failure, or is it a market design failure? A coordination failure implies that all parties would be better off if the platform existed and no one is building it due to a coordination problem (Nash equilibrium trap). A market design failure implies that the market structure itself makes it impossible for a private actor to capture sufficient surplus to justify building the platform (hold-up problem, incomplete contracts). These have different policy prescriptions: coordination failure calls for government signaling/mandate; market design failure calls for government as platform (public neutral infrastructure). Which is it? The paper currently argues for government as coordinator (NFRZ designation, FMCSA rulemaking) without specifying why a private platform could not emerge organically once the regulatory barrier is cleared.
