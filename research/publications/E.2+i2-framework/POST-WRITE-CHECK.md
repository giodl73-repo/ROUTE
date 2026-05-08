---
paper: E.2+i2-framework
title: "Interstate 2.0: A Design Framework for Throughput, Resilience, and Shared Transit"
post_write_date: 2026-05-08
rubric_version: v1.2
pipeline_stage: ready (Round 1 recheck passed — NPV reconciliation table added §03)
---

# POST-WRITE PIPELINE — E.2+i2-framework

---

## PHASE 1 — PAPER INVENTORY

```
Paper: E.2+i2-framework
Sections found: 01-introduction.tex, 02-framework-design.tex, 03-investment-portfolio.tex,
                04-throughput-analysis.tex, 05-resilience-case.tex,
                06-transit-layer.tex, 07-conclusion.tex
Plan found: no (no plan.md in directory)
Track: E — Interstate 2.0 Design (synthesis paper)
Venue: Transportation Research Part A or Transportation Policy
Key claims:
  1. Total I2.0 capital: $253B over 30 years; aggregate NPV $298B at 7%; B/C 2.2:1
     (abstract, §01-intro, §07-conclusion)
  2. Relay network ($40M) + intelligent routing ($200M) deliver more than half of total
     p95 improvement of entire $126B portfolio (§02-framework-design, Table 1)
  3. Transit integration layer ($2B) brings 12.4M transit-dependent Americans within
     30 miles of intercity connection (§07-conclusion, §06-transit-layer, abstract)
Primary number (from MODULE.md contract):
  "I2.0 portfolio: $X trillion cost, $Y trillion NPV; W% reliability gain"
Paper's stated primary number:
  $253B capital; $298B NPV (range $246B–$298B); 40% freight reliability variance reduction;
  57% PTI variance reduction (std dev 0.42 → 0.18)
Match: YES — module contract fulfilled (figures are billions not trillions)
```

---

## PHASE 2 — QUANTITATIVE CONSISTENCY CHECK

| Q-ID | Quantity | Abstract | §01-Intro | §02-Framework | §03-Portfolio | §07-Conclusion | Consistent? |
|------|----------|---------|-----------|---------------|---------------|----------------|-------------|
| Q-01 | Total capital | $253B | $253B | $253B | $251.5B ex-transit / $253.5B incl. | $253B | WARN ($253.5B in table vs $253B in abstract; rounding — acceptable) |
| Q-02 | Aggregate NPV | $298B | $298B | — | $298B (upper bound, note in §03) | $298B | PASS (consistently cited as upper bound; $246B–$298B range documented) |
| Q-03 | B/C ratio (portfolio) | 2.2:1 | 2.2:1 | — | 2.2:1 (table total) | 2.2:1 | PASS |
| Q-04 | Annual NPV range | — | — | — | $246B–$298B range (§03 reconcile note) | — | PASS |
| Q-05 | Managed lanes capital (Component 1) | $121B | $121B (via E.1 cite) | — | $121B | — | PASS |
| Q-06 | Managed lanes annual benefit (E.2 portfolio) | — | — | — | $12.7B (stale; should be $11.2B after E.1 correction) | — | FAIL — §03 uses pre-correction E.1 benefit figure |
| Q-07 | Managed lanes NPV (E.2 portfolio Component 1) | — | — | — | $86.4B | — | WARN — this is E.2's own DCF computation, not E.1's $101B; needs cross-reference note |
| Q-08 | Managed lanes B/C (E.2 portfolio) | — | — | — | 2.3:1 (stale; should be 2.0:1) | — | FAIL — §03 and §01-intro use pre-correction 2.3:1 |
| Q-09 | Relay network p95 improvement | — | — | 87h→58h (Phase 0) | — | — | PASS |
| Q-10 | Donner hardening p95 | — | — | 50h→40h (Phase 1) | — | — | PASS |
| Q-11 | Managed lanes Phase 2 | — | — | $121B (Phase 2 "finishing move") | — | — | PASS |
| Q-12 | Relay capex | — | — | $40M | Table 1 ($40M) | — | PASS |
| Q-13 | Donner hardening capex | — | — | $800M (Phase 1) | Table 1 ($800M) | — | PASS |
| Q-14 | 48h SLA first achievable | — | — | "Phase 1 — Donner Hardening; p95 first achievable here" | — | — | PASS |
| Q-15 | Transit layer: $2B | Abstract | §01-intro | — | $2B Component 7 | §07-conclusion | PASS |
| Q-16 | Transit-dependent pop: 12.4M | Abstract | §01-intro | — | — | §07-conclusion | PASS |
| Q-17 | Diamond interchange B/C: 2.76:1 | — | — | — | 2.76:1 | §07 ("2.76:1") | PASS |
| Q-18 | Compound hardening B/C: 3.1:1 | — | — | — | 3.1:1 | 3.1:1 | PASS |
| Q-19 | PTI variance: 0.42→0.18 | — | — | — | — | §07 | PASS |
| Q-20 | Gulf Coast I-10 D1 2050 = 9.1 | — | §01-intro | — | §03 ("D1 score of 9.1, projected to 2050") | — | PASS |
| Q-21 | NPV reconcile table present (KEY FIX) | — | — | — | YES (Table 3: tab:npv-reconcile, sec:npv-reconcile) | — | PASS — KEY FIX CONFIRMED |
| Q-22 | $246B–$298B range documented | — | — | — | YES (§03 note below reconcile table) | — | PASS |
| Q-23 | Investment sequencing table present (KEY FIX) | — | — | YES (Table 1: tab:intervention-rank) | — | — | PASS — KEY FIX CONFIRMED |
| Q-24 | Relay-first argument (KEY FIX) | — | — | YES (sequencing section with Phase 0–3) | — | — | PASS — KEY FIX CONFIRMED |

