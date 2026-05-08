---
paper: B.4+t1-intersection-resilience
type: post-write-check
author: research-post-write
created: 2026-05-08
---

# POST-WRITE CHECK: B.4 — T1/T1 Intersection Resilience

## PHASE 1 — READ THE PAPER

```
Paper: B.4+t1-intersection-resilience
Sections found: 01-introduction.tex, 02-literature.tex, 03-k-connectivity.tex,
                04-diamond-design.tex, 05-investment-case.tex, 06-conclusion.tex
Plan found: NO (no plan.md found in B.4 directory)
Track: B — Network Gaps & Missing Links
Venue: (not specified; inferred Transportation Science or Transportation Research Part B)
Key claims:
  1. 9 of 15 T1/T1 intersections have k=1 (single-point-of-failure) — abstract, §03
  2. $4.5B diamond program achieves k≥3 at all critical junctions — abstract, §05
  3. Full portfolio 30-yr NPV = $18.6B ($14.0B for top-5 confirmed intersections) — §05 Table
Primary number (from MODULE.md contract): Not checked (no plan.md)
Paper's stated primary number: $4.5B capital for k≥3 at all 9 SPF intersections
Match: CANNOT VERIFY without plan.md
```

---

## PHASE 2 — QUANTITATIVE CONSISTENCY CHECK

| Q-ID | Quantity | Abstract | §01 (Intro) | §03 (k-conn) | §04 (Design) | §05 (Investment) | §06 (Conclusion) | Consistent? |
|------|----------|---------|------------|-------------|-------------|-----------------|-----------------|-------------|
| Q-01 | SPF intersections (k=1) | 9 | 9 | 9 | — | 9 | 9 | PASS |
| Q-02 | Total T1/T1 intersections | 15 | 15 | 15 | — | 15 | 15 | PASS |
| Q-03 | Total diamond program cost | $4.5B | $4.5B | — | — | $4,790M | $4.5B | WARN (table shows $4,790M vs rounded $4.5B — minor) |
| Q-04 | Portfolio NPV | — | — | — | — | $18.6B | — | PASS |
| Q-05 | Atlanta NPV | — | — | — | $380M cost | $2.8B NPV, B/C 3.7 | — | PASS |
| Q-06 | Phase 1 cost (ATL+JAX+TOL) | — | — | — | — | $930M | $930M | PASS |
| Q-07 | Phase 1 NPV | — | — | — | — | $6.5B | — | PASS |
| Q-08 | ATRI truck cost rate | — | — | — | $225/hr | $225/hr | — | PASS |
| Q-09 | 50-mile zone radius | — | §01 | §03 | §04 | §05 | — | PASS |
| Q-10 | k-improvement factor | — | — | — | — | 0.70 | — | PASS |
| Q-11 | I-75 Atlanta managed lane cost | — | $27B | — | — | $27B | $27B | PASS |
| Q-12 | Top-5 NPV share | — | — | $8.8B of $14.0B = 63% | — | — | — | PASS |

**Issues found:**

- **Table discrepancy (P3)**: Total column shows "4,790" (in $M) but prose rounds to $4.5B. Technically $4,790M = $4.79B which rounds to $4.8B, not $4.5B. Abstract and introduction both say "$4.5 billion." Either the table total or the rounded prose figure needs reconciling. The sum of the investment column: 480+650+510+820+390+420+280+340+310+180+160+140+110+290+310+260+190 = checking... 480+650=1130, +510=1640, +820=2460, +390=2850, +420=3270, +280=3550, +340=3890, +310=4200, +180=4380, +160=4540, +140=4680, +110=4790. Sum = $4,790M. So the table is correct ($4.79B), but abstract/introduction says "$4.5 billion." This is a P2 inconsistency: either the program scope or the rounding is wrong.

