---
paper: D.1+climate-exposure
title: "Climate Exposure in the Interstate System: FEMA Flood Zones and 2050 Projections"
post_write_date: 2026-05-08
rubric_version: v1.2 (scored); v1.3 amendment documented in §03-methods
pipeline_stage: ready (Round 1 recheck passed)
---

# POST-WRITE PIPELINE — D.1+climate-exposure

---

## PHASE 1 — PAPER INVENTORY

```
Paper: D.1+climate-exposure
Sections found: 01-introduction.tex, 02-background.tex, 03-methods.tex,
                04-flood-exposure.tex, 05-winter-wildfire.tex,
                06-2050-projections.tex, 07-conclusion.tex
Plan found: no (no plan.md in directory)
Track: D — Climate and Incident Exposure
Venue: Transportation Research Part D (or equivalent resilience/climate track)
Key claims:
  1. I-10 Gulf Coast Louisiana has 127 consecutive SFHA miles — most flood-exposed
     corridor in the national system (§01-introduction, §04-flood-exposure, §07-conclusion)
  2. I-80 Donner Pass D1 = 7.8 on 900 annual closure-hours: highest winter-closure
     corridor (§03-methods, §07-conclusion; abstract)
  3. Under NOAA Intermediate SLR (0.5m by 2050), Gulf Coast I-10 Louisiana rises
     from D1 = 8.4 to projected D1 = 9.1 (abstract, §06-2050-projections, §07-conclusion)
Primary number (from MODULE.md contract): X corridor-miles SFHA; top-3 by max consecutive miles
Paper's stated primary number: 127 consecutive SFHA miles (Gulf Coast LA); D1 scores 8.4 → 9.1
Match: PARTIAL — module says "X corridor-miles SFHA" (placeholder); paper delivers specific
  numbers as intended. Module contract is fulfilled; the placeholder was never filled in MODULE.md.
```

---

## PHASE 2 — QUANTITATIVE CONSISTENCY CHECK

| Q-ID | Quantity | Abstract | §Intro | §Methods | §Conclusion | Consistent? |
|------|----------|---------|--------|----------|-------------|-------------|
| Q-01 | Gulf Coast LA consecutive SFHA miles | — | 127 mi | 127 mi (ECH100 calc) | 127 mi | PASS |
| Q-02 | I-10 LA current D1 score | 8.4 | 8.4 | 8.4 (recovered via μ=5.0) | 8.4 | PASS |
| Q-03 | I-10 LA 2050 projected D1 | 9.1 | 9.1 | §projections | 9.1 | PASS |
| Q-04 | Donner D1 score | 7.8 | 7.8 | 7.8 (δ=0.78 × 10) | 7.8 | PASS |
| Q-05 | Donner closure-hours/yr | — | 900 hr/yr | 900 hr/yr | 900 hr/yr | PASS |
| Q-06 | Structural multiplier μ | — | — | 5.0 (§03-methods, sec:d1-normalize) | — | PASS |
| Q-07 | Recovery discount δ (Donner) | — | — | 0.78 (§03-methods, sec:d1-normalize) | — | PASS |
| Q-08 | ECH100 anchor: 50 hr/100mi → D1=5 | — | — | present in Eq. | — | PASS |
| Q-09 | 70%/30% flood weighting (f_c/f_t) | 70%/30% | — | Eq. (1) | — | PASS |
| Q-10 | Five highest-scoring D1 corridors | listed | — | — | three listed | WARN (abstract lists 5; conclusion discusses top 3 — acceptable) |
| Q-11 | NOAA SLR intermediate: 0.5m by 2050 | 0.5m | 0.5m | 0.5m | 0.5m | PASS |
| Q-12 | Louisiana subsidence rate | — | "1–2 cm/yr" | 1.5 cm/yr median | — | WARN (intro says 1–2 cm/yr, methods uses 1.5 cm/yr; should align to "1.5 cm/yr median") |
| Q-13 | Gulf Coast effective local SLR 2050 | — | — | 0.875m | — | PASS (isolated in §projections) |
| Q-14 | PROTECT program funding | $8.7B | $8.7B | — | $8.7B | PASS |
| Q-15 | Number of corridors scored | 227 | 227 | — | — | PASS |
| Q-16 | I-35 Oklahoma D1 | 7.2 | — | — | — | PASS (abstract only; not restated in conclusion — minor) |
| Q-17 | I-90 Snoqualmie D1 | 7.1 | — | — | — | PASS (abstract only) |
| Q-18 | I-10 Gulf Coast TX D1 | 7.8 | — | — | — | PASS (abstract only) |

