---
paper: F.2+intercity-bus-corridors
title: "Intercity Bus Corridors: Travel Time, Coverage, and Equity on the Interstate 2.0 Network"
post_write_date: 2026-05-08
rubric_version: v1.3
---

# POST-WRITE CHECK: F.2 — Intercity Bus Corridors

## PHASE 1 — PAPER INVENTORY

```
Paper: F.2+intercity-bus-corridors
Sections found: 01-introduction.tex, 02-background.tex, 03-methods.tex,
                04-travel-time.tex, 05-operator-economics.tex, 06-equity-impact.tex,
                07-conclusion.tex
Plan found: NO (no plan.md in directory)
Track: F — Transit Integration (F.2, depends on F.1)
Venue: Transportation Research Part A (per MODULE.md Track F)
Key claims:
  1. T1 bus travel times 28–45% shorter than current bus alternatives on every corridor
     (§04-travel-time, Table 5 summary §4.12; §07-conclusion)
  2. 24 million annual passengers at market equilibrium on 12 corridors
     (§03-methods gravity model; §05-economics; §07-conclusion)
  3. Effective average speed 54–58 mph (with stop-penalty model applied)
     (§03-methods "Stop-Penalty Adjustment" paragraph)
Primary number (from MODULE.md contract):
  "I2.0 bus corridor travel time vs. current best alternative on each T1; PTI benefit
   for bus passengers"
Paper's stated primary number: 28–45% improvement; effective speed 54–58 mph; PTI 1.15
Match: YES — travel time comparison delivered for 12 corridors
```

---

## PHASE 2 — QUANTITATIVE CONSISTENCY CHECK

| Q-ID | Quantity | Abstract | §Intro | Table/Body | §Conclusion | Consistent? |
|------|----------|----------|--------|------------|-------------|-------------|
| Q-01 | Travel time improvement range | 28–45% | 28–45% | 28–45% (§04 summary) | 28–45% | PASS |
| Q-02 | Average speed (cruising) | 62 mph | 62 mph | 62 mph (§03 Eq.1) | 62 mph | PASS |
| Q-03 | Effective speed with stops | 54–58 mph | — | 54–58 mph (§03 Stop-Penalty) | — | PASS |
| Q-04 | PTI target (managed lanes) | ≤1.15 | ≤1.15 | 1.15 (§02, §03) | ≤1.15 | PASS |
| Q-05 | Annual passengers (equilibrium) | 24M | 24M | 24M (§03, §05) | 24M | PASS |
| Q-06 | N_stops formula | — | — | floor(d/150) (§03 Eq. near line 34) | — | PASS |
| Q-07 | Stop dwell time | — | — | 8 min (§03 lines 17, 31) | — | PASS (two consistent statements in §03) |
| Q-08 | NYC–Chicago T1 bus time | — | — | 12.6h (§04 calc) | — | PASS |
| Q-09 | Current I-80 PTI | 1.86 | 1.86 | 1.86 (§02, §04) | — | PASS |
| Q-10 | Bus operator jobs (T1 corridors) | — | — | 12,000 (§05 summary) | 12,000 | PASS |
| Q-11 | EIT subsidy cap | — | — | $15/passenger (§05 line 73) | — | NOTE: see below |
| Q-12 | EAS average subsidy | — | — | $175/passenger (§05, §06) | — | PASS (consistent) |
| Q-13 | Houston–Chicago break-even year-1 | — | — | ~$18.46/passenger (slightly above $15 cap) | — | PASS (acknowledged) |

**KEY-FIX STATUS CHECKS:**

**[PASS] Stop-penalty model present in §03**
Section 03-methods.tex contains two formulations of the travel time model:
- Lines 7–13: initial $T_{bus} = d/v_{bus} + N_{stops} \cdot t_{stop}$ (Eq.1, v_bus=62 mph)
- Lines 19–49: "Stop-Penalty Adjustment" paragraph with Eq.2 (same formula restated
  with v_cruising=65 mph and the explicit $N_{stops} = \lfloor d/150 \rfloor$ formula,
  effective speed 54–58 mph derivation)

The $N_{stops} = \lfloor d/150 \rfloor$ formula is present (§03 line 34).
The effective speed 54–58 mph is derived and stated (§03 lines 42–43).