- **Atlanta intersection naming conflict (P2)**: The manual validation table (§03 Table tab:k-manual) and the investment case table (§05 Table tab:investment) show different priority intersections. The investment case table uses Atlanta (I-75×I-85, B2=0.68 highest priority), Jacksonville (I-10×I-95, B2=0.61), Toledo (I-75×I-90, B2=0.58) as Phase 1. The k-connectivity table (§03) does NOT include Atlanta or Toledo in its "9 worst cases" section (it discusses Omaha, Chattanooga, San Antonio, Boston, Jacksonville). The abstract says "Atlanta (I-75/I-85), Jacksonville (I-10/I-95), and Toledo (I-75/I-90) in Phase 1." But neither Atlanta nor Toledo appear in the k-connectivity worst-cases list in §03. This is a structural inconsistency. The k-connectivity table (Table tab:k-connectivity) does not list Atlanta at all — it shows Omaha, Chattanooga, San Antonio, Boston, Jacksonville, Salt Lake City, Sioux Falls, Memphis, Las Cruces as the 9 SPF intersections.

  **Investigation**: The investment table (§05) shows "I-75×I-85, Atlanta, k=1" but the k-connectivity worst-cases text (§03) does not include Atlanta in the 9 SPF intersections. The manual validation table (§03, added as revision §3.6) validates "Atlanta I-75/I-85" as k=1. This means Atlanta is in the investment table (§05) as k=1 and confirmed in the validation table but is NOT named in the original 9 SPF list in §03. **The §03 main k-connectivity table (Table tab:k-connectivity) must be missing Atlanta.** The table has exactly 9 k=1 entries but none is Atlanta. This is a P1 error: either Atlanta is missing from the main k-connectivity table, or one of the listed 9 is wrong.

  **Cross-check**: Section §05 investment table lists 13 intersections that require investment (9 k=1 + 4 k=2). Atlanta (I-75×I-85) appears at the top. The k-connectivity table in §03 lists 9 k=1 intersections: Omaha, Chattanooga, San Antonio, Boston, Jacksonville, Salt Lake City, Sioux Falls, Memphis, Las Cruces — no Atlanta, no Toledo. The §05 table adds Atlanta, Toledo, Gary, Des Moines, Barstow, Montgomery among SPF — 9 SPF in §05 are: Atlanta, Jacksonville, Toledo, Boston, San Antonio, Gary, Des Moines, Barstow, Montgomery. These two sets of 9 are DIFFERENT. **This is a P1 structural inconsistency between §03 and §05.**

- **Detroit/Denver labels in §05 investment table**: The table shows "I-75×I-90 Detroit" twice — once as k=1 (Toledo, which is actually I-75×I-90) and once as k=4 (Detroit). Toledo is NOT I-75×I-90; Toledo is on I-75 (north-south) and I-80/I-90 (east-west Ohio Turnpike). The table at §03 shows "I-75×I-90, Toledo" and "I-75×I-90, Detroit" which is impossible — two different intersections cannot have the same designation. The §05 investment table bottom row says "I-75×I-90, Detroit, k=4" which matches §03's best-case discussion. The §05 Phase 1 includes "I-75×I-90, Toledo, k=1, $340M" which is the Toledo partial cloverleaf. The corridor designations should be different (Toledo is more likely I-75×I-80/I-90, i.e., the Ohio Turnpike). P2 — needs clarification of intersection designation.

- **Denver labeled as I-5×I-90**: §05 table footnote shows "I-5×I-90, Denver, CO, k=3" — Denver is not on I-5 (which is the West Coast corridor). Denver is on I-70 and I-25. The correct designation is "I-70×I-25, Denver." This matches §03 which says "I-70×I-25, Denver, k=3." The §05 table row shows "I-5×I-90 Denver, CO" which is factually wrong. **P1 error: I-5×I-90 is incorrect designation for Denver.**

```
CONSISTENCY: FAIL — 2 P1 failures, 2 P2 warnings
P1: [§03 vs §05 SPF intersection set mismatch — Atlanta/Toledo missing from §03 table],
    [§05 Denver labeled I-5×I-90 instead of I-70×I-25]
P2: [$4,790M table total vs $4.5B prose — reconcile to $4.8B or adjust scope],
    [Toledo intersection designation ambiguous — I-75×I-90 vs I-75×I-80/I-90]
P3: [Abstract says "average $300M per junction" — $4,790M / 13 investments = $369M average, not $300M]
```

---

## PHASE 3 — CONTRACT CHECK

No plan.md found. Assessment against stated contributions.

