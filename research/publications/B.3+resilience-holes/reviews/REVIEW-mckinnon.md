---
reviewer: Alan McKinnon
persona: Alan McKinnon — Professor of Logistics, Kühne Logistics University, Hamburg; specialist in freight transport economics, logistics efficiency, and supply chain resilience
round: 1
date: 2026-05-07
score: 2/4
---

> **Note:** This is an AI-generated simulated review, written by Claude in the voice of a named-expert persona. It is not an actual review by the named person and does not reflect their views or endorsement.

## Overall

The paper introduces a conceptually sound and practically important framework for identifying compound resilience failures in highway infrastructure. The B1×D1 product metric is a useful first-order prioritization tool and the investment case logic is well-structured. My concern is with the closure cost estimates — specifically the $1.6B/year Donner figure — which drive the entire NPV calculation. These estimates rest on several assumptions that are not adequately justified: the truck count, the rerouting behavior fraction, the delay cost per truck, and the event frequency. Any one of these could be off by 50%; compounded together, the uncertainty range around $1.6B/year could easily span $0.5B to $3.5B/year. A paper making a $15.8B NPV claim needs to show that the NPV sign (positive) is robust to this uncertainty range, not just that the midpoint estimate produces a high CBR. Score: 2/4. The conceptual framework merits publication; the quantitative case needs substantial revision.

## What Works

The compound exposure concept is clearly defined and the case for its non-additive character is well-made. The three-way comparison (high B1/low D1, low B1/high D1, high B1/high D1) is instructive and the Donner case is a good anchor. The paper correctly identifies that compound exposure changes the investment logic — that a project fixing both dimensions simultaneously dominates either single-dimension fix — and the NPV arithmetic in Table 3 is correct given the inputs.

The PROTECT program reform recommendation is the paper's most policy-useful contribution. The observation that current PROTECT grant criteria are effectively D1-only (SFHA miles, closure frequency) without B1 network topology criteria is accurate — I have reviewed PROTECT grant applications and this omission is a genuine gap. The recommendation to add compound score weighting is specific, actionable, and does not require statutory change. This section should be published regardless of what happens to the quantitative case.

The identification of 11 compound exposure corridors using the B1 > 7 AND D1 > 6 thresholds is a useful national-scale catalog. Table 2 is the paper's most durable empirical contribution — a list of corridors that practitioners and planners can immediately use for resilience planning.

## What Doesn't Work

**The $1.6B/year Donner cost estimate is not adequately justified.** The paper derives this figure from: 8,000 trucks/day × 50 closures/year × 18 hours mean duration × $225/hr × behavioral fractions (0.4 rerouting, 0.6 waiting). Each of these inputs requires justification.

*8,000 trucks/day*: The paper cites ATRI 2024 for this figure, but the ATRI freight performance data for I-80 at Truckee gives Annual Average Daily Truck Traffic (AADTT) for the full corridor — including summer months when closures are rare. The relevant metric is AADTT during the winter closure season (November–April), which is lower due to seasonal freight patterns. If winter AADTT is 6,000 (25% lower), the annual cost estimate drops to $1.2B.

*50 closures/year, 18 hours mean duration*: The Caltrans 2023 operational log data is the right source. But the paper should report the distribution of closure durations, not just the mean. If the distribution is right-skewed (most closures 4-6 hours; occasional 48-72 hour events that dominate the mean), the expected cost calculation should use the full distribution, not the mean duration. A few long closures may drive most of the cost.

*$225/hr truck operating cost*: This ATRI figure is the fully-loaded operating cost including driver time, fuel, maintenance, and equipment. For a rerouting truck adding 5.5 hours of driving, this is appropriate. For a waiting truck, the relevant cost is the idle cost (fuel at idle, driver time) which is substantially lower — perhaps $80-100/hr. The paper applies $225/hr to both rerouting (correct) and waiting (overcounts waiting cost by ~2.3×).

*Behavioral fraction (0.4 rerouting, 0.6 waiting)*: This split is not cited. The actual split depends on load type, delivery commitment, and individual shipper decision-making. For just-in-time loads, the rerouting fraction is higher (sunk cost of waiting exceeds rerouting cost); for bulk commodities, waiting may be preferred. A citation or sensitivity analysis is needed.

**The NPV analysis lacks a sensitivity table.** Given the acknowledged uncertainty in the cost inputs, the paper should present a sensitivity table showing NPV as a function of ±50% variation in each key input (truck count, closure frequency, delay cost, behavioral fraction). If the NPV remains strongly positive across all ±50% scenarios, the investment case is robust. If it turns negative under plausible downside inputs, the case requires more careful argument. A 30-year NPV of $15.8B looks impressive, but if the ±50% sensitivity range spans $4B to $27B, the sign is robust but the magnitude is not.

**The Gulf Coast $0.82B/year estimate is derived less carefully than the Donner estimate.** Section 4.2 states "combined annual benefit: approximately $820M/year" for Gulf Coast I-10 without showing the calculation. The Donner calculation is shown step by step; the Gulf Coast calculation is asserted. For a paper whose primary quantitative contribution is the NPV comparison across corridors, all four portfolio corridors should have their cost estimates shown with equal transparency.

## The Question I'd Push On

The paper's NPV calculation assumes that closure cost savings accrue uniformly across the 30-year project life. But the behavioral and economic context for freight shipment decisions will change: electric trucks with smaller fuel cost disadvantages from detours, autonomous vehicle platooning that reduces driver cost per hour, and logistics network redesign in response to known closure risk. If shippers have already adapted their logistics to the Donner closure pattern — routing supply chains to avoid I-80 Sierra crossing during high-risk periods, maintaining buffer inventory to absorb 18-hour delays — then the marginal benefit of eliminating the closure is lower than the current cost estimate suggests. Has the paper considered the induced adaptation response, and if so, how does it affect the closure cost baseline?