**[WARN — P1] Duplicate formula creates confusion**
Section 03 presents the travel time formula twice with different parameter choices:
- Eq.1 uses $v_{bus} = 62$ mph (already penalized for acceleration/deceleration)
- Eq.2 ("Stop-Penalty Adjustment") uses $v_{cruising} = 65$ mph + explicit stop term

The distinction is that Eq.1 embeds a 3 mph deduction while Eq.2 uses raw cruising
speed. This is explained in text but creates a risk that reviewers will think the
paper's travel time results (from §04) use Eq.1 (62 mph) without the stop penalty,
rather than Eq.2 (65 mph + stop-penalty yielding 54–58 mph effective). Checking §04
corridor calculations: NYC–Chicago uses "780/62 + 3×8/60 = 12.58h" — this uses
v_bus=62 and explicit stop penalty (Eq.1 with the stop term added). Result is
consistent with effective speed = 780/(12.58-0.4) ≈ 64 mph — which does NOT match
the 54–58 mph effective speed range in the Stop-Penalty paragraph.

**The discrepancy:** Eq.2 (Stop-Penalty) yields 54–58 mph for 300–900 mile corridors
using v_cruising=65. Eq.1 applied in §04 uses v_bus=62 and adds explicit stop time,
yielding higher effective speeds than the 54–58 mph claim. Specifically for NYC–Chicago
(780 mi, 3 stops): 780/(780/62 + 24/60) = 780/(12.58+0.4) ≈ 60 mph effective, not 54–58.

Fix: Reconcile Eq.1 and Eq.2. Either (a) use a single formula consistently and update
the effective speed range claim, or (b) clearly state that §04 corridor times use Eq.1
(62 mph with explicit stop time) and that the 54–58 mph range in Eq.2 uses 65 mph
cruising — producing slightly more conservative estimates. The current text implies
§04 results were produced by the stop-penalty model, but the arithmetic uses Eq.1.

**[PASS] "Local Access Assumption" paragraph in §03**
Section 03-methods.tex lines 72–80 contain the "Local Access Assumption" paragraph:
"The ridership gravity model assumes that travelers can reach T1/T1 hub stops. For
general-population travelers with private vehicles, this assumption is reasonable...
For transit-dependent travelers (zero-vehicle households), the model implicitly assumes
local feeder service connects them to the hub. This feeder assumption is not costed
in the operator economics analysis (Section~\ref{sec:operator-economics}); its
omission means the 24 million annual passenger estimate represents an upper bound
for markets where feeder service does not already exist."
Local Access Assumption: PRESENT and correctly scoped.

**[PASS] EIT reframe in §07**
Section 07-conclusion.tex lines 26–28 reference both 49 U.S.C. § 41731 (new authority
required) and 49 U.S.C. § 5311(f) (existing authority, no frequency mandate). The
§ 5311(f) reference is present and correctly characterized.

**[WARN — P2] EIT terminology still used in §05 without § 5311(f) cross-reference**
Section 05-operator-economics.tex lines 71–88 introduce "Essential Intercity
Transportation (EIT)" designation and the $15/passenger cap. The EIT designation is
framed as "modeled on Essential Air Service" but the text does not mention § 5311(f)
here — the closest existing authority is only in §07. A reader building on §05 alone
encounters "EIT" as an invented designation without regulatory grounding.
Fix: Add a cross-reference in §05 noting that "the closest existing statutory basis for
this designation is 49 U.S.C. § 5311(f) (rural intercity bus); an EIT designation as
specified here would require new authorization" — parallel to §07's language.

```
CONSISTENCY: 1 P1 (formula inconsistency) + 1 P2 (EIT in §05)
P1 (must fix):
  - §03 travel time formula inconsistency: Eq.1 (v=62) vs Eq.2 (v=65) produce
    different effective speeds; §04 corridor calculations use Eq.1; the "54–58 mph
    effective speed" claim in the Stop-Penalty paragraph does not match §04's
    arithmetic. Reconcile or annotate clearly.
P2 (should fix):
  - §05 EIT section: add § 5311(f) cross-reference (existing authority) alongside
    the EIT proposal, matching §07's treatment.
P3 (minor):
  - §04 Atlanta–Dallas: "800/62 + 3×8/60 = 13.30h ≈ 12.9h" — the rounding is wrong
    (13.30 rounds to 13.3h, not 12.9h). Table 5 shows 12.9h. Either the in-text
    calculation has an error or the stop count differs from what is stated (the text
    says 3 stops: Birmingham, Meridian, Shreveport). Check and reconcile.
```

