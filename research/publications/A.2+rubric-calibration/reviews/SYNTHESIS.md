---
paper: A.2+rubric-calibration
round: 1
reviewers: [adamic, elefteriadou, mckinnon, neumark, hanson]
date: 2026-05-07
scores: [3/4, 3/4, 3/4, 2/4, 3/4]
avg_score: 2.8/4
min_score: 2/4
stage_advancement: revision
---

# Review Synthesis — A.2+rubric-calibration Round 1

## Headline Assessment

The panel accepts the conceptual contribution — inductive rubric calibration, the forward-only versioning protocol, and the structural diagnosis of the congestion-stress paradox are all legitimate advances for transportation infrastructure scoring. The paper is blocked on two fronts: it claims empirical validation for the v1.2 rubric using only internal consistency evidence (Neumark, 2/4), and it relies on independence correlation tests computed against a B2 dimension that the paper itself acknowledges is unreliable (Adamic). A revision that addresses external validation and the B2-conditioned independence claims will be in range for acceptance.

## What the Panel Agrees On (Earned Stakes)

**E1 — Forward-only protocol is a genuine methodological contribution.** All five reviewers endorsed the forward-only versioning protocol as scientifically sound. The three-criterion rationale (citation stability, calibration analysis validity, score attribution) is well-articulated and novel in the rubric literature (Adamic, McKinnon, Neumark, Hanson).

**E2 — The IRI-to-PTI proxy failure is correctly diagnosed.** The mechanistic explanation (rough rural pavement ≠ operational speed unreliability) is accepted across the panel. The iri_fallback_max = 5.0 fix is defensible as a conservative bound, even if alternative approaches exist (Elefteriadou, Adamic, McKinnon).

**E3 — A4 (International Trade Corridor) is the strongest of the three new dimensions.** The categorical structure (non-border corridors A4=0), the Laredo anchor ($277B, 16,000 CVPD), and the low A2 correlation (r=0.22) are all endorsed. I-35 A4=8.5 is considered well-grounded (McKinnon, Hanson).

**E4 — C3/C1 correlation acknowledgment is honest.** The panel agrees the paper is right to flag C3 (Economic Opportunity) as tracking C1 (Population Reach) too closely, and to defer C3 reform to v1.3 rather than adding confusion to the v1.2 calibration record (Hanson, Neumark).

**E5 — The before-after comparison tables are analytically exemplary.** Tables 1, 2, and 3 are transparent and allow dimension-level interpretation. The panel notes that the "partial fix, wrong direction" result (v1.1 IRI cap failed to close the gap) is a stronger result than a clean resolution would have been (Elefteriadou, Adamic).

## Contested Stakes

| Reviewer | Stake | Counter | State |
|---|---|---|---|
| Elefteriadou | BPR-based V/C-to-PTI is a better fallback than IRI for rural corridors | McKinnon notes HPMS V/C coverage gaps; Adamic defers to traffic engineering expertise | Open — BPR path needs explicit evaluation or dismissal |
| Adamic | B2 partial-graph renders independence tests for B4 unreliable | Hanson, McKinnon accept the independence results as directionally valid; Neumark sides with Adamic | Contested — requires sensitivity analysis or explicit caveat |
| Neumark | Paper needs external validation against an independent outcome variable | McKinnon argues ATRI data (available in B.2) serves this function; Adamic agrees cross-referencing is needed | Open — ATRI ρ=0.72 in B.2 could be cited here as partial external validation |
| Hanson | I-35 A4=8.5 applies a terminus effect to the full 1,568-mile corridor | McKinnon accepts the simplification as defensible given corridor-level scoring granularity | Contested — geographically weighted A4 is methodologically superior but may be deferred to v1.3 |
| McKinnon | B4 conflates peacetime logistics and strategic mobilization without distinguishing investment implications | Elefteriadou has no objection; Hanson partially concurs | Open — B4 documentation should clarify the two-component structure even if the single dimension is retained |

## P1 — Blocking (must address before recheck)

