---
paper: A.1+arterials-tiering
round: 1
recheck_date: 2026-05-07
recheck_reviewers: [hanson, adamic]
recheck_verdicts: [PASS, PASS-WITH-NOTE]
scores_before: {hanson: 3, adamic: 2, transport-policy: 3, traffic-engineer: 3, freight-economist: 3}
scores_after: {hanson: 3, adamic: 3, transport-policy: 3, traffic-engineer: 3, freight-economist: 3}
avg_before: 2.8
avg_after: 3.0
min_before: 2
min_after: 3
stage: ready
---

# Recheck Synthesis — A.1+arterials-tiering Round 1

## Result

Both recheck reviewers pass. **Paper advances to `ready`.**

| Reviewer | Round 1 | Recheck | Change |
|---|---|---|---|
| Susan Hanson (Transport Geographer) | 3/4 | PASS — 3/4 | held |
| Lada Adamic (Network Scientist) | 2/4 | PASS-WITH-NOTE — 3/4 | +1 |
| Transport Policy | 3/4 | not rechecked | held |
| Traffic Engineer | 3/4 | not rechecked | held |
| Freight Economist | 3/4 | not rechecked | held |
| **Mean** | **2.8/4** | **3.0/4** | **+0.2** |
| **Min** | **2/4** | **3/4** | **+1** |

## What the Revision Resolved

**P1.1 — α circularity (Hanson).** The circular calibration-validation loop is broken. α=0.65 is now presented as the midpoint of the stable region (α ≥ 0.55), not as a parameter estimated from STRAHNET. STRAHNET is demoted to post-hoc consistency check. The Section 5.3 transportation planning document frequency (47/50, 94%) is the independent external calibration source. All three evidence streams — document frequency, STRAHNET, ATRI — are now genuinely independent.

**P1.2 — B2 reliability (Adamic).** The directional-ranking-only claim is explicit in Section 3. Three independent validations are listed: (i) degree-node agreement — the 8 T1 corridors (I-5, I-10, I-35, I-40, I-75, I-80, I-90, I-95) are also the 8 highest-degree nodes by intersection count, identifiable from network topology without any Brandes computation; (ii) STRAHNET 100% alignment; (iii) ATRI ρ=0.72. The simplified Brandes limitation is carried forward as acknowledged future work.

## Carried Notes (not blocking)

- **Hanson P3:** The Section 5.3 transportation plan document calibration (94% of state DOT plans) is a stronger policy-facing anchor than STRAHNET alignment — consider foregrounding it in the abstract.
- **Adamic P3:** Future work should implement a validated full Brandes on the 50-state national graph. Until then, any downstream B2 usage must restrict to directional-ranking claims.

## P2 Items Status

P2.1 (v1.1 threshold note), P2.2 (capacity formula), P2.3 (I-40 qualification) were addressed in the same revision pass per the revision plan and are accepted without recheck. P2.4 (A2 sensitivity note) and P2.5 (EV charging gap) were similarly addressed. No P2 items are outstanding.

## Stage Advancement

Paper moves from `revision` to `ready`. No further review required before venue submission.
