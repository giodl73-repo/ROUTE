---
paper: C.2+national-max-flow
type: post-write-check
author: research-post-write
created: 2026-05-08
---

# POST-WRITE CHECK: C.2 — National Max-Flow

## PHASE 1 — READ THE PAPER

```
Paper: C.2+national-max-flow
Sections found: 01-introduction.tex, 02-background.tex, 03-methods.tex, 04-bottleneck-analysis.tex,
                05-incident-simulation.tex, 06-missing-links.tex, 07-conclusion.tex
Plan found: NO (no plan.md in C.2 directory)
Track: C — Freight & Throughput
Venue: (not specified; inferred Transportation Science or Networks)
Key claims:
  1. Three nationally binding bottleneck arcs: I-95 Baltimore-Washington (V/C 2.1+),
     I-80 Donner Pass (V/C 0.82), Dallas I-35/I-45 interchange (V/C 1.9+) — abstract, §04
  2. Donner Pass closure reduces NE-Pacific max-flow by 23% — abstract, §05
  3. I-69 completion increases Gulf-to-Midwest max-flow by 18% — abstract, §06
Primary number (from MODULE.md contract): Cannot verify (no plan.md)
Paper's stated primary number: 4.8M vpd current national max-flow; 23% Donner loss; 18% I-69 gain
Match: Internally consistent
```

---

## PHASE 2 — QUANTITATIVE CONSISTENCY CHECK

| Q-ID | Quantity | Abstract | §01 (Intro) | §03 (Methods) | §04 (Bottleneck) | §05 (Incident) | §06 (Missing Links) | §07 (Conclusion) | Consistent? |
|------|----------|---------|------------|--------------|-----------------|---------------|---------------------|-----------------|-------------|
| Q-01 | Donner throughput loss | 23% | 23% | — | — | 23% | — | 23% | PASS |
| Q-02 | I-69 max-flow gain | 18% | 18% | — | — | — | 18% | 18% | PASS |
| Q-03 | Donner capacity (vpd) | 91,200 | 91,200 | 91,200 | 91,200 | — | — | — | PASS |
| Q-04 | National max-flow current | 4.8M vpd | 4.8M vpd | — | 4.8M | — | — | 4.8M | PASS |
| Q-05 | 2050 target max-flow | 7.2M vpd | 7.2M vpd | — | 7.2M | — | — | 7.2M | PASS |
| Q-06 | I-95 Balt-Wash V/C | 2.1+ | 2.1+ | — | 2.1+ | — | — | 2.1+ | PASS |
| Q-07 | Dallas interchange V/C | 1.9+ | 1.9+ | — | 1.9+ | — | 1.4 (post-I-69) | 1.9+ (baseline) | PASS |
| Q-08 | I-40 V/C post-Donner closure | — | — | — | — | 0.84 | — | 0.84 | PASS |
| Q-09 | Compound I-40 V/C | — | — | — | — | 1.11 | — | — | PASS |
| Q-10 | Donner annual cost | — | — | — | — | $3.4B/yr | — | $3.4B/yr | PASS |
| Q-11 | I-69 construction cost | — | — | — | — | — | $22B | — | PASS |
| Q-12 | **I-69 NPV at 7% (narrative)** | — | — | — | — | — | **−$14.7B** | — | **FAIL** |
| Q-13 | **I-69 NPV at 7% (table)** | — | — | — | — | — | **+$2.1B** | — | **FAIL** |
| Q-14 | I-70W NPV at 5% | — | — | — | — | — | +$6.7B | +$6.7B | PASS |
| Q-15 | NE-Pacific baseline max-flow | — | — | — | 320,000 vpd | 320,000 | — | — | PASS |
| Q-16 | ATRI truck cost rate | — | — | $225/hr | $225/hr | $225/hr | $225/hr | $225/hr | PASS |

**P1 FAILURE — I-69 NPV INCONSISTENCY:**

**§06 I-69 NPV narrative states:**
"30-year present value at 7% discount: $590M × 12.4 = $7.3B; NPV = $7.3B − $22B = −$14.7B (negative at 7%)"

**§06 I-69 NPV sensitivity TABLE (Table tab:i69-sensitivity) states:**
"Single-commodity (+18% gain): 7% = +$2.1B"

These two numbers are contradictory. The narrative says NPV = −$14.7B at 7% for the single-commodity case. The table says NPV = +$2.1B at 7% for the single-commodity case. They cannot both be correct.

**Root cause analysis**: The narrative uses total annual benefit of $590M/yr (capacity gain $110M + reliability gain $480M = $590M). PV factor at 7%/30yr = 12.4. PV = $7.3B. NPV = $7.3B − $22B = −$14.7B.