**KEY FIX VERIFICATION — E.2 SPECIFIC:**

1. NPV reconciliation table (sec:npv-reconcile) in §03:
   - Table present: YES (Table 3: "I2.0 Portfolio Benefit Reconciliation at 7%/30 Years")
   - Label: YES (sec:npv-reconcile referenced in table)
   - $298B derivation traceable: YES (gross PV $387B − capital $89B = $298B upper bound;
     $387B − $141B = $246B lower bound; both documented in text below table)
   STATUS: KEY FIX CONFIRMED

2. Investment sequencing section in §02 with relay-first argument:
   - Section present: YES ("Investment Sequencing: What the Simulation Reveals")
   - Table present: YES (Table 1: tab:intervention-rank)
   - Relay ($40M) as first intervention: YES (Phase 0 — Relay Network, $40M, p95: 87h→58h)
   - Donner hardening ($800M) → 58h→40h, 48h SLA first achievable: YES (Phase 1 — Donner
     Hardening ($800M): "48-hour SLA first achievable here at the p95 level")
   - Managed lanes ($121B) as Phase 3 finishing move: NOTE — paper uses Phase 2 for managed
     lanes, Phase 3 for Donner Tunnel. Instruction says "Phase 3 finishing move" but paper
     labels managed lanes as Phase 2. This is a label difference, not a content difference —
     the argument that managed lanes are the "finishing move" (not the foundation) is present.
   STATUS: KEY FIX CONFIRMED for argument; Phase label ("Phase 2" vs "Phase 3") differs from
     instruction spec but content is correct.

3. $246B–$298B NPV range (capital timing assumption):
   - Present: YES in §03 reconciliation note
   STATUS: KEY FIX CONFIRMED

**CRITICAL STALE CITATION (E.1 correction not propagated):**
- §01-intro: "2.3:1 benefit-cost ratio" for E.1 — STALE (should be 2.0:1 after E.1 correction)
- §03 Component 1: "$12.7B annual benefit" — STALE (should be $11.2B after E.1 correction)
- §03 Component 1: "B/C ratio: 2.3:1" — STALE (should be 2.0:1 after E.1 correction)
- These three items in E.2 must be updated when E.1 abstract is corrected.

