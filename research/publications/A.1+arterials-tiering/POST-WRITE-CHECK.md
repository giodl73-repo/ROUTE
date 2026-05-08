---
name: POST-WRITE-CHECK — A.1 Interstate Arterials: Tiering the National Highway Network
slug: a1-arterials-tiering-post-write
type: review
status: draft
author: research-post-write
created: 2026-05-08
updated: 2026-05-08
---

# POST-WRITE-CHECK: A.1 — Interstate Arterials: Tiering the National Highway Network

## PHASE 1 — PAPER SUMMARY

```
Paper: A.1+arterials-tiering
Sections found: 01-introduction.tex, 02-background.tex, 03-data-scoring.tex,
                04-tier-classification.tex, 05-validation.tex, 06-arterial-map.tex,
                07-implications.tex, 08-conclusion.tex
Plan found: YES (plan.md)
Track: A — Corpus & Scoring
Venue: Transportation Research Part A
Key claims:
  1. Eight Primary Arteries carry majority of national truck freight ton-miles and hold
     highest betweenness centrality — centrality-adjusted Tier 1 (§04, Tab 2)
  2. Congestion-stress paradox: I-110 outscores I-80 on aggregate but is structurally
     less important; betweenness centrality (B2) is the correct primary tier signal (§04.2)
  3. Centrality-adjusted T1 (8 corridors) achieves 100% STRAHNET alignment vs 85% for
     aggregate-score T1 (13 corridors) (§05, Tab 1)
Primary number (from MODULE.md contract): Tier 1 carries ≥50% ton-miles;
                                          Brandes gap Tier1/Tier2 ≥ 3×
Paper's stated primary number: >50% of national truck freight ton-miles (abstract);
                               Brandes gap not precisely quantified in text
Match: PARTIAL — ton-miles claim present; Brandes 3× ratio not numerically stated
```

---

## PHASE 2 — QUANTITATIVE CONSISTENCY CHECK

| Q-ID | Quantity | Abstract | §Intro | Table/Body | §Conclusion | Consistent? |
|------|----------|---------|--------|-----------|-------------|-------------|
| Q-01 | Total interstate route miles | 48,800 | 48,800 | 48,792 (§03) | 48,800 | WARN: §03 says 48,792 (precise corpus); abstract/conclusion say 48,800 (rounded FHWA figure) — different sources, should note |
| Q-02 | Corridor count | 227 | 227 | 227 | — | PASS |
| Q-03 | T1 count (centrality-adjusted) | 8 | 8 | 8 (Tab 2) | 8 | PASS |
| Q-04 | T2 count (centrality-adjusted) | "approximately 25" | 25 | 25 | — | PASS |
| Q-05 | T3 count (centrality-adjusted) | 80 (abstract) | 61 (§01 contrib) | 61 (Tab 2) | — | **FAIL: abstract says "approximately 80 Regional Feeders" but §01 Contributions and Tab 2 say 61** |
| Q-06 | T4 count (centrality-adjusted) | 114 (abstract) | 133 (§01 contrib) | 133 (Tab 2) | — | **FAIL: abstract says 114 Local Access routes; §01 Contributions and Tab 2 say 133** |
| Q-07 | Aggregate-score T1 count | — | 13 (§01) | 13 (Tab 1) | — | PASS |
| Q-08 | STRAHNET corpus count | — | ~200 (background §05) | 197 (§05 body) | — | WARN: §02 says "approximately 200", §05 says 197 — acceptable rounding but should be consistent |
| Q-09 | T1 STRAHNET alignment (centrality) | — | — | 100% (Tab 1) | — | PASS |
| Q-10 | Aggregate-score T1 STRAHNET alignment | — | — | 85% (Tab 1) | — | PASS |
| Q-11 | ATRI Spearman ρ | — | — | 0.72 centrality / 0.61 aggregate (Tab 2) | — | PASS |
| Q-12 | α weight for composite tier | — | — | 0.65 (§04.3) | — | PASS |
| Q-13 | T1 total route miles (~36%) | — | — | 17,662 mi (§07) | — | WARN: Tab 2 lists T1 miles not summed explicitly; 17,662 is cited in §07 without explicit derivation |
| Q-14 | T1 managed lane cost estimate | — | — | $177–265B (§07) | — | PASS (based on 17,662 mi × $10–15M/mi = $176.6–$264.9B; rounds correctly) |
| Q-15 | IIJA roads/bridges allocation | $110B (§02) | $110B (§02) | — | — | PASS |
| Q-16 | I-110 drops to tier | — | "T4" in §04.3 body | T2 in scores-all.csv | — | **FAIL: §04.3 says "I-110 drops to centrality-adjusted T4" but scores-all.csv has I-110 = T2; this is the key factual error** |
| Q-17 | I-880 drops to tier | — | "T2" in §04.3 | T3 in scores-all.csv | — | **FAIL: §04.3 says I-880 drops to T2 but scores-all.csv has I-880 = T3** |
| Q-18 | Planning doc review (12 LRTPs in §05 vs 50 in A.2) | — | 12 LRTPs (§05.3) | — | — | WARN: A.2 §06 external validation cites 50 LRTPs; this paper cites 12. Different claims (12 reviewed for T1 presence here vs 50 for broader ranking consistency in A.2). Cross-paper inconsistency; should acknowledge scope difference |
| Q-19 | Rubric version referenced | — | "12 dimensions" implied v1.0 | Tab 1 footnote: "ROUTE rubric v1.0" noted | — | WARN: scoring tables use v1.0 but conclusion references A.2 will "quantify precisely" — rubric version should be explicit on all score citations |
| Q-20 | B2 partial-graph caveat | — | §03 para "B2 Reliability Caveat" present | — | §07 "Limitations" present | PASS — caveat is present and explicit |