| Promise | Section | Delivered? | Gap |
|---------|---------|-----------|-----|
| k-connectivity analysis of all 15 T1/T1 intersections | §03 | Partial — 9 listed in main table but not same 9 as §05 | Partial |
| 9 single-point-of-failure intersections identified | §03, §05 | Yes, but different sets | INCONSISTENT |
| 5 extreme-priority diamond targets | §01 | Omaha, Chattanooga, San Antonio, Boston, Jacksonville in §01; but §05 Phase 1 = Atlanta, Jacksonville, Toledo | INCONSISTENT |
| Geographic constraint principle | §03 | Yes | ✓ |
| Investment comparison vs managed lanes | §05, §06 | Yes ($4.5B vs $27B) | ✓ |
| Manual validation of top-5 | §03 §3.6 | Yes (Atlanta, Jacksonville, Toledo, Richmond, Sacramento) | ✓ |

```
CONTRACT: PARTIAL
Promises kept: 4/6
Gaps: [SPF intersection set not consistent between §03 and §05; Phase 1 priority list changes between §01 and §05]
MODULE.md primary number delivered: CANNOT VERIFY
```

---

## PHASE 4 — REFEREE SIMULATION

**Referees selected**: R-Network, R-Traffic, R-Policy

---

**REFEREE 1 — R-Network** (Lada Adamic archetype — Transportation Science / PNAS)
Recommendation: **Major Revision**

SUMMARY: The k-connectivity methodology is sound and the Menger/Edmonds-Karp formulation is correctly stated. However, the paper has a structural inconsistency between the k-connectivity results table (§03) and the investment analysis table (§05) — they list different sets of 9 SPF intersections. A referee checking §03 against §05 will find that Atlanta, Toledo, Gary, Des Moines, Barstow, and Montgomery appear in §05 but not §03's main k-connectivity table, while Omaha, Chattanooga, Salt Lake City, Sioux Falls, Memphis, and Las Cruces appear in §03 but have different investment table entries. This cannot be attributed to data update — it requires reconciliation of which 9 intersections are k=1.

MAJOR CONCERNS:
[I-01] §03 Table tab:k-connectivity and §05 Table tab:investment list different sets of 9 k=1 intersections. If the algorithm produced the §03 results, the investment analysis must use those same results. If the investment analysis revised the intersection set, §03's table must be updated. This is the central methodological inconsistency of the paper.
[I-02] §05 investment table shows "I-5×I-90, Denver, CO, k=3" — Denver is not on I-5. The correct designation is I-70×I-25. This is a factual error in a central data table.
[I-03] The TIGER junction snapping issue (§3.6) is acknowledged but addressed only for 5 intersections. The remaining 10 use unvalidated graph-computed k values; the paper should either validate all 15 or bound the error more carefully than "sensitivity testing shows <5% NPV change."

MINOR CONCERNS:
[I-04] Edmonds-Karp time complexity claimed as O(VE²) — correct, but the paper should note this is the BFS-augmenting path variant; the standard Edmonds-Karp is known for O(VE²), but the worst-case operations figure given (7.4×10¹³) seems large for V=3,200, E=48,000; verify.
[I-05] "Unit edge capacities" used for k-connectivity but weighted capacities used for max-flow — distinguish clearly that k-connectivity uses unit capacity (counting paths) while the NPV model uses actual vehicle capacity.

---

**REFEREE 2 — R-Traffic** (Elefteriadou archetype)
Recommendation: **Major Revision**

SUMMARY: The diamond zone design is thoughtful and the geometric standards are appropriate. The 50-mile zone radius is defensible on freight economic grounds. The k-connectivity analysis is methodologically correct but the inconsistency between §03 and §05 requires resolution.

MAJOR CONCERNS:
[I-06] The 65-mph design speed standard is stated as requiring 600-foot minimum radius. HCM 7 Table 3-3 gives minimum radius of 1,500 feet for 65-mph design speed on freeways; 600 feet is closer to the 45-mph standard. Verify the 600-foot figure — if it is from a different design standard (e.g., AASHTO Blue Book for interchange ramps rather than mainline), cite the specific table.
[I-07] The k-improvement factor κ = 0.70 is stated as "the fraction of incident-hours eliminated by diamond connectivity" but no citation or derivation is provided. This is a key parameter for all NPV calculations; its basis must be documented.