**KEY FIX CHECK — ECH100 normalization in §03-methods:**
- ECH100 subsection present: YES (sec:d1-normalize label present at \subsection)
- μ=5.0 structural multiplier documented: YES ("structural multiplier μ = 5.0" stated explicitly)
- δ=0.78 recovery discount documented: YES ("recovery-time discount factor δ = 0.78 is applied to Donner Pass")
- Both yield D1=7.8 for Donner consistently: PASS
- Both yield D1=8.4 for Gulf Coast LA consistently: PASS

```
CONSISTENCY: PASS — 1 minor warning (subsidence notation)
P1 (must fix): none
P2 (should fix):
  - Q-12: Align intro "1–2 cm/yr" subsidence language with methods "1.5 cm/yr median"
P3 (minor):
  - Abstract lists 5 highest-scoring corridors; conclusion discusses only top 3 by name;
    acceptable but a brief reconciliation note in conclusion would improve traceability
```

---

## PHASE 3 — CONTRACT CHECK

| Promise (from MODULE.md) | Paper section | Delivered? | Gap |
|--------------------------|---------------|-----------|-----|
| Score all 227 corridors against D1 dimension | §03-methods, §04, §05 | Yes | ✓ |
| Top-3 by max consecutive SFHA miles | Abstract, §04, §07 | Yes | ✓ |
| Flood zone overlay (FEMA NFHL) | §03-methods | Yes | ✓ |
| 2050 climate projection for top corridors | §06 | Yes | ✓ |
| Investment priority order changed by 2050 | §07-conclusion | Yes | ✓ |
| D1 score anchored on Donner as worst-case winter | §03-methods (winter closure scoring) | Yes | ✓ |
| ECH100 normalization (v1.3 amendment from recheck) | §03-methods sec:d1-normalize | Yes | ✓ |
| μ=5.0 structural multiplier documented | §03-methods sec:d1-normalize | Yes | ✓ |
| δ=0.78 recovery discount documented | §03-methods sec:d1-normalize | Yes | ✓ |
| Policy implication for PROTECT program | §01-intro, §07-conclusion | Yes | ✓ |

```
CONTRACT: PASS
Promises kept: 10/10
Gaps: none
MODULE.md primary number delivered: YES (consecutive SFHA miles prominently stated and
  scored; D1 scores tabulated for top corridors)
```

---

## PHASE 4 — REFEREE SIMULATION

Selected referees: R-Traffic, R-Policy, R-Equity

---

**REFEREE 1 — R-Traffic (Elefteriadou archetype)**
Recommendation: Major Revision

SUMMARY: The D1 dimension is well-motivated, but the v1.3 normalization section introduces
parameters (μ=5.0, δ=0.78) that are calibration choices, not empirically derived coefficients.
The paper acknowledges μ=5.0 is "economically motivated but not empirically calibrated"
(per RECHECK-SYNTHESIS) yet presents the score as a precise quantitative finding. The
methodology section does not fully close this gap. Additionally, the ECH100 flood calculation
uses p_flood = 0.03 (100-year SFHA on 3-event expected frequency) without citing the basis
for 3-event frequency in a 100-year return period event.

MAJOR CONCERNS:
[I-01] §03-methods: μ=5.0 structural multiplier is asserted as principled but not derived.
  The paper says "flood inundation requires replacement of roadway surface, drainage structures,
  and embankments rather than simply reopening a gate" — but a multiplier of 5 implies flood
  costs are 5× winter costs per ECH100 unit. This ratio is not derived from observed cost data.
  Suggest: (a) bound μ with sensitivity analysis (μ=3.0, 5.0, 7.0), or (b) tie μ to a specific
  cited source on relative flood vs. winter closure repair cost ratios.
[I-02] §03-methods (Eq. ech100-flood): p_flood = 0.03 described as "100-year SFHA on a
  3-event expected frequency" — this language conflates return period with frequency. A
  100-year SFHA has annual exceedance probability 0.01. "3-event expected frequency" over what
  period? Clarify the probabilistic interpretation and cite source.