**CONSISTENCY: 4 FAILURES, 5 WARNINGS**

```
P1 (must fix):
  - Q-05: Abstract says "~80 Regional Feeders" but paper body/tables say 61. Difference of 19
    corridors. The correct number is 61 (matches Tab 2 and §01 contributions list).
    Fix: Change abstract "approximately 80 Regional Feeders" to "61 Regional Feeders"

  - Q-06: Abstract says "114 Local Access routes" but paper body/tables say 133.
    Total check: 8+25+61+133=227 ✓; 8+25+80+114=227 ✓ — both add to 227 but internal counts
    differ. The correct counts are 61/133 (from Tab 2 and §01).
    Fix: Change abstract "114 Local Access routes" to "133 Local Access routes"

  - Q-16: §04.3 text says "I-110 drops to centrality-adjusted T4" but scores-all.csv shows
    I-110 = T2 (score 19.0, T2 boundary ≥15 in v1.1; note v1.2 threshold is ≥11 for T3).
    I-110 at 19.0/150 is T2 per v1.2 thresholds (≥19 = T2). The "drops to T4" claim is wrong.
    Fix: Change to "I-110 drops to centrality-adjusted T2" — correct per scores-all.csv

  - Q-17: §04.3 text says "I-880 drops to T2" but scores-all.csv shows I-880 = T3 (15.0).
    Fix: Change to "I-880 drops to T3" — correct per scores-all.csv

P2 (should fix):
  - Q-01: Route miles 48,800 in abstract vs 48,792 in §03. Use consistent figure or explicitly
    note 48,792 is the corpus total and 48,800 is the FHWA rounded official figure.
  - Q-19: Add explicit rubric version tag "v1.0 rubric" or "v1.1 rubric (centrality-adjusted)"
    to the key scoring tables, consistent with A.2's forward-only protocol.
  - Q-08: Standardize STRAHNET count as 197 (verified) throughout; remove "~200" in §02.
  - Q-18: Clarify that 12 LRTPs is the subset reviewed for this specific T1 identification
    test; distinguish from A.2's broader 50-LRTP review.

P3 (minor):
  - §07 Implications: the forward reference to FAF5 "A.2 will quantify this precisely" is
    reasonable but should note the expected direction (T1 carries >50% ton-miles).
  - The MODULE.md contract requires the Brandes T1/T2 gap ≥ 3×. The paper demonstrates
    centrality qualitatively but never states the numeric gap ratio. Consider adding one
    sentence with the ratio from the data.
```