**P1.1 — External validation.** Add a paragraph in Section 6 (Calibration Methodology) demonstrating that v1.2 tier classifications correlate with at least one external outcome measure. The most available candidate is the ATRI bottleneck cost density result from B.2: the reported Pearson ρ=0.72 between tier classification and bottleneck cost density is precisely the cross-validation this paper needs. Cite B.2 findings explicitly. If ρ=0.72 is not yet published, run the correlation from the ATRI seed data and report it here. A leave-one-out test of the 8 centrality-adjusted T1 corridors against ATRI rank is sufficient.

**P1.2 — B2-conditioned independence test caveat.** Add a caveat to Section 6.3 (Independence test for new dimensions): "The independence correlations reported here are conditioned on v1.2 B2 scores, which are computed on a partial 31-state graph. B4 correlations with B2 (r=0.18) may understate the true correlation because B2 is systematically underestimated for corridors in states with incomplete TIGER/Line coverage. These correlations should be revalidated when the full-graph B2 computation is available in v1.3." This does not require rerunning the analysis — it requires honest conditional language.

**P1.3 — BPR-to-PTI path evaluation.** Add a paragraph in Section 3.2 (The Fix: iri_fallback_max) addressing why BPR-estimated V/C-based PTI was not used as the primary fallback. If HPMS V/C data is unavailable for the relevant rural segments (WY, NV), state this explicitly. If it is available, explain why IRI was preferred. Either answer is acceptable; the current silence is not.

## P2 — Important (address for quality)

**P2.1 — Corpus construction description.** Add 1-2 paragraphs (or a citation) describing how the 227-corridor corpus was constructed: corridor definition criteria, how state-spanning corridors are handled, whether spur designations are included, and the data sources used for corpus assembly. Without this, the statistical results in Section 6 are not reproducible.

**P2.2 — Anchor calibration stability.** Add a bootstrap confidence interval or sensitivity range for the 10th/90th percentile anchor values used in Section 6.3. With N=227, a 1,000-iteration bootstrap of each anchor is feasible computationally and would show whether the anchor choices are stable enough to support the tier classification claims.

**P2.3 — B4 two-component documentation.** Add a sentence or two noting that B4 combines a STRAHNET baseline (all-interstate peacetime logistics) with an installation-proximity bonus (strategic nuclear/STRATCOM access), and that the investment implications of the two components differ. This does not require splitting B4 into two dimensions — it requires acknowledging the decomposition explicitly so policy readers do not assume a high B4 score implies the same investment case regardless of which component drives it.

**P2.4 — C4 hand-curation appendix.** Provide either (a) a data appendix showing the USDA NASS/ERS sources used for the C4 anchor derivations, or (b) an explicit statement that C4 anchor scores are provisional pending v1.3 programmatic USDA ERS integration and should not be treated as independently reproducible. Current language ("hand-curated in v1.2") does not meet TRR reproducibility standards.

**P2.5 — Geographic coverage analysis for A4/B4/C4.** Add a paragraph in Section 4 (or a supplementary table) examining whether the three new strategic dimensions produce geographic blind spots in the corpus — specifically, which regions have corridors that cannot score above the B4 baseline regardless of actual strategic function. Southeast, Appalachia, and Pacific Northwest deserve explicit treatment.

## Score Summary

| Reviewer | Score | Primary concern |
|---|---|---|
| Lada Adamic | 3/4 | B2 partial-graph renders independence tests conditional; C4 hand-curation is reproducibility gap |
| Lily Elefteriadou | 3/4 | BPR-to-PTI path not evaluated; A3=5.0 cap not anchored to HCM PTI distribution |
| Alan McKinnon | 3/4 | B4 conflates peacetime and strategic mobilization without separating investment implications |
| David Neumark | 2/4 | No external validation; corpus construction undescribed; anchor stability untested |
| Susan Hanson | 3/4 | A4 terminus effect applied to full corridor; geographic coverage gaps in new dimensions |

## Next

After P1 items (P1.1 external validation, P1.2 B2 caveat, P1.3 BPR path evaluation) are addressed: recheck with Neumark (external validation was his blocking concern) and Adamic (B2 conditionality). Elefteriadou should review the BPR path response. McKinnon and Hanson can accept P2 items in the revision without a full recheck.
