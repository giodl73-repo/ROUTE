---
paper: E.1+managed-lanes
title: "Managed Freight Lanes: Throughput, Transit Time, and NPV"
post_write_date: 2026-05-08
rubric_version: v1.2
pipeline_stage: ready (Round 1 recheck passed — NPV corrected $115B → $101B; B/C 2.3:1 → 2.0:1)
---

# POST-WRITE PIPELINE — E.1+managed-lanes

---

## PHASE 1 — PAPER INVENTORY

```
Paper: E.1+managed-lanes
Sections found: 01-introduction.tex, 02-literature.tex, 03-data.tex,
                04-throughput.tex, 05-npv.tex, 06-implications.tex,
                07-conclusion.tex
Plan found: no (no plan.md in directory)
Track: E — Interstate 2.0 Design
Venue: Transportation Research Part B or Transportation Science
Key claims:
  1. Corridor-weighted average managed lane capacity: 2,108 pcphpl (not 2,400)
     (§04-throughput Table 2, §05-npv sensitivity table)
  2. Aggregate portfolio NPV: $101B at 7% discount rate (revised central estimate);
     B/C: 2.0:1 (§04-throughput text, §05-npv sensitivity table)
  3. Transcontinental transit time reduction: 4.5 → 3.5 days
     (§04-throughput, abstract, §07-conclusion)
Primary number (from MODULE.md contract):
  "Managed lanes: transit −20%; PTI 1.8→1.15; NPV $X M/mi"
Paper's stated primary number:
  Transit −22% (4.5→3.5 days); PTI 1.58→2.21 → 1.15 target; NPV $101B portfolio
  ($115B pre-correction, $101B corrected central; ~$8.5M/mi at 11,900 miles)
Match: YES — module contract fulfilled with corrected numbers
```

---

## PHASE 2 — QUANTITATIVE CONSISTENCY CHECK

| Q-ID | Quantity | Abstract | §Intro | §04-Throughput | §05-NPV | §07-Conclusion | Consistent? |
|------|----------|---------|--------|----------------|---------|----------------|-------------|
| Q-01 | Program NPV | $115B STALE | — | $101B (corrected) | $101B (corrected, sensitivity table) | $177–265B range (program cost range) | FAIL — abstract says "B/C of 2.3:1" and implies $115B; text says $101B corrected |
| Q-02 | B/C ratio | 2.3:1 STALE | 2.3:1 STALE | 2.0:1 corrected | 2.0:1 (sensitivity table) | 2.0:1 context | FAIL — abstract/intro use 2.3:1; sections use 2.0:1 |
| Q-03 | Annual benefit | — | — | $11.2B/yr (corrected) | $12.7B in NPV table row | FAIL — §04 says $12.7B → $11.2B (12% reduction); §05 Table still shows $12.7B |
| Q-04 | Weighted avg capacity | — | 2,400 pcphpl (first-order) | 2,108 pcphpl (corrected) | 2,108 pcphpl (sensitivity table) | — | WARN — §intro uses 2,400 as "first-order"; §04 introduces 2,108 as correction — this is intentional but should be clearer |
| Q-05 | Transit time: current | 4.5 days | 4.5 days | 4.5 days | — | 4.5 days | PASS |
| Q-06 | Transit time: I2.0 | ~3.5 days | — | ~3.5 days | — | 3.5 days | PASS |
| Q-07 | PTI current range | 1.8–2.2 | 1.8–2.2 | 1.58–2.21 | — | 1.58–2.21 | WARN (abstract 1.8–2.2 vs. section 1.58–2.21 — minor rounding) |
| Q-08 | PTI target | 1.15 | 1.15 | ≤1.15 | — | ≤1.15 | PASS |
| Q-09 | Capital cost (portfolio) | — | $177–265B | $121B | $121B (NPV table) | $177–265B | WARN ($121B is central; $177–265B is cost range; should distinguish "central estimate" vs "range") |
| Q-10 | I-95 B/C | — | — | — | 3.1:1 | — | PASS |
| Q-11 | I-90 B/C | — | — | — | 1.6:1 | — | PASS |
| Q-12 | I-40 exclusion (V/C = 0.84) | yes | yes | 0.84 | — | yes | PASS |
| Q-13 | NY-LA annual shipper benefit | — | — | $10.5B | — | $10.5B | PASS |
| Q-14 | Truck fraction T1: 8% | — | — | §03-data (methods) | — | — | PASS |
| Q-15 | 7 corridors in program | yes | yes | yes | yes | yes | PASS |
| Q-16 | Breakeven portfolio avg: 14 yr | — | — | — | yes | — | PASS |
| Q-17 | I-40 excluded from program | yes | yes | yes | yes | yes | PASS |