For the table to show +$2.1B at 7%, the required PV would be $24.1B, requiring annual benefit of $24.1B / 12.4 = $1.94B/yr — more than three times the narrative's $590M/yr figure.

The $1.94B/yr would be consistent with including freight growth (1.8%/yr compounding) in the PV calculation, but the narrative explicitly shows a non-growth calculation ($590M × 12.4). The table likely includes freight growth in its computation, which the narrative does not. **This must be explicitly labeled.**

**The conclusion (§07) says**: "I-69 completion... NPV is negative at 7 percent but approaches zero when freight growth is included." This is consistent with the narrative (−$14.7B) but inconsistent with the table (+$2.1B at 7% single-commodity).

**Resolution required**: Either (a) the table's 7% column should be −$14.7B (matching the narrative, without growth), or (b) the narrative should clarify that the table includes freight growth and the narrative does not, making the two figures measure different things. The table header and footnote do not mention freight growth.

```
CONSISTENCY: FAIL — 1 P1 failure (I-69 NPV at 7%: narrative −$14.7B vs table +$2.1B)
P1: [§06 I-69 NPV narrative contradicts §06 I-69 NPV table at 7% discount rate]
P2: [§05 cites ROUTE_C1 as source for Caltrans closure data; should cite Caltrans2023 directly]
P3: [§02 attributes Edmonds-Karp to ROUTE_A1 (A.1 Arterials Tiering); should verify A.1 describes
    the algorithm, or cite FordFulkerson1956 directly as done in §03]
```

---

## PHASE 3 — CONTRACT CHECK

No plan.md found. Assessment against stated contributions (§01 Contributions).

| Promise (from §01) | Section | Delivered? | Gap |
|--------------------|---------|-----------|-----|
| National bottleneck identification (three binding arcs) | §04 | Yes | ✓ |
| Incident simulation at national scale (Donner + I-35) | §05 | Yes | ✓ |
| Missing link capacity validation (I-69 + I-70W) | §06 | Yes (with NPV inconsistency) | Partial |
| Single-commodity limitation paragraph | §03 §3.6 | Yes | ✓ |
| I-69 NPV sensitivity table (3 discount rates × 3 commodity scenarios) | §06 Table tab:i69-sensitivity | Yes (but inconsistent with narrative) | Partial |
| C.1 citation for PTI model | §06 | Yes (\citep{ROUTE_C1}) | ✓ |

```
CONTRACT: PARTIAL
Promises kept: 5/6
Gaps: [I-69 NPV narrative and table contradict each other — fix required]
MODULE.md primary number delivered: CANNOT VERIFY (no plan.md)
```

---

## PHASE 4 — REFEREE SIMULATION

**Referees selected**: R-Network, R-Economics, R-Traffic

---

**REFEREE 1 — R-Network** (Lada Adamic archetype)
Recommendation: **Major Revision**

SUMMARY: The Edmonds-Karp application to the national freight graph is the most ambitious quantitative component of the ROUTE series so far. The methodology section is clearly written, the graph construction is defensible, and the demand clustering from FAF5 is appropriate. The single-commodity sensitivity check is exactly the right acknowledgment of the model's limitations. However, the I-69 NPV inconsistency between the narrative and the table is a P1 error that will be caught by any referee who checks the arithmetic.

MAJOR CONCERNS:
[I-01] I-69 NPV: the narrative in §06 shows NPV = −$14.7B at 7% (no growth). Table tab:i69-sensitivity shows +$2.1B at 7% for the single-commodity case. These cannot both be correct without an explicit statement that the table includes freight growth while the narrative does not. Add a table footnote: "NPV values include FAF5 freight growth projection of 1.8%/yr through 2050; without growth, single-commodity NPV at 7% = −$14.7B as shown in text."
[I-02] The Edmonds-Karp time complexity is O(VE²). For V=3,200 and E=48,000, worst-case = 3,200 × (48,000)² = 7.37×10¹² — the paper states 7.4×10¹³ which is off by a factor of 10. Verify: 3,200 × 48,000² = 3,200 × 2.304×10⁹ = 7.37×10¹². The paper's figure of 7.4×10¹³ is incorrect by one order of magnitude.
[I-03] The single-commodity sensitivity uses SCTG commodity codes to split demand, but the FAF5 data is in tons, not vehicles. Converting from ton-based commodity shares to vehicle-based V/C impact requires a load factor assumption. State the load factor used.

