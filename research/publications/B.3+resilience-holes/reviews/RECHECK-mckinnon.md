---
reviewer: Alan McKinnon
paper: B.3+resilience-holes
review_type: recheck
round: 1
date: 2026-05-08
pp_items_rechecked: [PP2.4, PP2.5]
verdict: PASS-WITH-NOTE
score: 3/4
---

> AI-generated simulated recheck.

## Items Rechecked

### PP2.4 — Donner NPV correction, propagated to all sections

**Original concern:** The Donner NPV was built on a $225/hr waiting cost for stranded trucks, which is the ATRI in-motion operating rate. The correct idle rate (~$91/hr) produces a substantially lower D1 annual benefit and NPV. The compound investment case was arithmetically unsound at the original rate, and all sections citing the Donner annual cost, NPV, CBR, or payback period needed correction.

**What the revision did:** The correction was applied in two passes. The first pass corrected Section 03; a subsequent post-write propagation pass updated all remaining sections containing the disruption figures:

- **Section 01 (Introduction):** $1.3B annual cost stated with explicit breakdown ($900M B1 rerouting + $400M D1 idle at corrected rate). The phrase "at corrected idle rate" is present, providing traceability to the rate correction.
- **Section 04 (tab:compound):** Donner annual cost column shows 1.30, consistent across all other corridors in the table.
- **Section 05 (Investment Sequencing):** Donner annual disruption cost referenced as $1.3B in the D1-only ordering discussion, correctly positioned against Gulf Coast's $0.82B.
- **Section 07 (Conclusion):** NPV $12.1B, payback 3.1 years, CBR 4.0:1 — all consistent with a $1.3B annual benefit at 7% discount rate over 30 years.

The $1.6B figure is absent from all sections. Internal consistency is complete.

**Is the fix adequate?** Yes. The correction is now consistent throughout the paper. A freight economist tracing the Donner NPV derivation — from the waiting cost rate in Section 01 through the annual benefit in Section 04 to the NPV calculation in Section 07 — will find a coherent and defensible cost model. The 4.0:1 CBR at a 7% real discount rate is a compelling case for federal infrastructure priority; freight economists will recognize this as well above OMB guidance thresholds.

### PP2.5 — Cross-track citation and D1 dependency documentation (02-background.tex)

**Original concern:** B.3's NPV model relies on D1 scores from Paper D.1, but B.3 did not cite D.1 or document this cross-track dependency. A reader evaluating B.3's compound exposure thresholds and NPV model could not trace the D1 score inputs to their source paper.

**What the revision did:** Section 02 (Background) now cites Paper D.1 (\citep{ROUTE_D1}) in the D1 dimension description paragraph and includes an explicit statement that B.3's compound exposure corpus validation requires D.1 to be finalized before the compound thresholds can be validated. The D1 subsection references D.1 as the source of the D1 scores used in B.3's analysis.

**Is the fix adequate?** Yes. The citation closes the traceability gap. The cross-track dependency statement is correctly documented.

## Verdict

Both PP items are resolved. PP2.4 is correctly propagated to all four affected sections with no inconsistencies. PP2.5 is addressed through a citation and explicit dependency statement. The paper's investment case is internally consistent and externally traceable.

**P3 note for authors:** The D1 scores used in B.3's compound exposure analysis are drawn from Paper D.1 v1.2. Paper D.1 v1.3 introduced an ECH100 normalization (structural multiplier μ = 5.0) that adjusts raw D1 scores for compound exposure intensity. B.3's compound exposure cost model should reference the v1.3 normalized D1 score rather than the v1.2 raw score for full consistency with the current D.1 methodology. The direction of the adjustment would modestly increase D1 benefit for high-compound corridors (Donner, Gulf Coast), further strengthening the tunnel NPV — so using the v1.2 raw score is conservative rather than inflated. Authors should add a footnote in Section 04 noting that D1 scores in the compound table are drawn from D.1 v1.2; a v1.3 recalculation incorporating the ECH100 normalization is planned. This is a minor alignment issue and does not block advancement.