**KEY FIX VERIFICATION — CRITICAL NUMBER CHECKS:**

1. Weighted average capacity = 2,108 pcphpl (not 2,400):
   - §04-throughput Table 2 (tab:corridor-capacity): CONFIRMED — "Weighted avg. 2,108" in last row
   - §04 text: CONFIRMED — "corridor-weighted average capacity is 2,108 pcphpl (not 2,400)"
   - §05-npv sensitivity table: CONFIRMED — "Central (2,108 pcphpl weighted avg.)" row present
   - Abstract: FAIL — abstract says "B/C ratio of 2.3:1" which implies old 2,400 figure
   - Introduction: FAIL — intro uses "2,400 pcphpl" as program basis; correction is in §04
   STATUS: 2,108 present in §04 and §05 but abstract/intro still show pre-correction figures

2. NPV = $101B (not $115B):
   - §04-throughput: CONFIRMED — "aggregate NPV from $115B to $101B"
   - §05-npv sensitivity table (tab:npv-sensitivity):
     CONFIRMED — "$101B" in "Central (2,108 pcphpl)" + "1.8%/yr" cell
     CONFIRMED — "$115B" in "High (2,400 pcphpl original)" + "1.8%/yr" cell (correctly labeled as old)
   - §05 text: CONFIRMED — "the revised central estimate using Table tab:corridor-capacity is $101B"
   - §05 NPV table (tab:npv): FAIL — Portfolio row shows "$115.0" as aggregate NPV
   - Abstract: FAIL — abstract says "B/C ratio of 2.3:1" implying $115B figure
   STATUS: $101B present in §04 text and §05 sensitivity table but §05 main NPV table (tab:npv)
           still shows $115B portfolio total. Abstract unchanged.

3. B/C = 2.0:1 (not 2.3:1):
   - §04 text: CONFIRMED — "B/C ratio changes from 2.3:1 to 2.0:1"
   - §05 sensitivity table: CONFIRMED — $101B cell implies 2.0:1
   - §05 NPV table (tab:npv): FAIL — Portfolio row shows "2.3" as B/C
   - Abstract: FAIL — "aggregate benefit-cost ratio of 2.3:1 over 30 years"
   STATUS: 2.0:1 in §04 text and §05 sensitivity table; 2.3:1 still in abstract and §05 NPV table

4. Annual benefit revised to ~$11.2B (from $12.7B):
   - §04 text: CONFIRMED — "annual benefit by approximately 12% relative to the first-order
     estimate: from $12.7B/yr to $11.2B/yr"
   - §05 NPV table (tab:npv): FAIL — Portfolio row shows "12.7" as annual benefit
   - §05 text benefit descriptions: FAIL — individual corridor descriptions use $2.4B, $1.0B etc.
     which sum to $12.7B; no per-corridor suppression applied
   STATUS: $11.2B stated in §04 but §05 NPV table retains $12.7B. A 12% uniform reduction
           needs to propagate to either revised per-corridor figures or a portfolio-level note.

```
CONSISTENCY: FAIL — 4 failures
  Primary failures: abstract and §05 NPV table (tab:npv) still show pre-correction
  figures ($115B NPV, 2.3:1 B/C, $12.7B/yr). §04 text and §05 sensitivity table
  correctly show $101B, 2.0:1, $11.2B.
P1 (must fix):
  - Abstract must be updated: replace "2.3:1" with "2.0:1" and imply $101B
  - §05 NPV table (tab:npv) Portfolio row must be updated: NPV $115.0 → $101.0,
    B/C 2.3 → 2.0, Annual Benefit 12.7 → 11.2
  - Individual corridor rows in tab:npv should be proportionally adjusted (or a
    footnote added: "Portfolio annual benefit corrected from $12.7B to $11.2B;
    per-corridor figures proportionally reduced by 12%")
P2 (should fix):
  - §01-introduction: reference to "2,400 pcphpl" should be labeled as first-order
    or replaced with "approximately 2,100 pcphpl" after §04 correction
  - PTI notation: abstract "1.8–2.2" vs section "1.58–2.21" — use more precise range
P3 (minor):
  - Capital cost framing: $121B (central) vs $177–265B (full range) — distinguish
    clearly in abstract and conclusion
```

