---
reviewer: Jarrett Walker
persona: Jarrett Walker — Transit network consultant; author, "Human Transit: How Clearer Thinking about Public Transit Can Enrich Our Cities and Our Lives" (2011); founder, Jarrett Walker + Associates
round: 1
date: 2026-05-07
score: 2/4
---

> **Note:** This is an AI-generated simulated review, written by Claude in the voice of a named-expert persona. It is not an actual review by the named person and does not reflect their views or endorsement.

## Overall

The paper claims 62 mph average bus speed across 12 T1 corridors and uses this figure to generate all the travel-time and competitiveness findings. This number is wrong, or at least undefended, and it is load-bearing for every downstream result in the paper. The problem is the stop penalty. Class T1 bus service on T1 corridors is not an express coach with 2-3 stops. It is a network service connecting 50+ T1/T2 hub stops across the corridor. Every intermediate stop imposes an acceleration/deceleration penalty: at highway speeds, a stop event (decelerate from 65 mph, dwell 3-5 minutes, accelerate back to 65 mph) costs approximately 4-6 minutes including recovery time. Across a 500-mile corridor with 8-10 intermediate stops, that is 32-60 minutes of stop penalty — more than the paper's 3 mph allowance from a hypothetical 65 mph baseline would account for. At 62 mph average with 8 stops, the implied dwell-plus-acceleration loss is implausibly small. The paper needs either: (a) a stop penalty model that justifies 62 mph under realistic intermediate stop assumptions, or (b) a revised speed figure that is operationally achievable, with the downstream travel time and competitiveness findings adjusted accordingly.

## What Works

The PTI 1.15 threshold finding is the paper's strongest operational contribution. Identifying that schedule-based timetabling becomes viable when the Planning Time Index drops below 1.15 — and demonstrating that I2.0 managed lanes achieve PTI 1.15 on the modeled corridors — provides a clear reliability standard that bus operators can use in service design. This is a clean, useful finding that the paper presents well. Connecting PTI to operator economics (timetable scheduling enables higher load factors, reduces recovery time buffer requirements) is the right causal chain and it is argued clearly.

The 5 corridors with no current service (Atlanta-Dallas, Houston-Chicago, Chicago-Minneapolis, Denver-SLC, Gulf Coast) represent the paper's most important network contribution. These are genuine market gaps that the T1 bus network could address — particularly Atlanta-Dallas and Houston-Chicago, which would be among the highest-demand new corridors in the country if reliable time-competitive service existed. The paper correctly identifies that these corridors have no current viable bus option because of travel time competitiveness, not insufficient demand.

The equity findings on "in-between" communities (Coalinga, Effingham, Durant) with zero-vehicle household rates of 15-28% are compelling and well-sourced. This is the paper's strongest equity contribution — it identifies communities that are currently stranded between metro endpoints and would gain access through T1/T2 intermediate stops.

## What Doesn't Work

The 62 mph average speed claim is not supported by a stop penalty model. The paper appears to assume that "managed lane speed" (65 mph) minus a 3 mph allowance yields 62 mph average. But average speed is not (top speed − small discount); it is total distance divided by total time including all stops. For a corridor like Chicago-Minneapolis (410 miles, probable intermediate stops in Milwaukee, Madison, Eau Claire, Rochester, and others), the stop penalty at 5 minutes per stop across 5 intermediate points is 25 minutes, reducing the effective average speed by approximately 5-7 mph below the managed-lane cruising speed — not 3 mph. The paper needs a stop schedule model, not a discount factor.

The 24M annual passenger estimate requires stronger calibration against existing bus markets. The gravity model used should be described in enough detail that reviewers can assess the calibration data. If the gravity model is calibrated on existing Greyhound/FlixBus/Megabus routes, the T1 bus product — higher frequency, managed lane speed, reliable timetable — is a genuinely different product from current bus, and the demand extrapolation may overestimate willingness-to-pay for the reliability premium among the target market (primarily low-income travelers for whom price is more salient than travel time). This is the Neumark concern; I share it from a service design perspective.

The 45% load factor assumption for operator economics ($2.80/bus-mile, break-even at $0.12-0.14/mile fare) is not compared to actual load factors on existing intercity bus corridors. Megabus and FlixBus publish limited data but Greyhound historical load factors (pre-pandemic) averaged 40-55% depending on corridor; the 45% assumption is plausible but needs a citation.

## The Question I'd Push On

For the Atlanta-Dallas corridor (one of the five with no current service): how many intermediate T1/T2 stops would a viable timetabled service make, and what is the stop-penalty-adjusted average speed given those stops? If the answer is 8 stops with a 5-minute penalty each, the effective average speed drops from 62 mph to approximately 54-55 mph and the Atlanta-Dallas trip time increases by 35-45 minutes. Does T1 bus remain competitive with driving and with air at that revised speed? Show your work.
