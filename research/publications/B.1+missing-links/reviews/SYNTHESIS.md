---
paper: B.1+missing-links
round: 1
reviewers: [transport-geographer, rural-economist, transport-policy, network-scientist, equity-researcher]
date: 2026-05-07
scores: [3/4, 3/4, 3/4, 2/4, 2/4]
avg_score: 2.6/4
min_score: 2/4
stage_advancement: revision
---

# Review Synthesis — B.1+missing-links Round 1

## Headline Assessment

The paper makes a genuine contribution: the first population-weighted county-level coverage gap analysis of the US interstate system, with a reproducible headline number (66.5M, 20.4%) and a ranked priority corridor list. The panel finds the research direction important, the data sources appropriate, and the coverage efficiency metric useful. Three reviewers recommend accept with major revision; two recommend major revision with concern.

**The paper advances to revision stage.** The P1 items below must be addressed before recheck.

---

## What the Panel Agrees On (Earned Stakes)

**E1 — The 30-mile standard is well-justified.** All five reviewers accepted the threshold without fundamental challenge. The three-rationale derivation (emergency response, agricultural freight, labor market) is the paper's most solid foundation. No reviewer proposed a different threshold.

**E2 — The four-zone taxonomy is empirically grounded and policy-relevant.** The Northern Tier / Appalachians / Gulf South / Rural West classification was validated by all reviewers. The Type 1/2/3/4 classification adds useful analytical depth. No reviewer challenged the geographic cluster assignments.

**E3 — Coverage efficiency (gap population / \$B) is the right ranking criterion for this analysis.** All reviewers accepted it as a useful first-pass metric. The transport policy reviewer flagged the omission of political feasibility; the rural economist flagged the omission of freight value — but neither proposed abandoning the metric, only supplementing it.

**E4 — The C3 economic opportunity correlation is the paper's strongest surprise finding.** Four of five reviewers highlighted it positively. The rural economist cautioned against over-interpreting it causally; the equity researcher wanted it extended to transit-dependent population. Consensus: it belongs in the abstract and should be more prominently foregrounded.

**E5 — I-3 and Northern Tier as top-2 ranked corridors is well-supported.** Three reviewers explicitly validated these rankings as consistent with their own domain knowledge. The transport policy reviewer noted they are also the most politically difficult — which creates a tension the paper should acknowledge.

---

## Priority Items

### P1 — Blocking (must address before recheck)

**P1.1 — Quantify the county centroid artifact for large-county states.**
*(Transport geographer, Network scientist)*
The paper acknowledges the centroid limitation but does not quantify it. At minimum: (a) list the 10–15 counties where the centroid effect most severely overstates the access deficit (likely CA, AZ, NV large counties); (b) provide a corrected headline number that excludes centroid-artifact counties or flags them as uncertain. If the corrected number is still 17–18%, the paper's case is unchanged; if it drops to 14%, the paper needs to be reframed. The paper must resolve this before it can be published.

**P1.2 — Verify the 1,465 interchange node count against FHWA data.**
*(Network scientist)*
A national interstate network of 48,800 miles should have significantly more than 1,465 interchange nodes. If nodes are underrepresented, the coverage analysis systematically overstates the gap. Either verify the count against the FHWA Interchange Safety Analysis Tool (iSAT) database or add a clear caveat that the analysis uses intersection nodes only (not all ramp access points) and that this may overstate the gap.

**P1.3 — Acknowledge the construction-era equity harms and commit to alignment process.**
*(Equity researcher)*
The paper cannot frame I2.0 as an equity investment without acknowledging that prior interstate construction caused equity harm. Add one paragraph in the background section acknowledging the Rondo/Overtown/Brookside pattern, cite \citet{Foxx2017} or equivalent documentation, and note that I2.0 corridor alignment for Type 2 and proposed T1 corridors will require community impact assessment. This is a one-paragraph addition that substantially strengthens the paper's equity credibility.

**P1.4 — Contextualize \$292B against federal highway budgets.**
*(Transport policy)*
The combined corridor cost is not grounded in any planning horizon. Add: (a) comparison to IIJA road budget (\$110B/5yr); (b) a "Phase 1 priority" subset (top 4 corridors at \~\$116B) that fits within one IIJA authorization cycle; (c) explicit acknowledgment that the full list is a 30-year program, not a single appropriation.

