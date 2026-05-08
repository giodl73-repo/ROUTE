---
reviewer: David Neumark
paper: F.3+relay-marketplace
review_type: recheck
round: 1
date: 2026-05-08
pp_items_rechecked: [P1.1]
verdict: PASS
score: 3/4
---

> AI-generated simulated recheck.

## Items Rechecked

### P1.1 — Hub fee economics and market structure framing (02-why-relay-fails-today.tex)

**Original concern:** The paper compared $1,050 relay driver cost against $1,456 solo driver cost and claimed relay was cheaper. This omitted hub fees ($100/swap × ~6 swaps = $600), making relay actually more expensive per trip at $1,650 vs. $1,456. The economic case was built on an incomplete cost comparison; the market structure argument needed a rigorous per-ton-mile reframing, not a per-trip direct cost claim.

**What the revision did:** A new paragraph ("The full per-trip comparison including hub fees") has been added immediately after the visible cost comparison. The paragraph explicitly states: relay costs $1,650 per trip versus $1,456 for solo driving once hub fees are included. It then explicitly concedes that relay is *more expensive* per trip as a direct cost. The economic case is reframed around asset utilization: relay trucks operate at 98% utilization versus 48% for solo drivers, meaning the same freight ton-miles require roughly half the truck fleet. The paragraph includes a calculation showing approximately $450 million in fleet capital savings for a carrier running 500 transcontinental routes annually, and states the correct comparison is per-ton-mile per dollar of fleet capital — a comparison that favors relay by approximately 2.0× on routes exceeding 1,500 miles. The $8,700/truck/year capital savings figure is shown with a formula including the annualized capital recovery factor at 7%.

**Is the fix adequate?** Yes. This is exactly the reframing the P1.1 concern required. Three things are now correct that were wrong before:

First, the paper no longer claims relay is cheaper per trip. It acknowledges the opposite, which is honest economics and eliminates the most obvious target for adversarial criticism.

Second, the asset utilization argument is now the primary economic case, not a secondary observation. The 98% vs. 48% utilization comparison is factually correct under HOS constraints and is the right lens for evaluating relay economics. A carrier economist evaluating relay adoption will immediately recognize this framing as valid.

Third, the per-ton-mile comparison is stated explicitly rather than implied. The 2.0× advantage on routes over 1,500 miles is a testable, falsifiable claim — the kind of rigorous formulation the original review found absent.

**Residual concern (P3, non-blocking):** The $450 million fleet capital savings figure ("500 routes × 260 operating days × $8,700 capital savings per truck-year ÷ 2,500 mi/route ≈ fleet-level savings at scale") relies on carrier-level assumptions (500 transcontinental routes, 2,500 mi/route) that are not cited. The calculation is internally consistent, but the carrier scale assumption should be grounded in FHWA carrier census data or a named representative carrier. Authors should add a footnote noting that the $450M figure is illustrative for a large-carrier scenario; small carriers (3–4 trucks) would see proportionally smaller absolute savings but the per-route capital advantage ratio is identical.

## Verdict

P1.1 is resolved. The per-ton-mile reframing is what the review asked for; the honest acknowledgment that relay is more expensive per trip strengthens rather than weakens the economic argument by preempting the obvious objection. The asset utilization argument is a legitimate and rigorous basis for the market case. Score rises from 2/4 to 3/4.
