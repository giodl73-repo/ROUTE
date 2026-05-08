---
paper: F.3+relay-marketplace
title: "The Relay Marketplace: Platform Design for 48-Hour National Freight and the AV Transition"
post_write_date: 2026-05-08
rubric_version: v1.3
panel_status: Round 1 complete — 2 P1 blockers unresolved; this POST-WRITE-CHECK
  supersedes the Round 1 SYNTHESIS.md with the user-specified targeted checks.
---

# POST-WRITE CHECK: F.3 — The Relay Marketplace

## PHASE 1 — PAPER INVENTORY

```
Paper: F.3+relay-marketplace
Sections found: 01-introduction.tex, 02-why-relay-fails-today.tex,
                03-the-container-model.tex, 04-insurance-framework.tex,
                05-hub-slot-system.tex, 06-marketplace-design.tex,
                07-av-transition.tex, 08-conclusion.tex
Plan found: YES (plan.md)
Panel status: Round 1 complete; SYNTHESIS.md and REVISION-PLAN.md written;
              2 P1 blockers documented (P1.1 hub fees, P1.2 AB5)
Track: F — Transit Integration (extended to relay marketplace)
Venue: Management Science / Transportation Research Part C
Key claims:
  1. Relay cheaper than solo: $1,050 vs $1,456 per 2,800-mile trip
     (§01 introduction, §02-why-relay-fails body)
  2. Relay/existing infra p95 = 58h; relay/I2.0 p95 = 45.4h
     (§01 introduction lines 17–18)
  3. Mode 2 IC layer: requires FMCSA rulemaking (18–24 months)
     (§04-insurance lines 53–54)
Primary number (from plan.md): relay p95=58h on existing infra (90% of I2.0 gain
  at 0.02% of cost); $33B annual efficiency gain from relay adoption at scale
Paper's stated primary number: 58h (§01), $33B (§01 aggregate problem statement)
Match: YES — primary numbers delivered
```

---

## PHASE 2 — QUANTITATIVE CONSISTENCY CHECK

| Q-ID | Quantity | Abstract | §Intro | Table/Body | §Conclusion | Consistent? |
|------|----------|----------|--------|------------|-------------|-------------|
| Q-01 | Relay trip cost | $1,050 | $1,050 | $1,050 (§02 C_relay calc) | — | PASS |
| Q-02 | Solo trip cost | $1,456 | $1,456 | $1,456 (§02 C_solo calc) | — | PASS |
| Q-03 | Relay p95 (existing infra) | 58h | 58h | 58h (§01 line 18) | — | PASS |
| Q-04 | I2.0 relay p95 | 45.4h | 45.4h | 45.4h (§01 line 18) | — | PASS |
| Q-05 | Relay savings vs solo (%) | 28% | 28% | 28% ($406/$1,456) | — | PASS |
| Q-06 | Relay infra cost (NY-LA) | $40M | $40M | $40M (§01) | $40M | PASS |
| Q-07 | Managed lane cost | $121B | $121B | $121B (§01) | $121B | PASS |
| Q-08 | Relay % of I2.0 cost | 0.02% | 0.02% | 0.02% | 0.02% | PASS |
| Q-09 | SLA improvement (relay/existing) | 90% | 90% | 90% (§01) | 90% | PASS |
| Q-10 | Mode 2 rulemaking timeline | 18–24 months | — | 18–24 months (§04 line 53) | 18–24 months | PASS |
| Q-11 | Hub swap fee (base) | $100 | — | $100 (§06 Table 8) | — | PASS |
| Q-12 | Atlanta hub daily trucks | 28,740 | — | 28,740 (§05) | — | PASS |
| Q-13 | AV savings per trip | $3,800 | $3,800 | ~$3,800 (§07 calc: $3,400–$4,200 range) | $3,800 | WARN (see below) |
| Q-14 | Annual AV savings NY-LA | $11B | — | $11.1B (§07 calc) | — | PASS |
| Q-15 | Hub capex | $5M | — | $5M (§05, §06) | $5M | PASS |
| Q-16 | Carrier population | 70,000+ | 70,000+ | 70,000+ (§02) | — | PASS |

**KEY-FIX STATUS CHECKS — USER-SPECIFIED ITEMS:**

---

### P1.1 — Hub Fees in Cost Comparison

**[FAIL — P1] Hub fees not included in relay cost comparison in current paper**