---

## PHASE 3 — CONTRACT CHECK

| Promise (from plan.md) | Paper section | Delivered? | Gap |
|------------------------|---------------|-----------|-----|
| Score all 227 corridors against 12 dimensions | §03, Tab 1 (dim table) | Yes | ✓ |
| Use B2 centrality and A2 freight intensity as primary tiering signals | §04.3, Eq.1 | Yes, with α=0.65 weight | ✓ |
| Cluster into 3–4 tiers using natural breaks in joint B2/A2 distribution | §04.1, §04.3 | Yes (4 tiers) | ✓ |
| Validate against STRAHNET designation | §05.1, Tab 1 | Yes | ✓ |
| Validate against ATRI bottleneck frequency | §05.2, Tab 2 | Yes, with ρ | ✓ |
| Schematic "metro map" visual output | §06 (arterial map section) | Section exists; actual figure referenced but no figure file present in dir | PARTIAL — figure placeholder only |
| Tier 1: ~8 routes carrying >40% national freight ton-miles | §08 conclusion: "majority" | Qualitative only; MODULE.md requires ≥50% ton-miles | PARTIAL |
| Tier 2: ~25 routes; Tier 3: ~80 routes; Tier 4: ~114 routes | Tab 2: 25/61/133 | YES but abstract still says 80/114 (P1 fix) | PARTIAL until abstract corrected |

```
CONTRACT: PARTIAL
Promises kept: 6/8
Gaps:
  1. Schematic arterial map figure is referenced but no figures/ directory exists in
     publications/A.1+arterials-tiering/. Panel cannot evaluate the visual output.
     Either note it as external asset or include figure stub.
  2. Ton-miles claim: plan says >40%, MODULE.md says ≥50%. Paper says "majority" and
     ">50%" in abstract. The quantification contract is partially delivered: the claim is
     present but FAF5 attribution is noted as incomplete. Add explicit "estimated from
     FAF5 zone-traversal method, pending routing-based attribution" hedge.
MODULE.md primary number delivered: PARTIAL — ton-miles stated, Brandes 3× ratio not stated numerically
```

---

## PHASE 4 — REFEREE SIMULATION

**Selected referees**: R-Traffic (centrality method), R-Network (graph algorithm), R-Policy (investment case)

---

```
REFEREE 1 — R-Traffic (Transportation Research Part A archetype)
Recommendation: Major Revision

SUMMARY: The paper makes an important contribution by demonstrating the congestion-stress
paradox and proposing a centrality-adjusted classification. The core finding is credible
and the validation against STRAHNET is elegant. However, the BPR centrality implementation
is not clearly distinguished from HCM-based capacity analysis, and the α parameter
selection is insufficiently justified for a methods paper — "stable region" is asserted
but the bounds are not formally tested.

MAJOR CONCERNS:
[I-01] §04.3 Eq.1: The composite weight α = 0.65 is identified from sensitivity analysis
       but the sensitivity test (§04.4) only reports that T1 assignments are "stable for
       α ≥ 0.55." The paper does not test whether α affects T2/T3 assignments, only T1.
       If α changes T2/T3 assignments substantially, the full tier framework is not stable.
[I-02] §03: A3 scores use IRI proxy with capped maximum. The paper acknowledges this is
       a limitation but does not quantify how many corridors are affected or what the
       distribution of IRI-sourced vs PTI-sourced A3 scores looks like. For a paper
       claiming to score 227 corridors, this is a significant data quality disclosure gap.
[I-03] Tab 1 (ATRI): The note "ATRI bottleneck counts per corridor are approximated from
       public report location data" is inadequate disclosure. The public ATRI report does
       not list precise lat/lon for all locations; attribution to corridors requires judgment
       calls. The methodology for location-to-corridor attribution must be described.

MINOR CONCERNS:
- Abstract: T3/T4 counts inconsistent with body (P1 issue flagged above)
- §05.3: 12 LRTPs reviewed; selection criteria not stated — could be cherry-picked
- §07 Implications: $10–15M/mi for managed lane construction needs a current cost citation;
  this range is from older FHWA studies and may be low for current conditions
```

