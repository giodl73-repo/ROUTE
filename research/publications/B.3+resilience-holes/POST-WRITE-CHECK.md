---
paper: B.3+resilience-holes
type: post-write-check
author: research-post-write
created: 2026-05-08
---

# POST-WRITE CHECK: B.3 — Resilience Holes

## PHASE 1 — READ THE PAPER

```
Paper: B.3+resilience-holes
Sections found: 01-introduction.tex, 02-background.tex, 03-investment-case.tex,
                04-compound-exposure.tex, 05-investment-sequencing.tex,
                06-policy-implications.tex, 07-conclusion.tex
Plan found: NO (no plan.md found in B.3 directory)
Track: B — Network Gaps & Missing Links
Venue: (not specified; inferred Transportation Research Part A or Transport Policy)
Key claims:
  1. 11 corridors satisfy the compound resilience hole condition (B1>7 AND D1>6) — §04
  2. Donner Pass freight tunnel NPV = $12.1B at 7% over 30 years — §03
  3. Top-4 compound corridors account for $3.1B in annual freight disruption — abstract
Primary number (from MODULE.md contract): Not checked (no plan.md; MODULE.md not present)
Paper's stated primary number: $12.1B NPV (§03 corrected figure)
Match: PARTIAL — see P1 failures below
```

---

## PHASE 2 — QUANTITATIVE CONSISTENCY CHECK

| Q-ID | Quantity | Abstract | §01 (Intro) | §03 (Investment) | §04 (Compound) | §05 (Sequencing) | §07 (Conclusion) | Consistent? |
|------|----------|---------|------------|-----------------|---------------|-----------------|-----------------|-------------|
| Q-01 | Donner annual disruption cost | $3.1B (top-4 total) | **$1.6B** | $1.3B (B1+D1) | **$1.60B** (Table 3) | $1.6B (D1-only comparison) | **$1.6B** + "exceeds $15B NPV" | **FAIL** |
| Q-02 | Donner 30-yr NPV | — | — | **$12.1B** (eq + table) | — | — | **$15B+** (old value) | **FAIL** |
| Q-03 | Donner waiting cost rate | — | — | **$91/hr** (correctly stated) | — | — | — | PASS in §03 |
| Q-04 | Donner rerouting cost rate | — | — | $225/hr (correctly stated as motion rate) | $225/hr | — | — | PASS |
| Q-05 | Combined annual benefit | — | — | **$1.3B** | — | — | **$1.6B** | **FAIL** |
| Q-06 | CBR | — | — | **4.0:1** | — | — | 4.0:1 (implied but not stated) | PASS in §03 |
| Q-07 | Top-4 portfolio NPV | — | — | $25.8B | — | — | — | PASS |
| Q-08 | Number of compound corridors | 11 | 11 | — | 11 | (7 listed in table) | 11 | PASS |
| Q-09 | Donner closures/yr | 50 | 50 | 50 | ~50 | — | 50 | PASS |
| Q-10 | Donner mean closure duration | 18 hr | 18 hr | 18 hr | 18 hr | — | — | PASS |

**P1 FAILURES (must fix before panel):**

1. **§01 Introduction, paragraph 3**: States "$1.6 billion in annual freight disruption cost" — this is the pre-correction figure. The corrected combined annual benefit is **$1.3B** (§03). Fix: change to "$1.3 billion."

2. **§04 Table 3 (tab:compound)**: I-80 Donner Pass "Annual cost" = **$1.60B** — pre-correction value. Fix: change to **$1.30B**.

3. **§05 Investment Sequencing, §5.2 paragraph on D1-only ordering**: States "despite Donner's $1.6B annual disruption cost" — pre-correction. Fix: change to "$1.3B."

4. **§07 Conclusion, key finding 2**: States "At $4 billion construction cost and **$1.6 billion** in annual avoided disruption, the simple payback period is 2.5 years. The 30-year NPV at a 7% discount rate **exceeds $15 billion**." Both numbers are the old pre-correction values. Fix:
   - "$1.6 billion" → "$1.3 billion"
   - "exceeds $15 billion" → "is $12.1 billion"
   - Payback period: $4.0B / $1.3B = 3.1 years (not 2.5 years). Fix payback period too.

