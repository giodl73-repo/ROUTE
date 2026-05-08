---
name: POST-WRITE-CHECK — A.2 Rubric Calibration
slug: a2-rubric-calibration-post-write
type: review
status: draft
author: research-post-write
created: 2026-05-08
updated: 2026-05-08
---

# POST-WRITE-CHECK: A.2 — Rubric Calibration: Which 12 Dimensions Actually Differentiate Interstate Corridors

## PHASE 1 — PAPER SUMMARY

```
Paper: A.2+rubric-calibration
Sections found: 01-introduction.tex, 02-v10-baseline.tex, 03-v11-amendments.tex,
                04-v12-additions.tex, 05-before-after.tex, 06-calibration-method.tex,
                07-conclusion.tex
Plan found: NO (no plan.md in directory)
Track: A — Corpus & Scoring
Venue: Transportation Research Part A (implied by Track A)
Key claims:
  1. Three errors in v1.0/v1.1 — IRI proxy artificially maxed A3, B2 on partial graph,
     three strategic dimensions missing — caused I-110 to rank above I-80 (§01, §02)
  2. v1.2 adds three strategic dimensions (A4 USMCA trade, B4 military, C4 agricultural)
     that correct the T1 ranking inversion; I-80 now leads T1 (§04, §05)
  3. The 12→15 dimension expansion (v1.0→v1.2) expands max score from 120 to 150;
     all prior scores preserved under forward-only versioning protocol (§03, §06)
Primary number (from MODULE.md contract): ≤9 of 12 dimensions survive; ≥2 correlated pairs retired
Paper's stated primary number: Abstract says rubric evolved to 15 dimensions (added 3 new
  ones to original 12, reaching v1.2). No correlated pair retirements yet — those are deferred.
Match: PARTIAL — paper documents 12→15 expansion not 12→≤9 reduction. The MODULE.md
  contract anticipated dimension *reduction* (retirement of weak/correlated dims);
  the paper instead documents dimension *addition*. The retirement analysis is deferred to
  a future v1.3 calibration (noted in §07 conclusion). This is a contract gap that
  must be disclosed explicitly in the paper.
```

---

## PHASE 2 — QUANTITATIVE CONSISTENCY CHECK