Section 02 (§02 "Hidden Cost Illusion") presents:
```
C_solo = $0.52/mi × 2,800mi = $1,456/trip
C_relay = 6 × 7h × $25/h = $1,050/trip
```

The relay cost of $1,050 excludes hub fees. The paper acknowledges this implicitly:
"The relay savings of $406 per trip appear real, but the carrier also sees an estimated
coordination cost of $50–150 per handoff (scheduling, documentation, hub fee) across
5 handoffs per NY–LA trip, eroding the apparent advantage."

However, the paper does NOT include hub fees in the headline $1,050 figure or compute
the total relay cost including hub fees. The hub fee is $100/swap (§06 Table 8 base rate).
6 swaps on a NY–LA relay run = 6 × $100 = $600 in hub fees.

**Correct fully-loaded relay cost:**
- Driver wages: $1,050 (6 drivers × 7h × $25/hr)
- Hub fees: $600 (6 swaps × $100)
- Total relay trip cost: $1,650

**This makes relay MORE EXPENSIVE than solo on a per-trip basis ($1,650 vs. $1,456).**

The SYNTHESIS.md (Round 1 panel, Neumark review) identified this: "Hub fees ($675 at
6 swaps × $112.50) bring relay total to ~$1,725 — more expensive than solo on per-trip
basis." The REVISION-PLAN.md P1.1d requires: "Replace the per-trip headline comparison
($1,050 vs. $1,456) with the per-ton-mile fully-loaded comparison as the paper's central
quantitative claim; retain the per-trip figures in a footnote with an explicit note that
hub fees are excluded from the per-trip relay figure."

**The P1.1 revision has NOT been made to the paper.** The current §02 still presents
$1,050 vs. $1,456 as the central cost comparison without including hub fees.

**Impact on paper integrity:** The abstract (main.tex line 11–12) states "cheaper than
solo long-haul ($1,050 vs. $1,456 per 2,800-mile trip)" — this claim is incorrect when
hub fees are included. Until P1.1 is resolved, the paper's central economic claim is false
as stated.

**Required fix:** Per REVISION-PLAN.md P1.1a–P1.1d:
1. Build per-ton-mile comparison (relay: ($1,050 + $600) / ~21 tons avg. = $78.57/ton;
   solo: $1,456 / ~21 tons = $69.33/ton on driver wages alone; add asset utilization
   savings to relay side: $8,700/truck/year ÷ 260 routes/year = $33.46/trip → relay
   fully-loaded including utilization: ($1,650 - $33) = $1,617 → $77/ton vs $69/ton
   on wages only; but relay utilization advantage reduces fleet capital, which on a
   per-ton-mile basis must be expressed as avoided fleet cost).
2. Retain $1,050 vs. $1,456 with footnote: "Hub fees excluded from driver wage
   comparison; fully-loaded per-ton-mile comparison in Table X includes hub fees,
   asset utilization savings, and shortage premium."

---

### P1.2 — Mode 2 AB5 Restriction

**[FAIL — P1] Mode 2 IC restriction under California AB5 not acknowledged in current paper**

Section 04 (§04-insurance) presents Mode 2 as a viable "independent contractor gig
layer" for relay drivers. The insurance architecture describes an "OAC Rider" model
and "platform gap coverage while relay-engaged." The only mention of Mode 2 regulatory
risk is: "If the Mode 2 rulemaking encounters obstacles (adverse court decisions on
gig worker classification, congressional opposition, state-level conflicts)" — a
generic hedge.

California AB5 (California Labor Code § 2775 et seq., effective January 1, 2020)
codifies the ABC test from Dynamex Operations West, Inc. v. Superior Court (4 Cal.5th
903, 2018). Under the ABC test, a worker is an employee unless the hiring entity can
show ALL THREE:
- A: The worker is free from control in connection with performance
- B: The worker performs work outside the usual course of the hiring entity's business
- C: The worker is customarily engaged in an independently established trade

A relay driver performing relay driving for a relay hub operator FAILS prong B:
relay driving IS the usual course of the hub operator's business. Therefore, relay
drivers in California would be classified as employees under AB5, regardless of FMCSA
rulemaking.

This is not a minor regulatory footnote. California ports handle approximately 40% of
US containerized imports; the most critical relay hubs for the NY–LA corridor (Sacramento,
Los Angeles/Fontana) are in California. Mode 2 IC relay cannot operate at California
hubs under current state law.

