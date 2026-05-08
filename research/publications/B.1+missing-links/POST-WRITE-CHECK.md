---
name: POST-WRITE-CHECK — B.1 Missing Links: Gap Analysis of the US Interstate Network
slug: b1-missing-links-post-write
type: review
status: draft
author: research-post-write
created: 2026-05-08
updated: 2026-05-08
---

# POST-WRITE-CHECK: B.1 — Missing Links: Gap Analysis of the US Interstate Network

## PHASE 1 — PAPER SUMMARY

```
Paper: B.1+missing-links
Sections found: 01-introduction.tex, 02-background.tex, 03-methods.tex,
                04-thirty-mile-standard.tex, 05-gap-taxonomy.tex,
                06-priority-corridors.tex, 07-validation.tex, 08-conclusion.tex
Plan found: YES (plan.md)
Track: B — Gap Analysis
Venue: Transportation Research Part A
Key claims:
  1. 66.5M Americans (20.4%) in counties >30 miles from any interstate on-ramp; corrected
     estimate 48.8M (14.8%) after removing county-centroid artifacts (§04, abstract)
  2. Four geographic gap zones; 1,510 gap counties; 12 priority corridors close most
     of the coverage deficit (§05, §06)
  3. Gap counties score 41% higher on C3 (Economic Opportunity Access) than non-gap
     counties — coverage gap and economic opportunity gap are co-located (§04, §08)
Primary number (from MODULE.md contract): K missing links with gap score ≥7.5;
  avg nearest-interstate X miles
Paper's stated primary number: 66.5M Americans / 48.8M corrected — coverage gap
  headcount; 12 priority corridors ranked by coverage efficiency
Match: PARTIAL — the MODULE.md contract asks for corridors scoring ≥7.5 (ROUTE rubric
  score); the paper ranks corridors by coverage efficiency (gap population/$B) rather
  than rubric score. No ROUTE composite scores are given for proposed corridors; the
  corridors are evaluated by coverage metric, not by "gap score ≥7.5" threshold.
```

---

## PHASE 2 — QUANTITATIVE CONSISTENCY CHECK