MINOR CONCERNS:
- Louisiana subsidence 1–2 cm/yr in intro vs. 1.5 cm/yr in methods should be reconciled.
- NFHL coverage completeness (P1.3 from REVISION-PLAN) not fully addressed in current text;
  still flagged as a P1 item from Round 1.

---

**REFEREE 2 — R-Policy (Puentes archetype)**
Recommendation: Minor Revision

SUMMARY: Strong policy framing and the consecutive-SFHA-miles distinction is genuinely useful
for PROTECT program reform. The 2050 projection methodology is clearly explained. The main gap
is that the paper does not specify whether the PROTECT formula change pathway is regulatory
(FHWA rulemaking, ~3-5 years) or statutory (requires legislation) — this was P2.3 in the
REVISION-PLAN and remains unaddressed in the current draft.

MAJOR CONCERNS:
[I-03] §07-conclusion: The policy recommendation states PROTECT criteria "should incorporate"
  consecutive SFHA miles but does not state what legislative or administrative vehicle would
  effectuate this. A one-paragraph note on the rulemaking pathway (or statutory barrier)
  would transform this from aspiration to actionable recommendation.

MINOR CONCERNS:
- PROTECT dollar allocation gap (P2.1: how much funding would shift from total-miles to
  consecutive-miles formula) is not quantified. The argument is qualitative; quantification
  would make it compelling to a Congressional Budget Office audience.

---

**REFEREE 3 — R-Equity (Schmitt/Hanson archetype)**
Recommendation: Accept (with minor revisions)

SUMMARY: The paper correctly identifies Gulf Coast I-10 as the highest-priority climate
adaptation investment in the national system. The equity dimension — who lives in the
inundated corridor — is implied but not stated. The PROTECT program's current formula
(total SFHA miles) likely disadvantages rural and low-income coastal communities who rely
on I-10 for evacuation, but this is not analyzed.

MAJOR CONCERNS: none blocking

MINOR CONCERNS:
[I-04] §07-conclusion: The "lead-time urgency" paragraph is the strongest policy-relevant
  passage, but it does not note who bears the cost of inaction. Gulf Coast Louisiana communities
  (predominantly low-income, high proportion of households without vehicles) face evacuation
  route degradation from rising exposure. A single sentence connecting the D1 score to
  evacuation-route equity would strengthen the policy case without overreaching.

---

## PHASE 5 — ABSTRACT CHECK

```
ABSTRACT: ~220 words (slightly above 200-word target)
Primary result stated: YES — "The five highest-scoring D1 corridors are I-10 Gulf Coast
  Louisiana (8.4)..." and "Gulf Coast I-10 D1 score rises from 8.4 to a projected 9.1"
Method named: YES — "FEMA NFHL for flood zone overlay, NOAA 2050 sea level rise projections...
  USDA/FEMA historical closure frequency data"
Policy implication: YES — "The 2050 exposure profile fundamentally changes the investment
  priority order: Gulf Coast I-10 hardening becomes the most urgent climate adaptation
  investment in the national highway system"
Track chain position: PARTIAL — mentions "D1 dimension of the ROUTE rubric" and v1.2
  corpus; does not explicitly state this is Track D Paper 1 or cite D.2 as companion.
  Acceptable but a brief forward-reference to D.2 would help.
```

---

## PHASE 6 — INTERNAL CROSS-PAPER CONSISTENCY

```
CROSS-PAPER CONSISTENCY:
  Papers cited: ROUTE_A1 (tier classification), ROUTE_D1 (D1 rubric note — self-reference)
  Values cross-checked:
    - D.2 cites D.1 for D1 scores (8.4, 7.8) — PASS (D.1 states these in abstract/conclusion)
    - E.2 cites D.1 for "D1 score of 9.1 (projected to 2050)" in §03-investment-portfolio — PASS
    - D.1 conclusion cites D.2 as companion (§07-conclusion: "The companion paper D.2") — PASS
    - D.1 conclusion incorrectly uses \citep{ROUTE_D1} (self-cite) to reference D.2 — WARN
      (§07-conclusion line "The companion paper D.2 \citep{ROUTE_D1}" — should be ROUTE_D2)
  Stale citations (pre-correction): none identified
  Cross-paper number consistency:
    - D.2 abstract uses $5.4B top-5 total (old, pre-suppression) but text applies suppression → $5.1B
      D.1 does not cite D.2 cost numbers, so no propagation issue.
```