MINOR CONCERNS:
[I-04] §04 lists I-80 Donner Pass as rank-10 in the top-10 constrained arcs table (V/C=0.82). But the paper argues Donner is a binding bottleneck for NE-Pacific max-flow. The V/C metric does not reveal this — the paper correctly explains the "free-flow rural segment fallacy" but the Donner entry in the table could mislead readers into thinking it's the 10th most constrained arc overall. A footnote on the table would help.

---

**REFEREE 2 — R-Economics** (Neumark archetype)
Recommendation: **Major Revision**

SUMMARY: The max-flow approach correctly identifies network bottlenecks rather than just high-delay locations. The distinction between economic bottlenecks (ATRI list) and network bottlenecks (min-cut) is well-drawn. The I-69 analysis is the most economically interesting component, but the NPV presentation is internally contradictory.

MAJOR CONCERNS:
[I-05] The I-69 NPV inconsistency (narrative vs. table) is a substantive error. The conclusion (§07) says I-69 is "negative at 7 percent" which agrees with the narrative (−$14.7B) but contradicts the table (+$2.1B at 7%). The paper cannot have three different claims about the same quantity. Fix: add freight-growth footnote to table; make narrative and conclusion consistent.
[I-06] The annual benefit calculation for I-69 ($590M/yr = $110M capacity + $480M reliability) uses very different methodologies for the two components. The $110M capacity figure uses vpd × truck fraction × miles × ATRI rate — this is an operating cost reduction for existing traffic, not a capacity gain. The $480M reliability figure is the Dallas V/C improvement cost reduction. Neither component is clearly a "freight benefit" in the standard BCA sense (consumer surplus, not just cost transfer). Clarify whether this is a benefit-cost calculation or a cost-reduction calculation.
[I-07] The NPV conclusion states that co-benefits (incident avoidance, military mobility) "are likely to tip the decision." Military mobility is not a standard FHWA benefit-cost category and should not be presented as likely to tip a civilian infrastructure investment decision. Reframe: these are qualitative co-benefits not captured in the NPV model.

MINOR CONCERNS:
[I-08] I-70W resilience NPV (+$6.7B at 5%) uses "attributed resilience benefit approximately $1.8B/yr" — where does the $1.8B come from relative to the $2.5B computed annual Donner throughput loss? The attribution factor is not explained.

---

**REFEREE 3 — R-Traffic** (Elefteriadou archetype)
Recommendation: **Minor Revision** (after NPV fix)

SUMMARY: The graph construction is sound. The BPR-based PTI costs are consistently applied at $225/hr from ATRI. The Donner incident simulation is well-executed. The I-40 cascade finding (V/C 0.84 post-Donner, 1.11 compound) is the most actionable finding in the paper. The NPV issue needs fixing; the methodology is otherwise solid.

MAJOR CONCERNS:
[I-09] The Dallas V/C post-I-69 drops from 1.9+ to "approximately 1.4" (§06). This is a substantial improvement, but the 1.4 post-I-69 V/C still exceeds the 1.3 BPR calibration range. The congestion cost reduction computed for the Dallas node should acknowledge this: at V/C=1.4, BPR-based delay estimates are in the extrapolation zone and likely underestimate residual congestion.

MINOR CONCERNS:
[I-10] The I-95 Baltimore-Washington section is identified as the #1 constrained arc by V/C (2.1+) but the Donner Pass is identified as the binding min-cut for NE-Pacific max-flow. The I-95 arc apparently does not appear in the NE-Pacific min-cut despite its higher V/C — why? Explain whether I-95 bypasses are available that relieve the min-cut even at V/C=2.1.

---

## PHASE 5 — ABSTRACT CHECK

```
ABSTRACT: ~175 words
Primary result stated: YES — "23 percent" Donner loss, "18 percent" I-69 gain, "4.8 million
  commercial vehicles per day" current capacity, "7.2 million" 2050 target
Method named: YES — "Edmonds-Karp algorithm applied to the ROUTE directed graph"
Policy implication: YES — "manageable through managed lanes on 7 T1 corridors combined with
  I-69 and I-70W completion"
Track chain position: YES — extends C.1 to national scale; references B-track bottleneck findings
Word count: ~175 — within 150-200 target range. PASS.
```

**Issue**: Abstract states I-70W "increases Gulf-to-Midwest max-flow by 18%" — this is the I-69 figure, not I-70W. I-70W's baseline gain is 4.7% (NE-Pacific), not 18%. **The abstract conflates I-69 and I-70W gains.** Check: "I-69 completion increases Gulf-to-Midwest max-flow by 18%. I-70W... reduces Donner-closure throughput loss from 23 percent to 9 percent." Reading the abstract again: "I-69 completion increases Gulf-to-Midwest max-flow by 18%." — this IS correctly attributed to I-69. PASS. (My initial read was incorrect — re-verified.)