---

## PHASE 3 — CONTRACT CHECK

| Promise (from MODULE.md) | Paper section | Delivered? | Gap |
|--------------------------|---------------|-----------|-----|
| Travel time comparison on each T1 corridor | §04 (12 corridors) | YES, Table 5 | ✓ |
| PTI benefit for bus passengers | §02-background, §03 PTI buffer | YES, PTI 1.15 vs 1.86 quantified | ✓ |
| Current best alternative comparison | §04 each corridor | YES | ✓ |
| Amtrak comparison where service exists | §04 + Table 5 | YES, 5 corridors | ✓ |
| N_stops formula (floor(d/150)) | §03 Stop-Penalty | YES (Eq.2 line 34) | ✓ |
| Effective speed 54–58 mph | §03 Stop-Penalty | YES (but inconsistent with §04 calcs) | ~ |
| Local Access Assumption documented | §03 paragraph | YES | ✓ |
| § 5311(f) EIT reframe | §07 | YES | ✓ |
| § 5311(f) in §05 where EIT introduced | §05 | NO — missing | ✗ |

```
CONTRACT: PARTIAL
Promises kept: 8/9
Gaps:
  - §05 EIT introduction does not reference § 5311(f); only §07 conclusion does.
  - Effective speed 54–58 mph claim in §03 not consistent with §04 calculations.
MODULE.md primary number delivered: YES (28–45% improvement demonstrated on 12 corridors)
```

---

## PHASE 4 — REFEREE SIMULATION

**REFEREE 1 — R-Traffic (Elefteriadou archetype)**
Recommendation: Major Revision

SUMMARY: The travel time model is well-structured but the two parallel formulas create
an inconsistency that undermines confidence in the corridor-level results. The §04
calculations appear to use Eq.1 (62 mph effective speed with explicit stop penalty)
while the "Stop-Penalty Adjustment" paragraph in §03 presents Eq.2 as the operative
formula. Until these are reconciled, the numerical results cannot be independently
verified.

MAJOR CONCERNS:
[I-01] §03 travel time formula inconsistency (see P1 above). The paper presents Eq.1
and Eq.2 as if they are the same model, but they use different base speeds (62 mph vs.
65 mph) and produce different effective speeds. The corridor calculations in §04 use
Eq.1 arithmetic (explicitly shown: "780/62 + 3×8/60 = 12.58h"). But the abstract and
introduction claim travel times consistent with "62 mph average speed," which implies
Eq.1 is the canonical formula. The Stop-Penalty paragraph implies Eq.2 is canonical.
Fix: Remove one of the two formulas. Use Eq.2 (v=65 mph, explicit stop penalty) as
the canonical model, recalculate all §04 corridor times using v=65, and update the
"62 mph average" language in the abstract to reflect the actual average speed with stops.
Or retain Eq.1 (v=62 already includes deceleration penalty) and delete the redundant
Stop-Penalty paragraph, replacing it with one clarifying sentence.

[I-02] §04 Atlanta–Dallas arithmetic error: "800/62 + 3×8/60 = 13.30h ≈ 12.9h"
The correct calculation is 800/62 + 24/60 = 12.90 + 0.40 = 13.30h, which does NOT
round to 12.9h. Table 5 shows 12.9h for this corridor. There is an inconsistency
between the in-text calculation and the table value. Fix: correct either the table
entry or the in-text equation result.

MINOR CONCERNS:
- The 8-minute stop dwell time (§03) is described as "3 min deceleration + 3 min
  boarding + 2 min departure" but passenger boarding protocols for ADA-accessible
  coaches at highway hubs often run 5–8 minutes for wheelchair-accessible boarding
  alone. The 3-minute boarding assumption may be optimistic for full accessibility
  compliance. Consider sensitivity analysis at 10 and 12 minutes.

---

**REFEREE 2 — R-Policy (Puentes archetype)**
Recommendation: Minor Revision

SUMMARY: The EIT framing is well-constructed and the $15/passenger cap is well-justified
relative to EAS. The §5311(f) reference in §07 is appropriate. Main gap is the absence
of § 5311(f) in §05 where EIT is first introduced.