| Q-ID | Quantity | Abstract | §Intro | Table/Body | §Conclusion | Consistent? |
|------|----------|---------|--------|-----------|-------------|-------------|
| Q-01 | v1.0 dimension count | 12 | 12 | 12 (§02, Tab) | 12 | PASS |
| Q-02 | v1.2 dimension count | 15 | 15 (implied: 12+3) | 15 (120→150 pt max) | 15 | PASS |
| Q-03 | I-110 v1.0 total score | 30.1 (abstract implies "above I-80") | — | 30.1/120 (Tab §02) | — | PASS |
| Q-04 | I-80 v1.0 total score | — | — | 25.2/120 (Tab §02) | — | PASS |
| Q-05 | I-110 v1.1 total score | — | — | 25.1/120 (Tab §03) | — | PASS |
| Q-06 | I-80 v1.1 total score | — | — | 20.2/120 (Tab §03) | — | PASS |
| Q-07 | I-110 v1.2 total score | — | — | 19.0/150 (Tab §04, §05) | — | PASS (matches scores-all.csv: I-110=19.0) |
| Q-08 | I-80 v1.2 total score | — | — | 38.7/150 (Tab §04, §05) | — | PASS (matches scores-all.csv: I-80=38.7) |
| Q-09 | I-35 A4 score (Laredo) | — | — | **8.5 in §04 text vs 10 cited in §05 table** | — | **FAIL** |
| Q-10 | I-80 net gain v1.1→v1.2 | — | — | **"18.5 points" in §05 text vs B4=6.5+C4=7.5=14.0** | — | **FAIL** |
| Q-11 | I-80 A4 score | — | — | 0.0 (Tab §04); text confirms "0.0 on A4" | — | PASS |
| Q-12 | I-80 B4 score | — | — | 6.5 (Tab §04) | — | PASS |
| Q-13 | I-80 C4 score | — | — | 7.5 (Tab §04) | — | PASS |
| Q-14 | I-80 v1.2 gap vs I-110 | — | — | "+19.7 points" (§04 and §05) | — | **WARN: 38.7−19.0=19.7 ✓ consistent, but derivation contradicts** |
| Q-15 | US-287 v1.2 score | — | — | 35.5/150 (§05) | — | PASS (not in scores-all.csv to verify) |
| Q-16 | I-35 C4 score | — | — | 8.0 in §04 text; "10" in §05 table footnote "A4=10" | — | **FAIL** |
| Q-17 | Laredo annual trade value | $277B | — | $277B (§04 A4, §04 conclusion) | — | PASS |
| Q-18 | Laredo daily commercial vehicles | 16,000/day | — | 16,000/day (§04) | — | PASS |
| Q-19 | A3/A1 Pearson r (correlation paradox) | — | — | r=0.71 (§06) | — | PASS (no cross-section check available) |
| Q-20 | A4/A2 Pearson r (independence) | — | — | r=0.22 (§06 table) | — | PASS |
| Q-21 | STRAHNET rank correlation (§06) | — | — | ρ=0.81 (§06 external validation) | — | WARN: A.1 validation reports ρ=0.72 for ATRI, not for STRAHNET; A.2 §06 says ρ=0.81 for STRAHNET — different metric, should cite explicitly which metric |
| Q-22 | ATRI bottleneck density ρ (§06) | — | — | ρ=0.72 (§06) | — | PASS (consistent with A.1 §05.2 value) |
| Q-23 | Transportation plan frequency (§06) | — | — | 50 state LRTPs, 47/50 = 94% (§06) | — | WARN: A.1 cites 12 LRTPs/94%; A.2 §06 cites 50 LRTPs/94%; same percentage with different sample size — suspicious coincidence; verify or state these are different reviews |
| Q-24 | v1.1 tier thresholds (/120) | — | — | T1≥21, T2≥15, T3≥9 (§04 table) | — | WARN: §02 says natural breaks at 26.0, 20.0, 14.0 for v1.0; §04 says v1.1 thresholds are ≥21, ≥15, ≥9 — these are different thresholds; the change is not explained |
| Q-25 | v1.2 tier thresholds (/150) | — | — | T1≥26, T2≥19, T3≥11 (§04 table) | — | PASS (proportionally scaled from v1.1) |
| Q-26 | B2 partial-graph caveat | — | §01 intro: "partial graph" explicitly mentioned | §03: "partial directed graph, 31 states" | §07: deferred to v1.3 | PASS — caveat present throughout |
| Q-27 | Rubric version on scores | — | — | §06: scores-all.csv carries rubric_version column | — | WARN: scores-all.csv in actual data/ directory has NO rubric_version column — only route, score, tier. The §06 description of the multi-version schema is not reflected in the actual data file |

**CONSISTENCY: 3 FAILURES, 6 WARNINGS**

### Critical Arithmetic Failure — Q-10: I-80 Net Gain Calculation

§05 text states: "I-80 gained 18.5 points: B4=6.5 (Cheyenne/FE Warren proximity) and C4=7.5 (Corn Belt, dual-coast export access)."

Arithmetic: B4=6.5 + C4=7.5 = **14.0**, not 18.5. The stated gain is wrong by 4.5 points.

The v1.1→v1.2 actual transition: v1.1 I-80=20.2, v1.2 I-80=38.7. Difference = 18.5.
So the total gain IS 18.5 — but B4+C4 only accounts for 14.0. The missing 4.5 points must come from somewhere.

Possible explanation: every interstate gets B4≥5.0 (STRAHNET baseline). I-80 had some B4 credit under v1.1 already (the STRAHNET baseline 5.0 should have been included in v1.1 scoring). If B4 is entirely new in v1.2 (no STRAHNET credit in v1.1), then the gain from B4 is 6.5 (not 6.5-0=6.5). But C4 is also new (0 in v1.1). That gives 6.5+7.5=14.0. Still 4.5 short.

The 18.5 gain is confirmed by table arithmetic (38.7−20.2=18.5). The cited components (B4=6.5, C4=7.5) sum to 14.0. There is an unaccounted 4.5 points. The §06 also mentions A4, B4, C4 are new — if B4 baseline was NOT in v1.1, then +5.0 (baseline) + additional B4 above baseline (1.5) + C4(7.5) = 14.0 still. Alternatively, if some dimension OTHER than the three new ones also changed (e.g., B1, B2 recomputed), this is not disclosed.

**This is a P1 arithmetic failure that must be resolved before panel.**

