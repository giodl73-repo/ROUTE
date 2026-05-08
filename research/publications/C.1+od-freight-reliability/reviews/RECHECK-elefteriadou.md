---
reviewer: Lily Elefteriadou
paper: C.1+od-freight-reliability
review_type: recheck
round: 1
date: 2026-05-07
pp_items_rechecked: [PP1.4]
verdict: PASS-WITH-NOTE
score: 3/4
---

> **Note:** AI-generated simulated recheck review.

## Items Rechecked

### PP1.4 — BPR Calibration Range Limitation (03-methods.tex)

**Original concern:** The BPR volume-delay function was applied at V/C = 1.86 for I-80 Bay Area segments, which extrapolates well beyond the function's empirically calibrated range (V/C ≤ 1.3 per HCM 7). At this saturation level the BPR quartic term departs systematically from observed stop-and-go behavior, making PTI estimates unreliable at exactly the corridor segments where reliability is most consequential to the paper's argument.

**What the revision did:** A dedicated paragraph titled "BPR Calibration Range Limitation" has been inserted in Section 3 immediately after the PTI equation (Eq. 3). The paragraph (a) explicitly states the V/C ≤ 1.3 calibration bound per HCM 7; (b) identifies I-80 Bay Area peak V/C = 1.86 as exceeding that bound; (c) correctly explains the direction of the bias — BPR underestimates congestion delay at high V/C because the empirical stop-and-go relationship steepens more sharply than the quartic term; (d) reframes the BPR-derived PTI of 1.86 as a **conservative lower bound** on actual PTI; and (e) cites NPMRDS probe-vehicle speed data reporting peak travel time ratios of 2.1–2.4 on the I-80 corridor, which corroborates both the order of magnitude of the BPR estimate and the direction of the bias.

**Is it satisfactory?** Yes. This is the methodologically correct response to the PP1.4 concern. Explicit acknowledgment of an extrapolation limitation, combined with independent empirical corroboration from NPMRDS probe data, is the appropriate treatment in a methods section. Crucially, the authors recognize that the bias direction actually *strengthens* the paper's core claim: because BPR underestimates PTI at high V/C, managed lane improvements that reduce PTI from 1.86 to 1.15 (the I2.0 scenario) are understatements of the true reliability gain. The conservative framing is analytically honest and scientifically defensible.

The NPMRDS citation (PTI 2.1–2.4 for the Bay Area) is appropriate for corroboration purposes. I would note that probe-data coverage on I-80 varies by segment and year; the paper should ensure the NPMRDS access period is cited precisely. This is a minor documentation issue, not a blocking concern.

## Verdict

The revision resolves PP1.4 fully: the BPR limitation is now disclosed, the bias direction is correctly characterized, and all PTI-dependent findings are explicitly framed as conservative lower bounds. The paper's conclusion is stronger as a result — the managed lane case is not undermined by the limitation but actually reinforced by it.

**P3 note for authors:** Future work should consider a direct NPMRDS-based PTI calibration for the Bay Area segments, replacing the BPR extrapolation with observed probe data. This would elevate the PTI findings from "conservative lower bound" to primary estimate and would be publishable as a standalone methods note supporting the C-series findings.