---

```
REFEREE 2 — R-Network (Transportation Science / PNAS archetype)
Recommendation: Major Revision

SUMMARY: The betweenness centrality application is methodologically sound but the
partial-graph problem is more severe than the paper acknowledges. The paper notes
the B2 caveat but does not quantify the sensitivity of T1 assignments to graph
incompleteness. For a claim that 8 specific corridors are the national arterials,
the reader needs to know whether that claim is robust to completing the graph.

MAJOR CONCERNS:
[I-04] §03 B2 Caveat: Betweenness centrality on a 31-state partial graph produces
       systematically biased scores for corridors in the 19 unrepresented states.
       The paper says "relative rankings among the 8 highest-centrality corridors are
       directionally correct" — but this assertion is not tested. A simulation removing
       31 states from a complete graph and checking rank stability would directly
       address this concern.
[I-05] §04.3: The Brandes implementation uses "simplified predecessor tracking" (§03).
       Brandes' algorithm's correctness depends on proper predecessor tracking; a
       simplified version may produce incorrect intermediate values. The paper should
       cite which variant was implemented and whether it was validated against a known
       graph.
[I-06] Scalability: the national graph has 12,421 edges and 12,011 nodes. For Brandes
       O(VE) on a sparse graph this is computationally feasible but the paper does not
       state runtime or whether the graph is treated as directed or undirected. This
       matters for freight flows (directed) vs. access flows (undirected).

MINOR CONCERNS:
- No reproducibility package mentioned; graph construction methodology is described
  but a minimal reproducibility artifact (edge list + score table) would enable
  independent verification
```

---

```
REFEREE 3 — R-Policy (Transport Policy archetype)
Recommendation: Accept with Minor Revision

SUMMARY: This is exactly the kind of practical classification work that transportation
planning lacks. The tier framework is clearly motivated and the IIJA policy hook is
current and relevant. The schematic map is mentioned but not present in the manuscript;
without it the visual communication claim is incomplete. The investment case numbers
are order-of-magnitude reasonable.

MAJOR CONCERNS:
[I-07] §06 Arterial Map: The section title promises a schematic visualization but the
       manuscript contains only a figure reference (\includegraphics{figures/...}) with
       no actual figure included. The map is described as the paper's "visible output."
       Panel cannot evaluate this contribution. Include the map or explicitly state it
       is a supplementary online-only product.

MINOR CONCERNS:
- §07 Tier 4 / Rural Access: "Several T4 corridors are the only interstate within 50
  miles for significant rural populations" — this is quantified in B.1 but should have
  at least an order-of-magnitude figure here (e.g., "43 T4 corridors serve as the sole
  interstate within 50 miles for more than 500,000 rural residents" — estimate from B.1)
- §02 Investment background: ASCE $1.2T backlog figure is from 2021 report; update to
  most current if available
```

---

## PHASE 5 — ABSTRACT CHECK

```
ABSTRACT: ~170 words
Primary result stated: PARTIAL — "eight Primary Arteries carrying over 50% of national
  truck freight ton-miles" YES; "four tiers: eight Primary Arteries ... approximately 25
  Major Connectors; 80 Regional Feeders; and 114 Local Access routes" — T3/T4 counts wrong (P1)
Method named: YES ("12-dimensional rubric, using Brandes betweenness centrality and FAF5
  commodity flow as primary signals")
Policy implication: YES ("tier classification provides a principled basis for Interstate 2.0
  investment prioritization")
Track chain position: YES ("The tier classification provides a principled basis...")
```

P1: Abstract states wrong T3/T4 counts (80/114 vs correct 61/133). Must fix before panel.

---

## PHASE 6 — INTERNAL CROSS-PAPER CONSISTENCY

