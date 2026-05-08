---
reviewer: Lily Elefteriadou
paper: B.3+resilience-holes
review_type: recheck
round: 1
date: 2026-05-08
pp_items_rechecked: [PP2.4]
verdict: PASS
score: 3/4
---

> AI-generated simulated recheck.

## Items Rechecked

### PP2.4 — Donner Pass waiting cost rate, propagated to all sections

**Original concern:** The D1 benefit calculation used $225/hr as the waiting cost rate for stranded trucks during closure events. This is the ATRI full in-motion operating cost (driver pay + fuel at speed + amortized fixed costs). A truck idling during a closure does not incur fuel-at-speed or in-motion amortized costs. The correct idle rate is approximately $91/hr (driver pay ~$89/hr + idle fuel ~$2/hr). Applying the in-motion rate inflated the annual disruption estimate, the NPV, and the cost-benefit ratio throughout the paper.

**What the revision did:** The correction was applied in two passes. The first pass corrected Section 03 (investment case), revising the D1 benefit from ~$700M/year to ~$400M/year and the combined annual benefit from $1.6B to $1.3B/year. A post-write check identified that the corrected figures had not propagated to all affected sections. The second pass propagated $1.3B consistently to all four sections containing the disruption estimate:

- **Section 01 (Introduction):** Line 7 now reads "$1.3 billion in annual freight disruption cost (B1 rerouting penalty $900M + D1 idle waiting cost $400M at corrected idle rate)" — consistent with the $91/hr idle rate correction.
- **Section 04 (Compound Exposure), Table tab:compound:** The Donner annual cost column shows 1.30 (not 1.60).
- **Section 05 (Investment Sequencing):** The D1-only ordering discussion references "Donner's $1.3B annual disruption cost exceeding Gulf Coast's $0.82B" — consistent with the corrected figure.
- **Section 07 (Conclusion):** The Donner NPV is stated as $12.1 billion at 4.0:1 CBR and 3.1-year simple payback — all consistent with the $1.3B annual benefit at 7% discount rate over 30 years.

All four sections are internally consistent with the $91/hr idle rate correction. The $1.6B figure does not appear in any section.

**Is the fix adequate?** Yes. The propagation is complete and consistent. The introduction, compound exposure table, investment sequencing narrative, and conclusion all agree on $1.3B annual disruption cost, $12.1B NPV, 4.0:1 CBR, and 3.1-year payback. A reader tracing the Donner figures from the abstract through to the conclusion will find no internal inconsistency.

The revised NPV of $12.1B at 4.0:1 is analytically sounder than the original inflated figure. A 4.0:1 cost-benefit ratio at a 7% real discount rate is well above the threshold for federal infrastructure investment priority (OMB guidance for discretionary programs typically requires 1.5–2.0:1). The paper's central finding — that the Donner freight tunnel is the highest-NPV single investment in the I2.0 program — holds at the corrected figures.

**Residual concern (P3, non-blocking):** The portfolio table in Section 05 reports a total portfolio NPV of approximately $62B against $12.4B invested (5.0:1 portfolio CBR). This portfolio figure includes Donner at the corrected $12.1B NPV. Authors should confirm that the portfolio CBR of 5.0:1 incorporates the Donner correction; the portfolio-level CBR should also be updated if it was calculated from the pre-correction Donner figure. A footnote confirming that the portfolio NPV reflects the corrected $1.3B Donner annual benefit (not the original $1.6B) would close this residual.

## Verdict

PP2.4 is fully resolved and consistently propagated across all four sections. The correction is precise, the rate basis ($91/hr idle vs. $225/hr in-motion) is explicitly stated in the introduction, and all downstream figures are internally consistent. Score rises from 2/4 to 3/4.