---

## PHASE 7 — PRE-PANEL CHECKLIST

```
═══════════════════════════════════════════════════════
POST-WRITE COMPLETE: D.1+climate-exposure
═══════════════════════════════════════════════════════

Validation results:
  Consistency:       PASS — 1 minor warning (subsidence notation discrepancy intro vs. methods)
  Contract:          PASS — 10/10 promises delivered including KEY FIX items
  Referee sim:       Major Revision (R-Traffic on μ calibration); Minor Revision (R-Policy);
                     Accept (R-Equity)
  Abstract:          ~220 words, primary number stated, method named, policy implication present
  Cross-paper:       1 citation error (self-cite ROUTE_D1 in §07 should be ROUTE_D2 for D.2)

KEY FIX VERIFICATION:
  [x] ECH100 normalization subsection present in §03-methods (sec:d1-normalize) — CONFIRMED
  [x] μ=5.0 structural multiplier documented — CONFIRMED ("structural multiplier μ = 5.0")
  [x] δ=0.78 recovery discount documented — CONFIRMED ("recovery-time discount factor δ = 0.78")
  [x] Both parameters recover the v1.2 D1 scores — CONFIRMED (7.8 for Donner, 8.4 for Gulf LA)

P1 blockers (fix before panel review):
  [I-01] μ=5.0 unsupported as point estimate → Add sensitivity analysis μ ∈ {3.0, 5.0, 7.0}
           showing D1 score range for Donner and Gulf LA; or cite FHWA flood vs. winter
           repair cost ratio literature to ground the multiplier empirically.
  [I-02] p_flood = 0.03 probabilistic framing ambiguous → Clarify: "annual flood event
           probability 0.01 × 3 for multi-storm expectation over 100-year analysis period"
           or restate using actuarial language with citation.
  [CITE] §07-conclusion \citep{ROUTE_D1} should be \citep{ROUTE_D2} (companion paper cite)

P2 items (should fix):
  [I-03] Legislative pathway for PROTECT reform not specified → add rulemaking vs. statutory
           distinction note in §07-conclusion (was P2.3 in REVISION-PLAN, still unaddressed)
  [Q-12] Subsidence language alignment: intro "1–2 cm/yr" → "1.5 cm/yr median" to match §03
  [NFHL] Coverage completeness (REVISION-PLAN P1.3) not present in current text — add
           supplementary note on corridor coverage completeness

P3 items (optional polish):
  - Trim abstract to 200 words; move track-chain position forward reference (mention D.2)
  - Add equity dimension sentence in §07-conclusion on evacuation-route equity (I-04)
  - μ=5.0 future calibration note (from McKinnon RECHECK) — acknowledged but not yet stated
    in paper text itself; add P3 note to §03-methods

PRE-PANEL CHECKLIST:
[x] KEY FIX — ECH100 normalization subsection in §03-methods: PRESENT
[x] KEY FIX — μ=5.0 structural multiplier documented: PRESENT
[x] KEY FIX — δ=0.78 recovery discount documented: PRESENT
[ ] P1 — μ=5.0 sensitivity analysis or empirical grounding: NOT YET DONE
[ ] P1 — p_flood probabilistic framing clarified: NOT YET DONE
[ ] P1 — ROUTE_D1 → ROUTE_D2 citation in §07-conclusion: NOT YET FIXED
[ ] P2 — PROTECT legislative pathway note: NOT YET DONE
[x] MODULE.md primary quantitative contract delivered: PASS (consecutive SFHA miles + scores)
[x] Net vs gross cost clearly stated: N/A (exposure paper, no cost estimates)
[ ] All \citep{} keys exist in references.bib: NOT VERIFIED (bib file not read)
[x] Cross-paper citations use corrected values: PASS (D.2 $5.1B not cited here)
[x] Rubric version tagged: YES — "ROUTE v1.2 corpus" and "v1.3 Amendment" labeled
[x] Abstract states primary quantitative result: PASS
[ ] Referee P1 blockers addressed (I-01, I-02): NOT YET DONE

VERDICT: FIXES REQUIRED
Fixes required: 4 (P1: 3 fixes; 1 citation correction)
Next: Resolve P1 items, then run /panel:publication review D.1+climate-exposure
═══════════════════════════════════════════════════════
```