| Q-ID | Quantity | Abstract | §Intro | Table/Body | §Conclusion | Consistent? |
|------|----------|---------|--------|-----------|-------------|-------------|
| Q-01 | Gap population (raw) | 66.5M/20.4% | 66.5M/20.4% (§01) | 66,513,310/20.4% (Tab §04) | 66.5M (§08) | PASS |
| Q-02 | Gap population (corrected) | 48.8M/14.8% | NOT mentioned | 48,848,099/14.8% (Tab §04) | NOT mentioned | **FAIL: corrected figure present in abstract and §04 table but absent from §01 intro and §08 conclusion** |
| Q-03 | Total gap counties | 1,510 | 1,510 | 1,510 | 1,510 (§08) | PASS |
| Q-04 | Total continental counties | 3,105 | — | 3,105 (§03) | — | PASS |
| Q-05 | Within-30-mile coverage % | 79.6% | — | 79.6% (Tab §04) | 79.6% (§08) | PASS |
| Q-06 | Within-30-mile population | — | — | 262,331,636 (Tab §04) | — | PASS (262.3M / 330M = 79.5% ≈ 79.6%) |
| Q-07 | C3 score gap counties | — | — | 5.8 (§04, §07) | — | PASS |
| Q-08 | C3 score non-gap counties | — | — | 4.1 (§04, §07) | — | PASS |
| Q-09 | C3 differential percentage | — | "40% higher" (§01) | "41% difference" (§04) | "41% higher" (§08) | **WARN: §01 says 40%; §04 says 41%; §08 says 41%. The arithmetic: (5.8-4.1)/4.1 = 41.5%, rounds to 41-42%. Abstract says "41% higher" ✓. §01 says 40% — inconsistent** |
| Q-10 | Northern Tier gap counties | — | — | 44+44+50+43=181 (§05) | — | PASS (Tab §05 total says ~181) |
| Q-11 | Northern Tier gap population | — | — | 502k+325k+1.28M+1.39M=3.5M (§05 text) vs Tab §05 "~3,499,000" | — | PASS (sums match) |
| Q-12 | Appalachian gap counties | — | — | 26+66+30+42=164 (§05) | — | PASS (Tab §05 says ~164) |
| Q-13 | Rural Gulf South gap population | — | — | 2.63M+0.75M+1.12M+3.46M=7.96M vs Tab §05 "~7,959,000" | — | PASS (within rounding) |
| Q-14 | Northern Tier corridor pop served | — | — | 3,200,000 (§06 text, §07 Tab) | — | PASS (consistent) |
| Q-15 | Northern Tier corridor cost | — | — | $75B (§06, §07 Tab) | — | PASS |
| Q-16 | Northern Tier efficiency | — | — | 43K/\$B (§06 Tab) | — | WARN: 3,200,000/$75B = 42,667 ≈ 43K/\$B ✓ — rounds correctly but check Table column says "43K/\$B" |
| Q-17 | I-3 coverage efficiency | — | — | 46K/\$B (§06 Tab) | — | PASS: 2,180,000/$47B = 46,383 ≈ 46K/\$B ✓ |
| Q-18 | Total 12-corridor cost | — | — | $292B (§06 Tab total) | — | WARN: Sum from table: 47+75+26+18+21+13+17+5+23+8+13+26=292 ✓ — but "Phase 1" §06 sums I-69($18)+I-14($26)+Appalachian($17)+I-57($5)+I-87($8)+Oregon($13)=$87B, described as "roughly one IIJA authorization cycle" but text says "Phase 1 ($71B)" — $87B ≠ $71B — **FAIL** |
| Q-19 | Coverage after 12 corridors | — | — | 83.0% (§07 Tab) | 83% (§08) | PASS |
| Q-20 | Gap population served by 12 corridors | — | — | 11,070,000 (§07 Tab) | 11.1M (§08) | PASS |
| Q-21 | Starting coverage % | — | — | 79.6% (§07 text) | 79.6% (§08) | PASS |
| Q-22 | 30-mile standard: 35 min rural driving | — | §01: "approximately 35 minutes" | §03: cited from 3 sources | — | PASS |
| Q-23 | Arizona gap counties centroid artifact | — | — | §04: Pima County 189 miles (described) | — | PASS |
| Q-24 | I2.0 coverage target 99% by 2040 | abstract | §01 not cited | §04: mentioned | §08 (future work) | WARN: Abstract prominently cites "99% national coverage by 2040" — this is a planning aspiration, not a paper result. Should be flagged as "I2.0 program target" not a paper finding. |
| Q-25 | Phase 1 investment total | — | — | §06 text: "$71B, Years 1-5" | — | **FAIL: §06 body describes Phase 1 as $71B but then lists corridors summing to $87B in the same paragraph** |
| Q-26 | Map figure reference | abstract | — | §04: fig:gap-map (includegraphics) | — | WARN: figure file path referenced but no figures/ directory found; figure asset status unknown |
| Q-27 | Rubric version for proposed corridor scores | — | — | §03: "scores marked estimated" — no version stated | — | **FAIL: proposed corridor scores not tagged with rubric version** |

**CONSISTENCY: 3 FAILURES, 5 WARNINGS**

```
P1 (must fix):
  [I-01] §01 intro says C3 differential is "40% higher" but §04 and §08 correctly say 41%.
         Fix: Change §01 "40%" to "41%" for consistency with the calculated value.

  [I-02] §08 conclusion does not mention the corrected 48.8M/14.8% figure — it cites only
         the raw 66.5M/20.4% as "the gap." The corrected figure is the paper's headline
         (abstract leads with it). A referee reading §08 first will not know about the
         correction. Fix: Add to §08 conclusion: "The corrected estimate, after removing
         county-centroid artifacts, is 48.8 million people (14.8%) — the conservative
         headline the policy analysis should use."

  [I-03] §06 Phase 1 budget: text says "$71B, Years 1–5" but then lists corridors whose
         costs sum to $87B in the same paragraph. Either the $71B is wrong (should be $87B)
         or the corridor list for Phase 1 is wrong.
         Fix: Recheck Phase 1 corridor assignments and costs; ensure the stated total
         ($71B or corrected figure) matches the sum of listed Phase 1 corridors.

P2 (should fix):
  [I-04] §03 proposed corridor scores: "scores marked estimated" — add explicit rubric
         version tag (e.g., "estimated v1.2 scores"). MODULE.md contract requires gap
         scores ≥7.5 using the calibrated rubric — without version tags, the scores are
         ambiguous. The contract is also not fully delivered: actual ROUTE composite scores
         for proposed corridors are not shown in any table.

  [I-05] §01 intro: the "66.5 million" figure should be introduced with a parenthetical
         noting the correction on first use: "66.5 million (raw; 48.8 million after
         centroid correction)" — so readers who read sequentially encounter the nuance.

  [I-06] §04 I2.0 coverage target "99% by 2040": add explicit framing as "I2.0 program
         target" not a paper result. The paper shows 12 corridors get to 83%; the gap
         to 99% requires rural access spurs and ongoing investment not analyzed here.

  [I-07] Figure placeholder for gap-counties-map.pdf: confirm figures/ directory exists
         or note as supplementary. Missing figure referenced in §04.

P3 items (optional polish):
  - §02 Background equity section: strong and appropriately placed. Consider whether
    it warrants a subsection (it currently runs long for a background subsection).
  - §07 FHWA Future Interstate Study validation: useful cross-reference but the 2000
    study projected 2020 traffic — 6-year-old projections now. Acknowledge staleness.
  - §06 rural access spurs cost ($100–500M): appropriately scoped but no data source
    for the $1–5M per spur estimate. Add a citation or note it is a planning estimate.
```

