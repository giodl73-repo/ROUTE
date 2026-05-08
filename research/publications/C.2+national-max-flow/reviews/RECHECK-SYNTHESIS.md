---
paper: C.2+national-max-flow
round: 1
review_type: recheck-synthesis
date: 2026-05-07
blocking_items_resolved: 1
blocking_items_remaining: 0
verdict: ADVANCE
next_stage: ready
---

# Recheck Synthesis — Round 1 Recheck
## National Max-Flow: Capacity and Bottleneck Analysis of the Interstate Network

---

## Recheck Scope

One recheck review was commissioned: Lada Adamic (network-scientist), who held the sole blocking item (P1.1) and scored the paper 2/4 in Round 1. No other reviewers were re-engaged for the recheck — the other four reviewers scored 3/4 and their P2 items are addressed in the same revision pass but not subject to formal recheck.

---

## P1.1 Resolution

**Item:** Single-commodity sensitivity analysis (or multi-commodity extension) to demonstrate that the binding bottleneck identification is robust to the single-commodity max-flow assumption.

**Revision:** Section 3.6 ("Single-Commodity Sensitivity Analysis") added. Two-class sensitivity check using FAF5 SCTG commodity codes: high-value manufactured goods and chemicals (SCTG 7/8/9/17/30/34/38; 62% value / 28% weight; V/C rerouting threshold 0.85) vs. bulk freight (38% / 72%; threshold 1.05). Results: Donner closure −23% central (two-class range −19% to −27%), I-69 gain +18% central (two-class range +14% to +21%), I-40 compound saturation V/C 1.11 (two-class range 0.98–1.18, network failure threshold crossed in central case). I-69 NPV sensitivity table (Table 5): 3 discount rates × 3 commodity scenarios, with explicit breakeven characterization.

**Adamic verdict:** PASS-WITH-NOTE. Score 3/4 (from 2/4). Accepts the two-class bounding exercise as sufficient for the paper's claims. Notes that −23% within −19% to −27% confirms directional robustness and that the ±4 pp magnitude uncertainty is modest relative to the finding's policy significance. Notes full multi-commodity network flow as P3 future work (not a blocking condition).

---

## P2 Items Status

The P2 items from Round 1 were addressed in the same revision cycle. They are recorded here for completeness but are not subject to formal recheck review at this stage:

| Item | Description | Status in revision |
|---|---|---|
| P2.1 | Capacity value derivation documentation (Elefteriadou) | Capacity appendix added to Section 3.1; HCM source, peak-hour factor, directional asymmetry documented for three binding arcs |
| P2.2 | Max-flow utilization ratio by O-D cluster (Neumark) | Table 3 added with current utilization ratios (actual FAF5 demand / max-flow capacity) for all 8 cluster pairs |
| P2.3 | I-69 NPV sensitivity table (Puentes) | Table 5 added: 3 discount rates × 3 commodity scenarios; breakeven rate stated |
| P2.4 | Commodity composition of investment recommendations (McKinnon) | Section 6.1 paragraph added with FAF5 commodity value weights for I-69 incremental flow |
| P2.5 | Distinction between congestion-binding and resilience-binding bottlenecks (Neumark) | Section 4.1 paragraph added; Dallas and I-95 classified as congestion-binding; Donner as resilience-binding |
| P2.6 | Donner Pass peak-demand caveat (Elefteriadou) | Section 4.2 footnote added: V/C 0.82 is AADT-based; peak-season days approach V/C 1.0 |
| P2.7 | I-69 multistate NEPA coordination acknowledgment (Puentes) | Section 6.1 final paragraph added |

---

## Adamic P3 Note (Future Work)

Adamic's recheck adds one P3 suggestion: a full multi-commodity network flow LP implementation as the methodologically correct successor to the two-class bounding exercise. This is a meaningful research extension requiring a separate implementation effort; it is recorded as a future-work item and is not a condition of this paper's acceptance.

---

## Panel Decision

**The blocking item (P1.1) is resolved. No new blocking items introduced by the recheck. Paper advances to `ready`.**

Revised mean score (Adamic recheck applied, other reviewer scores held from Round 1):

| Reviewer | Round 1 | Recheck |
|---|---|---|
| Lada Adamic (network-scientist) | 2/4 | **3/4** |
| Lily Elefteriadou (traffic-engineer) | 3/4 | 3/4 |
| Alan McKinnon (freight-economist) | 3/4 | 3/4 |
| David Neumark (rural-economist) | 3/4 | 3/4 |
| Robert Puentes (transport-policy) | 3/4 | 3/4 |
| **Panel mean** | 2.8/4 | **3.0/4** |

---

## Next Steps

1. Update `_panel.yaml`: stage → `ready`, Adamic score → 3, round 1 recheck recorded.
2. Paper is cleared for venue submission to Transportation Science.
3. Adamic's P3 (full multi-commodity max-flow) may be addressed as a companion paper in the C-module (C.3 or C.4) rather than as a revision to this paper.
