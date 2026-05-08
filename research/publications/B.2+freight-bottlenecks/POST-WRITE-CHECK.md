---
name: POST-WRITE-CHECK — B.2 Freight Bottlenecks: Where the Interstate System Exceeds Capacity
slug: b2-freight-bottlenecks-post-write
type: review
status: draft
author: research-post-write
created: 2026-05-08
updated: 2026-05-08
---

# POST-WRITE-CHECK: B.2 — Freight Bottlenecks: Where the Interstate System Exceeds Capacity

## PHASE 1 — PAPER SUMMARY

```
Paper: B.2+freight-bottlenecks
Sections found: 01-introduction.tex, 02-background.tex, 03-methods.tex,
                04-bottleneck-identification.tex, 05-economic-cost.tex,
                06-paradox-in-bottleneck-data.tex, 07-weather-bottlenecks.tex,
                08-implications.tex, 09-conclusion.tex
Plan found: YES (plan.md)
Track: B — Gap Analysis
Venue: Transportation Research Part B: Methodological
Key claims:
  1. Top-50 ATRI bottlenecks cost $22.7B/yr; T2 connectors are 64% of locations but
     T1 corridors are 62% of cost — cascade multiplier 1.73× (§05)
  2. Donner Pass is the 11th most expensive bottleneck ($1.6B/yr) if weather closures
     included — not in ATRI top-100 because ATRI measures recurring congestion only (§07)
  3. The $4B Donner freight tunnel has a 2.5-year payback (§07, §08)
Primary number (from MODULE.md contract): M corridors at V/C > 0.85; top-10 ATRI cost $X billion
Paper's stated primary number: $22.7B total (top-50); T1 cascade multiplier 1.73×;
  M corridors A1 ≥ 6.0 = 14 (§04 body: "14 corridors score A1 ≥ 6.0")
Match: PARTIAL — M corridors identified (14); primary $X billion figure ($22.7B) stated.
  MODULE.md says "top-10 ATRI annual cost $X billion total" — paper gives top-50 total
  ($22.7B), not specifically top-10. Top-10 sum can be computed from Tab §04: $7,538M.
```

---

## PHASE 2 — QUANTITATIVE CONSISTENCY CHECK