MAJOR CONCERNS:
[I-03] §05 EIT introduction (lines 71–88): introduces "Essential Intercity Transportation
(EIT) designation" as a novel policy tool without noting that § 5311(f) is the closest
existing authority and that EIT would require new statutory authorization. The paper
correctly makes this distinction in §07 but not in §05, creating risk that the $15/
passenger subsidy proposal is read as achievable under current law. Fix: one sentence
in §05 after the EIT introduction: "An EIT designation as specified here would require
new statutory authorization; the closest existing authority, 49 U.S.C. § 5311(f) (FTA
rural intercity bus), funds rural intercity connections but does not mandate frequency
or cap fares."

MINOR CONCERNS:
- The managed lane concession agreement recommendation in §07 (open access for buses)
  is strong policy but needs a statutory hook. Federal highway concession agreements
  are governed by 23 U.S.C. § 156; bus access requirements as a concession condition
  should cite this or note legislative gap.

---

**REFEREE 3 — R-Economics (Neumark archetype)**
Recommendation: Minor Revision

SUMMARY: The operator economics are correct and the break-even arithmetic is solid.
The key risk is the distance-invariance claim: the paper correctly notes that break-even
load factor is independent of route distance in the linear cost model, but this is a
strong assumption. Driver wages per mile are approximately constant (solo long-haul),
but per-mile fuel costs vary with terrain, weather, and speed — factors that affect the
mountain corridors (Denver–Salt Lake) differently than flat corridors.

