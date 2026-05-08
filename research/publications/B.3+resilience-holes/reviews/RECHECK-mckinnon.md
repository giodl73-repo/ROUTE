---
reviewer: Alan McKinnon
paper: B.3+resilience-holes
review_type: recheck
round: 1
date: 2026-05-07
pp_items_rechecked: [PP2.4, PP2.5]
verdict: PASS-WITH-NOTE
score: 3/4
---

> **Note:** AI-generated simulated recheck review.

## Items Rechecked

### PP2.4 — Donner NPV Correction (shared with Elefteriadou)

**Original concern:** The Donner NPV was built on a $225/hr waiting cost for stranded trucks, which is the ATRI in-motion operating rate. The correct idle rate (~$91/hr) produces a substantially lower D1 benefit and NPV. The compound investment case was arithmetically unsound at the original rate.

**What the revision did:** The D1 benefit calculation has been corrected to $91/hr idle rate (driver + idle fuel), with explicit note distinguishing idle from in-motion cost basis. Annual D1 benefit revises to ~$400M/year; combined benefit to $1.3B/year; NPV to $12.1B; CBR to 4.0:1. All tables updated consistently.

**Is it satisfactory?** Yes. The correction is precise and the rate basis is now documented in the text. A 4.0:1 CBR remains a compelling case for public infrastructure investment — freight economists will recognize this as well above OMB guidance for federal infrastructure projects (typically 1.5–2.0:1 for discretionary programs). The compound investment advantage over single-dimension alternatives is preserved at the corrected figures, which is the central claim of the paper.

### PP2.5 — Cross-Track Citation and Dependency Documentation (02-background.tex)

**Original concern:** The D1 benefit calculation in B.3 relies on D1 scores from Paper D.1, but B.3 did not cite D.1 or document this cross-track dependency. A reader evaluating B.3's NPV model could not trace the D1 score inputs to their source.

**What the revision did:** Section 2 (Background) now cites Paper D.1 (\citep{ROUTE_D1}) in the D1 dimension description paragraph and includes an explicit sentence: "B.3's compound exposure thresholds require D.1 to be finalized before the compound corpus can be validated." The D1 dimension subsection now references the D.1 paper as the source of the D1 scores used in B.3's analysis.

**Is it satisfactory?** Yes. The citation resolves the traceability gap. A reader can now follow the dependency chain: B.3 NPV → D1 score inputs → D.1 paper → FEMA NFHL + Caltrans/WSDOT closure logs. The cross-track dependency statement is useful documentation of the paper's reliance on D.1 for the compound corpus validation step.

## Verdict

Both PP items are resolved. The NPV correction (PP2.4) eliminates the unit cost error that was the primary concern, and the cross-track citation (PP2.5) closes the traceability gap. The paper's investment case is now internally consistent and externally traceable.

**P3 note for authors:** The D1 benefit calculation in B.3 currently uses the D1 raw score from Paper D.1 v1.2. Paper D.1 v1.3 introduced a structural multiplier (μ = 5.0) in the D1 normalization that adjusts raw scores for compound exposure intensity. B.3's compound exposure cost model should reference the v1.3 normalized D1 score rather than the v1.2 raw score for full consistency with the D.1 methodology. This is a minor alignment issue — the direction of the adjustment (normalized score is higher for high-compound corridors) would increase the D1 benefit modestly, further strengthening the tunnel NPV — but the paper should either reference v1.3 explicitly or note that it uses the v1.2 raw score as a conservative input.