---

## PHASE 3 — CONTRACT CHECK

| Promise (plan.md + MODULE.md) | Paper section | Delivered? | Gap |
|-------------------------------|---------------|-----------|-----|
| Primary number: 66.5M gap population (plan.md) | Abstract, §04, §08 | YES (66.5M raw; 48.8M corrected) | ✓ |
| K missing links with gap score ≥7.5 (MODULE.md) | §06 | NO — corridors not scored on ROUTE rubric composite; coverage efficiency used instead | ✗ |
| Population-weighted county coverage analysis | §03, §04 | YES | ✓ |
| Four gap types classified | §05 | YES | ✓ |
| 12 priority corridors ranked by coverage efficiency | §06, §07 Tab | YES | ✓ |
| C3 correlation check | §04, §07.3 | YES (41% differential, Mann-Whitney U) | ✓ |
| Sensitivity to cost assumptions | §07.4 | YES (3 scenarios) | ✓ |
| Coverage simulation (re-run after adding corridors) | §07.1 | YES (Tab §07) | ✓ |

```
CONTRACT: PARTIAL
Promises kept: 6/8
Gaps:
  1. MODULE.md contract requires corridors scored against "calibrated ROUTE rubric" with
     gap scores ≥7.5. No ROUTE composite scores appear for proposed corridors. Coverage
     efficiency alone is used for ranking. Add estimated ROUTE composite scores for the
     top-12 proposed corridors (even if marked estimated) to satisfy the MODULE.md contract.
  2. Phase 1 budget arithmetic inconsistency ($71B stated, $87B listed) — this is a
     contract deliverable: the phased investment plan.
MODULE.md primary number delivered: PARTIAL — 66.5M/48.8M coverage gap delivered;
  ROUTE composite scores for proposed corridors missing
```

---

## PHASE 4 — REFEREE SIMULATION

**Selected referees**: R-Equity (urban displacement / rural access), R-Economics (coverage efficiency methodology), R-Network (graph algorithm and centroid methodology)

---

```
REFEREE 1 — R-Equity (Transportation Research Part A / Journal of Transport Geography)
Recommendation: Accept with Minor Revision

SUMMARY: This is a rare paper that both identifies a coverage gap AND takes equity
harm seriously. The construction-era equity harms section (§02) is unusually strong for
a transportation geography paper. The recommendation for community impact assessment
on proposed corridors is appropriate. The main equity concern is that the coverage
efficiency ranking implicitly prioritizes corridors by cost-effectiveness rather than
need — which may under-prioritize the poorest and most isolated communities.

MAJOR CONCERNS:
[I-15] Coverage efficiency (gap population/$B) is a utilitarian metric that rewards
       high-density gap zones over sparse ones. The Rural West (Zone 4) scores low on
       efficiency because its counties have few people — but those communities may have
       the greatest relative need (no alternative access, extreme isolation). Consider
       an equity-weighted efficiency metric that gives higher weight to low-income/zero-
       alternative counties. Currently, the C3 correlation is shown as a post-hoc check
       (§07) rather than an input to the ranking.

MINOR CONCERNS:
- §08 conclusion lists economic arguments for priority corridors but does not return
  to the equity harm acknowledgment in §02. A brief closing note connecting the
  proposed corridors to affected communities would complete the framing.
- The California centroid artifact discussion (§05 Zone 4) is correct but should
  note that Los Angeles County itself contains communities with genuine access deficits
  (Antelope Valley, eastern San Bernardino) despite the county-level centroid artifact.
```

---