---

## PHASE 6 — INTERNAL CROSS-PAPER CONSISTENCY

```
CROSS-PAPER CONSISTENCY:
  Papers cited: ROUTE_C1 (C.1 O-D reliability), ROUTE_A1 (A.1 arterials), ROUTE_A2 (rubric),
                ATRI_costs2024, HPMS2018, TIGER2023, FAF5_2023, FordFulkerson1956, IIJA2021,
                FHWA_freight2023
  Values cross-checked: 5 cross-paper values

  - ROUTE_C1 (C.1) cited for Caltrans closure data in §05: C.1 does use Caltrans2023 as
    primary source. C.2 citing C.1 for this data is secondary citation — acceptable for 
    narrative flow but should also cite Caltrans2023 directly as primary source. P2.
    
  - ROUTE_C1 cited in §06 for PTI model and I-69 findings: C.2 §06 states "the PTI for 
    HOU-CHI drops from 1.8–2.2 to approximately 1.35" and "the threshold at which a 
    managed-lane SLA becomes commercially viable (ROUTE_C1)." C.1 §05 uses PTI ~1.45 
    for HOU-CHI current; the 1.8–2.2 range cited in C.2 is not the figure in C.1 (C.1 
    says ~1.45). The range 1.8–2.2 in C.2 §06 is a new claim not sourced from C.1. Fix:
    either cite the correct C.1 figure (1.45) or attribute the 1.8–2.2 range to a 
    different source. P1 flag.
    
  - §02 cites ROUTE_A1 for Edmonds-Karp algorithm — ROUTE_A1 is A.1 Arterials Tiering.
    If A.1 does not contain the E-K description, this is a wrong citation. C.1 §01 also
    cites ROUTE_A1 for this. Check A.1 paper. P2 flag.
    
  - §06 states I-69 gain "consistent with C.1's finding" — C.1 §05 discusses I-69 distance
    reduction (290 miles) and interchange bypass. C.2 §06 reports 18% max-flow gain. C.1
    does not report a 18% max-flow figure (different metric). The claim of consistency is
    about mechanism (bypass of Dallas), not about identical numbers. Acceptable framing.
    PASS.

  - No B.3 NPV cited in C.2 — no stale citation risk from the B.3 correction.

Stale citations: NONE (B.3 not cited)
P1: [C.2 §06 cites "PTI drops from 1.8–2.2" as from C.1, but C.1 states PTI ~1.45; 
     reconcile C.2's HOU-CHI PTI range with C.1's estimate]
P2: [§05 cite Caltrans2023 directly alongside ROUTE_C1]
    [§02 ROUTE_A1 for E-K — verify A.1 contains algorithm description]
```

---

## PHASE 7 — PRE-PANEL CHECKLIST