### P2 — Important (address for quality)

**P2.1 — Add transit-dependent population as a secondary gap metric.**
*(Equity researcher)*
Table 2 should add a column for no-vehicle household share (ACS B08201) in gap counties. This enriches the equity analysis without changing the headline findings. The 30-mile standard matters most for those without cars; showing that gap counties also have higher no-vehicle shares strengthens the equity argument.

**P2.2 — Acknowledge induced demand / economic multiplier as a gap in the efficiency metric.**
*(Rural economist)*
Add a paragraph to the methods section noting that coverage efficiency does not capture: induced freight activity, labor market integration effects, or agricultural supply chain efficiency gains. Cite Michaels (2008) and Chandra (2000) as the relevant identification literature. Frame the efficiency metric as a lower bound on total social value.

**P2.3 — Add road-network distance spot-check for 5 representative gap counties.**
*(Transport geographer, Network scientist)*
For 5 counties in mountainous or terrain-constrained zones (e.g., one Appalachian, one Montana, one Nevada, one Maine, one Upper Peninsula Michigan), compare Haversine distance to estimated road-network distance. Present as a supplementary table. This demonstrates the paper's honest handling of the Euclidean approximation limitation.

**P2.4 — Strengthen the Rural West / Type 3 treatment.**
*(Transport geographer, Equity researcher)*
Section 5.4 dismisses the Rural West too quickly. Distinguish between: (a) centroid-artifact counties (where a tract-level correction would move them inside the 30-mile threshold) and (b) genuine isolation counties (where even tract-level analysis shows communities >30mi from any interstate). The genuine isolation counties in Nevada, Montana, and Idaho deserve a more specific investment recommendation than "rural access spurs."

### P3 — Enhancements (for a stronger paper)

**P3.1 — Add a feasibility-adjusted corridor ranking.**
*(Transport policy)*
Supplement Table 4 with a column for "political/regulatory feasibility" (1–5 scale based on: state boundary count, existing partial build, terrain difficulty, environmental sensitivity). Compute a feasibility-adjusted efficiency metric. This gives practitioners a near-term priority list alongside the optimal long-term ranking.

**P3.2 — Quantify corridor service area overlap.**
*(Network scientist)*
Section 7.1 notes that combined corridors serve 11.1M vs. the sum of individual estimates, but doesn't quantify overlap. Add a supplementary table showing which corridor pairs have significant service area overlap (>20% of counties served by both). This informs phasing: build corridors with low overlap first for maximum coverage-per-dollar.

**P3.3 — Add air quality co-benefits for highway-adjacent communities.**
*(Equity researcher)*
If I2.0 managed freight lanes shift trucks from GP lanes to express lanes (and ultimately to electric trucks), the communities adjacent to T1 corridors gain air quality benefits. A brief mention of this co-benefit — with reference to FHWA air quality methodology — would complete the equity framing.

---

## Score Summary

| Reviewer | Score | Primary concern |
|---|---|---|
| Transport geographer | 3/4 | County centroid artifact must be quantified |
| Rural economist | 3/4 | Causal claim for C3 correlation needs care |
| Transport policy | 3/4 | Political feasibility and budget context missing |
| Network scientist | 2/4 | Euclidean vs. road distance; 1,465 node count |
| Equity researcher | 2/4 | Construction-era harm acknowledgment; transit-dependent population |
| **Panel mean** | **2.6/4** | — |
| **Panel minimum** | **2/4** | — |

Mean 2.6/4 is above the 2.5 threshold for revision advancement. Minimum 2/4 meets the floor. The paper advances to revision.

---

## Stage Advancement

**draft → revision**

Required before recheck:
- [ ] P1.1 Quantify centroid artifact (new analysis)
- [ ] P1.2 Verify 1,465 node count
- [ ] P1.3 Equity paragraph (construction-era harms)
- [ ] P1.4 Budget contextualization

Recommended before recheck:
- [ ] P2.1 No-vehicle household column in Table 2
- [ ] P2.2 Economic multiplier acknowledgment
- [ ] P2.3 Road-network distance spot-check
- [ ] P2.4 Strengthen Rural West treatment