MAJOR CONCERNS:
[I-04] §05 break-even analysis: the distance-invariance finding ("Break-even load factor
is distance-invariant in this model") is presented as a general result but only holds
when $c_{mile}$ (cost per bus-mile) is constant. For the Denver–Salt Lake City corridor
(mountain grades, weather), $c_{mile}$ is likely 15–20% higher than the flat-corridor
$2.80/mile used in the model. This would raise the break-even load factor for this
corridor specifically. The paper correctly identifies Denver–Salt Lake as a marginal
corridor projected to need EIT subsidy, but the subsidy estimate is based on the same
flat-rate cost model. Fix: add a mountain-corridor cost premium note for Denver–Salt
Lake (and potentially Seattle–Portland for the Cascades segment).

MINOR CONCERNS:
- 12,000 direct jobs from T1 bus corridors needs a denominator: current US intercity
  bus industry employs ~24,000. Adding 12,000 is a 50% expansion (noted in §05).
  The 24,000 figure should be cited with year (the paper cites BTS_intermodal2023;
  confirm this source provides the employment figure specifically).

---

## PHASE 5 — ABSTRACT CHECK

Abstract word count: ~175 words
Primary result stated: YES — "I2.0 bus travel times are 28–45% faster than current
  bus alternatives on every corridor"
Method named: YES — "We analyze 12 T1 bus corridors enabled by the Interstate 2.0
  hub network (F.1)"
Policy implication: YES — "$0.12/mile, T1 express bus is the lowest-cost intercity
  travel option in its markets"
Track chain position: YES — explicitly cites F.1 hub network as dependency

```
ABSTRACT: ~175 words (within 150–200 target)
Primary result stated: YES — 28–45% improvement, 24M passengers
Method named: YES
Policy implication: YES ($0.12/mile fare, EIT for marginal corridors)
Track chain position: YES (F.2 builds on F.1 hub network)
```

---

## PHASE 6 — CROSS-PAPER CONSISTENCY

Papers cited in F.2:
- ROUTE_F1 (F.1): F.2 cites F.1 for "12.4 million transit-dependent Americans within
  30 miles of a hub at hub increment cost of $2 billion (0.8% of total I2.0)."
  F.1 abstract/§07 confirms: 12.4M, $2B, 0.8%. PASS.
- ROUTE_E2 (E.2): cites "PTI target of ≤1.15, sustained at 65 mph, separated from
  general-purpose traffic." E.2 §01 confirms I2.0 PTI design target. PASS.
- ROUTE_C1 (C.1): cites "PTI 1.86 on I-80 between the Midwest and the coasts."
  C.1 section 01 references PTI; verify against C.1's §04 or §06.
- ROUTE_B1 (B.1): Houston–Chicago dependency on I-69 completion cited.

```
CROSS-PAPER CONSISTENCY:
  Papers cited: ROUTE_F1, ROUTE_E2, ROUTE_C1, ROUTE_B1, plus external
  Values cross-checked: F.1 figures confirmed; E.2 PTI target confirmed
  Stale citations: None identified
```

---

## PHASE 7 — PRE-PANEL CHECKLIST

```
═══════════════════════════════════════════════════════
POST-WRITE COMPLETE: F.2+intercity-bus-corridors
═══════════════════════════════════════════════════════

Validation results:
  Consistency:       1 P1 (formula inconsistency) + 1 P2 + 1 P3
  Contract:          PARTIAL — §05 EIT missing § 5311(f); effective speed
                     claim inconsistent with §04 calculations
  Referee sim:       Major Revision from R-Traffic (formula); Minor from others
  Abstract:          ~175 words; primary result stated; no P1 issues in abstract itself
  Cross-paper:       PASS

P1 blockers (fix before panel review):
[I-01] §03 travel time formula inconsistency: two formulas (Eq.1 v=62 mph, Eq.2 v=65
  mph) produce different effective speeds. §04 corridor calculations use Eq.1 arithmetic.
  The "54–58 mph effective speed" claim in the Stop-Penalty paragraph does not match
  §04's implied effective speeds (approximately 60 mph for NYC–Chicago).
  → Choose ONE canonical model. Recommended fix: remove the duplicate Eq.2
    "Stop-Penalty Adjustment" block (§03 lines 19–49) and replace with one sentence
    noting that effective average speed accounting for stops is 59–62 mph for the
    300–900 mile range analyzed (derived from Eq.1). Update abstract "62 mph average"
    to "59–62 mph average including intermediate stops."

[I-02] §04 Atlanta–Dallas arithmetic: "13.30h ≈ 12.9h" is incorrect (13.30 rounds to
  13.3h). Table 5 shows 12.9h. Reconcile: either the text calculation is wrong (use
  floor(800/150)=5 stops → 800/62 + 5×8/60 = 12.90 + 0.67 = 13.57h?) or the table
  is wrong. Recalculate with the canonical formula and verify Table 5 entry.

P2 items (should fix):
[I-03] §05 EIT section (lines 71–88): add § 5311(f) note as closest existing authority.
  One sentence after the EIT definition in §05 (see Referee 2 suggested language above).

P3 items (optional polish):
  - §04 stop dwell time: note 8-minute dwell is baseline; ADA-accessible boarding on
    rural highway hubs may run 10–12 minutes. Add sensitivity note.
  - §05 mountain-corridor cost premium: note that $2.80/mile flat-rate assumption
    understates costs for Denver–Salt Lake; this reinforces (rather than undermines)
    the EIT case for that corridor.
  - §05 "12,000 jobs" citation: confirm BTS_intermodal2023 provides the 24,000
    current-employment figure; if not, find the correct source.

PRE-PANEL CHECKLIST:
□ P1: §03 formula reconciled — single canonical model, §04 arithmetic consistent
□ P1: Atlanta–Dallas arithmetic corrected (§04 text and Table 5 aligned)
□ MODULE.md primary number delivered: 28–45% improvement, PTI 1.15 — YES
□ BPR extrapolation: N/A
□ Net vs gross: EIT framing correctly treats $15/pass as operating cost subsidy, not
     capital — YES
□ All \citep{} keys: verify ROUTE_F1, ROUTE_E2, ROUTE_C1, BusRegulatoryReform1982,
     Schwieterman2019, FlixBus2023, EAS_DOT2023, BTS_intermodal2023
□ Cross-paper: F.1 figures ($2B, 12.4M, 0.8%) confirmed consistent
□ Rubric version: N/A (no rubric scores cited)
□ Abstract primary result: YES (28–45% improvement, 24M passengers)
□ Referee P1 blockers: I-01 formula reconciliation, I-02 arithmetic fix

VERDICT: FIXES REQUIRED
Fixes required: 3 (2 P1 + 1 P2)
Next: fix §03 formula duplication, correct Atlanta–Dallas arithmetic, add §05 §5311(f)
  reference, then run /panel:publication review F.2+intercity-bus-corridors
═══════════════════════════════════════════════════════
```