| Q-ID | Quantity | Abstract | §Intro | Table/Body | §Conclusion | Consistent? |
|------|----------|---------|--------|-----------|-------------|-------------|
| Q-01 | Total top-50 bottleneck cost | $22.7B | — | $22.7B (§05 heading) | $22.7B (§09) | PASS |
| Q-02 | T2 share of locations | 64% | — | 64% (Tab §05: 32/50) | 64% (§09) | PASS |
| Q-03 | T1 share of cost | 62% | — | 62% (Tab §05: $14.1B/$22.7B) | 62% (§09) | PASS |
| Q-04 | Cascade multiplier | — | — | 1.73× (Tab §05) | 1.73× (§09) | PASS |
| Q-05 | T1 cost per location | — | — | $783M/location (Tab §05) | — | PASS (14,095/18=783.1 ✓) |
| Q-06 | T2 cost per location | — | — | $269M/location (Tab §05) | — | PASS (8,621/32=269.4 ✓) |
| Q-07 | Total T1 + T2 cost | — | — | 14,095+8,621=22,716 (Tab §05) | — | PASS (22,716M = $22.7B ✓) |
| Q-08 | Atlanta cluster cost (T2 locations only) | — | — | **"$3.0 billion" (§04) vs "$3.0 billion in annual cost" (§05)** | — | **WARN: §04 says "I-285/I-20 ($916M) + I-285/I-85 ($789M) + I-75/I-285 ($687M) + I-75/I-85 ($617M) = $3,009M ≈ $3.0B" — BUT I-75/I-285 and I-75/I-85 locations are attributed to I-75 (T1), not I-285 (T2). §05 then says "Atlanta's four T2 bottleneck locations account for $3.0B" — but ranks 8 (I-75/I-285) and 10 (I-80/I-94) are T1 locations. Atlanta's T2 locations are ranks 1+4 only = $916+$789=$1.705B, not $3.0B** |
| Q-09 | Top-10 total | — | — | $7,538M (Tab §04) | — | PASS (sum: 916+848+812+789+762+731+698+687+654+641=7,538 ✓) |
| Q-10 | Donner Pass annual closures | — | — | ~50/year (§07, §03) | — | PASS |
| Q-11 | Donner mean closure duration | — | — | 18h (§07, §03) | — | PASS |
| Q-12 | Donner truck volume | — | — | 8,000/day (§07) | — | PASS |
| Q-13 | Donner rerouting cost rate | — | — | $225/hr (§07, §03) | — | **WARN: §03 methods states $225/hr for "rerouting premium (detour fuel cost and lost time)" but §02 background states ATRI uses $150/hr for "total cost of one hour of truck delay." The paper uses two different per-hour cost rates without explaining why weather rerouting ($225) exceeds congestion delay ($150). This is a real methodological distinction but needs explicit justification.** |
| Q-14 | Donner annual cost | — | — | $1.6B/year (§07, §09) | $1.6B (§09) | PASS (50×18×8000×$225/24h×$1M/1000= let me verify: 50×18=900h×8000=7.2M truck-hours×$225=$1.62B ✓) |
| Q-15 | Snoqualmie annual closures | — | — | ~40/year (§07, §03) | — | PASS |
| Q-16 | Snoqualmie mean duration | — | — | 12h (§07, §03) | — | PASS |
| Q-17 | Snoqualmie truck volume | — | — | 6,000/day (§07) | — | PASS |
| Q-18 | Snoqualmie annual cost | — | — | $0.65B/year (§07) | — | PASS (40×12×6000×$225/1000/1M=40×12=480×6000=2.88M×225=$648M≈$0.65B ✓) |
| Q-19 | Donner freight tunnel cost | — | — | $4B (§07, §08, §09) | $4B (§09) | PASS |
| Q-20 | Donner tunnel payback period | — | — | 2.5 years (§07: $4B/$1.6B/yr=2.5yr) | 2.5yr (§09) | PASS ($4B/$1.6B=2.5 ✓) |
| Q-21 | Donner ranking (11th with weather) | abstract | — | "11th most expensive" (§07) | §09 | PASS |
| Q-22 | A1 ≥6.0 corridor count | — | — | 14 corridors (§04) | — | PASS |
| Q-23 | A1/ATRI Spearman ρ | — | — | ρ=0.67 (§04) | — | PASS (cited once, no cross-check) |
| Q-24 | I-95 ATRI location count | — | — | 9 of top 50 (§04) | — | PASS |
| Q-25 | I-95 annual bottleneck cost | — | — | "$4.9B/yr combined" (§08) | — | WARN: §08 says "I-95 (9 ATRI locations, $4.9B/yr combined)" — this is a new figure not in Tab §04. The Tab shows individual I-95 locations at ranks 2, 3, 5, 9 = $848+$812+$762+$654=$3.076B for those 4. Nine locations at $4.9B implies 5 more I-95 locations not shown in top-10. Plausible but verify; and note these 9 locations are across different ATRI ranks (some outside top-10) |
| Q-26 | I-40 V/C | — | §01: "I-40 V/C = 0.84 (at target)" | §04: "V/C = 0.84, at target" | §09 | PASS |
| Q-27 | Rubric version for ATRI/ROUTE join | — | — | plan.md says "v1.1 scores"; paper §03 says "centrality-adjusted tier (from ROUTE_A1)" | — | **WARN: The paper uses tier classifications from A.1 (centrality-adjusted), not raw v1.1 scores. Plan.md says "use v1.1 ROUTE scores" but the actual tier attribution (T1 vs T2) uses A.1's centrality-adjusted output. The rubric version used for A1 score calculations is not stated.** |
| Q-28 | BPR extrapolation caveat | — | — | NOT PRESENT for V/C > 1.3 | — | **FAIL: §02 background describes HCM LOS F at V/C > 1.0. The paper discusses V/C > 0.85 as the A1 ≥ 6.0 threshold. BPR formula is used implicitly (§02 references HCM capacity but doesn't use BPR explicitly). However, the SKILL.md requires checking whether "BPR V/C > 1.3 caveat present where needed." Atlanta locations at LOS F (V/C > 1.0) approach the extrapolation zone. No caveat present.** |

**CONSISTENCY: 2 FAILURES, 5 WARNINGS**

### Critical Issue — Q-08: Atlanta Cluster Tier Attribution

§04 identifies four Atlanta top-10 locations: 
- Rank 1: I-285/I-20 — I-285 is T2 (per Tab §03: "T2 Major Connectors")
- Rank 4: I-285/I-85 — I-285 is T2
- Rank 8: I-75/I-285 — attributed to I-75 in Tab §04 (T1)  
- Rank 10: I-80/I-94 Chicago — NOT an Atlanta location (this is Chicago)

§04 intro says "Atlanta corridor cluster accounts for $3.0 billion in annual freight congestion cost" citing 4 locations. But Rank 10 is Chicago (I-80/I-94). The 4 Atlanta locations should be Ranks 1, 4, 8, and one more not Rank 10.

Looking at the actual text: §04 says "four locations in the top 10: I-285/I-20 ($916M), I-285/I-85 ($789M), I-75/I-285 ($687M), and I-75/I-85 downtown connector ($617M)." These 4 sum to $3,009M ≈ $3.0B. But two of these (Rank 8 = I-75/I-285 at T1; and a 4th at $617M not in the table shown) include T1 attributions.

Then §05 says "Atlanta's four T2 bottleneck locations account for $3.0 billion" — but if Rank 8 is I-75 (T1), the T2-only Atlanta cost is $916+$789+$617 = $2.322B, not $3.0B.

This is a tier attribution error. §05 should read "Atlanta's four bottleneck locations (two T2, two involving T1)" or correct the T2 total cost.

```
P1 (must fix):
  [I-01] §05 "Atlanta's four T2 bottleneck locations account for $3.0 billion":
         The four Atlanta top-10 locations include I-75/I-285 (attributed to I-75 = T1
         in the paper's own Tab §04 and classification in §03). The pure T2 Atlanta
         cost is lower than $3.0B. Either:
         (a) Correct the total to reflect only the T2 Atlanta locations, or
         (b) Rephrase to "Atlanta's four bottleneck locations (T2 beltway + T1 approaches)"
         Fix: Restate as "Atlanta's four bottleneck cluster locations — two on T2 connector
         I-285 ($916M, $789M) and two on T1 I-75 approaches ($687M, and I-75/I-85 at $617M)
         — account for $3.0 billion in combined annual cost. The T2-only I-285 locations
         account for $1.7B; the T1 I-75 locations account for $1.3B."

  [I-02] BPR V/C > 1.3 caveat absent: Add a sentence in §02 or §03 acknowledging that
         BPR-based V/C estimates (used implicitly via HCM capacity calculation in the A1
         scoring) are unreliable above V/C = 1.3 (LOS F breakdown, queuing), and that
         for Atlanta and I-95 locations operating at LOS F, the V/C-based cost estimates
         are likely underestimates of true delay. This is the standard caveat required
         by the ROUTE skill protocol.

P2 items (should fix):
  [I-03] §03 Methods: cost rate discrepancy. Weather rerouting uses $225/hr but congestion
         delay uses ATRI's $150/hr. Add an explicit paragraph explaining:
         "Weather closure costs use a rerouting premium of $225/hr (vs. ATRI's $150/hr
         for congestion delay) because rerouting incurs additional fuel cost over 200–300
         extra miles that pure delay does not. The $75/hr premium is estimated from
         FHWA detour cost studies [citation needed]. Sensitivity: at $150/hr, Donner
         annual cost = $1.08B (vs. $1.6B at $225/hr)."
         This disclosure directly affects the Donner payback calculation (2.5yr vs 3.7yr).

  [I-04] §08 I-95 $4.9B/yr: cite or show the calculation — which 9 locations are included
         and what their individual costs are. The table shows only top-10 results; 5 of
         the 9 I-95 locations must be outside that range. Add a note or footnote.

  [I-05] Rubric version: plan.md says v1.1 scores; paper uses A.1 centrality-adjusted
         tier. Clarify explicitly in §03: "Corridor tier classifications follow the
         centrality-adjusted framework of A.1 (ROUTE_A1), which uses v1.1 scores with
         betweenness centrality adjustment. A1 scores for bottleneck identification are
         from the v1.1 ROUTE corpus."

  [I-06] §04: Rank 10 is listed as "I-80/I-94 Chicago" in Tab §04 but §04 intro says
         "Atlanta metropolitan corridor dominates the top of the list with four locations
         in the top 10." If Rank 10 is Chicago, Atlanta has only three unambiguously
         Atlanta locations in ranks 1-10 (not four). Verify the fourth Atlanta location
         and confirm whether it is within top-10 or in positions 11-50.

P3 items (optional polish):
  - §09 conclusion investment sequence: Snoqualmie ($3B, "similar payback") should
    include the actual payback calculation ($3B/$0.65B/yr = 4.6yr) — not "similar to Donner"
  - §08: I-75 current PTI 1.84 cited without source. Add citation (FHWA FPM or ATRI PTI data).
  - §04 I-95 A1 score 5.2 (moderate): this may seem inconsistent with I-95 having 9 ATRI
    top-50 locations. Add a clarifying sentence: "I-95's A1 score of 5.2 reflects the
    corridor's 1,919-mile length diluting extreme urban congestion; the worst 150 miles
    (New York to Baltimore) would score 9.5+ on A1 alone."
```

---

## PHASE 3 — CONTRACT CHECK

| Promise (plan.md + MODULE.md) | Paper section | Delivered? | Gap |
|-------------------------------|---------------|-----------|-----|
| M corridors at A1 ≥ 6.0 (MODULE.md: V/C > 0.85) | §04: 14 corridors | YES | ✓ |
| Top-10 ATRI annual cost $X billion | Tab §04: $7.538B | YES | ✓ |
| Tier distribution of ATRI bottleneck cost | Tab §05, §06 | YES (64%/62% finding) | ✓ |
| Bottleneck density per tier (locations/100mi) | Referenced from A.1 §05; not restated | PARTIAL — repeated implicitly but not recomputed for B.2 |
| T2 connectors dominate count, T1 dominates cost | §05, §06, §09 | YES | ✓ |
| Donner Pass as highest-impact non-ATRI bottleneck | §07 | YES | ✓ |
| $22.7B primary number in abstract | Abstract | YES | ✓ |
| Atlanta I-285 cascade analysis | §05, §06, §08 | YES | ✓ |

```
CONTRACT: PASS (with minor gaps)
Promises kept: 7/8
Gaps:
  1. Bottleneck density (ATRI locations per 100 miles by tier) is mentioned from A.1
     but not explicitly computed for B.2's 50-location dataset. Add a brief restatement.
MODULE.md primary number delivered: YES — $22.7B stated consistently; M=14 corridors at A1≥6.0;
  cascade multiplier 1.73× is the secondary finding (compelling result not in the contract)
```

---

## PHASE 4 — REFEREE SIMULATION

**Selected referees**: R-Traffic (BPR/HCM methods, cascade cost methodology), R-Economics (cost calculation, net vs gross), R-Policy (investment sequencing)

---

```
REFEREE 1 — R-Traffic (Transportation Research Part A/B archetype)
Recommendation: Major Revision

SUMMARY: The integration of ATRI data with a tier classification is methodologically
original and the cascade multiplier result is significant. The paper is well-written.
However, the Atlanta tier attribution error (§05) is a factual mistake that undermines
the paper's central empirical claim. Additionally, the absence of any BPR validity
caveat for the high-congestion locations is a significant omission for a methods paper.

MAJOR CONCERNS:
[I-07] §05: Atlanta "T2 locations" cost of $3.0B includes I-75 (T1) locations. The
       attribution is inconsistent with the paper's own tier table. This is not a minor
       editorial error — it directly affects the T1/T2 cost comparison that is the
       paper's primary finding. If T1 accounts for MORE of Atlanta's cost than claimed,
       the cascade multiplier may be even higher than 1.73×.

[I-08] §02: The BPR V/C relationship is linear and well-calibrated for V/C < 1.0.
       For V/C > 1.3 (several Atlanta and I-95 locations), the BPR function produces
       travel time estimates that are extrapolations beyond the empirically validated
       range. The paper uses HCM LOS-F language (§02) but does not acknowledge that the
       ATRI cost estimates for these locations may have wide uncertainty bands. Add:
       "For locations at LOS F (V/C > 1.0), ATRI's GPS-derived travel time measurements
       are reliable regardless of BPR model behavior; however, the cost-per-truck-hour
       computation assumes linear scaling with delay, which may understate costs during
       breakdown conditions where trucks queue for multiple cycles."

MINOR CONCERNS:
- §07: the weather bottleneck analysis uses two different cost rates ($225/hr weather
  vs $150/hr congestion) without explicit justification. This will be flagged by any
  reviewer who reads carefully.
- Tab §04: adding a "Tier" column matching the tier attribution in the text would make
  the T1/T2 count check easier for readers.
```

---

```
REFEREE 2 — R-Economics (Journal of Economic Perspectives archetype)
Recommendation: Major Revision

SUMMARY: The paper presents a clear and computable estimate of freight bottleneck costs.
The cascade multiplier analysis is interesting. The main economic concern is that the
paper conflates direct truck operating cost with economic welfare cost. ATRI's $22.7B
is a direct operational cost estimate; the welfare cost of freight delay (including
inventory, supply chain disruption, and consumer surplus effects) is substantially
higher. The paper acknowledges this ("true societal cost is substantially higher")
but then uses the $22.7B figure for all investment justification without bounds.

MAJOR CONCERNS:
[I-09] §05: The Donner freight tunnel NPV ($4B cost, $1.6B/yr benefit = 2.5yr payback)
       uses ONLY the weather bottleneck cost as the benefit. But the tunnel would also
       reduce congestion delay costs (Donner has congestion costs even on non-closure
       days during peak traffic periods). The payback calculation is conservative —
       saying so would strengthen, not weaken, the investment case.

[I-10] §07: Weather closure cost uses $225/hr while ATRI delay cost uses $150/hr.
       This means the Donner analysis ($1.6B/yr at $225) is on a different cost basis
       than the ATRI analysis ($22.7B at $150). The comparison in §07 — "Donner is the
       11th most expensive bottleneck" — compares apples and oranges. At $150/hr (ATRI
       basis), Donner annual cost = $1.08B. At $150/hr, it would rank ~14th or 15th,
       not 11th. This does not change the paper's qualitative conclusion (Donner is
       critically underranked by ATRI) but the specific rank claim (11th) may be wrong.
       Recompute the Donner rank on the ATRI cost basis ($150/hr) for consistency.

MINOR CONCERNS:
- §05 "does not include inventory carrying costs, supply chain disruptions, or broader
  economic multiplier effects": this disclaimer is appropriate but should include an
  order-of-magnitude estimate of the multiplier from the literature. Schrank (2021)
  cites a multiplier of ~1.5× for total economic vs direct transport cost.
```

---

```
REFEREE 3 — R-Policy (Transport Policy archetype)
Recommendation: Accept with Minor Revision

SUMMARY: This paper provides practical investment sequencing guidance grounded in
empirical cost data. The Donner freight tunnel analysis is particularly compelling
as a concrete, actionable policy recommendation. The I-285 T1 upgrade discussion
is nuanced and avoids oversimplification.

MAJOR CONCERNS:
[I-11] §08 Investment Sequencing: the paper recommends Donner tunnel as "Phase 1
       priority 1" without noting that tunnel construction requires environmental review
       (likely EIS for a major federal highway project), a process that takes 5–10 years.
       The 2.5-year payback is compelling but the tunnel won't be operational in 2.5 years
       from today. Add: "The economic case is compelling; the constraint on Phase 1
       timing is NEPA review, estimated at 5–7 years for a project of this magnitude."

MINOR CONCERNS:
- The managed lane PTI targets (§08) are specified as corridor-level targets. In practice,
  PTI improvement is segment-specific — the entire I-95 corridor won't reach PTI ≤ 1.15;
  only the bottleneck segments would. Clarify that the targets apply to the bottleneck
  segments (roughly the NJ/MD/DC/northern VA segments), not the full 1,919 miles of I-95.
```

---

## PHASE 5 — ABSTRACT CHECK

```
ABSTRACT: ~155 words
Primary result stated: YES — "$22.7 billion" annual cost stated; "T2 connectors account
  for 64% of ATRI bottleneck locations but only 38% of total annual congestion cost" ...
  WAIT: abstract says "only 38% of total annual congestion cost" but body §05 says "62%
  of cost" for T1. T2 at 38% and T1 at 62% is consistent. Check: "only 38%" in abstract
  vs "64% locations / 62% cost" in body — the abstract's 38% for T2 cost is consistent
  with body's 62% for T1 cost ($8.6B of $22.7B = 37.9% ≈ 38%). PASS.
Method named: YES — "integrate ATRI rankings with the ROUTE 12-dimensional corridor
  scoring framework" — NOTE: same "12-dimensional" issue as B.1; v1.2 is 15-dimensional
Policy implication: YES — "Atlanta's I-285/I-75/I-85 corridor cluster accounts for
  $4.2 billion alone" — NOTE: abstract says $4.2B for Atlanta cluster, but §04 sums
  the 4 Atlanta top-10 locations at $3.0B. Abstract's $4.2B may include additional
  Atlanta locations beyond the top-10. This is an INCONSISTENCY — **FAIL**
Track chain position: YES — "T1 cascade multiplier" connects to A.1 tier classification
```

### Abstract Arithmetic Error: Atlanta Cluster

Abstract: "Atlanta's I-285/I-75/I-85 corridor cluster accounts for $4.2 billion alone."
§04 body: "Combined, the Atlanta corridor cluster accounts for $3.0 billion."

$4.2B ≠ $3.0B. This is the same Atlanta cluster being discussed. The abstract uses a larger figure than the body. Either:
- The abstract includes ATRI locations 11–50 that are in Atlanta (beyond the 4 in the top-10), or
- It is an uncorrected draft figure.

This is a P1 consistency failure.

---

## PHASE 6 — INTERNAL CROSS-PAPER CONSISTENCY

```
CROSS-PAPER CONSISTENCY:
  Papers cited: A.1 (ROUTE_A1), B.1 (forward ref as context), B.3 (forward ref), ATRI2024
  Values cross-checked: 5

  1. A.1 tier classification cited in §03: "T1 Primary Arteries: I-5, I-10, I-35, I-40,
     I-75, I-80, I-90, I-95" — consistent with A.1 Tab 2 ✓
  2. A.1 ATRI ρ=0.72: cited in §04 "Spearman ρ = 0.67 (B.2 computation)"
     NOTE: A.1 reports ρ=0.72 for ATRI bottleneck density vs tier; B.2 reports ρ=0.67
     for A1 score vs ATRI bottleneck density. These are different correlations (tier vs
     score) measured differently — acceptable, but should be flagged as different metrics.
  3. scores-all.csv: I-285 is T3 in scores-all.csv (score=12.8). But B.2 §03 cites
     I-285 as T2 ("T2 Major Connectors: remaining interstates with tier score ≥15.0 (v1.1 rubric)").
     I-285 at 12.8 is T3, not T2. The paper's T2 classification of I-285 is wrong vs
     the actual data. **FAIL — T2/T3 attribution of I-285 affects the Atlanta cost split**
  4. B.1 cross-reference: §01 correctly distinguishes coverage gaps (B.1) from bottlenecks
     (B.2) as "not the same places" — consistent with B.1's geographic analysis ✓
  5. "$22.7B" cited consistently in abstract, §05 heading, and §09 — PASS

  Critical stale/wrong citation:
  - I-285 tier: scores-all.csv shows I-285 = T3 (12.8). B.2 classifies it as T2.
    This affects the entire T1/T2 cost analysis in §05 and §06. If I-285 is T3:
    - T3 locations at I-285 ($3B+) would not be in the T1/T2 breakdown
    - The cascade multiplier calculation changes
    This may be the most consequential data error in the paper.
```

---

## PHASE 7 — PRE-PANEL CHECKLIST

```
═══════════════════════════════════════════════════════════
POST-WRITE COMPLETE: B.2+freight-bottlenecks
═══════════════════════════════════════════════════════════

Validation results:
  Consistency:       2 FAILURES (Atlanta cost in abstract vs body; I-285 tier vs scores-all.csv),
                     5 WARNINGS
  Contract:          PASS (7/8 with minor gaps)
  Referee sim:       Major Revision (R-Traffic + R-Economics); Accept/Minor (R-Policy)
  Abstract:          ~155 words, $22.7B stated, Atlanta $4.2B in abstract vs $3.0B in body
  Cross-paper:       I-285 classified T2 in paper vs T3 in scores-all.csv — critical discrepancy

P1 blockers (fix before panel review):
[I-01] Abstract says "Atlanta's I-285/I-75/I-85 corridor cluster accounts for $4.2 billion"
       but §04 body says "$3.0 billion." Reconcile these figures. Either:
       (a) $4.2B includes ATRI locations outside top-10 (e.g., Atlanta locations ranked
           11–50) — if so, document the full Atlanta cluster in §04 and update §04 to $4.2B, or
       (b) $4.2B is a stale draft figure — correct abstract to $3.0B.

[I-02] §05: "Atlanta's four T2 bottleneck locations account for $3.0 billion" —
       the I-75/I-285 location (Rank 8) and I-75/I-85 location are attributed to I-75 (T1)
       in Tab §04. The pure T2 Atlanta cost is $916M+$789M+$617M=$2.3B (or $1.7B if only
       I-285 locations). Correct the Atlanta T2 attribution or restate explicitly
       as "Atlanta cluster (T2 I-285 + T1 I-75 approaches)."

[I-03] I-285 tier classification: scores-all.csv shows I-285 = T3 (12.8), not T2.
       The paper's §03 classifies I-285 as T2 (threshold ≥15.0 on v1.1 rubric).
       Resolve: either (a) the paper is using a different threshold or rubric version
       than scores-all.csv (acceptable — note the version), or (b) I-285 is T3 and
       the entire T1/T2 cost analysis needs to be recomputed including I-285 as T3.
       This is the paper's most consequential data error.

[I-08-bpr] BPR V/C > 1.3 caveat: add to §02 or §03 that BPR estimates are extrapolations
       for LOS F locations (V/C > 1.0+) and that ATRI GPS data is the primary source for
       those locations (relieving the extrapolation concern for actual cost measurement,
       but flagging it for the ROUTE A1 score component).

P2 items (should fix):
[I-04] §03: Justify the $225/hr weather rerouting rate vs ATRI's $150/hr delay rate.
       Recompute Donner rank on $150/hr basis to check if it remains "11th most expensive."
[I-05] §08: I-95 $4.9B/yr — show or cite the 9 locations that sum to this figure
[I-06] Rubric version: "12-dimensional ROUTE scoring framework" → v1.2 is 15-dimensional;
       plan.md says v1.1 scores used — add explicit version statement in §03

P3 items (optional polish):
  - §09 Snoqualmie: compute actual payback ($3B/$0.65B=4.6yr) instead of "similar payback"
  - §08: note NEPA/EIS timeline constraint on Donner tunnel
  - §04: add "Tier" column to Tab §04 for easier T1/T2 attribution by readers

PRE-PANEL CHECKLIST:
□ Atlanta cluster cost reconciled ($4.2B abstract vs $3.0B body)
□ §05 Atlanta "T2 locations" tier attribution corrected
□ I-285 tier classification resolved against scores-all.csv (T3 vs T2)
□ BPR V/C > 1.3 caveat added for LOS F locations (§02 or §03)
□ $22.7B total cited consistently in abstract, §05, §09 (currently PASS)
□ Weather rerouting rate ($225/hr) vs congestion rate ($150/hr) justified
□ Rubric version tagged (v1.1 per plan.md) on all score citations
□ MODULE.md primary number delivered: $22.7B stated; M=14 corridors ✓
□ Cross-paper: I-285 tier consistent with scores-all.csv
□ Abstract Atlanta figure matches body

VERDICT: FIXES REQUIRED
Fixes required: 4 P1 (including critical I-285 tier and Atlanta arithmetic), 3 P2
Next: run /panel:publication review B.2+freight-bottlenecks after P1 fixes
═══════════════════════════════════════════════════════════
```