MINOR CONCERNS:
[I-08] The weigh station bypass component (§04) is not reflected in the investment cost tables. If it's included in the per-intersection cost estimates, note that explicitly.

---

**REFEREE 3 — R-Policy** (Puentes archetype)
Recommendation: **Accept with Minor Revisions** (pending P1 fixes)

SUMMARY: The policy case for diamond interchange zones is well-made. The comparison to managed lanes is particularly strong. The IIJA NHPP funding eligibility analysis is accurate. The implementation of Phase 1 as a single design contract covering three zones is a practical recommendation that shows awareness of procurement realities.

MAJOR CONCERNS:
[I-09] The Section §01 Findings list says Phase 1 priority is "Atlanta (I-75/I-85), Jacksonville (I-10/I-95), and Toledo (I-75/I-90)" but §03 identifies the extreme-priority SPF intersections as "I-35×I-80 at Omaha, I-40×I-75 at Chattanooga, I-10×I-35 at San Antonio, I-90×I-95 at Boston, and I-10×I-95 at Jacksonville." The Phase 1 selection in the introduction and conclusion does not match the §03 priority analysis. This will confuse policy readers about which sites actually need to be funded first.

MINOR CONCERNS:
[I-10] The FHWA Safety Investment category eligibility for Toledo (§05) — confirm this applies to interchange reconstruction at this scale, not just signal and marking improvements.

---

## PHASE 5 — ABSTRACT CHECK

```
ABSTRACT: ~120 words
Primary result stated: YES — "$4.5 billion in diamond interchange investment would eliminate the k=1
  condition at all critical junctions"
Method named: YES — "edge k-connectivity," "diamond interchange zone (50-mile access-controlled zone)"
Policy implication: YES — "$4.5 billion ... Phase 1 ... Atlanta, Jacksonville, Toledo"
Track chain position: YES — "T1/T1 intersections," extension of B.1 missing links concept
Word count: ~120 words — below 150-word target
```

**Issue**: Abstract names Atlanta, Jacksonville, Toledo as Phase 1, which conflicts with §03's extreme-priority list (Omaha, Chattanooga, San Antonio, Boston, Jacksonville). Resolve §03/§05 discrepancy first, then update abstract.

---

## PHASE 6 — INTERNAL CROSS-PAPER CONSISTENCY

```
CROSS-PAPER CONSISTENCY:
  Papers cited: ROUTE_B1 (B1 scores, betweenness corpus), ROUTE_B3 (betweenness corpus),
                ROUTE_D1 (D1 climate, flood zone exposure at Jacksonville),
                ROUTE_E1 (I-75 Atlanta managed lane $27B)
  Values cross-checked: 4/4

  - ROUTE_B1 cited for B1 scores and diamond concept: correct
  - ROUTE_B3 cited jointly with ROUTE_B1 for betweenness corpus: NOTE — B.3 is the 
    resilience holes paper, not a betweenness paper. The betweenness corpus is from B.2 
    (freight bottlenecks). This may be a citation key error: should be ROUTE_B2, not 
    ROUTE_B3. P2 flag.
  - ROUTE_D1 cited for Jacksonville flood zone (D1=7.8): verify D.1 actually reports this 
    value for I-10 Gulf Coast. Cross-check: B.3 §04 shows I-10 Gulf Coast TX V/C D1=7.6 
    (not 7.8 used in B.4). Possible off-by-one — B.4 says "D1=7.8 for the I-10 corridor" 
    but B.3 shows I-10 Gulf Coast TX D1=7.6. P2 flag.
  - ROUTE_E1 cited for $27B Atlanta managed lanes: consistent with B.4 §01, §05, §06.

Stale citations (pre-correction): NONE — B.4 does not cite B.3 NPV directly.
```

**P2 flag**: `\citep{ROUTE_B1, ROUTE_B3}` in §05 for betweenness corpus — should likely be `\citep{ROUTE_B1, ROUTE_B2}`.

---

## PHASE 7 — PRE-PANEL CHECKLIST

