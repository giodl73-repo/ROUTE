---
reviewer: Lily Elefteriadou
persona: Lily Elefteriadou — Herbert Wertheim College of Engineering Term Professor, University of Florida; Director, McTrans Center; NCHRP Project Panel member
round: 1
date: 2026-05-07
score: 3/4
---

> **Note:** This is an AI-generated simulated review, written by Claude in the voice of a named-expert persona. It is not an actual review by the named person and does not reflect their views or endorsement.

## Overall

The paper's closure cost model is well-specified at the economic level but rests on traffic assignment assumptions that are underspecified. The lognormal closure duration distribution is a convenient parametric assumption that is not validated against empirical data, and the traffic volume during closure events is almost certainly not equal to average daily volume. Both of these affect the expected-cost integral in ways that could be either conservative or liberal, and the paper should quantify the direction and magnitude.

Score: 3/4 — solid contribution to the closure cost literature; the traffic modeling assumptions need validation or explicit bounding before submission to *Transportation Research Part E*.

## What Works

The max(wait_cost, reroute_cost) model correctly captures the network equilibrium under closure: some shippers wait at the closure point, others reroute, and the split depends on the relative costs of each option. The model does not assume a fixed reroute fraction, which is the right approach — the split is endogenous to the reroute cost, which depends on detour length (B1) and available alternate capacity.

The B1 multiplier (1 + detour_miles/100) is a clean formalization of what traffic engineers observe in practice: isolated corridors have higher effective closure costs per unit volume because rerouting is both longer and more congested (since fewer vehicles can reroute to the limited alternates without creating secondary congestion on those routes). The fact that Donner B1=8.3 explains the 4.2× cost ratio against Dallas B1=5.9 is an elegant result.

The paper's distinction between closures with viable reroutes (Dallas, Baltimore) and closures where waiting dominates (Donner during extended winter events) maps onto a traffic assignment distinction between elastic and inelastic demand responses, which is the right conceptual structure.

## What Doesn't Work

The lognormal assumption for closure duration deserves more scrutiny than a footnote. Lognormal is the standard parametric assumption in many reliability applications, but closure duration distributions are empirically known to be event-type-dependent:
- Incident-related closures (crash, vehicle fire): approximately lognormal, right-skewed, modal duration ~2–4 hours.
- Weather-related closures (winter, flooding): more bimodal — many short closures (1–6 hours) plus a heavy tail of multi-day events. This is not well-approximated by a single lognormal.
- Disaster-related closures (bridge damage, major flooding): extreme right tail; a single event can account for 30–50% of a corridor's multi-year closure cost.

If disaster-related closures are included in the distribution and the tail is heavier than lognormal, the expected value of the integral (and hence E[cost]) is understated. The paper should at minimum report the empirical duration data for each corridor's historical closures, fit the distribution, and report whether lognormal is a reasonable fit. For Donner, where WSDOT/Caltrans data are available for 20+ years of closure events, this is feasible.

The traffic volume during closure is assumed to be average daily volume (ADV). This is almost certainly wrong for weather-related closures: closures are triggered by events that simultaneously suppress trip generation. A severe Sierra storm that closes Donner also reduces traffic demand (fewer drivers attempt the route during dangerous conditions). Using ADV during a weather-related closure overstates the volume stranded at the closure point.

The correct approach is a demand adjustment: V_closure = ADV × (1 - demand_reduction_factor), where the demand reduction factor is event-type-specific (perhaps 0.3–0.5 for major winter events, 0.1–0.2 for shorter events). This would reduce the estimated closure cost for weather-dominated corridors (Donner, Snoqualmie) and shift the ranking somewhat.

## The Question I'd Push On

For urban corridors (Dallas Interchange, I-95 Baltimore), the reroute option creates secondary congestion on alternates. The B1 multiplier models this through detour miles, but secondary congestion is not a linear function of detour miles — it is a function of alternate route capacity utilization. If the alternate route is already at 70% capacity (which is typical for urban corridors), adding diverted traffic from a major closure pushes it into breakdown, and the effective reroute cost rises nonlinearly.

Has the paper modeled the secondary congestion effect on alternate routes for urban corridors? For Dallas (B1=5.9, $0.8B/yr), the alternates (I-635, I-20, SH-183) carry substantial background traffic. If the model assumes the alternate is freely flowing, the reroute cost is understated and the B1 multiplier may be too low for urban corridors. Conversely, if urban alternates are already congested, it strengthens the case that urban interchange closures are costlier than the model implies — and might change the relative ranking of Dallas vs. the rural corridors.