```
CONSISTENCY: FAIL — 4 P1 failures (sections 01, 04, 05, 07 all retain old $1.6B / $15B figures)
P1: [01-intro $1.6B], [04-table $1.60B], [05-sequencing $1.6B comparison], [07-conclusion $1.6B + $15B + 2.5yr payback]
P2: [Abstract says "$3.1B in annual freight disruption" for top 4 — this is the sum across all 4 corridors; verify arithmetic: $1.3B + $0.82B + $0.65B + $0.48B = $3.25B, not $3.1B — round-check needed]
P3: [Gulf Coast §03 uses $225/hr for operating penalty — comment that this is the in-motion rate, same as rerouting; acceptable for non-idle vehicles]
```

---

## PHASE 3 — CONTRACT CHECK

No plan.md found in B.3 directory. Assessment is based on paper self-description.

| Promise (from paper's introduction) | Section | Delivered? | Gap |
|-------------------------------------|---------|-----------|-----|
| Define compound resilience hole (B1>7 AND D1>6) | §02, §04 | Yes | ✓ |
| Identify 11 compound corridors | §04 | Yes | ✓ |
| Quantify compound exposure costs (frequency × magnitude × rerouting) | §04 Table 3 | Yes (with P1 errors) | Partial |
| Investment NPV superiority of compound investments | §03 Table 1 | Yes (§03 corrected) | ✓ |
| Investment sequencing via B1×D1 product | §05 | Yes | ✓ |
| Policy implications for PROTECT grant criteria | §06 | Yes | ✓ |
| D.1 citation for D1 scores used in compound exposure | §04 | Yes via \citep{ROUTE_D1} | ✓ |

```
CONTRACT: PARTIAL
Promises kept: 6/7
Gaps: [§04 Table 3 Donner annual cost still shows pre-correction $1.60B — reduces delivered accuracy]
MODULE.md primary number delivered: CANNOT VERIFY (no plan.md or MODULE.md in directory)
```

---

## PHASE 4 — REFEREE SIMULATION

**Referees selected**: R-Traffic, R-Economics, R-Policy

---

**REFEREE 1 — R-Traffic** (Lily Elefteriadou archetype — Transportation Research Part A)
Recommendation: **Major Revision**

SUMMARY: The compound exposure concept is well-motivated and the investment case for Donner tunnel is the strongest section. However, the paper still contains inconsistent numbers across sections — the abstract, §04, §05, and §07 use pre-correction annual disruption figures ($1.6B) while §03 uses corrected figures ($1.3B, $91/hr idle rate). This is not a minor inconsistency; it changes the CBR from 5.75:1 (old) to 4.0:1 (corrected) and the payback from 2.5 to 3.1 years. A referee reading §03 vs. §07 will flag this immediately.

MAJOR CONCERNS:
[I-01] §03 Donner NPV uses $91/hr idle rate correctly, but §01, §04, §05, and §07 all retain the old $1.6B total which was based on $225/hr for idle trucks. The paper is internally inconsistent on its central finding.
[I-02] §04 Table 3 (Compound Resilience Hole Corridors) shows Donner annual cost = $1.60B. After the waiting-cost correction, this should be $1.30B. Referees cross-checking Table 3 against §03 will catch this.
[I-03] §07 states "exceeds $15 billion" for Donner NPV — the corrected figure is $12.1B as shown in §03 Table 1. These are in the same paper; having the conclusion contradict the results section is a desk-reject risk at any respectable journal.

MINOR CONCERNS:
[I-04] BPR function not cited for D1 closure cost modeling — the paper uses BPR-style frequency-weighted cost but does not acknowledge its limitations in this context.
[I-05] The compound priority metric (B1×D1 product) is asserted as correct but no sensitivity analysis is offered for alternative combination functions (e.g., min, geometric mean).

---

**REFEREE 2 — R-Economics** (David Neumark archetype — Journal of Economic Perspectives)
Recommendation: **Major Revision**

SUMMARY: The investment case is sound in structure but the NPV inconsistency between sections undermines the quantitative credibility. The distinction between idle cost ($91/hr) and motion cost ($225/hr) is conceptually important and §03 makes it correctly; but the remaining sections undermine it by using the composite $1.6B figure.

MAJOR CONCERNS:
[I-06] The paper never explicitly states what the pre-correction $1.6B figure was based on. A reader who knows only the corrected paper will not understand why §07 says $1.6B while §03 says $1.3B. The revision should either (a) correct all instances, or (b) add a note in §03 explicitly reconciling the difference and deprecating the old figure.
[I-07] §03 Portfolio NPV table shows Gulf Coast I-10 annual benefit = $0.82B with CBR = 3.0:1. The text says annual benefit is "approximately $820M/year" and cost is $2.9B. 30-yr PV at 7% = $0.82B × 12.4 = $10.2B; NPV = $10.2B - $2.9B = $7.3B. But table shows NPV = $5.8B. The $5.8B implies annual benefit of ($5.8B + $2.9B) / 12.4 = $0.70B/yr — inconsistent with the $0.82B stated. This arithmetic does not reconcile. P2 flag.
[I-08] No sensitivity analysis for discount rate on the portfolio NPV. All NPV figures use 7%; a table showing 5% and 10% cases would substantially strengthen the investment case.

MINOR CONCERNS:
[I-09] "Portfolio payback period at undiscounted cash flows is approximately 2.1 years from Phase 1 completion" (§05) — this is not portfolio payback; it is the payback for one project treated as representative. Clarify.

---

**REFEREE 3 — R-Policy** (Robert Puentes archetype — Transport Policy)
Recommendation: **Accept with Minor Revisions** (after P1 fixes)

SUMMARY: The policy recommendation — add B1×D1 compound score to PROTECT grant criteria — is concrete, actionable, and within existing FHWA statutory authority. The paper correctly identifies that PROTECT currently evaluates only D1-equivalent criteria. The Phase 1–4 sequencing is credible given IIJA funding structure. Cleanup of the internal number inconsistency is required before publication.

MAJOR CONCERNS:
[I-10] The abstract describes the Donner tunnel's "dual-benefit structure" without stating the corrected NPV. A reader who reads only the abstract gets no NPV figure; §07 gives the wrong one. The abstract should state $12.1B.

MINOR CONCERNS:
[I-11] The FAST Act critical infrastructure designation (§06) cites 49 U.S.C. §70101. Verify that this authority applies to highway infrastructure specifically (not just pipeline/pipeline infrastructure). The standard FHWA critical infrastructure authority is in 23 U.S.C. §101 et seq.

---

## PHASE 5 — ABSTRACT CHECK

```
ABSTRACT: ~135 words
Primary result stated: PARTIAL — "$3.1 billion in annual freight disruption cost" (top-4 total)
  but Donner NPV ($12.1B) is NOT stated in abstract. The abstract describes the compound concept
  and qualitatively states NPV superiority but gives no corrected NPV figure.
Method named: YES ("B1 score >7 and D1 score >6")
Policy implication: YES (investment in compound-exposure corridors yields higher NPV)
Track chain position: YES (references B1 and D1)
Word count target: 150–200 words; current ~135 — slightly short
```

**Recommendation**: Add one sentence to the abstract: "The highest-priority compound exposure investment, a Donner Pass freight tunnel at \$4 billion, has an estimated 30-year NPV of \$12.1 billion at a 7\% discount rate, a 4.0:1 cost-benefit ratio, and a 3.1-year simple payback."

---

## PHASE 6 — INTERNAL CROSS-PAPER CONSISTENCY

```
CROSS-PAPER CONSISTENCY:
  Papers cited: ROUTE_A2 (rubric), ROUTE_B1 (B1 isolation), ROUTE_D1 (D1 climate), ROUTE_B4 (diamond), ROUTE_C2 (max-flow)
  Values cross-checked: 4/5
  
  - ROUTE_D1 cited for D1 scores: correct — B.3 uses D.1's D1 dimension scores
  - ROUTE_B1 cited for B1 isolation scores: correct
  - ROUTE_A2 cited for v1.1 rubric: correct
  - ROUTE_B4 cited for diamond concept: correct
  - ROUTE_C2 citation in §07 conclusion ("Future research will extend...") is forward-looking — no current value claimed; no check needed

Stale citations (pre-correction): NONE in cross-paper citations — the inconsistency is internal to B.3, not a cross-paper stale citation.

B.3 NPV for cross-paper purposes: ROUTE_B3 entry in references.bib should reflect $12.1B (not $15.8B). Check whether E.2 or other papers cite ROUTE_B3 for the NPV figure — those would need updating.
```

---

## PHASE 7 — PRE-PANEL CHECKLIST

```
═══════════════════════════════════════════════════════
POST-WRITE COMPLETE: B.3+resilience-holes
═══════════════════════════════════════════════════════

Validation results:
  Consistency:       FAIL — 4 P1 failures (§01, §04, §05, §07 retain old $1.6B/$15B figures)
  Contract:          PARTIAL — no plan.md; paper delivers stated contributions except for stale numbers
  Referee sim:       Major Revision (R-Traffic, R-Economics); Accept with minor revisions (R-Policy)
  Abstract:          ~135 words, no NPV figure stated, section-chain position clear
  Cross-paper:       PASS — no stale cross-paper citations found

P1 blockers (fix before panel review):
[I-01/I-02] §01 intro and §04 Table 3: "$1.6B" / "$1.60B" annual disruption cost →
  Fix: change to "$1.3B" in both locations. The corrected breakdown: $0.9B rerouting (B1 benefit)
  + $0.4B waiting (D1 benefit, at $91/hr idle) = $1.3B total.
[I-03/I-04] §05 sequencing §5.2: "despite Donner's $1.6B annual disruption cost" →
  Fix: change to "$1.3B"
[I-05] §07 conclusion key finding 2: three sub-fixes required:
  (a) "$1.6 billion in annual avoided disruption" → "$1.3 billion"
  (b) "30-year NPV at a 7% discount rate exceeds $15 billion" → "is $12.1 billion"
  (c) "simple payback period is 2.5 years" → "simple payback period is 3.1 years"
  (= $4.0B cost / $1.3B annual benefit)
[I-06] Abstract: add corrected NPV sentence ($12.1B, 4.0:1 CBR, 3.1-year payback)

P2 items (should fix):
[I-07] §03 Gulf Coast NPV arithmetic: check $0.82B/yr × 12.4 = $10.2B PV → NPV = $7.3B, but
  table shows $5.8B. Either the annual benefit is lower (~$0.70B/yr) or the NPV calc uses
  a shorter horizon. Reconcile the table footnote with the text.
[I-08] Add 5%/10% discount rate sensitivity columns to portfolio NPV table (Table 2)
[I-09] No sensitivity analysis for compound threshold (B1>7 AND D1>6 vs. alternatives)

P3 items (optional polish):
- Abstract: increase word count to 150+ words
- §05 "portfolio payback" language is ambiguous — clarify it means Donner-only undiscounted
- §06 FAST Act citation: verify 49 U.S.C. §70101 vs. 23 U.S.C. highway authority

PRE-PANEL CHECKLIST:
□ All P1 consistency failures resolved (§01, §04, §05, §07 updated to $1.3B/$12.1B/3.1yr)
□ MODULE.md primary quantitative contract delivered in paper (verify after plan.md created)
□ BPR extrapolation acknowledged where V/C > 1.3 (not applicable — paper uses ATRI costs, not BPR)
□ Net vs gross cost clearly stated (not conflated) — PASS: tunnel cost vs. annual benefits are separate
□ All \citep{} keys exist in references.bib — PASS for keys used: ROUTE_A2, ROUTE_B1, ROUTE_B4,
  ROUTE_D1, ROUTE_C2, ATRI_costs2024, ATRI2024, Caltrans2023, NOAA_SLR2022, NOAA_ClimateAtlas2022
  NOTE: NOAA_ClimateAtlas2022 cited in §04 — NOT found in references.bib. P2 fix needed.
□ Cross-paper citations use corrected values (B.3 NPV = $12.1B) — internal consistency first
□ Rubric version tagged (v1.1 in §04 per corpus reference) — PASS
□ Abstract states primary quantitative result — FAIL (no NPV cited; add $12.1B sentence)
□ Referee P1 blockers addressed

VERDICT: FIXES REQUIRED
Fixes required: 6 P1, 3 P2 (including NOAA_ClimateAtlas2022 bib entry missing)
Next: Fix P1 items, then run /panel:publication review B.3+resilience-holes
═══════════════════════════════════════════════════════
```
