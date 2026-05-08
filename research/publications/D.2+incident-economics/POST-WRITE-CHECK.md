---
paper: D.2+incident-economics
title: "The Economics of Corridor Closures: Freight Cost and Redundancy Value"
post_write_date: 2026-05-08
rubric_version: v1.2
pipeline_stage: ready (Round 1 recheck passed — both blocking reviewers passed)
---

# POST-WRITE PIPELINE — D.2+incident-economics

---

## PHASE 1 — PAPER INVENTORY

```
Paper: D.2+incident-economics
Sections found: 01-introduction.tex, 02-background.tex, 03-methods.tex,
                04-closure-costs.tex, 05-redundancy-value.tex,
                06-investment-implications.tex, 07-conclusion.tex
Plan found: no (no plan.md in directory)
Track: D — Climate and Incident Exposure
Venue: Transportation Research Part E or Journal of Transport Economics and Policy
Key claims:
  1. Top-5 corridors account for $5.4B (central) / $5.1B (suppression-adjusted) in
     annual freight disruption cost (§04-closure-costs, abstract, §07-conclusion)
  2. Donner Pass costs 4.2× more per closure event than the Dallas interchange
     (§04, §01-introduction, §07-conclusion)
  3. Donner Pass redundancy value = $1.9B/yr from a hypothetical I-70W alternate
     (§05-redundancy-value, §07-conclusion)
Primary number (from MODULE.md contract): "Top-5 closure annual cost $Y billion;
  break-even $Z billion"
Paper's stated primary number: $5.4B central; $5.1B with demand suppression;
  Donner redundancy $1.9B/yr; snowshed $0.8B capital, $0.96B/yr benefit
Match: YES — module contract fulfilled; suppression-corrected $5.1B is the
  "results throughout" figure per the text.
```

---

## PHASE 2 — QUANTITATIVE CONSISTENCY CHECK

| Q-ID | Quantity | Abstract | §Intro | §Methods | §04 Table/Text | §07-Conclusion | Consistent? |
|------|----------|---------|--------|----------|----------------|----------------|-------------|
| Q-01 | Top-5 annual cost (central) | $5.4B | $5.4B ("2.4B+1.2B+0.8B+0.6B+0.4B") | — | $5.40B (Table 1 total) | "$5.4 billion" | FAIL — abstract and conclusion use $5.4B; text says "results throughout are reported with suppression applied" ($5.1B). Table shows $5.40 without suppression. See P1 blocker. |
| Q-02 | Donner annual cost | $2.4B | $2.4B | — | $2.40B | $2.4B | PASS |
| Q-03 | Gulf Coast LA annual cost | $1.2B | $1.2B | — | $1.20B | $1.2B | PASS |
| Q-04 | Dallas interchange cost | $0.8B | $0.8B | — | $0.80B | $0.8B | PASS |
| Q-05 | I-95 Baltimore cost | $0.6B | $0.6B | — | $0.60B | — | PASS |
| Q-06 | I-35 Oklahoma cost | $0.4B | $0.4B | — | $0.40B | — | PASS |
| Q-07 | Suppression-adjusted top-5 | — | — | — | $5.1B (in-text, §04 after Demand Suppression para) | — | WARN — $5.1B is the corrected number but not restated in abstract or conclusion |
| Q-08 | Donner: 4.2× per-event vs. Dallas | Abstract | §01-intro | — | §04 (clarification: "per event, not per closure-hour") | §07-conclusion | PASS — consistent; important clarification in §04 that 4.2× is per-event not per-hour |
| Q-09 | Donner frequency: 50/yr | — | — | §03-methods | §04 | — | PASS |
| Q-10 | Donner mean duration: 18 hr | — | — | §03-methods | §04 | — | PASS |
| Q-11 | Donner B1 penalty multiplier: 6.5 | — | — | §03-methods | §04 | — | PASS |
| Q-12 | Donner detour distance: 550 mi | — | — | §03-methods | §04 | — | PASS |
| Q-13 | ATRI truck cost rate: $225/hr | — | — | §03-methods | §04 | — | PASS |
| Q-14 | Switch duration d* = 7.06/7.1 hr | — | — | 7.06 hr | 7.1 hr | — | WARN (rounding: §03 says 7.06, §04 says 7.1 — acceptable) |
| Q-15 | Demand suppression φ(d): 40% at 24hr | — | — | — | §04 (Demand Suppression subsection) | — | PASS — present, formula documented |
| Q-16 | Corrected top-5 with suppression: $5.1B | — | — | — | $5.1B (stated in §04 Demand Suppression para) | — | FAIL — $5.1B only appears in §04; not in abstract or conclusion |
| Q-17 | Donner redundancy value: $1.9B/yr | — | — | — | §05 | §07-conclusion | PASS |
| Q-18 | Snowshed: $0.8B capital, 0.83yr payback | — | — | — | §05 (implied) | §07-conclusion | PASS (conclusion states these explicitly) |
| Q-19 | Top-15 total: $6.2B / $6.15B | Abstract: $6.2B | — | — | Table: $6.15B | — | WARN ($6.2B in abstract vs. $6.15B in table — round to $6.2B in table or state "approximately $6.2B" in abstract) |
| Q-20 | Sensitivity table: central $5.4B | — | — | — | Table 2 (sensitivity) | — | PASS (sensitivity table present) |