---

## PHASE 3 — CONTRACT CHECK

| Promise (from MODULE.md) | Paper section | Delivered? | Gap |
|--------------------------|---------------|-----------|-----|
| Managed lanes: transit −20% | §04 (4.5→3.5 days = −22%) | YES | ✓ |
| PTI 1.8→1.15 | §04 (1.58–2.21 → ≤1.15 target) | YES | ✓ |
| NPV $X M/mi (positive) | §05 (~$8.5M/mi at $101B/11,900mi) | YES | ✓ |
| Throughput analysis by corridor | §04 (Table 1: V/C, PTI, AADT) | YES | ✓ |
| I-40 exception documented | §04, §07 | YES | ✓ |
| Corridor capacity table (KEY FIX: 2,108 pcphpl) | §04 (Table 2: tab:corridor-capacity) | YES | ✓ |
| NPV sensitivity table 3×3 (KEY FIX) | §05 (Table 3: tab:npv-sensitivity) | YES | ✓ |
| NPV corrected to $101B (KEY FIX) | §04 text + §05 sensitivity table | YES — in text | PARTIAL — not in main NPV table |
| B/C corrected to 2.0:1 (KEY FIX) | §04 text + §05 sensitivity table | YES — in text | PARTIAL — §05 tab:npv still shows 2.3 |
| Abstract updated to $101B/$2.0:1 | abstract | NO — abstract unchanged | ✗ |

```
CONTRACT: PARTIAL
Promises kept: 8/10
Gaps:
  - Abstract not updated to reflect $101B/$2.0:1 correction
  - §05 main NPV table (tab:npv) not updated to reflect correction; mismatch
    between tab:npv ($115B, 2.3:1, $12.7B) and tab:npv-sensitivity ($101B, 2.0:1, $11.2B)
MODULE.md primary number delivered: YES (transit −22%; PTI →1.15; NPV $101B in text)
```

---

## PHASE 4 — REFEREE SIMULATION

Selected referees: R-Traffic, R-Economics, R-Network

---

**REFEREE 1 — R-Traffic (Elefteriadou archetype)**
Recommendation: Minor Revision

SUMMARY: The HCM7 corridor-level capacity table (Table 2) is exactly what was required by the
Round 1 recheck, and it is present and well-specified. The PCE values (2.0 level, 3.5–3.8
mountain) are appropriate. The weighted average of 2,108 pcphpl is correctly computed and
consistently stated in §04. The problem is that §05's main NPV table has not been updated;
it still shows the pre-correction values. This is a table inconsistency, not a methodological
error, and it is easily fixed.

MAJOR CONCERNS:
[I-01] §05 Tab:npv "Portfolio" row shows NPV=$115B, B/C=2.3, Annual benefit=$12.7B.
  These values are inconsistent with the corrected results stated in §04 (NPV=$101B,
  B/C=2.0, Annual=$11.2B) and the sensitivity table (tab:npv-sensitivity). The main
  table must be updated before panel review.
[I-02] Abstract states "B/C ratio of 2.3:1" — this is the pre-correction value.
  Must be updated to 2.0:1 in the abstract.

MINOR CONCERNS:
[I-03] HCM7 Exhibit 26-9 for I-70 PCE=3.8 on mountain grades: should be cited
  explicitly in Table 2 footnote (Elefteriadou P3 note from RECHECK).

---

**REFEREE 2 — R-Economics (Neumark archetype)**
Recommendation: Minor Revision

SUMMARY: NPV sensitivity table (3×3: capacity scenario × demand growth) is now present and
satisfies the Round 1 requirement. The B/C > 2.0 finding survives the weakest cell ($81B,
1.7:1 at low/1.5%). The inconsistency between tab:npv and tab:npv-sensitivity is the
remaining P1 blocker — both must show the same central estimate.

MAJOR CONCERNS:
[I-04] §05 NPV table and sensitivity table cite different portfolio NPVs ($115B vs $101B).
  A reader consulting only the NPV table gets the wrong central estimate. Fix: update
  tab:npv to show $101B portfolio or add a prominent note directing readers to
  tab:npv-sensitivity for the corrected figures.

