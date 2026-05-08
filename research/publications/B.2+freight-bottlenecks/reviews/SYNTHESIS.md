---
paper: B.2+freight-bottlenecks
round: 1
reviewers: [mckinnon, elefteriadou, puentes, neumark, adamic]
date: 2026-05-07
scores: [3/4, 3/4, 3/4, 2/4, 3/4]
avg_score: 2.8/4
min_score: 2/4
stage_advancement: revision
---

# Review Synthesis — B.2+freight-bottlenecks Round 1

## Headline Assessment

The panel accepts the ATRI-to-ROUTE integration as methodologically sound and the T1/T2 cost paradox as a genuine empirical finding. The I-40 zero-bottleneck result receives unanimous endorsement as the paper's strongest validation. Two blocking issues emerge: Neumark (2/4) requires a counterfactual — the paper diagnoses $22.7B in costs but cannot bound achievable savings per investment dollar — and Adamic flags that "cascade multiplier" is borrowed terminology without the propagation theory to support it, which will draw scrutiny at TRR. A revision addressing these two concerns is likely to advance to acceptance.

## What the Panel Agrees On (Earned Stakes)

**E1 — I-40 zero-bottleneck finding is the paper's strongest empirical result.** All five reviewers endorsed the convergence between ROUTE's V/C = 0.84 prediction and ATRI's revealed zero-bottleneck as the best evidence for classification validity. The monotonicity (lowest V/C → no ATRI locations) is exactly what a correctly calibrated scoring model should produce (McKinnon, Elefteriadou, Puentes, Neumark, Adamic).

**E2 — Centrality-adjusted T1 classification outperforms aggregate-score T1 in ATRI validation.** The panel agrees that Section 6's comparison (centrality-adjusted 8-corridor T1 outperforms aggregate-score 13-corridor T1 on ATRI cost per location) is the strongest methodological cross-validation in the paper. It prefers one classification method over another using independent data (Adamic, McKinnon, Puentes).

**E3 — Atlanta I-285 T1 upgrade case is well-structured.** All reviewers accept the I-285 argument as an evidence-based reclassification proposal: functional mismatch (T2 corridor doing T1 work) + economic consequence ($3.0B/yr) + specific investment recommendation. The panel endorses this as the paper's most policy-useful finding (Puentes, McKinnon, Neumark, Adamic).

**E4 — ATRI cost figures are appropriately scoped.** The paper's acknowledgment that $22.7B is direct trucking cost only (not inventory, supply chain, or secondary effects) is honest and correctly calibrated for what ATRI measures. The panel accepts the figure as stated (McKinnon, Neumark).

**E5 — Investment sequencing is operationally plausible.** Donner Pass → T1 managed lanes (highest bottleneck density first) → T2 Atlanta relief is accepted as internally consistent with the cost analysis. The Donner Pass payback calculation, if sourced and validated, supports Phase 1 priority (Puentes, McKinnon).

## Contested Stakes

| Reviewer | Stake | Counter | State |
|---|---|---|---|
| Adamic | "Cascade multiplier" is a borrowed term without network propagation theory | McKinnon accepts the intuitive framing; Elefteriadou does not object to the terminology | Contested — rename or add minimal cascade theory |
| Neumark | $22.7B diagnosis requires cost-reduction counterfactual to support investment recommendations | McKinnon agrees managed lane elasticity should be cited; Puentes sides with Neumark on investment framing | Open — managed lane cost-reduction literature must be cited |
| Adamic | Spearman ρ = 0.67 overstates significance due to spatial autocorrelation across adjacent corridors | No reviewer directly contested; Elefteriadou supports the methodology concern | Open — spatial independence check needed |
| McKinnon | Value-at-risk effect is a third cascade multiplier mechanism asserted without FAF5 data | Adamic concurs; other reviewers do not address | Open — verify against FAF5 or remove |
| Elefteriadou | PTI ≤ 1.15 target lacks HCM grounding | Puentes notes the policy implications depend on whether the target is achievable; Neumark concurs | Open — cite standard or provide derivation |
| Puentes | Donner Pass $4B and $1.6B/yr avoided cost need citation chain | Neumark concurs | Open — source the methodology or show the calculation |