```
CROSS-PAPER CONSISTENCY:
  Papers cited: A.2 (forward ref), B.1 (forward ref), C.2 (forward ref), ROUTE_E2 (forward ref)
  Values cross-checked: 4
  Issues:
    1. §04.3 footnote cites ROUTE_A2 for rubric v1.2 thresholds — forward reference, acceptable
    2. §05.3 cites 12 LRTPs for external validation; A.2 §06 cites 50 LRTPs — different scope,
       should be disambiguated to avoid reader confusion
    3. scores-all.csv (v1.2 data) shows I-110=T2, I-880=T3, I-225=T3 — the paper's §04.3
       text describing tier drops is inconsistent with the data file (P1 issue I-16, I-17)
    4. B2 partial-graph caveat in this paper (§03) is consistent with A.2 §03's description
       of B2 instability — PASS
  Stale citations: None (forward refs only; no back-citations to papers not yet written)
```

---

## PHASE 7 — PRE-PANEL CHECKLIST

```
═══════════════════════════════════════════════════════════
POST-WRITE COMPLETE: A.1+arterials-tiering
═══════════════════════════════════════════════════════════

Validation results:
  Consistency:       4 FAILURES, 5 WARNINGS
  Contract:          PARTIAL (6/8 promises; figure missing; ton-miles hedge needed)
  Referee sim:       Major Revision (R-Traffic + R-Network); Accept with Minor Revision (R-Policy)
  Abstract:          ~170 words, primary number stated (ton-miles ≥50%), T3/T4 counts WRONG
  Cross-paper:       1 data file conflict (I-110/I-880 tier labels in text vs scores-all.csv)

P1 blockers (fix before panel review):
[I-01-abs] Abstract says "approximately 80 Regional Feeders; and 114 Local Access routes"
           → Change to "61 Regional Feeders; and 133 Local Access routes" (matches Tab 2, §01)
[I-02-text] §04.3 body says "I-110 drops to centrality-adjusted T4"
           → Change to "I-110 drops to centrality-adjusted T2" (scores-all.csv: I-110=T2)
[I-03-text] §04.3 body says "I-880 drops to T2"
           → Change to "I-880 drops to T3" (scores-all.csv: I-880=T3, score=15.0)
[I-07-fig]  §06 Arterial Map: figure reference exists but no figures/ directory present
           → Either add figure or note "figure available as supplementary material"

P2 items (should fix):
[I-04] Route miles: 48,800 (abstract/conclusion) vs 48,792 (§03) — standardize or explain
[I-05] STRAHNET count: "~200" in §02 → use 197 to match §05 data
[I-06] Rubric version tag: add explicit "v1.0 rubric" label to scoring tables
[I-08] ATRI attribution methodology: expand the note in §05.2 to explain corridor attribution

P3 items (optional polish):
- State Brandes T1/T2 centrality ratio numerically (MODULE.md contract requires ≥3×)
- Update ASCE 2021 backlog citation if more recent figure is available
- §07: Add order-of-magnitude T4 rural isolation figure from B.1

PRE-PANEL CHECKLIST:
□ Abstract T3/T4 counts corrected (61/133 not 80/114)
□ §04.3 I-110 tier label corrected (T2 not T4)
□ §04.3 I-880 tier label corrected (T3 not T2)
□ Figure placeholder in §06 resolved (add or note as supplementary)
□ MODULE.md primary quantitative contract delivered (ton-miles ≥50% stated; Brandes 3× add)
□ BPR extrapolation not applicable to this paper (no BPR formula used)
□ Net vs gross cost not applicable (no cost claims)
□ Rubric version tagged as v1.0 on scoring tables
□ Cross-paper: I-110/I-880 tier labels match scores-all.csv
□ Abstract states primary quantitative result (with corrected counts)
□ Referee P1 blockers addressed (α stability test, graph completeness note)

VERDICT: FIXES REQUIRED
Fixes required: 4 P1, 4 P2
Next: run /panel:publication review A.1+arterials-tiering after P1 fixes
═══════════════════════════════════════════════════════════
```