```
REFEREE 2 — R-Economics (Journal of Economic Perspectives archetype)
Recommendation: Major Revision

SUMMARY: The coverage analysis methodology is sound. The economic opportunity
correlation is the paper's strongest contribution. However, the causal claim is too
strong: "the investment that closes the coverage gap also connects below-average
economic regions to the national economy" conflates correlation with causation.
The research design cannot establish that building these corridors would increase
economic opportunity; it only shows that gap counties are currently economically
disadvantaged.

MAJOR CONCERNS:
[I-16] §08 Conclusion: "The transportation case and the economic case point to the same
       corridors" conflates correlation with causal investment justification. The
       literature review cites Michaels (2008), Chandra (2000), and Giroud (2013) —
       all of which identify causal effects of highway access on economic outcomes. But
       these effects were estimated for the 1950s–1990s interstate system; whether
       similar effects would obtain from new rural corridors in the current economy is
       not established. Qualify: "consistent with the literature showing positive economic
       effects from highway access, though causal identification in this context would
       require a quasi-experimental design."
[I-17] Construction cost estimates ($30M/mi upgrade, $75M/mi greenfield) lack a
       primary citation. These figures have a wide range in practice (NEC core capacity
       program: $100M+/mi; simple rural 4-lane: $10–15M/mi). The efficiency rankings
       are sensitive to these estimates (§07.4 shows rank stability but within the ±50%
       range — beyond that range, rankings could shift). Cite the source for these
       per-mile figures or expand the sensitivity range.

MINOR CONCERNS:
- Northern Maine Interstate: $21B for 280 miles greenfield = $75M/mi which is stated
  as the greenfield rate. But the plan.md says greenfield rate is $75M/mi — this is
  consistent. However, $75M/mi is a recent-year urban-adjacent estimate; genuine
  rural Maine greenfield may be lower ($20–40M/mi). Consider a rural-greenfield rate.
```

---

```
REFEREE 3 — R-Network (Transportation Science / PNAS archetype)
Recommendation: Major Revision

SUMMARY: The coverage analysis methodology is rigorous and the DBSCAN clustering for
gap taxonomy is a sophisticated technique. The main methodological concern is the
county centroid approach, which the paper acknowledges but does not fully correct.
The correction heuristic (land area > 2,000 sq mi AND gap distance < 2.5×√(A/π)) is
novel but unpublished and not formally validated. A paper with this as a central
methodological step needs to validate the heuristic.

MAJOR CONCERNS:
[I-18] The centroid artifact correction heuristic (§04) identifies 136 artifact counties
       with 17.7M people. This correction drives the difference between the 66.5M
       headline and the 48.8M corrected figure — a 27% change in the primary result.
       The heuristic has not been validated: the paper does not show that counties
       flagged as artifacts actually have most of their population near an interstate
       (i.e., that the artifact diagnosis is correct). A spot-check table (e.g., 10
       flagged counties with actual population-weighted distance vs centroid distance)
       would substantially strengthen confidence in the correction.

[I-19] HighwayGraph interchange nodes (1,465 nodes): the paper notes this undercounts
       access points and "introduces a conservative bias of 5–10%." This bias estimate
       is not derived quantitatively — it is asserted. If the true coverage fraction is
       5–10% higher than reported, the primary finding (48.8M gap) could be 2.4–4.9M
       lower (49M × 5–10%). That would bring the corrected gap to 43–46M people — a
       meaningful difference for the policy case. Estimate this more rigorously.

MINOR CONCERNS:
- DBSCAN parameters (ε = 100 miles, min cluster size 5) should be validated — are
  these parameters sensitive? Different ε could merge the Appalachian and Gulf South
  clusters or split the Northern Tier.
```

---

## PHASE 5 — ABSTRACT CHECK

```
ABSTRACT: ~175 words
Primary result stated: YES — "corrected estimate of 48.8 million people (14.8%)" and
  "upper bound of 66.5 million (20.4%)" — both stated
Method named: YES — "Census Bureau county centroids, TIGER/Line highway geometry, and
  the ROUTE 12-dimensional corridor scoring framework"
Policy implication: YES — "Twelve priority corridors phased across three federal
  authorization cycles ($71B Phase 1, $60B Phase 2, $161B Phase 3)"
Track chain position: PARTIAL — B.1's relationship to A.1 (requires tier classification)
  is not mentioned; a reader unfamiliar with the ROUTE module won't understand the
  12-dimensional scoring context
Note: Abstract references $71B Phase 1 — this is the figure inconsistent with the
  $87B sum of listed Phase 1 corridors (P1 issue I-03). The abstract value must match
  the corrected figure once I-03 is resolved.
```

---

## PHASE 6 — INTERNAL CROSS-PAPER CONSISTENCY

