---
paper: B.3+resilience-holes
review_type: recheck-synthesis
round: 1
date: 2026-05-07
reviewers_rechecked: [elefteriadou, mckinnon]
verdict: ADVANCE
new_stage: ready
---

> **Note:** AI-generated simulated recheck synthesis.

## Recheck Summary

Two rechecks were required: Elefteriadou (PP2.4, score 2/4) and McKinnon (PP2.5, score 2/4) were the blocking reviewers from Round 1.

**PP2.4 — Waiting cost rate correction (both reviewers):** The Donner D1 benefit calculation used $225/hr (ATRI in-motion rate) for stranded trucks. The correct idle rate is $91/hr (driver + idle fuel). Revision corrects the rate, revises D1 benefit from ~$700M to ~$400M/year, combined benefit from $1.6B to $1.3B/year, NPV from $15.8B to $12.1B, and CBR from 5.75:1 to 4.0:1. All tables updated consistently. The investment priority ordering is unchanged.

**PP2.5 — Cross-track citation (McKinnon):** B.3 now cites Paper D.1 (\citep{ROUTE_D1}) in Section 2 and explicitly documents the cross-track dependency: B.3's compound corpus validation requires D.1 to be finalized first.

Both rechecks return **PASS-WITH-NOTE**.

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

- **Elefteriadou:** Include O&M cost sensitivity in Table 1 (tunnel O&M $96–225M/year at analogous facilities); show NPV with and without O&M.
- **McKinnon:** Align D1 benefit calculation with D.1 v1.3 normalized score (μ = 5.0 structural multiplier) rather than v1.2 raw score; or note explicitly that v1.2 raw score is used as a conservative input.

These items do not block advancement. Authors should address them in the next revision pass.

## Panel Decision

The paper advances. Both blocking items (PP2.4 and PP2.5) have been addressed to the satisfaction of the blocking reviewers. Mean panel score rises from 2.6/4 to 3.0/4. No remaining P1 items.

The corrected Donner NPV ($12.1B, 4.0:1 CBR) is analytically sounder than the original and remains the highest-NPV project in the compound exposure portfolio. The compound investment advantage over single-dimension alternatives is preserved.

**Stage advances to: `ready`**