**KEY FIX CHECK — sensitivity table in §04:**
- Two-way sensitivity table present: YES (Table 2: ATRI cost × closure frequency)
- Rows/columns: ATRI cost (±20%), closure frequency (±30%) — PASS
- Central estimate $5.4B in table: YES
- φ(d) demand suppression correction in §04: YES (Demand Suppression Adjustment subsection)
- φ(d) formula documented: YES (Eq. present; φ(d) = 1.0 − 0.4 × min(d/24h, 1.0))
- Corrected top-5 total = $5.1B: YES (stated in §04 Demand Suppression para)

**CRITICAL CONSISTENCY FAILURE:**
The abstract states "$5.4 billion" as top-5 total. The §04 text says "Results throughout are
reported with suppression applied" and derives $5.1B as the corrected number. The Table 1
total shows $5.40B (unadjusted). The conclusion says "$5.4 billion" (unadjusted). This creates
a direct contradiction: the abstract/conclusion cite the pre-suppression number while the
methods section says all results use the suppression-adjusted number.

```
CONSISTENCY: FAIL — 2 failures (Q-01, Q-16: $5.4B vs $5.1B inconsistency across sections)
             1 warning (Q-07: $5.1B not propagated to abstract/conclusion)
             1 warning (Q-19: $6.2B vs $6.15B rounding)
P1 (must fix):
  - Abstract, Table 1 total row, and conclusion must align: if "results throughout are
    reported with suppression applied" is the claim, then $5.1B must appear in abstract
    and conclusion, and Table 1 should note the adjustment or show both figures.
    Options: (a) Change abstract/conclusion to $5.1B and add "(suppression-adjusted;
    $5.4B without suppression)" note; or (b) change "results throughout" to "except
    for the corridor-level table" and clearly demarcate which number is the headline claim.
P2 (should fix):
  - Q-19: Abstract $6.2B vs Table $6.15B — round table to $6.2B or abstract to $6.15B
  - Q-14: §03 d* = 7.06 hr vs §04 d* = 7.1 hr — minor rounding; should be consistent
P3 (minor):
  - φ(d) saturation at 24hr (plateau at 60%) is not cited; should reference FHWA incident
    management guidance (per Neumark P3 note in RECHECK-SYNTHESIS)
```

---

## PHASE 3 — CONTRACT CHECK

| Promise (from MODULE.md) | Paper section | Delivered? | Gap |
|--------------------------|---------------|-----------|-----|
| Top-5 closure annual cost $Y billion | §04, abstract, conclusion | YES | ✓ |
| Break-even $Z billion (redundancy value) | §05, §07 | YES (Donner $1.9B/yr) | ✓ |
| Freight value × closure × detour model | §03-methods (Eq. 1-3) | YES | ✓ |
| B1 isolation penalty incorporated | §03-methods (§03 B1 subsection) | YES | ✓ |
| Redundancy investment case | §05, §07 | YES | ✓ |
| Sensitivity analysis (KEY FIX) | §04-closure-costs (Table 2) | YES | ✓ |
| Two-way sensitivity: ATRI × frequency | §04-closure-costs (Table 2) | YES | ✓ |
| Demand suppression φ(d) (KEY FIX) | §04-closure-costs (Demand Suppression para) | YES | ✓ |
| Corrected top-5 = $5.1B (KEY FIX) | §04 (in text, not in abstract/conclusion) | PARTIAL — present in §04 but not propagated to abstract/conclusion | ✗ |
| Policy recommendation | §07-conclusion | YES | ✓ |

```
CONTRACT: PARTIAL
Promises kept: 9/10
Gaps:
  - $5.1B corrected total is present in §04 text but not propagated to abstract and
    conclusion, which still show the pre-suppression $5.4B figure. This is the same
    inconsistency flagged in Phase 2. Fix required.
MODULE.md primary number delivered: YES (top-5 annual cost delivered; $5.1B with
  suppression; $5.4B without)
```

---