### Critical Failure — Q-09/Q-16: I-35 A4 Score Inconsistency

- §04 A4 section: "I-35 (Laredo TX): A4 = 8.5."
- §05 before-after table: I-35 row reads "A4=10 + grain C4=10" in the Key change column.
- §05 text: "v1.2: I-80 = 38.7/150 (T1 #1), I-110 = 19.0/150 (T2)" — this matches scores-all.csv.
- But I-35 v1.2 score = 33.8 (from Tab §05 and scores-all.csv). If A4=8.5 and C4=8.0 (from §04), the I-35 gain from v1.1 is 8.5+8.0+5.0(B4 baseline) = 21.5. I-35 v1.1 = 12.2, so v1.2 would be 12.2+21.5 = 33.7 ≈ 33.8 ✓ (rounding). This confirms A4=8.5 is correct.
- The §05 table's "A4=10" in the key-change column is wrong. Should be "A4=8.5".

```
P1 (must fix):
  [I-01] §05 text: "I-80 gained 18.5 points: B4=6.5 and C4=7.5" — but 6.5+7.5=14.0, not 18.5
         The 4.5 point gap is unexplained. Either:
         (a) A third component is missing from the explanation — identify and add it, or
         (b) The gain was actually 14.0 and the v1.1 I-80 score should be 24.7 (not 20.2)
         Resolve by checking dimension-by-dimension v1.1 vs v1.2 for I-80 and showing
         all components summing to 18.5.
         Fix: Correct the component list or the gain figure. Document all sources of gain.

  [I-02] §05 before-after table, I-35 row: "A4=10" in key-change column
         → Change to "A4=8.5" to match §04's A4 section and the arithmetic (33.8−12.2=21.6;
         8.5+8.0+5.0(B4)=21.5 ≈ 21.6 ✓)

  [I-03] §05 conclusion subsection bullet: "A4+B4+C4: I-80 gained 18.5 points from
         USMCA=9, military=8.5, agricultural=9.0" — this list shows A4=9, B4=8.5, C4=9.0
         but §04 shows I-80: A4=0.0 (not a border corridor), B4=6.5, C4=7.5. The values
         cited in §05 conclusion are all wrong for I-80.
         Fix: Correct to "I-80 gained [X] points: A4=0 (no border crossing), B4=6.5
         (FE Warren proximity), C4=7.5 (Corn Belt/dual-coast export)"
         AND reconcile why total gain ≠ 14.0 (see I-01 above).

P2 (should fix):
  [I-04] No plan.md — add a plan.md documenting the MODULE.md contract gap: paper delivers
         12→15 dimension expansion (addition) rather than the contracted 12→≤9 reduction
         (retirement). The retirement analysis is deferred; the paper should acknowledge
         this in §07 conclusion explicitly as a known gap vs the module contract.

  [I-05] §03 v1.1 tier thresholds (≥21, ≥15, ≥9) differ from §02 v1.0 natural break
         thresholds (26.0, 20.0, 14.0) with no explanation of the change. Add a sentence
         explaining that v1.1 thresholds were rescaled because the IRI cap reduced
         aggregate scores across all corridors, shifting the natural break positions.

  [I-06] §06: states scores-all.csv has rubric_version column. Actual file does not —
         only route, score, tier. Either: (a) the schema description is aspirational
         (document it as "planned schema" and note current file is single-version),
         or (b) update scores-all.csv to include the version column.

  [I-07] §06 STRAHNET ρ=0.81: This is a different metric than A.1's ρ=0.72 (ATRI).
         Both papers cite "external validation via STRAHNET" but use different ρ statistics.
         Clarify in §06 what exactly ρ=0.81 measures (rank correlation between composite
         score and STRAHNET designation binary? or STRAHNET segment-mile density?).

  [I-08] §06: 50 LRTPs with 47/50=94% citing T1 corridors as highest-priority —
         this is suspiciously identical to A.1's percentage (94%) from only 12 LRTPs.
         Verify these are different reviews or acknowledge they are the same data.

P3 (minor):
  - §07 conclusion mentions three v1.3 candidates (A3, B2, C4) — add that the MODULE.md
    contract's ≥2 correlated pair retirements are the primary objective for v1.3
  - Forward-only protocol is explained twice (§03 and §07) — consider consolidating
  - The §04 note "*T1 by strategic override" for I-95 and I-75 (scoring below v1.2
    threshold but held T1) is a significant methodology decision that should be in §04
    as an explicit rule, not just a table footnote
```

