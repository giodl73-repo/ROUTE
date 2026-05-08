---
reviewer: Susan Hanson
paper: A.1+arterials-tiering
review_type: recheck
round: 1
date: 2026-05-07
pp_items_rechecked: [P1.1]
verdict: PASS
score: 3/4
---

> **Note:** AI-generated simulated recheck review.

## Items Rechecked

### P1.1 — α circularity in estimation procedure

**Concern.** The original paper estimated α=0.65 by maximizing STRAHNET alignment, then validated the centrality-adjusted classification by showing 100% STRAHNET alignment. This was circular: the validation metric was the calibration target. The synthesis asked the authors to either (a) present α=0.65 as the midpoint of a stability region rather than as an estimated parameter, with STRAHNET demoted to post-hoc consistency check, or (b) use the Section 5.3 transportation planning document data as the independent calibration source.

**Revision.** The revised paper does both. The claim "α=0.65 estimated by maximizing STRAHNET alignment" has been removed from Section 3 and replaced with language establishing α=0.65 as the midpoint of the stable region (α ≥ 0.55), with the finding reframed: "for any reasonable centrality weight (α ≥ 0.55), the same 8 corridors emerge as T1 — the result is independent of the specific α value." STRAHNET alignment is now presented in Section 5.1 explicitly as a post-hoc consistency check, not as calibration evidence. The external calibration claim has been relocated to Section 5.3, where the 47/50 transportation planning document frequency (94%) is identified as the independent evidence that the 8 T1 corridors are practitioner-recognized.

**Verdict.** The circularity is broken. The logic now runs: (1) the result is stable across α ≥ 0.55 — this is the primary methodological finding; (2) Section 5.3 document frequency provides independent practitioner-facing calibration; (3) STRAHNET alignment is a post-hoc check against a third independent classification. The sequencing is correct and the three evidence sources are now genuinely independent of each other. My original concern is fully addressed.

**P3 note.** The Section 5.3 transportation planning document calibration is actually the stronger external anchor — 47/50 state DOT plans referencing the same 8 corridors is a practitioner-validation result that operates independently of both STRAHNET (a federal defense designation) and ATRI (revealed freight preferences). I recommend foregrounding the 5.3 evidence in the abstract rather than leading with STRAHNET alignment. A corridor that appears in 94% of state transportation plans is a more policy-actionable finding than a corridor that aligns with a 1993 strategic highway designation.

## Verdict

The α estimation circularity is resolved. The paper now presents a methodologically clean argument: the 8 T1 corridors are robust to all reasonable α values (stability-region finding), confirmed by three independent evidence sources (Section 5.3 document frequency, STRAHNET alignment, ATRI bottleneck density), none of which were used as calibration targets. PASS at 3/4 — my prior score stands; the revision addresses the blocking concern without introducing new issues.