```
CROSS-PAPER CONSISTENCY:
  Papers cited: A.1 (T1 classification), B.2 (forward ref), B.3 (forward ref),
                E.2 (forward ref), ROUTE_MODULE2026
  Values cross-checked: 5

  1. A.1 tier classification (8 T1 corridors): referenced correctly in §05 Zone 1
     "qualifying as T1 under the centrality-adjusted classification of ROUTE_A1" ✓
  2. ROUTE rubric "12-dimensional" referenced in abstract and §03 — but current rubric
     is v1.2 (15 dimensions). The abstract and methods reference "12-dimensional"
     which is v1.0/v1.1 framing. By the time B.1 was written, v1.2 had 15 dimensions.
     This is a stale rubric description. Fix: update to "15-dimensional (v1.2) rubric"
     or note that the analysis uses v1.1 scores (plan.md cites v1.1: "use v1.1 ROUTE
     scores for all 227 corridors").
  3. scores-all.csv: I-3 is listed in scores-all.csv as a T4 corridor (score=5.0) —
     this is the current I-3 (Boca Raton–Tampa), not the proposed I-3 Savannah–Detroit.
     The paper correctly distinguishes proposed vs existing I-3 but the score table
     confusion could arise.
  4. B.2 (forward ref): §08 says "Paper B.2 addresses bottlenecks" — consistent
  5. $22.7B from B.2: not cited in B.1 (appropriate — different topic)

  Stale rubric version: MEDIUM RISK — abstract/methods say "12-dimensional" when
    v1.2 has 15 dimensions. This needs correction for consistency with A.2.
```

---

## PHASE 7 — PRE-PANEL CHECKLIST

```
═══════════════════════════════════════════════════════════
POST-WRITE COMPLETE: B.1+missing-links
═══════════════════════════════════════════════════════════

Validation results:
  Consistency:       3 FAILURES, 5 WARNINGS
  Contract:          PARTIAL (6/8 — no ROUTE composite scores for proposed corridors;
                     Phase 1 budget arithmetic wrong)
  Referee sim:       Accept/Minor (R-Equity); Major Revision (R-Economics + R-Network)
  Abstract:          ~175 words, primary numbers stated (both raw and corrected)
  Cross-paper:       Rubric described as "12-dimensional" but v1.2 is 15-dimensional

P1 blockers (fix before panel review):
[I-01] §01 intro: "40% higher" → "41% higher" (matches §04 and §08)

[I-02] §08 conclusion: add the corrected 48.8M/14.8% figure. Currently §08 cites only
       the raw 66.5M, which leaves the panel without the paper's own preferred estimate.

[I-03] §06 Phase 1 budget: "$71B" stated but corridors listed sum to $87B.
       Either correct the dollar figure or correct the corridor list.
       Verify each Phase 1 corridor and restate the total.

P2 items (should fix):
[I-04] Abstract and methods: "12-dimensional corridor scoring framework" → update to
       "15-dimensional (v1.2) rubric" or clarify that analysis uses v1.1 scores (per plan.md)
       and add version tag on score citations

[I-05] §01 intro: introduce the 66.5M figure with a parenthetical "(48.8M corrected)"
       on first use so readers encounter the nuance early

[I-06] MODULE.md contract: add estimated ROUTE composite scores for proposed corridors
       to satisfy the "gap score ≥7.5" contract requirement

[I-07] §04: I2.0 99% coverage target — frame explicitly as "I2.0 program target" not
       a result of this paper's analysis

P3 items (optional polish):
  - §07 FHWA 2000 study: acknowledge the projections are now 20+ years old
  - §06 rural access spurs: add a citation for the $1–5M per spur estimate
  - §02 equity section: strong — consider promoting to its own section rather than
    being buried in Background subsection 4

PRE-PANEL CHECKLIST:
□ C3 differential: 41% used consistently (not 40%) in all sections
□ Corrected gap estimate (48.8M/14.8%) present in §08 conclusion
□ Phase 1 budget arithmetic resolved ($71B or corrected total)
□ MODULE.md primary quantitative contract: ROUTE composite scores for proposed corridors
□ Rubric version tagged: v1.1 (per plan.md) or v1.2 with caveats — not "12-dimensional"
□ BPR extrapolation caveat: not applicable (no BPR formula in this paper)
□ Net vs gross cost: phase investment totals clearly presented as capital costs only
□ All \citep{} keys: spot-checked; all appear to be labeled consistently
□ Cross-paper: rubric dimension count updated from 12 to 15 (v1.2)
□ Abstract states primary quantitative result (corrected 48.8M prominently)
□ Figure asset for gap-counties-map.pdf: confirm existence or note as supplementary
□ Referee P1 blockers addressed

VERDICT: FIXES REQUIRED
Fixes required: 3 P1, 4 P2
Next: run /panel:publication review B.1+missing-links after P1 fixes
═══════════════════════════════════════════════════════════
```