MINOR CONCERNS:
- Toll revenue arithmetic (PP1.3 from REVISION-PLAN) is still not fully worked out;
  this was P1.3 from Round 1. Was it resolved in recheck? RECHECK-SYNTHESIS notes
  Neumark confirmed PP1.2 (FAF citation) passed but PP1.3 (tolling model) is still
  listed as open P2 in RECHECK-SYNTHESIS P3 notes.
- Breakeven toll uptake (62% blended) should be stated and confirmed to cover O&M
  at 40% uptake scenario.

---

**REFEREE 3 — R-Network (Adamic archetype)**
Recommendation: Accept (with minor revisions)

SUMMARY: The throughput analysis correctly identifies V/C ratios at the 90th-percentile
segment as the binding constraint. The I-40 exclusion is convergently validated by HPMS
and ATRI data. Network-level effects are acknowledged (GP lane LOS improvement).

MAJOR CONCERNS: none blocking

MINOR CONCERNS:
[I-05] Table 1 presents "AADT (M)" values — confirm these are annual average daily traffic
  in millions (e.g., I-90 Chicago = 187M AADT/year), not AADT as typically stated in
  vehicles/day (~512k vpd for I-90 Chicago). If in millions annual, this is unusual
  notation; if in thousands of vpd, the table header is wrong. Should be stated clearly.

---

## PHASE 5 — ABSTRACT CHECK

```
ABSTRACT: ~210 words
Primary result stated: YES — "benefit-cost ratio of 2.3:1" (STALE — must be 2.0:1)
  and "managed freight lane program is positive across all 7 corridors"
Method named: YES — "analyze all 8 T1 corridors under the ROUTE v1.2 tier classification"
Policy implication: YES — "required managed lane investment (I-40 is the exception)"
Track chain position: YES — references T1 classification (Track A) implicitly
CORRECTION NEEDED: Abstract contains pre-correction B/C (2.3:1) — the single most
  visible number in the paper must be updated to 2.0:1
```

---

## PHASE 6 — INTERNAL CROSS-PAPER CONSISTENCY

```
CROSS-PAPER CONSISTENCY:
  Papers cited from E.1: ROUTE_A1, ROUTE_B2, ROUTE_B3, ROUTE_C1, ROUTE_D2,
    HCM7, ATRI_costs2024, FHWA_HPMS2023, FAF5_2023, IIJA2021
  Values cross-checked:
    - E.2 §01-intro cites E.1: "$121 billion capital, 2.3:1 benefit-cost ratio" — STALE
      E.2 cites the pre-correction B/C. E.2 must be updated when E.1 abstract is corrected.
    - E.2 §03-investment-portfolio Component 1: "$12.7B annual benefit, NPV $86.4B, B/C 2.3:1"
      These are all pre-correction figures. E.2 must be updated when E.1 is corrected.
    - E.2 §04-throughput cites E.1 for "7.2M commercial vehicles/day" — this is a throughput
      figure not affected by the capacity correction; PASS
    - C.1 PTI on I-80 = 1.86 cited in E.2 framework section — matches C.1 paper; consistent
  Stale citations (pre-correction):
    - E.2 §01-intro and §03-investment-portfolio cite E.1's pre-correction B/C (2.3:1) and
      annual benefit ($12.7B). Once E.1 abstract is corrected to 2.0:1 / $11.2B / $101B,
      E.2 must be updated to match.
    - CRITICAL CROSS-PAPER FINDING: E.2 does NOT currently cite $101B for E.1 NPV.
      E.2 cites $86.4B component NPV (Component 1 in portfolio table) — different from
      the $101B portfolio-level NPV in E.1 (the $86.4B is E.2's own DCF of E.1 benefits
      within the integrated portfolio; the $101B is E.1's standalone analysis). These are
      different numbers for different analyses. This needs a cross-reference clarification
      note in both papers.
```

---

## PHASE 7 — PRE-PANEL CHECKLIST

