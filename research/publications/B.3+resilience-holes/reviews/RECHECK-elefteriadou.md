---
reviewer: Lily Elefteriadou
paper: B.3+resilience-holes
review_type: recheck
round: 1
date: 2026-05-07
pp_items_rechecked: [PP2.4]
verdict: PASS-WITH-NOTE
score: 3/4
---

> **Note:** AI-generated simulated recheck review.

## Items Rechecked

### PP2.4 — Donner Pass Waiting Cost Rate (03-investment-case.tex)

**Original concern:** The D1 benefit calculation used $225/hr as the truck waiting cost rate. This is the ATRI full in-motion operating cost (driver pay + fuel at speed + amortized fixed costs). A truck idling during a closure does not incur fuel-at-speed or in-motion amortized costs. The correct idle rate is approximately $91/hr (driver pay ~$89/hr + idle fuel ~$2/hr). Applying the in-motion rate to a stopped truck inflated the D1 annual benefit and the Donner NPV.

**What the revision did:** The D1 benefit calculation in Section 3 has been corrected throughout. The waiting cost rate is now $91/hr, explicitly defined as "idle rate: driver $89 + fuel at idle $2," with a note distinguishing this from the full ATRI rate ($225/hr) which is correctly retained for rerouting cost calculations (where trucks are in motion on the I-50 alternate). The D1 annual benefit revises from approximately $700M/year to $400M/year. Combined annual benefit (B1 + D1) revises from $1.6B to $1.3B/year. NPV revises from $15.8B to $12.1B. Cost-benefit ratio revises from 5.75:1 to 4.0:1. The comparison table (Table 1) and portfolio table (Table 2) are updated to reflect the corrected figures throughout.

**Is it satisfactory?** Yes. This is exactly the correction requested. The distinction between idle rate and in-motion rate is now explicit in the text, which eliminates any ambiguity about which cost basis applies to which behavior (waiting vs. rerouting). The revised NPV of $12.1B at 4.0:1 CBR is still a strong investment case — a 4:1 benefit-cost ratio at a 7% real discount rate over 30 years is well above the threshold for public infrastructure investment priority. The paper's conclusion (compound investment in the tunnel dominates both single-dimension alternatives) holds at the corrected figures: the tunnel NPV of $12.1B still substantially exceeds the US-50 alternate ($8.6B) and snowshed hardening ($2.7B).

The revision makes the economics more credible, not less persuasive. The original inflated figure invited criticism that the entire NPV case was built on a unit cost error; the corrected figure is defensible against that challenge.

## Verdict

PP2.4 is fully resolved. The waiting cost correction is precise, the rate basis is now explicitly stated in the text, and all downstream figures (NPV, CBR, tables) are internally consistent with the corrected rate. The investment priority ordering is unchanged.

**P3 note for authors:** The Donner NPV model does not currently include ongoing O&M costs for tunnel operation (lighting, ventilation, winter maintenance of tunnel infrastructure, periodic lining inspection). Tunnel O&M at analogous facilities (Mont Blanc, Eisenhower/Johnson) runs $8–15M/year per tunnel mile. For a 12–15 mile tunnel this would be $96–225M/year in O&M, which would reduce the net annual benefit from $1.3B to approximately $1.1–1.2B/year and the NPV to approximately $10.5–11.4B — still clearly positive. A sensitivity line in Table 1 showing NPV with and without O&M would strengthen the investment case by demonstrating robustness to this cost component.
