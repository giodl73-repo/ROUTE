---
reviewer: elefteriadou
persona: Lily Elefteriadou — traffic engineer, University of Florida Transportation Institute, HCM technical committee
round: 1
date: 2026-05-07
score: 3/4
---

> **Note:** This is an AI-generated simulated review, written by Claude in the voice of a named-expert persona. It is not an actual review by the named person and does not reflect their views or endorsement.

## Overall

The paper correctly identifies the IRI-to-PTI proxy failure and the fix (iri_fallback_max = 5.0) is defensible as a conservative stopgap. Where I want more rigor is in the relationship between PTI and V/C: the paper treats "no PTI data" as equivalent to "IRI fallback applies," but there is a BPR-estimated V/C path to PTI that is never evaluated. A paper at TRR should address the HCM-based estimation path before claiming IRI is the only alternative.

## What Works

**Mechanistic diagnosis of the IRI proxy error.** The explanation is precise: high IRI in rural Wyoming reflects freeze-thaw pavement cycles and heavy truck wear, not congestion-induced delay. A corridor can have rough pavement (high IRI) and still operate at free-flow speed for 95% of operating hours. This distinction is elementary to traffic engineers but was not visible in the v1.0 rubric design, and the paper makes it clear.

**The iri_fallback_max = 5.0 interpretation.** Assigning the IRI cap the interpretive meaning of "moderately unreliable at worst" is epistemically honest: the cap is not a measured score; it is a bound on what can be claimed without PTI data. Communicating uncertainty through score bounding rather than point estimates is the right approach when data is absent.

**Table 2 (v1.0 to v1.1 delta) is properly frustrating.** Showing that the IRI cap reduced I-80's score by more than I-110's — thereby failing to close the gap — is a result the authors did not want but reported honestly. The "partial fix, wrong direction" result is scientifically more useful than a clean resolution would have been, because it forces the reader to understand what v1.1 actually changed.

**Correct deferral of A1 reform.** The paper is right not to reform A1 (Throughput Gap) in the same amendment cycle as the IRI fix. A1 has a legitimate function as a congestion stress indicator; conflating the A1 reform with the IRI fix would obscure which change caused which effect in the before-after comparison. Deferring A1 reform to v1.3 preserves analytical clarity.

## What Doesn't Work

**The BPR-to-PTI path is ignored.** PTI is the ratio of 95th-percentile travel time to free-flow travel time. When observed PTI from FHWA Freight Performance Measures is unavailable, there are two standard approximation paths: (1) the IRI proxy used here, and (2) a BPR-based PTI estimate using V/C ratio from HPMS. The BPR function (travel time = free-flow × (1 + α(V/C)^β) with standard HCM parameters α=0.15, β=4) converts a V/C estimate to a PTI estimate without requiring sensor data. HPMS provides AADT and lane counts from which V/C can be estimated. The paper never discusses why this path was not taken. For rural corridors like I-80 in Wyoming, V/C is low (capacity is not the constraint; weather is), so BPR-PTI would correctly produce low PTI scores. This would have been a better proxy than IRI, and its omission needs justification.

**The 5.0 cap is not calibrated against any PTI distribution.** The paper states that A3 = 5.0 means "moderately unreliable at worst," but this interpretation needs to be anchored in the PTI scale. What is the median PTI for a "moderately unreliable" interstate segment? If the A3 = 5.0 anchor corresponds to, say, PTI = 1.25 (25% travel time buffer needed), is that "moderate"? The HCM Level of Service framework provides conventional thresholds that could serve as calibration anchors. Without this, the 5.0 cap is arbitrary and a reviewer at TRR will send it back for justification.

**A3-A1 correlation (r=0.71) implication is understated.** The paper mentions the A3-A1 Pearson r=0.71 and calls it "the statistical signature of the congestion-stress paradox." This is correct. But the authors do not note that r=0.71 implies the two dimensions share ~50% of their variance (r²=0.50). Running a rubric with two dimensions that share half their variance is equivalent to double-weighting a single construct. The paper recommends v1.3 review of A1; it should also recommend that the A3-A1 correlation be reduced to r<0.50 as a calibration goal — otherwise the next rubric version will inherit the same double-weighting problem with better data.

## The Question I'd Push On

For the 40% of NHS lane-miles without FHWA Freight Performance Measures PTI coverage — including most of I-80 outside Sacramento and Salt Lake City — why was BPR-estimated V/C-based PTI not computed as the primary fallback, with IRI serving as a secondary fallback only when HPMS V/C data is also unavailable? The HPMS coverage for the corridors in question (Wyoming, Nevada) includes AADT and lane count data sufficient for a BPR approximation. The choice to use IRI rather than BPR-PTI is the single decision that most needs justification.