## P1 — Blocking (must address before recheck)

**P1.1 — Cost-reduction counterfactual for investment recommendations.** Add a paragraph in Section 8 (Implications) bounding the expected cost reduction per bottleneck dollar from managed lane deployment. Cite the managed lane literature (e.g., Small et al. 2006; NCHRP Report 722 on managed lane performance) for freight delay elasticity by bottleneck type. Show expected cost reduction under the managed lane scenario for at least the top-3 priority investments (I-95 Northeast, I-75 Atlanta, Donner Pass). Without this, the investment sequencing has no economic support beyond the cost ranking.

**P1.2 — Cascade multiplier: rename or formalize.** Either (a) rename to "T1 cost premium" or "tier cost ratio" to match what is actually computed (a cross-sectional average cost ratio, not a network propagation effect), or (b) add a minimal cascade propagation model demonstrating that T1 bottleneck costs propagate to downstream T2 and T3 network segments in a way that exceeds the direct cost — with at least a stylized network example or a citation to the network disruption propagation literature (e.g., Berdica 2002; Jenelius 2010). The current use of "cascade" without the propagation mechanism will be challenged in TRR review.

## P2 — Important (address for quality)

**P2.1 — PTI ≤ 1.15 standard: cite source or provide derivation.** Add the citation or derivation for the PTI ≤ 1.15 T1 standard in Section 8.3. If it is from the ROUTE E.2 framework paper, cite it. If it is derived from FHWA FPM thresholds or HCM LOS standards, show the derivation.

**P2.2 — Donner Pass cost methodology: show the calculation.** Expand the Donner Pass weather bottleneck cost derivation (currently in Section 7) to show the full calculation: closures/yr × hours × truck volume × $225/hr rerouting premium = $1.6B/yr. Source the $225/hr figure (ATRI uses $150/hr for operational cost; the $225/hr rerouting premium differential should be cited or explained). Cite the Caltrans incident database for the closure frequency and duration figures.

**P2.3 — Value-at-risk mechanism: verify or remove.** Either verify the value-at-risk cascade multiplier mechanism against FAF5 commodity flow data (compare freight value per truck-mile on T1 vs. T2 corridors using commodity value fields), or remove it from the three-mechanism attribution and note that the volume and rerouting cost mechanisms are sufficient to explain the 1.73× premium.

**P2.4 — Spatial autocorrelation check for Spearman ρ = 0.67.** Add a note in Section 4.2 acknowledging that adjacent corridors are not spatially independent observations and that the Spearman ρ standard error may be understated. Report cluster-robust standard errors grouped by geographic region, or a Moran's I test on the ATRI density residuals.

**P2.5 — I-285 graph-theoretic centrality in T1 subgraph.** Add the I-285 betweenness centrality computation in the T1 subgraph (corridors within 300 miles of Atlanta) as supporting evidence for the T1 reclassification argument. The route CLI's diamond analysis infrastructure appears to support this computation. If I-285 has high local betweenness in the T1 subgraph, this is a network-theoretic complement to the bottleneck cost argument.

## Score Summary

| Reviewer | Score | Primary concern |
|---|---|---|
| Alan McKinnon | 3/4 | ATRI omits queue spillback/secondary incidents; cascade multiplier third mechanism unverified |
| Lily Elefteriadou | 3/4 | PTI targets not HCM-grounded; V/C computation method unspecified; I-95 PTI uncited |
| Robert Puentes | 3/4 | No IIJA funding path; Donner Pass cost unsourced; I-285 upgrade bottleneck relocation risk |
| David Neumark | 2/4 | No cost-reduction counterfactual; cascade multiplier not identified; weather cost unvalidated |
| Lada Adamic | 3/4 | "Cascade multiplier" lacks propagation theory; Spearman ρ spatially dependent; I-285 graph computation missing |

## Next

After P1 items (P1.1 cost-reduction counterfactual, P1.2 cascade multiplier rename or formalize) are addressed: recheck with Neumark (counterfactual was his blocking concern) and Adamic (cascade terminology). McKinnon, Elefteriadou, and Puentes can accept P2 items in the revision without a full recheck.