## PHASE 4 — REFEREE SIMULATION

Selected referees: R-Economics, R-Traffic, R-Policy

---

**REFEREE 1 — R-Economics (Neumark archetype)**
Recommendation: Minor Revision

SUMMARY: The closure cost model is well-specified and the sensitivity analysis is now present
(Table 2). The main outstanding issue is the inconsistency between the $5.4B cited in the
abstract/conclusion and the $5.1B stated as the suppression-adjusted result in §04. A paper
cannot simultaneously say "results throughout are reported with suppression applied" and then
present $5.4B as the headline in the abstract. This is a fixable inconsistency, not a
methodological error.

MAJOR CONCERNS:
[I-01] Abstract/§07-conclusion cite $5.4B; §04 methods/text establish $5.1B as
  the suppression-adjusted headline. These must be reconciled. The paper cannot
  have two different headline numbers for the same quantity. Recommend: abstract
  and conclusion adopt $5.1B with a parenthetical noting the $5.4B pre-suppression
  estimate.
[I-02] Table 1 "Top-5 total" row shows $5.40B without indicating whether this is
  pre- or post-suppression. Given that §04 says results throughout use suppression,
  the table should either show $5.1B or add a footnote "Pre-suppression; see §4.5
  for suppression-adjusted estimates."

MINOR CONCERNS:
- φ(d) parameterization (40% rerouted at 24hr) lacks citation; add FHWA incident
  management guidance or empirical support.

---

**REFEREE 2 — R-Traffic (Elefteriadou archetype)**
Recommendation: Accept (with minor revisions)

SUMMARY: Lognormal validation is now documented (§03-methods), including Shapiro-Wilk
results for Donner (p=0.23) and Dallas (p=0.11). The B1 isolation multiplier is clearly
specified and the switch-duration computation is tractable. No major HCM-level concerns.

MAJOR CONCERNS: none blocking

MINOR CONCERNS:
[I-03] §04 Dallas section: "Dallas costs more per closure-hour" but the abstract says
  "Donner costs 4.2× more per event." These are not contradictory but a reader could
  misread. The paper has a clarifying paragraph in §04 — this should be cross-referenced
  from the abstract or introduction as a caveat.
[I-04] Gulf Coast I-10 uses triangular distribution (not lognormal) for hurricane closures
  due to n=8. The ±12% sensitivity to distribution choice is stated; this is adequate
  for publication.

---

**REFEREE 3 — R-Policy (Puentes archetype)**
Recommendation: Minor Revision

SUMMARY: The policy recommendation to compute redundancy value as a standard FHWA investment
criterion is well-specified and actionable. The paper correctly identifies that FHWA already
has the data inputs (incident database, HPMS, network graph). One gap: the paper does not
specify whether redundancy value computation would require regulatory change or could be
implemented administratively.

MAJOR CONCERNS: none blocking

MINOR CONCERNS:
[I-05] §07-conclusion: Policy recommendation states FHWA "should compute redundancy value"
  but doesn't specify the program vehicle (NHPP guidance update? FHWA technical advisory?).
  A single sentence on implementation pathway would sharpen the recommendation.

---

## PHASE 5 — ABSTRACT CHECK

```
ABSTRACT: ~220 words
Primary result stated: YES — "$5.4 billion in annual freight disruption cost" (NOTE: this
  is the pre-suppression number; KEY FIX issue — should be $5.1B suppression-adjusted
  OR clearly noted as pre-suppression)
Method named: YES — "closure frequency (from historical data), closure duration
  distribution, freight volume at closure point, and rerouting penalty per truck-hour"
Policy implication: YES — "compound exposure investment strategy over single-dimension
  congestion relief" (\citep{ROUTE_B3})
Track chain position: YES — cites ROUTE_B3, linking to Track B compound exposure finding
```

---

## PHASE 6 — INTERNAL CROSS-PAPER CONSISTENCY

```
CROSS-PAPER CONSISTENCY:
  Papers cited from D.2: ROUTE_B3 (compound exposure), ROUTE_B1 (isolation penalty),
    ROUTE_D1 (D1 exposure scores), FHWA_incident2022, Caltrans2023, ATRI_costs2024
  Values cross-checked:
    - D.2 uses Donner D1=7.8 implicitly (900 closure-hours matches D.1 §03); consistent
    - E.2 §03-investment-portfolio cites "$3.9B/yr in closure cost reduction" for
      compound hardening (from D.2) — consistent with D.2's $5.1B–5.4B range
    - E.2 §01-intro cites "$6.2 billion per year in closure costs" from D.2 — matches
      D.2 top-15 total ($6.15B rounded to $6.2B); consistent
    - D.2 §07-conclusion cites E.1 and E.2 as downstream users of the closure cost model:
      \citep{ROUTE_D1} used for D.2 self-cite — WARN (see D.1 cross-paper check)
  Stale citations (pre-correction):
    - No B.3 NPV correction issue ($12.1B) applies here; D.2 doesn't cite B.3 NPV
    - E.1 NPV correction ($115B→$101B): D.2 does not cite E.1 NPV, so no issue
    - ATRI cost rate: D.2 uses $225/hr consistently — matches ATRI_costs2024
```

