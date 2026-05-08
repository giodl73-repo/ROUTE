---
paper: B.3+resilience-holes
review_type: recheck-synthesis
round: 1
date: 2026-05-08
reviewers_rechecked: [elefteriadou, mckinnon]
verdict: ADVANCE
new_stage: ready
---

> AI-generated simulated recheck synthesis.

## Recheck Summary

Two rechecks were required: Elefteriadou (PP2.4, score 2/4) and McKinnon (PP2.4 + PP2.5, score 2/4) were the blocking reviewers from Round 1. The correction was applied in two passes: §03 was corrected first; a post-write check identified that the corrected figures had not propagated to §01, §04, §05, and §07. The 2026-05-08 recheck verifies the full propagation.

**PP2.4 — Waiting cost rate correction, propagated to all sections:** The Donner D1 benefit calculation used $225/hr (ATRI in-motion rate) for stranded trucks. The correct idle rate is $91/hr (driver + idle fuel). The correction was propagated to all four affected sections: §01 (introduction) explicitly states $1.3B annual cost with the idle rate breakdown; §04 (tab:compound) shows Donner at 1.30; §05 (investment sequencing) references $1.3B in the D1-only ordering discussion; §07 (conclusion) states NPV $12.1B, CBR 4.0:1, payback 3.1 years. The $1.6B figure is absent from all sections.

**PP2.5 — Cross-track citation (McKinnon):** B.3 cites Paper D.1 (\citep{ROUTE_D1}) in Section 02 and explicitly documents the cross-track dependency: B.3's compound corpus validation requires D.1 to be finalized first.

Elefteriadou returns **PASS** (3/4). McKinnon returns **PASS-WITH-NOTE** (3/4).

## Panel Score After Recheck

| Reviewer | Round 1 | Recheck |
|---|---|---|
| Chester | 3/4 | — (no recheck required) |
| McKinnon | 2/4 | **3/4** |
| Elefteriadou | 2/4 | **3/4** |
| Puentes | 3/4 | — |
| Hanson | 3/4 | — |
| **Mean** | **2.6/4** | **3.0/4** |

## P3 Notes Carried Forward

Both reviewers added P3 notes (non-blocking):

- **Elefteriadou:** The portfolio table in §05 reports a total portfolio CBR of 5.0:1. Authors should confirm this figure reflects the corrected $1.3B Donner annual benefit (not the original $1.6B) and add a footnote confirming which Donner input was used.
- **McKinnon:** The D1 scores in B.3's compound table are drawn from D.1 v1.2. D.1 v1.3 introduced an ECH100 normalization (μ = 5.0 structural multiplier). Authors should add a footnote noting that v1.2 raw scores are used as a conservative input; a v1.3 recalculation is planned.

These items do not block advancement. Authors should address them in the next revision pass.

## Panel Decision

The paper advances. Both blocking items (PP2.4 propagation and PP2.5 citation) have been addressed and verified. Mean panel score rises from 2.6/4 to 3.0/4. No remaining P1 items.

The corrected Donner figures ($1.3B annual cost, $12.1B NPV, 4.0:1 CBR, 3.1-year payback) are now consistent across all sections and internally defensible. The compound investment advantage over single-dimension alternatives is preserved at the corrected figures.

**Stage advances to: `ready`**
