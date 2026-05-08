---
reviewer: David Neumark
persona: David Neumark — Distinguished Professor of Economics, UC Irvine; Director, Center for Economics & Public Policy; specialist in labor markets and program evaluation
round: 1
date: 2026-05-07
score: 2/4
---

> **Note:** This is an AI-generated simulated review, written by Claude in the voice of a named-expert persona. It is not an actual review by the named person and does not reflect their views or endorsement.

## Overall

The 24M annual passenger estimate is the paper's central demand claim and it is insufficiently supported. Gravity models are appropriate tools for intercity travel demand estimation, but their reliability depends entirely on the quality of the calibration data. The paper states that the gravity model is "calibrated against existing bus markets," but T1 bus is not an existing bus market — it is a new product category with substantially different speed, reliability, and network characteristics from current Greyhound/FlixBus service. The demand extrapolation from existing bus routes to T1 bus routes may overestimate by 3-5x if the value-of-time improvements are priced into the fare (which the $0.12-0.14/mile figure does not obviously do). Alternatively, if T1 bus attracts auto-mode travelers at the same rate that high-speed rail has attracted auto-mode travelers in comparable corridors internationally, the 24M figure may be an underestimate. The paper needs to show its calibration work rather than citing the result.

## What Works

The operator economics section ($2.80/bus-mile, break-even at $0.12-0.14/mile fare, 45% load factor) is the paper's most transparent quantitative section. These figures are consistent with current intercity bus operator cost structures (American Bus Association, NTD data for commuter bus where comparable). The PTI 1.15 threshold for reliable timetabling is a genuine operational insight — the paper correctly identifies that schedule reliability is the product attribute that separates T1 bus from current bus service in the eyes of time-sensitive travelers, and it quantifies the reliability threshold rather than asserting it.

The 5 corridors with no current service represent the strongest demand case in the paper. Atlanta-Dallas and Houston-Chicago pre-I-69 are corridors where pent-up demand is demonstrable: air travel on these city pairs is substantial, auto travel is the dominant mode, and bus service is either absent or uncompetitively slow. These are the corridors where a gravity model calibrated on existing bus routes will most severely understate T1 bus demand — and where the paper should show a separate mode-choice model rather than relying solely on gravity.

The Amtrak competitiveness finding (T1 bus faster on all corridors except Memphis-Chicago) is directionally credible and the Memphis-Chicago exception (Amtrak City of New Orleans) is correctly identified. This is a useful competitive positioning finding.

## What Doesn't Work

The gravity model calibration is not described in sufficient detail. To evaluate the 24M figure, reviewers need to know: (a) what the calibration corridor set is (which existing bus routes, what their current ridership, and over what time period); (b) what the gravity model impedance function is (negative exponential, power function, composite); (c) what the mode-specific constants are and how they are adjusted for the T1 bus product attributes (speed, reliability, frequency); (d) what the R² of the calibrated model is against the calibration data. Without this, 24M is a number without a denominator.

The 3-5x uncertainty range I assign to the 24M figure is not arbitrary. US intercity bus ridership is approximately 30-40M annually today (across all operators). A single new T1 bus network claiming 24M additional annual passengers would represent a 60-80% increase in total US intercity bus ridership. This is not impossible — European intercity bus markets expanded by similar magnitudes after deregulation (FlixBus Germany: 2013 baseline of near-zero to 40M annual passengers by 2019). But European deregulation was a structural market opening, not an infrastructure upgrade. The paper needs to engage with the European deregulation analog more carefully.

The fare assumption ($0.12-0.14/mile) produces a competitive price point ($48-56 for a 400-mile trip), but the paper does not model price elasticity. If the T1 bus fare is set at break-even for the operator (no subsidy), and if the primary demand pool is price-sensitive low-income travelers (consistent with the paper's equity framing), then demand is highly price-elastic and small fare increases (due to lower-than-expected load factors) could produce significant ridership declines. The model should include at least a simple price-sensitivity scenario.

## The Question I'd Push On

If the gravity model is calibrated on existing bus routes and applied to T1 bus corridors, the calibration implicitly assumes that T1 bus riders have the same modal utility function as existing bus riders — with travel time improvements scaled proportionally. But T1 bus may attract a meaningfully different traveler profile: price-insensitive business travelers who currently fly, or auto-dependent travelers who currently drive, attracted by the reliability improvement. If the T1 bus traveler profile is 30% current-bus, 30% current-auto, 40% induced new demand, the 24M figure is derived from a fundamentally different demand pool than the calibration data. Has the paper modeled this decomposition?