---

## PHASE 3 — CONTRACT CHECK

| Promise (MODULE.md contract) | Paper section | Delivered? | Gap |
|------------------------------|---------------|-----------|-----|
| ≤9 of 12 dimensions survive | §07 conclusion | NO — dimensions expanded 12→15, not reduced | ✗ CONTRACT MISS |
| ≥2 correlated pairs retired | §06 calibration | NO — correlated pairs identified (A1/A3 r=0.71) but NOT retired; deferred to v1.3 | ✗ CONTRACT MISS |
| Variance + correlation analysis at N≥20 | §06 calibration method | YES — both tests documented | ✓ |
| Document v1.0→v1.1→v1.2 evolution | All sections | YES — full evolution documented | ✓ |
| Before-after comparison for key corridors | §05 Tab | YES — all 8 T1 + demoted corridors | ✓ |
| Identify which dimensions show high variance | §06 D3 example | PARTIAL — D3 flagged; no full variance table | PARTIAL |
| Forward-only versioning protocol | §03, §06 | YES | ✓ |

```
CONTRACT: PARTIAL/FAIL on primary contract numbers
Promises kept: 4/7
Contract misses:
  1. MODULE.md requires ≤9 dimensions survive. Paper adds 3 dimensions (12→15), netting
     more not fewer. This is a meaningful contract deviation that should be disclosed:
     "The v1.2 calibration expanded rather than contracted the rubric because the IRI
     and centrality fixes required adding strategic dimensions before retiring weak ones.
     The ≤9 target applies to the v1.3 calibration, after a full correlation and variance
     pass on the 15-dimension corpus."
  2. MODULE.md requires ≥2 correlated pairs retired. The paper identifies A1/A3 correlation
     (r=0.71) as the primary candidate but does not retire any pairs. Add explicit
     statement: "No pairs retired in this calibration cycle; A1/A3 is the leading candidate
     for v1.3 review pending real PTI data."
MODULE.md primary number delivered: NO — key contracts (≤9 dimensions, ≥2 retirements) not delivered; expansion documented instead
```

---

## PHASE 4 — REFEREE SIMULATION

**Selected referees**: R-Traffic (rubric calibration method), R-Economics (causal validity), R-Policy (planning implications)

---

```
REFEREE 1 — R-Traffic (Transportation Research Part A archetype)
Recommendation: Major Revision

SUMMARY: The calibration narrative is clearly written and the paradox diagnosis is
compelling. The IRI proxy problem is well-documented. However, the paper is internally
inconsistent on key quantitative claims (I-80 gain calculation, I-35 A4 score), and
the MODULE.md contract (dimension reduction) is not delivered. The external validation
section is stronger than most calibration papers in this space.

MAJOR CONCERNS:
[I-09] The I-80 gain calculation (§05) is arithmetically inconsistent: stated as 18.5
       but the cited components (B4=6.5, C4=7.5) sum to 14.0. This error undermines
       confidence in the paper's quantitative precision. Must be corrected with a full
       dimension-by-dimension reconciliation showing the complete v1.1→v1.2 delta.
[I-10] B2 scores are described as "estimated" throughout (partial graph) but the paper
       uses B2 to drive tier classification in A.1 and as a validation signal in A.2.
       If B2 is unstable, the validation results in §06 (ρ=0.81 STRAHNET) are also
       dependent on unstable input data. The paper should quantify B2 uncertainty bounds.
[I-11] Strategic override (I-95, I-75 held at T1 despite scoring below threshold) is
       applied without a formal rule. In any rubric paper, ad hoc overrides are a major
       red flag. Either define the override rule explicitly (e.g., "any corridor with
       betweenness centrality in top-8 and STRAHNET designation is T1 regardless of
       composite score") or remove the override and let the scores speak.

MINOR CONCERNS:
- §04 C4 scores are hand-curated, not data-driven. This must be disclosed clearly in
  the methods section, not buried in a subsection note.
```

---