```
═══════════════════════════════════════════════════════
POST-WRITE COMPLETE: B.4+t1-intersection-resilience
═══════════════════════════════════════════════════════

Validation results:
  Consistency:       FAIL — 2 P1 failures (§03/§05 SPF set mismatch; Denver corridor label)
  Contract:          PARTIAL — no plan.md; key SPF identification inconsistency between sections
  Referee sim:       Major Revision (R-Network, R-Traffic); Accept minor (R-Policy)
  Abstract:          ~120 words (below target), Phase 1 priority list may be wrong
  Cross-paper:       WARN — possible ROUTE_B3 should be ROUTE_B2; D1 score for I-10 TX
                     differs between B.3 (7.6) and B.4 citation (7.8)

P1 blockers (fix before panel review):
[I-01] §03 Table tab:k-connectivity and §05 Table tab:investment list different sets of
  9 k=1 intersections. These must be reconciled. The investment table (§05) appears to
  be the authoritative version (it has manual validation support). Update §03 table to
  match §05 intersection set, OR update §05 table to match §03 and reconcile Phase 1 
  priority list throughout.
  Current §03 SPF set: Omaha, Chattanooga, San Antonio, Boston, Jacksonville, Salt Lake City,
    Sioux Falls, Memphis, Las Cruces
  Current §05 SPF set: Atlanta, Jacksonville, Toledo, Boston, San Antonio, Gary, Des Moines,
    Barstow, Montgomery
  Overlapping only: Boston, Jacksonville, San Antonio (3 of 9)
[I-02] §05 investment table row: "I-5×I-90, Denver, CO, k=3" — Denver is on I-70×I-25, not
  I-5×I-90. Fix corridor designation. (I-5 is the Pacific Coast corridor; Denver is inland.)

P2 items (should fix):
[I-03] Total program cost: table sums to $4,790M ($4.79B) but prose says "$4.5 billion."
  Reconcile: either adjust scope to bring total to $4.5B, or update prose/abstract to "$4.8B."
[I-04] §05 citation ROUTE_B3 for betweenness corpus — likely should be ROUTE_B2 (freight
  bottlenecks paper, which contains the betweenness analysis). Verify and correct.
[I-05] D1 score for I-10 Gulf Coast TX: B.4 §05 cites D1=7.8 but B.3 §04 Table 3 shows
  D1=7.6 for "I-10 Gulf Coast TX (Houston)." Resolve to consistent value.
[I-06] κ=0.70 k-improvement factor needs citation or derivation footnote.

P3 items (optional polish):
- Abstract below 150-word target; add sentence on methodology
- §04 verify 600-foot radius against AASHTO standard for ramp design speed (not mainline)
- Phase 1 abstract description should match §05 Phase 1 analysis after P1 fix

PRE-PANEL CHECKLIST:
□ §03 and §05 SPF intersection sets reconciled (same 9 intersections in both tables) — FAIL
□ Denver corridor designation corrected (I-70×I-25, not I-5×I-90) — FAIL
□ MODULE.md primary quantitative contract delivered — CANNOT VERIFY (no plan.md)
□ BPR extrapolation acknowledged — NOT APPLICABLE (k-connectivity, not BPR)
□ Net vs gross cost clearly stated — PASS
□ All \citep{} keys in references.bib: ROUTE_B1, ROUTE_B3, ROUTE_B4, ROUTE_D1, ROUTE_E1,
  ATRI_costs2024, IIJA2021 — all present. MISSING: Menger1927, Brandes2001, Jenelius2010,
  Mattsson2015, FHWA_interchange2019, FHWA_ML2021, Browand2004, Hepinstall2010, HCM7,
  HPMS2018, FHWA_HPMS2023, ROUTE_diamond, WSDOT2023 — ALL MISSING from references.bib.
  This is a systemic P2 issue requiring bib entries for all external sources.
□ Cross-paper citations use corrected values — PASS (no B.3 NPV cited)
□ Abstract states primary result — PASS ($4.5B for k≥3)
□ Referee P1 blockers addressed — PENDING

VERDICT: FIXES REQUIRED
Fixes required: 2 P1 (SPF set reconciliation, Denver label), plus systemic bib gap
Next: Reconcile §03/§05 SPF intersection sets; then run /panel:publication review B.4
═══════════════════════════════════════════════════════
```