```
═══════════════════════════════════════════════════════
POST-WRITE COMPLETE: C.2+national-max-flow
═══════════════════════════════════════════════════════

Validation results:
  Consistency:       FAIL — 1 P1 (I-69 NPV narrative −$14.7B vs table +$2.1B at 7%)
  Contract:          PARTIAL — no plan.md; stated contributions mostly delivered; NPV inconsistency
  Referee sim:       Major Revision (R-Network, R-Economics); Minor (R-Traffic)
  Abstract:          ~175 words, primary numbers stated, track chain clear
  Cross-paper:       FAIL — P1: C.2 §06 HOU-CHI PTI "1.8–2.2" cited to C.1, but C.1 says ~1.45

SPECIFIC CHECKS:
  Single-commodity limitation paragraph: PRESENT in §03 (§3.6 "Single-Commodity Sensitivity 
    Analysis") — PASS. Two-class sensitivity implemented with commodity-value split (FAF5 SCTG).
    Donner range: −19% to −27% (central −23%); I-69 range: +14% to +21% (central +18%); 
    I-40 compound V/C range: 0.98–1.18 (central confirms V/C>1.0). PASS.
  I-69 NPV sensitivity table: PRESENT in §06 Table tab:i69-sensitivity — 3 discount rates 
    × 3 commodity scenarios. FAIL on content (7% single-commodity shows +$2.1B contradicting
    narrative's −$14.7B).
  C.1 citation for PTI model: PRESENT (\citep{ROUTE_C1} in §06) — but cited value (1.8–2.2)
    does not match C.1's stated PTI (~1.45). Fix the citation value.

P1 blockers (fix before panel review):
[I-01] §06 I-69 NPV: narrative shows −$14.7B at 7%; table shows +$2.1B at 7% single-commodity.
  Fix options:
  (a) If table includes freight growth (1.8%/yr) and narrative does not: add table footnote 
      "Values include FAF5 freight growth at 1.8%/yr; static (no-growth) NPV at 7% = −$14.7B."
      Update §07 conclusion which says "negative at 7%" to clarify: "negative at 7% without
      freight growth; positive with growth." 
  (b) If table is simply wrong: update table 7% column single-commodity entry to −$14.7B.
  (c) If narrative calculation is wrong: re-derive from table values.
  One of these must be chosen.
[I-02] §06 C.2 cites HOU-CHI PTI "1.8–2.2" as from ROUTE_C1, but C.1 states ~1.45. Either:
  (a) Use C.1's ~1.45 figure in C.2 ("PTI drops from approximately 1.45 to 1.35"), or
  (b) Attribute the 1.8–2.2 range to C.2's own model (and note it differs from C.1's estimate).
  The 1.8–2.2 range cannot be cited to C.1 without C.1 support.
[I-03] §03 Edmonds-Karp time complexity: paper states 7.4×10¹³ but V×E² = 3,200 × (48,000)²
  = 3,200 × 2.304×10⁹ = 7.37×10¹². The correct figure is 7.4×10¹², not 7.4×10¹³ 
  (off by 10×). Fix the exponent.

P2 items (should fix):
[I-04] §05 Caltrans closure data: add \citep{Caltrans2023} alongside \citep{ROUTE_C1}
[I-05] §02 ROUTE_A1 for Edmonds-Karp: verify A.1 describes the algorithm; if not, replace
  with \citep{FordFulkerson1956} (already correctly cited in §03)
[I-06] I-70W resilience benefit attribution ($1.8B/yr vs. $2.5B Donner loss): explain
  the 28% reduction factor ($2.5B × 0.72 = $1.8B) with explicit reasoning
[I-07] §07 "military mobility per B.4 scoring" — B.4 is T1/T1 intersection resilience;
  military mobility is more likely attributed to B.1 (missing links) or a strategic 
  mobility paper. Check which paper has the I-31 military score = 10.0.
  (§06 I-31/US-287 section states "B.4 scoring assigns military score of 10.0" — but B.4
  is the intersection resilience paper, not a B1/military scoring paper. This may be 
  intended to reference ROUTE_B1's scoring of I-31. Verify and correct citation.)

P3 items (optional polish):
- Add a "Network Limitations" subsection noting the graph does not capture toll road routing
  choices, HOS-compliance stops, or carrier rate structures
- The §04 "free-flow rural segment fallacy" section is excellent — consider a cleaner title
  like "Why Average Utilization Misleads Network Investment Analysis"

PRE-PANEL CHECKLIST:
□ All P1 consistency failures resolved:
  - I-69 NPV narrative vs table reconciled — FAIL
  - HOU-CHI PTI citation to C.1 corrected — FAIL
  - Edmonds-Karp complexity exponent (10¹² not 10¹³) — FAIL
□ MODULE.md primary quantitative contract delivered — CANNOT VERIFY (no plan.md)
□ BPR extrapolation acknowledged where V/C > 1.3 — PASS (§03 single-commodity section
  acknowledges V/C = 1.86 extrapolation context; §04 Dallas post-I-69 at 1.4 not flagged
  — add P2 note for Dallas)
□ Net vs gross cost clearly stated — PASS (cost-reduction framing in §06)
□ All \citep{} keys exist in references.bib: ROUTE_C1, ROUTE_A1, ROUTE_A2, ATRI_costs2024,
  HPMS2018, FAF5_2023, FHWA_freight2023, IIJA2021 — all present.
  MISSING: TIGER2023, FordFulkerson1956, HCM7 — NOT in references.bib. P2 fix needed.
□ Cross-paper citations use corrected values (B.3 NPV = $12.1B) — NOT CITED (no risk)
□ Single-commodity limitation paragraph present — PASS (§03.6 fully implemented)
□ I-69 NPV sensitivity table present — PASS (structure present; content has P1 inconsistency)
□ Abstract states primary quantitative results — PASS
□ Referee P1 blockers addressed — PENDING

VERDICT: FIXES REQUIRED
Fixes required: 3 P1 (NPV table/narrative, PTI citation, complexity exponent)
Next: Fix P1 items, then run /panel:publication review C.2+national-max-flow
═══════════════════════════════════════════════════════
```