```
REFEREE 2 — R-Economics (Journal of Economic Perspectives archetype)
Recommendation: Major Revision

SUMMARY: The rubric calibration is an interesting exercise in empirical instrument
refinement. The core concern is that the calibration process is inductive (corpus-driven)
without a formal bias correction for the analyst's priors. The corridor chosen to
illustrate the paradox (I-110 vs I-80) is not a randomly selected comparison; it is
the comparison that motivated the calibration. This creates a risk of overfitting the
rubric to a predetermined conclusion.

MAJOR CONCERNS:
[I-12] Selection bias: the calibration amendments were triggered by observing that I-110
       ranked above I-80 in v1.0. The fix (adding USMCA/military/agricultural dimensions)
       was designed to reverse this specific ranking. This is scientifically defensible
       only if the new dimensions are validated against a held-out set of corridors not
       used to motivate the amendment. The paper should show that the v1.2 dimensions
       correctly classify corridors BEYOND the I-110/I-80 pair — specifically, corridors
       that the analyst did not use as motivation.
[I-13] The A4 (International Trade) anchor score "5.0 if designated [USMCA]" introduces
       a binary signal as a continuous dimension. This is methodologically problematic:
       the USMCA designation is a categorical attribute, not a continuous variable. Using
       it as the anchor point for a 0–10 scale that awards additional points for crossing
       volume creates a semi-continuous dimension. Justify this or use a pure continuous
       measure (crossing AADT alone).

MINOR CONCERNS:
- Sensitivity analysis for the B4 (military) dimension: FE Warren AFB is directly on
  US-287 but only "adjacent to" I-80; the difference in B4 scores (9.0 vs 6.5) should
  be validated against a systematic rule, not just a judgment call about proximity.
```

---

```
REFEREE 3 — R-Policy (Transport Policy archetype)
Recommendation: Accept with Minor Revision

SUMMARY: The forward-only versioning protocol is the paper's strongest methodological
contribution and should be highlighted as such. The before-after comparison table is
compelling. The contract misses (no dimensions retired) are acceptable given the paper's
clear statement that retirement is deferred; the issue is whether to frame this as a
completed calibration or a partial calibration.

MAJOR CONCERNS:
[I-14] §07 Conclusion: the paper concludes that the rubric "places these corridors where
       they belong" after adding strategic dimensions, but does not acknowledge that the
       MODULE.md primary contract (≤9 dimensions, ≥2 retirements) was not delivered.
       This framing overstates the completeness of the calibration. Add an explicit
       paragraph: "The v1.2 calibration is partial: it corrects the most consequential
       errors (IRI proxy, strategic dimension absence) but defers the contraction phase
       to v1.3. Papers downstream of A.2 should note that the rubric remains at 15
       dimensions pending the v1.3 retirement pass."

MINOR CONCERNS:
- Forward reference to FHWA Freight Performance Measures (when available) as the solution
  for A3 is clear and appropriate.
```

---

## PHASE 5 — ABSTRACT CHECK

```
ABSTRACT: ~148 words
Primary result stated: YES — "I-110 (22-mile Pasadena connector) above I-80 (northern
  transcontinental) in the T1 ranking" is the paradox; "Adding three strategic dimensions
  resolves this misclassification" is the finding
Method named: YES ("rubric calibration process: what the corpus revealed, how each
  amendment was triggered")
Policy implication: PARTIAL — "source record for the rubric's current state and its known
  limitations" is archival framing, not a direct policy implication. Consider adding:
  "The corrected classification aligns with 94% of state transportation plans' priority
  corridor designations."
Track chain position: YES — "serves as the source record" positions it correctly in
  the Track A chain
```

Note: Abstract does not mention the MODULE.md contract misses (no retirements). This should be acknowledged so panel reviewers understand the paper's scope.

---

## PHASE 6 — INTERNAL CROSS-PAPER CONSISTENCY

```
CROSS-PAPER CONSISTENCY:
  Papers cited: A.1 (explicit back-reference), ROUTE corpus (scores-all.csv)
  Values cross-checked: 6

  1. I-80 v1.2 score: 38.7 — A.2 Tab §05 ✓; scores-all.csv ✓ — PASS
  2. I-110 v1.2 score: 19.0 — A.2 Tab §05 ✓; scores-all.csv: 19.0 ✓ — PASS
  3. I-35 v1.2 score: 33.8 — A.2 Tab §05 ✓; scores-all.csv: 33.8 ✓ — PASS
  4. I-35 A4 score: §04 says 8.5; §05 table key-change says A4=10 — FAIL (I-02 above)
  5. I-80 A4 score: §04 says 0.0; §05 conclusion says "USMCA=9" — FAIL (I-03 above)
  6. scores-all.csv rubric_version column: absent in actual file, present in §06 description — FAIL (I-06 above)

  Cross-paper consistency with A.1:
  - A.1 §05.3 cites 12 LRTPs / 94% frequency
  - A.2 §06 cites 50 state LRTPs / 94% (47/50) frequency
  - Same percentage, different sample — suspicious coincidence
  - A.1's ATRI ρ=0.72: confirmed in A.2 §06 ✓

  Stale pre-correction values: NONE — no citations to B.3 or other papers with known corrections
```