---

## PHASE 7 — PRE-PANEL CHECKLIST

```
═══════════════════════════════════════════════════════
POST-WRITE COMPLETE: D.2+incident-economics
═══════════════════════════════════════════════════════

Validation results:
  Consistency:       FAIL — $5.4B (abstract/conclusion) vs. $5.1B (§04 suppression-adjusted)
                     contradiction. All other numbers consistent.
  Contract:          PARTIAL — 9/10 promises; $5.1B not propagated to abstract/conclusion
  Referee sim:       Minor Revision (R-Economics P1: reconcile $5.4/$5.1B);
                     Accept (R-Traffic); Minor Revision (R-Policy legislative pathway)
  Abstract:          ~220 words; primary result stated ($5.4B — but this needs correction
                     to $5.1B per KEY FIX requirement)
  Cross-paper:       PASS — no stale citations; E.2 cite of "$6.2B" consistent with D.2

KEY FIX VERIFICATION:
  [x] Sensitivity table present in §04: CONFIRMED (Table 2: two-way ATRI cost × closure freq)
  [x] Demand suppression φ(d) correction in §04: CONFIRMED (Demand Suppression subsection)
  [x] φ(d) formula documented: CONFIRMED (φ(d) = 1.0 − 0.4 × min(d/24h, 1.0))
  [PARTIAL] Corrected top-5 total = $5.1B: CONFIRMED IN §04 TEXT but NOT in abstract/conclusion
  [FAIL] Abstract still shows $5.4B — must be updated to $5.1B

P1 blockers (fix before panel review):
  [I-01] Abstract shows $5.4B headline; §04 establishes $5.1B as suppression-adjusted.
          Fix: Update abstract to "$5.1 billion (suppression-adjusted; $5.4B without
          demand suppression)". Update §07-conclusion similarly.
  [I-02] Table 1 "Top-5 total" = $5.40B without suppression footnote. Fix: add footnote
          "Pre-suppression estimate. Suppression-adjusted total: $5.1B (see §4.5)." or
          update table values to reflect suppression-adjusted figures by corridor.

P2 items (should fix):
  [Q-19] Abstract "$6.2B" vs Table "$6.15B" — round table total to $6.2B
  [Q-14] §03 d*=7.06 vs §04 d*=7.1 — use 7.1 consistently
  [I-05] Policy recommendation in §07: add one sentence on FHWA program vehicle for
          redundancy value computation
  [φ-cite] φ(d) 40%-at-24hr: add FHWA citation for advance rerouting behavior parameter

P3 items (optional polish):
  - Abstract clarification: note that 4.2× is per-event not per-closure-hour (I-03)
  - Extreme-duration distribution sensitivity (Pareto test) — Elefteriadou P3 note
  - Compound closure correlation analysis for Donner redundancy value (REVISION-PLAN P2.1)

PRE-PANEL CHECKLIST:
[x] KEY FIX — sensitivity table (two-way: ATRI × frequency) in §04: PRESENT
[x] KEY FIX — φ(d) demand suppression correction in §04: PRESENT
[ ] KEY FIX — corrected top-5 total = $5.1B propagated to abstract: NOT DONE (abstract shows $5.4B)
[ ] P1 — Table 1 total row updated or footnoted: NOT DONE
[x] MODULE.md primary quantitative contract delivered: PASS
[x] Net vs gross cost clearly stated: YES — expected annual cost model is explicit
[ ] All \citep{} keys verified in references.bib: NOT VERIFIED
[x] Cross-paper citations use corrected values: PASS (B.3 $12.1B not cited; E.1 not cited)
[x] Rubric version tagged: YES — "ROUTE v1.2 corpus" referenced
[ ] Abstract states primary quantitative result with suppression adjustment: NOT YET ($5.1B missing)
[ ] Referee P1 blockers addressed (I-01, I-02): NOT YET DONE

VERDICT: FIXES REQUIRED
Fixes required: 2 (P1: abstract + table total reconciliation to $5.1B)
Next: Correct abstract/$5.1B propagation, then run /panel:publication review D.2+incident-economics
═══════════════════════════════════════════════════════
```