The SYNTHESIS.md (Schmitt review) identified this as P1: "AB5 (California), ABC test,
Dynamex ruling; Mode 2 is legally problematic in largest freight state — P1 blocker."

**The P1.2 revision has NOT been made to the paper.** Section 04 still presents Mode 2
as viable for California hubs without acknowledging AB5.

**Required fix per REVISION-PLAN.md P1.2a–P1.2b:**
1. Add paragraph in §04 (Mode 2 subsection) acknowledging California AB5 and the
   ABC test. Note that relay drivers at California hubs would be classified as employees
   under prong B unless the hub operator structures operations to satisfy all three
   prongs (likely impossible for the hub's primary business).
2. Revise Mode 2 geographic scope: California hubs (Sacramento, LA) must operate
   under Mode 1 (W-2) only. Mode 2 IC viable only in states without equivalent ABC
   test legislation.
3. Add: "Mode 1 W-2 employment is the operative model for California hub operations
   and should be treated as the default model nationally; Mode 2 provides supplemental
   surge capacity in jurisdictions without AB5-equivalent legislation."

---

**[WARN — P2] $3,800 per-trip AV savings: abstract figure vs. §07 derivation**

Abstract (main.tex line 37): "$3,800 per-trip savings"
§07 body (line 196): "full savings reach approximately $3,400–$4,200 per trip"
§07 calculation shows: $2,421 (driver savings) + $133 (platooning) = $2,554/trip
  then adds: $250 (HOS compliance), $400 (insurance), $150 (utilization)
  = $3,354 total — BELOW the $3,800 abstract figure

The $3,800 figure is at the middle of the $3,400–$4,200 range and is defensible as
a midpoint, but the arithmetic adds to approximately $3,354 or $3,400. The abstract
rounds up to $3,800 without clear justification for the upward selection. P3 issue.

```
CONSISTENCY: 2 P1 FAILURES (P1.1 hub fees, P1.2 AB5) + 1 P2 + 1 P3
P1 (must fix):
  - §02 and abstract: relay cost comparison excludes hub fees ($600 for 6 swaps);
    correct comparison must be per-ton-mile or add hub fees to relay side ($1,650 total).
  - §04 Mode 2: California AB5/ABC test not acknowledged; Mode 2 is legally inoperative
    at California hubs (LA, Sacramento) under current state law.
P2 (should fix):
  - Abstract $3,800 AV savings: §07 arithmetic yields ~$3,354; either revise abstract
    to "$3,400" or add explicit note on additional savings components.
P3 (minor):
  - Mode 2 rulemaking timeline "18–24 months" underestimates per GAO data
    (FMCSA NPRMs average 4.1 years); revise to "36–60 months base case" (REVISION-PLAN).
```

---

## PHASE 3 — CONTRACT CHECK

| Promise (from plan.md) | Paper section | Delivered? | Gap |
|------------------------|---------------|-----------|-----|
| Relay cost comparison vs solo | §02 (visible) | PARTIAL — hub fees excluded | ~ |
| Per-ton-mile framing | §02, §01 | NO — paper uses per-trip throughout | ✗ |
| Carrier fragmentation diagnosis | §02 §2.1 | YES | ✓ |
| Regulatory gap (49 CFR Part 395) | §02 §2.2 | YES | ✓ |
| First-mover disadvantage | §02 §2.3 | YES | ✓ |
| RDU 5-component spec | §03 | YES | ✓ |
| Mode 1 W-2 insurance | §04 §4.2 | YES | ✓ |
| Mode 2 IC insurance | §04 §4.3 | PARTIAL — AB5 risk not acknowledged | ~ |
| Hub slot three-tier system | §05 | YES | ✓ |
| Marketplace 4-sided platform | §06 | YES | ✓ |
| AV transition timeline | §07 | YES | ✓ |
| AV economics ($3,800/trip) | §07 | PARTIAL — calc yields $3,354–$3,400 | ~ |
| FMCSA rulemaking 3 definitions | §08 | YES | ✓ |

```
CONTRACT: PARTIAL
Promises kept: 10/13
Gaps:
  - Per-ton-mile framing not executed (P1.1 from REVISION-PLAN.md)
  - Mode 2 AB5 exposure not acknowledged (P1.2)
  - AV savings arithmetic doesn't reach abstract $3,800 cleanly (P2)
MODULE.md primary number delivered:
  - Relay p95 = 58h on existing: YES (§01)
  - $33B efficiency gain: YES (§01 aggregate)
  - 90% SLA gain at 0.02% cost: YES (§01)
```

---

## PHASE 4 — REFEREE SIMULATION

*Note: This paper has already completed Round 1 panel review. Referees below are
treated as the Round 2 reviewers who will evaluate whether REVISION-PLAN.md
items were addressed.*

**REFEREE 1 — R-Economics (Neumark archetype) — Round 2**
Recommendation: Reject (P1.1 unresolved)

SUMMARY: P1.1 (per-ton-mile reframing) is documented in REVISION-PLAN.md as a required
fix but has not been implemented. The paper's central economic claim ("relay is cheaper
than solo: $1,050 vs. $1,456") remains incorrect as stated when hub fees are included.

MAJOR CONCERNS:
[I-01] §02 "Hidden Cost Illusion" and abstract: relay trip cost $1,050 excludes hub fees.
  At $100/swap × 6 swaps = $600 hub fees, the fully-loaded relay cost is $1,650/trip
  versus $1,456 solo — relay is MORE EXPENSIVE on a per-trip driver+fee basis.
  The paper's economic case rests entirely on asset utilization savings and shortage
  premium elimination, but these are not presented in the headline comparison.
  Specific location of failure: abstract line 11 ("$1,050 vs. $1,456 per 2,800-mile
  trip") and §02 body lines 34–47 (the C_relay calculation).

MINOR CONCERNS:
- Atlanta hub revenue ($1,049M/year at $100/swap × 28,740 trucks/day): at the assumed
  swap fee, this implies a hub-operator-net margin of $945M/year on $5M capex
  (Table 8). A 2-day payback period is a striking claim. Reviewers will be skeptical;
  add a sensitivity analysis for $50/swap and $75/swap scenarios.

---

**REFEREE 2 — R-Equity (Schmitt archetype) — Round 2**
Recommendation: Reject (P1.2 unresolved)

SUMMARY: Mode 2 IC architecture is presented as viable without acknowledging California
AB5. The relay marketplace's most economically important hubs (Los Angeles, Sacramento)
are in the largest freight state, where Mode 2 IC is legally inoperative under the ABC
test. A paper proposing the relay marketplace as a national platform must address the
most restrictive jurisdiction at which it will be deployed.

MAJOR CONCERNS:
[I-02] §04 Mode 2 subsection: California AB5 (California Labor Code § 2775) is not
  mentioned. Relay drivers performing relay as their hub's primary business fail
  prong B of the ABC test (Dynamex, 2018). Mode 2 cannot operate at Los Angeles
  or Sacramento hubs without W-2 classification.
  Specific location: §04 lines 39–53 (Mode 2 introduction and insurance framework)
  present the IC model as viable without geographic or jurisdictional caveat.

MINOR CONCERNS:
- §04 "Mode Sequencing" (lines 55–59): mentions "adverse court decisions on gig worker
  classification, congressional opposition" as Mode 2 risks but does not name AB5 as an
  existing, enacted risk. The hedge language underweights a known legal constraint.

---

**REFEREE 3 — R-Network (Adamic archetype)**
Recommendation: Major Revision

SUMMARY: The platform design is ambitious and the four-sided architecture is well-specified.
The hub slot system is the most technically rigorous section. The main network science
concern is the relay matching mechanism: the paper describes a "combinatorial auction with
advance booking windows" (§06) but does not specify the matching algorithm, the objective
function, or the theoretical properties (stability, efficiency, incentive compatibility).

MAJOR CONCERNS:
[I-03] §06 "Slot Exchange" — Advance Booking and Real-Time Allocation: describes
  "first-available queue" for Tier 3 but does not specify the matching mechanism for
  Tier 1 and Tier 2. A platform matching trucks to drivers at scale needs an explicit
  algorithm (deferred acceptance / Gale-Shapley recommended per Roth & Sotomayor 1990).
  Without algorithm specification, the 15-minute grace protocol and the demand-responsive
  pricing section are ungrounded.

MINOR CONCERNS:
- Table 7 (§05 hub capacity): The 28,740 trucks/day Atlanta figure produces a 399-dock
  requirement, then the paper reduces to ~300 docks assuming 65% dock utilization.
  The 65% assumption is asserted without a basis. For an airport gate analogy, airport
  gate utilization at peak hubs is 85–90%; the 65% assumption is conservative but should
  be supported.

---

## PHASE 5 — ABSTRACT CHECK

Abstract word count: ~350 words (significantly above 150–200 target for ROUTE papers)
Primary result stated: YES — "cheaper than solo long-haul ($1,050 vs. $1,456 per
  2,800-mile trip)" — BUT THIS CLAIM IS P1-BLOCKED (hub fees excluded)
Method named: YES — "Monte Carlo simulation (5,000 trips per corridor)"
Policy implication: YES — FMCSA rulemaking, NFRZ designation
Track chain position: YES — cites relay as "Layer 0" of I2.0

```
ABSTRACT: ~350 words (70–75% above 200-word target — significantly over)
Primary result stated: YES (but P1-blocked — hub fee exclusion makes $1,050 incorrect)
Method named: YES
Policy implication: YES
Track chain position: YES
Note: abstract is approximately 350 words; target is 150–200. Substantial compression
  needed regardless of P1 resolution.
```

---

## PHASE 6 — CROSS-PAPER CONSISTENCY

Papers cited in F.3:
- ROUTE_C1 (C.1): relay simulation source. F.3 abstract: "relay alone reduces NY–LA
  p95 transit time from 87.2 hours to 58 hours." C.3 §07 Table 6 shows relay/GP p95
  = 58.3h. F.3 rounds to 58h; C.3 uses 58.3h. Minor rounding inconsistency — acceptable.
- ROUTE_E1 (E.1): managed lane NPV. F.3 uses $121B for managed lane construction cost.
  MODULE.md correction note says E.1 NPV = $101B. These are different quantities
  ($121B = construction cost; $101B = NPV at 7% discount). Both can be correct
  simultaneously. PASS.
- ROUTE_E2 (E.2): $253B I2.0 portfolio. Consistent across all F-track papers. PASS.

```
CROSS-PAPER CONSISTENCY:
  Papers cited: ROUTE_C1, ROUTE_E1, ROUTE_E2, plus external
  Values cross-checked:
    - Relay/GP p95: F.3 cites "58h"; C.3 Table 6 shows 58.3h — minor rounding ✓
    - $121B managed lane construction vs. $101B NPV: distinct quantities, both valid ✓
    - $253B I2.0 total: consistent ✓
  Stale citations: None identified
```

---

## PHASE 7 — PRE-PANEL CHECKLIST

```
═══════════════════════════════════════════════════════
POST-WRITE COMPLETE: F.3+relay-marketplace
═══════════════════════════════════════════════════════

CONTEXT: This paper completed Round 1 panel review (2026-05-08 per _panel.yaml).
Mean score 2.6/4, below the 3.0 promotion threshold. Two P1 blockers were identified
in SYNTHESIS.md and REVISION-PLAN.md. THIS POST-WRITE-CHECK CONFIRMS BOTH P1 BLOCKERS
ARE STILL UNRESOLVED IN THE CURRENT PAPER TEXT.

Validation results:
  Consistency:       2 P1 FAILURES + 1 P2 + 1 P3
  Contract:          PARTIAL — 10/13 promises kept; P1.1 and P1.2 undelivered
  Referee sim:       Round 2 Reject (Neumark, Schmitt on unresolved P1 items)
  Abstract:          ~350 words (significantly over); primary claim P1-blocked
  Cross-paper:       PASS

P1 blockers (fix before panel review):
[P1.1] Hub fees excluded from relay cost comparison.
  Current text (§02 lines 34–36, abstract line 11): relay cost = $1,050 excludes $600
  in hub fees (6 swaps × $100). Fully-loaded relay trip cost = $1,650 > $1,456 solo.
  The paper's headline "relay is cheaper" is incorrect as stated.
  → Fix (per REVISION-PLAN.md P1.1a–P1.1d):
    a) Build per-ton-mile comparison table including hub fees on relay side and
       asset utilization savings (fleet capital avoidance at $8,700/truck/year).
    b) Replace "$1,050 vs. $1,456" headline in abstract and §02 with the per-ton-mile
       comparison. Retain per-trip figures in footnote with explicit note that hub
       fees excluded.
    c) Show that relay cost-competitiveness holds on per-ton-mile basis once asset
       utilization ($33.46/trip credit) and shortage premium elimination are included.
    d) New abstract claim example: "relay reduces total system cost per ton-mile by
       X% versus solo long-haul when asset utilization and driver shortage premium
       are included."

[P1.2] California AB5/ABC test not acknowledged for Mode 2 IC layer.
  Current text (§04 lines 39–53): Mode 2 IC presented as viable without California
  AB5 caveat. Relay drivers at Los Angeles and Sacramento hubs fail prong B of
  California's ABC test (Dynamex, 2018): relay driving IS the hub's principal business.
  Mode 2 IC is legally inoperative at California hubs under current state law.
  → Fix (per REVISION-PLAN.md P1.2a–P1.2b):
    a) Add paragraph in §04 Mode 2 subsection: "California AB5 (California Labor Code
       § 2775 et seq.) and the ABC test from Dynamex Operations West, Inc. v. Superior
       Court (4 Cal.5th 903, 2018) restrict independent contractor classification for
       relay drivers at California hubs. Relay drivers performing relay as the hub
       operator's principal business fail prong B of the ABC test. Mode 2 IC relay
       operations at Los Angeles and Sacramento hubs are not viable under current
       California law; Mode 1 W-2 employment is the required model at those locations."
    b) Revise Mode 2 scope: "Mode 2 IC provides surge capacity in jurisdictions without
       AB5-equivalent legislation; Mode 1 W-2 is the required and preferred model."

P2 items (should fix):
[P2-AV] Abstract $3,800/trip AV savings: §07 arithmetic sums to $3,354 before
  rounding up to $3,800. Either (a) revise abstract to "$3,400" or (b) add explicit
  components summing to $3,800 in §07 calculation.
[P2-timeline] FMCSA rulemaking "18–24 months": GAO data (GAO-24-106178) shows FMCSA
  NPRMs average 4.1 years. Revise to "36–60 months base case; achievable in 18–24
  months if included in surface transportation reauthorization mandate."
[P2-market] §06 slot exchange: specify matching algorithm (deferred acceptance
  recommended) or acknowledge algorithm selection left to platform operator.

P3 items (optional polish):
  - Abstract compression: reduce from ~350 to ~200 words; trim AV transition summary
    (currently 6 sentences covering Phase 1–3 which are detailed in §07).
  - §05 Atlanta hub payback: add $50/swap and $75/swap sensitivity scenarios to
    Table 8 to preempt skepticism on the 2-day payback claim.
  - §05 65% dock utilization assumption: add basis or reference airport gate
    utilization literature for comparison.

PRE-PANEL CHECKLIST:
□ P1.1: Hub fees added to relay cost; per-ton-mile comparison built; abstract
     "$1,050 vs. $1,456" revised to per-ton-mile framing
□ P1.2: California AB5/ABC test paragraph added to §04 Mode 2 subsection;
     Dynamex citation added to references; Mode 2 scoped to non-AB5 jurisdictions
□ MODULE.md primary numbers: relay p95=58h, $33B efficiency gain — confirmed ✓
□ BPR extrapolation: N/A
□ Net vs gross cost: relay savings must be framed as efficiency gain in full system
     cost (not just driver wage comparison) — requires P1.1 fix
□ All \citep{} keys: verify ROUTE_C1, ROUTE_E1, ROUTE_E2, FMCSA_HOS2022,
     ATRI_shortage2024, BLS_trucking2023, FHWA_freight2023, Levinson2006,
     Williamson1979 exist in references.bib; ADD Dynamex2018 citation
□ Cross-paper: relay/GP p95 = 58h (F.3) vs 58.3h (C.3) — acceptable rounding
□ Rubric version: N/A
□ Abstract states primary result: P1-BLOCKED — after P1.1 fix, new primary claim
     must be per-ton-mile comparison, not the $1,050 vs $1,456 per-trip figures
□ Round 1 panel P1 blockers: UNRESOLVED — P1.1 and P1.2 both require new text

VERDICT: FIXES REQUIRED — NOT READY FOR PANEL ROUND 2
Fixes required: 2 critical P1 blockers + 3 P2 items
Priority order:
  1. P1.1: per-ton-mile table (§02 replacement) + abstract revision
  2. P1.2: AB5 paragraph (§04) + Dynamex citation
  3. P2-AV: $3,800 arithmetic correction
  4. P2-timeline: FMCSA timeline revision
  5. P2-market: algorithm specification
Then: re-run /panel:publication review F.3+relay-marketplace (Round 2)
═══════════════════════════════════════════════════════
```