```
CONSISTENCY: FAIL — 2 failures (Q-06, Q-08: stale E.1 figures propagated into E.2)
             1 warning (Q-07: $86.4B vs $101B clarification needed)
P1 (must fix):
  - §01-intro: "2.3:1 benefit-cost ratio" (E.1 cite) → "2.0:1"
  - §03 Component 1 annual benefit: "$12.7B" → "$11.2B"
  - §03 Component 1 B/C: "2.3:1" → "2.0:1"
P2 (should fix):
  - §03 Component 1: note that E.2's $86.4B component NPV is E.2's own portfolio DCF
    of E.1 benefits (vs E.1's standalone $101B NPV); add cross-reference clarification
  - Phase 0 label for managed lanes sequencing: currently "Phase 2" in paper; instruction
    says "Phase 3 finishing move" — add "final-phase" language in §02 sequencing section
P3 (minor):
  - Capital timing sensitivity (Neumark P3): present $246B and $298B as co-equal bounds
    in §07-conclusion with a sentence flagging capital timing as the primary sensitivity driver
  - Legislative pathway acknowledgment (Puentes P2): one sentence in §07 noting federal
    authorization requirement and cross-referencing F-track companion materials
```

---

## PHASE 3 — CONTRACT CHECK

| Promise (from MODULE.md) | Paper section | Delivered? | Gap |
|--------------------------|---------------|-----------|-----|
| Full I2.0 portfolio investment case | §03 (Table: tab:portfolio) | YES | ✓ |
| $X trillion cost, $Y trillion NPV | abstract ($253B cost, $298B NPV) | YES | ✓ |
| W% reliability gain | §07 (40% freight reliability; 57% PTI std dev) | YES | ✓ |
| Investment LP / prioritization framework | §02 (sequencing), §03 (phasing) | YES | ✓ |
| NPV reconciliation table (KEY FIX) | §03 (sec:npv-reconcile) | YES | ✓ |
| Investment sequencing section (KEY FIX) | §02 ("Investment Sequencing" subsection) | YES | ✓ |
| Relay-first argument ($40M → 87h→58h) | §02 (Table 1, Phase 0) | YES | ✓ |
| Donner hardening → 58h→40h, 48h SLA | §02 (Phase 1 text) | YES | ✓ |
| Managed lanes as finishing move | §02 ("finishing move, not the foundation") | YES | ✓ |
| $246B–$298B NPV range documented | §03 (reconciliation note) | YES | ✓ |
| E.1 NPV $101B cited (not $115B) | §01-intro, §03 | NO — E.2 still cites 2.3:1 / $12.7B | ✗ |

```
CONTRACT: PARTIAL
Promises kept: 10/11
Gaps:
  - E.2 does not yet cite E.1's corrected NPV ($101B) or B/C (2.0:1);
    §01-intro and §03 still use pre-correction figures. This is a dependency:
    fix requires E.1 abstract to be corrected first, then E.2 updated.
MODULE.md primary number delivered: YES (all five components of the synthesis delivered)
```

---

## PHASE 4 — REFEREE SIMULATION

Selected referees: R-Economics, R-Policy, R-Equity

---

**REFEREE 1 — R-Economics (Neumark archetype)**
Recommendation: Minor Revision

