---
reviewer: David Neumark
paper: A.2+rubric-calibration
review_type: recheck
round: 1
date: 2026-05-07
pp_items_rechecked: [PP1.1]
verdict: PASS
score: 3/4
---

> **Note:** AI-generated simulated recheck review.

## Items Rechecked

### PP1.1 — External validation of v1.2 rubric against independent outcome variables

**Concern.** The original paper demonstrated that v1.2 is internally consistent — the new dimensions reduced spurious correlations, resolved the ranking inversion, and passed pairwise independence tests — but never showed that the corrected rubric classifies corridors correctly according to any standard external to the rubric itself. My position was that internal consistency is necessary but not sufficient: a rubric that is self-consistent but wrong about the external world has not been validated. I specifically asked for (a) at least one external outcome measure, and (b) an anchor stability test to show that the 10th/90th percentile anchor choices are not sensitive to the sample.

**Revision.** The revised paper adds a full "External Validation" subsection in Section 6 (Calibration Methodology) with three external validators. First, STRAHNET alignment: Pearson ρ=0.81 between v1.2 tier score and the binary STRAHNET strategic designation across the 227-corridor corpus. Second, ATRI bottleneck density: Pearson ρ=0.72 between v1.2 T1 classification and ATRI bottleneck cost density per corridor-mile — this is the cross-validation the paper needed, drawing on real-world revealed congestion cost data rather than a federal designation. Third, transportation planning document frequency: 47 of 50 state DOT long-range transportation plans reference the same corridors as v1.2 T1, a practitioner-facing confirmation at 94%. In addition, the anchor stability test is present: a ±25% perturbation of all 10th/90th percentile anchor values produces no changes to T1 tier assignments — the tier boundaries are not near any anchor threshold.

**Verdict.** Both items I required are now present. The ρ=0.81 STRAHNET and ρ=0.72 ATRI correlations are the external validators an economist needs: they show that the rubric is tracking something in the real world independent of its own internal construction. For an applied economist, ρ=0.72 against ATRI bottleneck cost density is particularly compelling because ATRI data is operationally grounded in freight industry cost accounting — it is not a federal classification or a researcher's judgment. The ±25% anchor stability test is simple but sufficient: if a 25% swing in anchor values does not move any corridor across a tier boundary, the tier assignments are stable. My blocking concern is resolved.

**P3 note.** The ρ=0.81 STRAHNET correlation should report a confidence interval. With n=227 corridors, the confidence interval on a Pearson correlation is computable analytically (Fisher z-transform) and will be tight — approximately ±0.04 at 95% — but should be stated explicitly. A correlation reported without a confidence interval in a calibration paper invites the question of whether the result would replicate on a different sample. With 227 observations, the interval is narrow enough to be reassuring, but that reassurance is only communicated if the interval appears in the text.

## Verdict

External validation is present and meets the threshold for an empirical calibration paper: two independent external validators (STRAHNET ρ=0.81, ATRI ρ=0.72) plus practitioner confirmation (94% state plan reference rate), with anchor stability demonstrated under perturbation. The paper has moved from internally consistent to externally validated. PASS, score rises to 3/4 from 2/4.