```
═══════════════════════════════════════════════════════
POST-WRITE COMPLETE: E.1+managed-lanes
═══════════════════════════════════════════════════════

Validation results:
  Consistency:       FAIL — abstract and §05 main NPV table still show pre-correction
                     figures ($115B / 2.3:1 / $12.7B/yr); §04 text and §05 sensitivity
                     table correctly show $101B / 2.0:1 / $11.2B/yr
  Contract:          PARTIAL — 8/10 promises; abstract and main NPV table not updated
  Referee sim:       Minor Revision (R-Traffic I-01, I-02); Minor Revision (R-Economics I-04)
  Abstract:          ~210 words; primary result stated but STALE (2.3:1 must be 2.0:1)
  Cross-paper:       E.2 cites stale E.1 figures ($115B/$86.4B vs $101B) — needs update
                     after E.1 correction

KEY FIX VERIFICATION:
  [x] NPV corrected from $115B to $101B — CONFIRMED IN §04 TEXT AND §05 SENSITIVITY TABLE
  [x] B/C corrected from 2.3 to 2.0 — CONFIRMED IN §04 TEXT AND §05 SENSITIVITY TABLE
  [x] Corridor capacity table (2,108 pcphpl weighted average) present in §04 — CONFIRMED
  [x] NPV sensitivity table (3×3) present in §05 — CONFIRMED
  [x] Annual benefit revised to ~$11.2B — CONFIRMED IN §04 TEXT
  [FAIL] Abstract still shows 2.3:1 B/C (pre-correction) — NOT YET UPDATED
  [FAIL] §05 NPV table (tab:npv) Portfolio row still shows $115B / 2.3 / $12.7B — NOT UPDATED

P1 blockers (fix before panel review):
  [I-01+I-02] Update abstract: replace "2.3:1" with "2.0:1"; note $101B corrected NPV
  [I-01+I-04] Update §05 tab:npv Portfolio row: NPV=$115B → $101B; B/C=2.3 → 2.0;
               Annual=$12.7B → $11.2B; propagate 12% reduction to per-corridor rows
               or add footnote
  [CROSS] E.2 must be updated after E.1 correction: §01-intro "2.3:1" → "2.0:1";
          §03-investment-portfolio Component 1 annual benefit $12.7B → $11.2B

P2 items (should fix):
  [I-05] Table 1 AADT units clarification (M = millions annual vs thousands/day)
  [I-03] Add HCM7 Exhibit 26-9 citation for I-70 PCE=3.8 in Table 2 footnote
  [Q-09] Clarify $121B (central) vs $177–265B (range) — label "central estimate" in abstract
  [TOLL] Tolling model demand uptake (PP1.3 from REVISION-PLAN) — P2 item still open

P3 items (optional polish):
  - Breakeven toll uptake rate (62% blended) and O&M coverage at 40% uptake — state explicitly
  - PTI range: unify "1.8–2.2" (abstract) vs "1.58–2.21" (sections) notation
  - GP lane LOS counterfactual B/C (REVISION-PLAN P2.3) — not yet done

PRE-PANEL CHECKLIST:
[x] KEY FIX — corridor capacity table 2,108 pcphpl in §04: PRESENT
[x] KEY FIX — NPV $101B in §04 text: PRESENT
[x] KEY FIX — B/C 2.0:1 in §04 text: PRESENT
[x] KEY FIX — annual benefit $11.2B in §04 text: PRESENT
[x] KEY FIX — NPV sensitivity table (3×3) in §05: PRESENT
[ ] KEY FIX — Abstract updated to 2.0:1 / $101B: NOT DONE (abstract still shows 2.3:1)
[ ] KEY FIX — §05 tab:npv Portfolio row updated to $101B/2.0/11.2B: NOT DONE
[ ] MODULE.md primary number corrected in abstract: PARTIAL (transit −22% ✓; B/C STALE)
[x] Net vs gross cost clearly stated: YES (NPV is net; benefits vs costs separated in table)
[ ] All \citep{} keys verified: NOT VERIFIED
[ ] Cross-paper E.2 citations updated to reflect $101B / 2.0:1: NOT DONE (E.2 fix required)
[x] Rubric version: YES — "ROUTE v1.2" referenced
[ ] Abstract states corrected primary quantitative result: NOT YET ($101B / 2.0:1 missing)
[ ] Referee P1 blockers addressed (I-01, I-02, I-04): NOT YET DONE

VERDICT: FIXES REQUIRED
Fixes required: 3 (abstract update + §05 tab:npv update + E.2 cross-citation update)
Next: Update abstract + NPV table, confirm E.2 cross-citations, then run
/panel:publication review E.1+managed-lanes
═══════════════════════════════════════════════════════
```