SUMMARY: The NPV reconciliation table (sec:npv-reconcile) satisfies the Round 1 blocking
requirement — the $298B claim is now arithmetically traceable. The $246B–$298B range is
documented with the capital timing assumption explained. Two remaining issues: the E.1
cross-citation uses stale figures (2.3:1, $12.7B) that need updating after E.1 is corrected;
and the portfolio summary table (tab:portfolio) and reconciliation table (tab:npv-reconcile)
show different NPVs for Component 1 ($86.4B in reconcile vs. E.1's standalone $101B) without
explanation.

MAJOR CONCERNS:
[I-01] §01-intro and §03 cite E.1 with pre-correction figures (2.3:1, $12.7B/yr). After
  E.1 is corrected to 2.0:1 / $11.2B, E.2 must be updated to match. These are stale
  citations, not independent estimates.
[I-02] §03 tab:npv-reconcile shows Component 1 NPV as $36.6B (not $101B or $86.4B).
  The $36.6B is the net NPV within E.2's reconciliation framework; $86.4B is the component
  NPV in tab:portfolio; $101B is E.1's standalone estimate. Three different numbers for the
  same investment appear in the paper without explanation. A cross-reference note is needed.

MINOR CONCERNS:
- Capital timing framing: the $298B vs $246B distinction is present but understates the
  work the assumption is doing. Recommend flagging as Sensitivity S1 in §07-conclusion.

---

**REFEREE 2 — R-Policy (Puentes archetype)**
Recommendation: Minor Revision

SUMMARY: Investment sequencing section is now present and makes the relay-first argument
compellingly. The simulation evidence (Monte Carlo 10,000 trips) supporting the Phase 0
sequencing is well-specified and the cost-efficiency ranking ($/hr saved) is rigorous.
Main remaining gap: the paper still does not acknowledge the federal authorization pathway
for Phase 1 investments.

MAJOR CONCERNS: none blocking (per RECHECK-SYNTHESIS: Puentes accepts E.2 as research
synthesis rather than policy memo)

MINOR CONCERNS:
[I-03] §07-conclusion: no sentence acknowledging that Phase 1 managed lane deployment
  requires specific federal authorization. Puentes' P2 item from RECHECK-SYNTHESIS
  remains unaddressed. Add one sentence in §07 conclusion, cross-referencing F-track
  for detailed authorization analysis.

---

**REFEREE 3 — R-Equity (Schmitt archetype)**
Recommendation: Accept (with minor revisions)

SUMMARY: The transit integration layer analysis ($2B, 12.4M transit-dependent Americans)
provides the equity anchor for the paper. The relay-first sequencing is the most important
equity-relevant finding: the $40M relay network delivers immediate time-savings across
the income distribution of freight-dependent workers. The displacement analysis for missing
link corridors (REVISION-PLAN P1.3) is still absent from the current draft.

MAJOR CONCERNS: none blocking (per recheck status: P1.3 displacement analysis was a P1
blocker from REVISION-PLAN but RECHECK-SYNTHESIS shows paper advances to ready — this
P1 item is outstanding but not blocking panel)

MINOR CONCERNS:
[I-04] §03 Component 4 (Missing links): missing link construction sections do not include
  displacement/right-of-way acquisition analysis for urban segments. REVISION-PLAN P1.3
  remains unaddressed. At minimum, a one-paragraph acknowledgment of NEPA community
  impact assessment requirements for I-69 Houston segment.

---

## PHASE 5 — ABSTRACT CHECK

```
ABSTRACT: ~200 words (at target)
Primary result stated: YES — "$253 billion over 30 years, with aggregate NPV of $298 billion
  at a 7% discount rate (benefit-cost ratio 2.2:1)"
Method named: YES — "ROUTE corpus of 227 scored corridors and five tracks of analysis"
Policy implication: YES — "reduces national freight reliability variance by an estimated 40%,
  reduces transcontinental transit time from 4.5 to 3.5 days"
Track chain position: YES — "five tracks of analysis (corpus calibration, gap identification,
  freight economics, climate exposure, and I2.0 design)"
STALE CITATION: Abstract does not directly cite E.1 B/C, so the stale 2.3:1 figure
  does not appear in the abstract. Abstract is clean of the E.1 correction issue.
```

---

## PHASE 6 — INTERNAL CROSS-PAPER CONSISTENCY

```
CROSS-PAPER CONSISTENCY:
  Papers cited in E.2: ROUTE_A1, ROUTE_A2, ROUTE_B1, ROUTE_B3, ROUTE_B4,
    ROUTE_C1, ROUTE_C2, ROUTE_D1, ROUTE_D2, ROUTE_E1, F.1 (via ROUTE_E1 incorrectly —
    §06-transit-layer cites \citep{ROUTE_E1} to reference F.1, which is wrong key)

  Values cross-checked:

  FROM D.1 (D1 scores):
    - E.2 §03: "D1 score of 9.1 (projected to 2050)" for Gulf Coast I-10 — consistent
      with D.1 abstract and §06; PASS
    - E.2 §01-intro: "$6.2B per year in closure costs" from D.2 — consistent with D.2's
      top-15 total; PASS

  FROM E.1 (managed lane NPV):
    - E.2 §01-intro: "$121 billion capital, 2.3:1 benefit-cost ratio" — STALE (E.1
      corrected to 2.0:1); FAIL
    - E.2 §03 Component 1: "$12.7B annual benefit, 2.3:1 B/C" — STALE; FAIL
    - E.2 §03 Component 1: NPV $86.4B — this is E.2's own computation, NOT E.1's $101B
      standalone NPV. Clarification note required.

  FROM B.3 (B.3 Donner NPV correction):
    - B.3 NPV = $12.1B (corrected from $15.8B) per MODULE.md cross-paper guidance;
      E.2 does not cite B.3 NPV directly; N/A

  CITATION KEY BUG:
    - §06-transit-layer \citep{ROUTE_E1} appears to reference the F.1 paper (transit nodes),
      not E.1 (managed lanes). The cite key is wrong. Should be \citep{ROUTE_F1}.

  Stale citations:
    - E.2 §01-intro: E.1 B/C 2.3:1 → must update to 2.0:1 after E.1 correction
    - E.2 §03 Component 1: E.1 annual benefit $12.7B → $11.2B; B/C 2.3:1 → 2.0:1
```

---

## PHASE 7 — PRE-PANEL CHECKLIST

```
═══════════════════════════════════════════════════════
POST-WRITE COMPLETE: E.2+i2-framework
═══════════════════════════════════════════════════════

Validation results:
  Consistency:       FAIL — 2 stale E.1 citations (§01-intro 2.3:1; §03 Component 1
                     $12.7B / 2.3:1) from pre-correction E.1 figures;
                     1 citation key bug (§06 \citep{ROUTE_E1} should be ROUTE_F1)
  Contract:          PARTIAL — 10/11 promises; E.1 $101B not yet cited correctly
  Referee sim:       Minor Revision (R-Economics stale cites); Minor Revision (R-Policy);
                     Accept (R-Equity)
  Abstract:          ~200 words (at target); primary result stated; abstract is clean
                     (does not directly cite E.1 B/C so stale issue doesn't hit abstract)
  Cross-paper:       FAIL — E.2 cites stale E.1 B/C (2.3:1) and annual benefit ($12.7B)
                     in 2 locations; citation key bug in §06

KEY FIX VERIFICATION:
  [x] NPV reconciliation table (sec:npv-reconcile) in §03: CONFIRMED (Table 3)
  [x] $246B–$298B NPV range documented: CONFIRMED in §03 note below table
  [x] Investment sequencing section in §02: CONFIRMED ("Investment Sequencing" subsection)
  [x] Relay ($40M) as Phase 0, p95 87h→58h: CONFIRMED
  [x] Donner hardening ($800M) → 58h→40h, 48h SLA first achievable: CONFIRMED
  [x] Managed lanes as "finishing move" argument: CONFIRMED ("finishing move, not the foundation")
  [PARTIAL] Managed lanes labeled "Phase 2" in paper, not "Phase 3" — instruction says Phase 3.
    Content is correct (finishing move argument present); label differs.
  [FAIL] E.2 does NOT correctly cite E.1 $101B NPV — §01 and §03 still show 2.3:1 / $12.7B
  [FAIL] §06 citation key bug: \citep{ROUTE_E1} should be \citep{ROUTE_F1} for F.1 paper

P1 blockers (fix before panel review):
  [I-01] §01-intro: "2.3:1 benefit-cost ratio" (E.1 cite) → "2.0:1" (after E.1 correction)
  [I-01] §03 Component 1: annual benefit $12.7B → $11.2B; B/C 2.3:1 → 2.0:1
  [CITE] §06 \citep{ROUTE_E1} → \citep{ROUTE_F1} (wrong cite key for F.1 transit paper)

P2 items (should fix):
  [I-02] Add cross-reference note in §03 Component 1 clarifying that $86.4B is E.2's
          portfolio DCF of E.1 benefits (not E.1's standalone $101B NPV)
  [I-03] §07-conclusion: add one sentence on federal authorization requirement for Phase 1
          managed lanes; cross-reference F-track for detailed analysis (Puentes P2)
  [PHASE-LABEL] Relabel managed lanes as "Phase 3 — finishing move" in §02 sequencing
          table to match project specification (currently labeled "Phase 2")

P3 items (optional polish):
  - §07-conclusion: Capital timing as Sensitivity S1 — co-equal presentation of $246B
    and $298B bounds (Neumark P3 from RECHECK-SYNTHESIS)
  - Missing link displacement analysis (REVISION-PLAN P1.3) — NEPA acknowledgment
    for I-69 Houston (Schmitt I-04)
  - §03 reconcile table computation check: component NPVs sum to $135.7B (not $298B);
    the $298B is gross PV minus front-loaded capital. This arithmetic path should be
    stated more clearly in the table note.

PRE-PANEL CHECKLIST:
[x] KEY FIX — NPV reconciliation table (sec:npv-reconcile) in §03: PRESENT
[x] KEY FIX — $246B–$298B range documented: PRESENT
[x] KEY FIX — Investment sequencing section in §02: PRESENT
[x] KEY FIX — Relay-first argument with Phase 0/1/2/3 labeling: PRESENT
[x] KEY FIX — 48h SLA first achievable at Donner hardening (Phase 1): PRESENT
[ ] KEY FIX — E.1 $101B NPV correctly cited (not $115B or 2.3:1): NOT DONE (stale)
[ ] P1 — §06 citation key ROUTE_E1 → ROUTE_F1: NOT FIXED
[ ] P1 — §01-intro E.1 B/C 2.3:1 → 2.0:1: NOT DONE (requires E.1 fix first)
[ ] P1 — §03 Component 1 $12.7B → $11.2B and 2.3:1 → 2.0:1: NOT DONE
[x] MODULE.md primary number delivered: PASS ($253B, $298B, 40% reliability gain)
[x] Abstract states primary quantitative result: PASS ($298B, 2.2:1 in abstract)
[ ] All \citep{} keys verified: NOT VERIFIED (citation key bug in §06 identified)
[x] Cross-paper D.1 citations correct: PASS (9.1 D1 score consistent)
[ ] Cross-paper E.1 citations correct ($101B / 2.0:1): FAIL — stale in §01 and §03
[x] Rubric version: YES — "ROUTE v1.2" referenced
[ ] Referee P1 blockers addressed (I-01, I-02): NOT YET DONE

VERDICT: FIXES REQUIRED
Fixes required: 4 (§01-intro E.1 B/C update; §03 Component 1 benefit/B/C update;
  §06 citation key bug; §03 cross-reference note for $86.4B vs $101B)
Note: E.1 fix is a prerequisite — E.1 abstract must be corrected before these E.2
  citations can be updated consistently.
Next: Fix E.1 abstract first, then propagate $101B / 2.0:1 to E.2 §01 and §03,
  fix §06 citation key, then run /panel:publication review E.2+i2-framework
═══════════════════════════════════════════════════════
```
