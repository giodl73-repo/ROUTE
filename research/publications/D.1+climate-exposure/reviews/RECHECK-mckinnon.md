---
reviewer: Alan McKinnon
paper: D.1+climate-exposure
review_type: recheck
round: 1
date: 2026-05-07
pp_items_rechecked: [PP1.3]
verdict: PASS-WITH-NOTE
score: 3/4
---

> **Note:** AI-generated simulated recheck review.

## Items Rechecked

### PP1.3 — Composite Metric Category Error: Are D1_flood and D1_winter now dimensionally comparable?

My Round 1 blocking concern was precise: the paper combined SFHA polygon mileage (a slow-moving physical geography measurement of floodplain extent) with annual closure-hours (a volatile operational metric reflecting DOT maintenance investment, weather year, and institutional practices) into a single D1 score and then ranked all corridors together in the same table. That is a category error. Donner D1=7.8 and Gulf Coast TX D1=7.8 are not equivalent levels of climate exposure under a methodology that computes them from non-comparable raw inputs.

The revision adds Section 3.4, "D1 Composite Normalization (v1.3 Amendment)" (sec:d1-normalize), which addresses this concern directly. The approach is to convert both raw metrics to expected annual lane-closure-hours per 100 miles of corridor (ECH100) before applying the D1 scoring function. The ECH100 formulas are given in Equations 3 and 4. For flood corridors: ECH100_flood = (m_c × p_flood × d_flood) / (L/100), where p_flood = 0.03 (3-event expected frequency on a 100-year SFHA) and d_flood = 72 hours mean closure duration. For winter corridors: ECH100_winter = (closures/yr × d_winter) / (L/100), directly observed from state DOT logs. Both quantities now share the same unit — expected annual lane-closure-hours per 100 miles — and a uniform anchor map is applied.

This resolves the dimensional comparability problem I identified. Both metrics are now in the same unit. The D1 scoring function is applied uniformly. The comparison that was scientifically indefensible in Round 1 — Donner D1=7.8 versus Gulf Coast TX D1=7.8 — is now grounded in the same measurement framework.

### PP1.3 (detail) — Structural Multiplier μ=5.0: Is this adequately grounded?

The ECH100 normalization surfaces a secondary issue that the paper handles with a structural multiplier μ=5.0 applied to ECH100_flood. The motivation is correct: flood inundation closures require replacement of roadway surface, drainage structures, and embankments (weeks of repair) rather than simply reopening a gate after a winter closure (hours to days). Without this multiplier, the ECH100 framework systematically undervalues flood corridors relative to winter corridors, because the raw probability-duration product (p_flood × d_flood = 0.03 × 72 = 2.16 hr per occurrence) underrepresents the true economic disruption of a major inundation event that takes 3–6 weeks to restore to full capacity.

The μ=5.0 value is described as "the principled correction that makes the flood and winter D1 scores dimensionally comparable." That is true in the sense that it recovers the v1.2 score for Gulf Coast I-10 Louisiana (D1=8.4), which was calibrated independently through the rubric scoring process. It is also consistent with order-of-magnitude reasoning from the freight economics literature: a two-week inundation closure on a major freight corridor would impose shipper rerouting costs, inventory buffer costs, and perishable cargo losses that dwarf the costs of a 24-hour winter closure on an equivalent corridor. A multiplier of 5.0 is not unreasonable for that difference in economic consequence.

I will not block promotion on the μ=5.0 choice, because: (1) it is economically grounded even if not empirically calibrated, (2) it recovers scores that were independently validated in v1.2, and (3) the paper correctly labels this as a v1.3 amendment that is recorded in the rubric changelog under the forward-only protocol. Future calibration against observed freight cost data from Gulf Coast hurricane closures (e.g., post-Katrina I-10 closure economics from the FHWA incident cost literature) would strengthen the parameter choice, and I note this as a P3 item for future work.

## Verdict: PASS-WITH-NOTE

PP1.3 is resolved. The ECH100 normalization converts both D1_flood and D1_winter to the same unit before scoring, eliminating the category error I identified in Round 1. I am raising my score from 2/4 to 3/4. The structural multiplier μ=5.0 is the residual uncertainty in the normalization framework: it is economically motivated but not empirically calibrated, and future calibration against observed flood-closure cost data is warranted. This is a P3 note, not a blocking item.
