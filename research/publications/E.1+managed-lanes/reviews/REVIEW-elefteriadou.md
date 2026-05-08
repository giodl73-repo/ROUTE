---
reviewer: Lily Elefteriadou
persona: Lily Elefteriadou — Herbert Wertheim College of Engineering Term Professor, University of Florida; Director, McTrans Center; NCHRP Project Panel member
round: 1
date: 2026-05-07
score: 2/4
---

> **Note:** This is an AI-generated simulated review, written by Claude in the voice of a named-expert persona. It is not an actual review by the named person and does not reflect their views or endorsement.

## Overall

The paper proposes a managed freight lane program for the 8 highest-V/C T1 corridors and claims a 57,600 vpd capacity addition per corridor, a peak V/C reduction on I-75 Atlanta from 1.8 to 1.4, and a PTI of 1.08 on the managed lanes. These are specific numerical claims that rest on the 2,400 pcphpl managed lane capacity figure. That figure is at the upper end of the HCM7 range for managed lanes and requires assumptions about access point frequency and weaving that are not met in most of the corridors analyzed. The paper's throughput and NPV results are sensitive to this assumption in ways that are not acknowledged.

I recommend major revision. The managed lane capacity claim needs to be replaced with a corridor-by-corridor estimate that accounts for access point geometry, or bounded with a sensitivity analysis.

Score: 2/4 — the conceptual case for managed freight lanes is sound; the specific throughput and NPV claims require substantially more careful capacity analysis.

## What Works

The core argument — that freight-dedicated lanes with restricted access prevent induced passenger demand while adding effective freight capacity — is correct and well-supported by the managed lane literature. The distinction between general-purpose (GP) capacity additions, which induce both freight and passenger demand, and freight-only managed lanes, which contain the demand response, is the paper's most important conceptual contribution. This argument is sound regardless of the specific capacity figure used.

The GP lane LOS improvement finding — removing 22% of freight from GP lanes reduces I-75 Atlanta peak V/C from 1.8 to 1.4 — is directionally correct and is likely to survive a revised capacity analysis, though the specific numbers will shift.

The PTI cap at 1.15 for access-point turbulence is an appropriate and conservative engineering judgment. Capping PTI at 1.15 rather than using the theoretical 1.08 for access-free conditions is the right approach for corridors with interchange access; the paper is correct to acknowledge this.

The platooning fuel savings calculation (25% drag reduction × 30% market penetration = $0.039/vehicle-mile) is methodologically clean and uses defensible assumptions from the NREL platooning literature.

## What Doesn't Work

The 2,400 pcphpl managed lane capacity figure is taken from HCM7 Exhibit 12-2 (Basic Freeway Segments) at the upper end of the freeway capacity range. HCM7 provides this figure for "ideal conditions" — no access points within the analysis segment, no weaving, 100% passenger car equivalent. Managed freight lanes are emphatically not ideal-condition freeway segments for three reasons:

First, truck PCE. The paper correctly applies a 2.0 truck PCE in its capacity formula (2 lanes × 2,400 pcphpl / 2.0 PCE = 2,400 trucks/hour per lane), but the 2,400 pcphpl base capacity is itself derived under passenger-car-only conditions. When 100% of the lane is trucks at PCE=2.0, the effective capacity in truck vehicles per hour is 2,400 / 2.0 = 1,200 trucks/lane/hour, not 1,200 trucks/lane at a 2,400 pcphpl base — the formula is correct, but the 2,400 starting point should be the mixed-traffic freeway capacity, not the base ideal capacity.

Second, access points. Every interchange along the managed freight lane corridor is a potential access point, even if access is controlled. Long-haul freight lanes serving truck stops, weigh stations, and intermodal terminals require periodic access points. HCM7 Chapter 14 (Weaving Segments) and Chapter 13 (Merge/Diverge) show that access points reduce effective capacity by 5–20% depending on volume and geometry. For a 200-mile corridor with interchanges every 10–15 miles, this is not a marginal correction.

Third, grade effects. Several T1 corridors (I-80 Donner, I-70 Vail Pass, I-90 Snoqualmie) have sustained grades that substantially reduce truck capacity. HCM7 Chapter 12 provides grade-correction factors; at 5% grade for 2 miles, effective truck capacity is reduced by approximately 40% relative to level terrain. The paper does not apply grade corrections.

The result is that 2,400 pcphpl × 24h / PCE = 57,600 vpd per corridor is an upper-bound estimate under conditions that are not achieved in any of the corridors analyzed. A corridor-by-corridor estimate using access-point and grade corrections would produce substantially lower capacity additions for mountain corridors and modestly lower additions for flat corridors.

The $121B program cost and $115B aggregate NPV inherit this capacity overestimate. If true corridor capacity is 30–40% lower than claimed for mountain corridors, the NPV for those corridors is substantially lower.

## The Question I'd Push On

I-75 Atlanta is cited as the best example of the GP lane V/C improvement (1.8→1.4 peak). The 22% freight diversion figure — which drives this result — implies that 22% of current I-75 Atlanta peak-hour volume is trucks. What is the source for this freight fraction? HPMS data gives truck volume fractions for NHS corridors, but I-75 Atlanta in the peak hour has a substantially lower truck fraction than the daily average, because trucks preferentially operate off-peak to avoid congestion. If the peak-hour truck fraction is 12–15% rather than 22%, the V/C reduction is more like 1.8→1.6, not 1.8→1.4, and the PTI improvement for GP lanes shrinks accordingly. The diversion fraction needs to be cited and should use peak-hour rather than daily average truck fractions.