---

## PHASE 7 — PRE-PANEL CHECKLIST

```
═══════════════════════════════════════════════════════════
POST-WRITE COMPLETE: A.2+rubric-calibration
═══════════════════════════════════════════════════════════

Validation results:
  Consistency:       3 FAILURES, 6 WARNINGS
  Contract:          PARTIAL/FAIL — 4/7 (no dimension retirements, no ≤9 target delivered)
  Referee sim:       Major Revision (R-Traffic + R-Economics); Accept/Minor (R-Policy)
  Abstract:          ~148 words, primary paradox stated, module contract gap not disclosed
  Cross-paper:       3 data inconsistencies (I-35 A4, I-80 A4, scores-all.csv schema)

P1 blockers (fix before panel review):
[I-01] §05 text: I-80 gain stated as "18.5 points" with components "B4=6.5 + C4=7.5 = 14.0"
       → Identify the missing 4.5 points and list ALL components of the gain correctly,
         or correct the arithmetic statement. This is the paper's most critical error.

[I-02] §05 before-after table, I-35 key-change: "A4=10" → correct to "A4=8.5"
       (A4=8.5 is stated in §04 A4 section and is consistent with I-35's total score)

[I-03] §05 conclusion subsection: "I-80 gained 18.5 points from USMCA=9, military=8.5,
       agricultural=9.0" → ALL THREE VALUES ARE WRONG for I-80. I-80 has A4=0 (no border
       corridor), B4=6.5, C4=7.5. Correct to the actual dimension values for I-80.

[I-14] §07 Conclusion: add explicit acknowledgment that MODULE.md primary contracts
       (≤9 dimensions, ≥2 retirements) were not delivered in v1.2. State the deferral
       to v1.3 as a known gap.

P2 items (should fix):
[I-04] Add plan.md documenting module contract gap
[I-05] §03 v1.1 thresholds vs §02 v1.0 thresholds: explain the shift
[I-06] §06: scores-all.csv schema description doesn't match actual file — clarify as
       "planned schema" or update the data file
[I-07] §06 STRAHNET ρ=0.81 vs ATRI ρ=0.72: clarify what each correlation measures
[I-08] LRTP frequency (50 plans/94%): verify vs A.1's 12 plans/94%; explain if different
[I-11] Strategic override rule for I-95/I-75: formalize in §04 as an explicit criterion

P3 items (optional polish):
- §04 C4 hand-curated scores: add explicit disclosure that these are judgment-based,
  pending USDA ERS integration
- §07: note MODULE.md contract targets for v1.3 (A1/A3 retirement, B2 stabilization,
  data-driven C4) to make the forward roadmap clear

PRE-PANEL CHECKLIST:
□ I-80 gain calculation corrected with full component list summing to 18.5
□ I-35 A4 score corrected (8.5 not 10) in §05 table
□ I-80 v1.2 dimension values corrected in §05 conclusion (A4=0, B4=6.5, C4=7.5)
□ MODULE.md contract gap disclosed in §07 conclusion
□ Rubric version tagged explicitly on score citations (v1.0, v1.1, v1.2)
□ BPR extrapolation caveat: not applicable (no BPR V/C formula in this paper)
□ Net vs gross cost: not applicable
□ Cross-paper citations: I-35/I-80 dimension values consistent with §04 and scores-all.csv
□ Abstract acknowledges partial contract delivery
□ Referee P1 blockers addressed (arithmetic, strategic override formalization)

VERDICT: FIXES REQUIRED
Fixes required: 4 P1 (including critical arithmetic error), 6 P2
Next: run /panel:publication review A.2+rubric-calibration after P1 fixes
═══════════════════════════════════════════════════════════
```
