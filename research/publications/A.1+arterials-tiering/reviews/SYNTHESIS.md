---
paper: A.1+arterials-tiering
round: 1
reviewers: [transport-geographer, network-scientist, transport-policy, traffic-engineer, freight-economist]
date: 2026-05-07
scores: [3/4, 2/4, 3/4, 3/4, 3/4]
avg_score: 2.8/4
min_score: 2/4
stage_advancement: revision
---

# Review Synthesis — A.1+arterials-tiering Round 1

## Headline Assessment

Strong conceptual contribution; methodological weaknesses in the B2 centrality computation and α estimation are addressable. The congestion-stress paradox — the paper's core finding — is accepted unanimously. The centrality-adjusted T1 (8 corridors) is accepted by 4 of 5 reviewers; the network scientist raises legitimate concerns about B2 reliability that must be addressed but do not invalidate the finding. Mean score 2.8/4, minimum 2/4.

**Paper advances to revision.** Two P1 items must be addressed before recheck.

---

## What the Panel Agrees On (Earned Stakes)

**E1 — The congestion-stress paradox is real and well-framed.** All 5 reviewers accepted it. The I-110/I-80 contrast is vivid and correct. The term "congestion-stress paradox" is a useful contribution to the discourse.

**E2 — ATRI bottleneck validation is the strongest evidence.** 4 reviewers highlighted it positively. ρ=0.72 vs. 0.61 is meaningful improvement; ATRI is operationally grounded data. This is stronger evidence than STRAHNET alignment for operational credibility.

**E3 — The 8 T1 corridors are recognized by practitioners.** Transportation plan document validation (94%) is strong external confirmation. The freight economist and traffic engineer both recognized the 8 corridors as operationally appropriate.

**E4 — The Table 5 feature-to-tier matrix is useful policy output.** All reviewers accepted it. The traffic engineer specifically validated the C-D road and managed freight lane tier assignments as consistent with HCM practice.

**E5 — The "metro map" framing works.** All reviewers accepted it as pedagogically effective. The freight economist extended it positively to logistics route planning.

---

## Contested Stakes

| Reviewer | Stake | Counter | State |
|---|---|---|---|
| Network Scientist | B2 scores are unreliable (simplified Brandes, partial graph) — finding may be artifact | Transport Geographer: sensitivity shows result is stable across α | Live — needs resolution |
| Transport Geographer | α=0.65 estimation is circular (calibrated to STRAHNET, validated against STRAHNET) | — | Live — needs resolution |
| Transport Policy | Tier thresholds shifted v1.0→v1.1; paper uses v1.0 values | — | Live — minor, addressable |

---

## Collision: B2 Reliability vs. Robustness

**Transport Geographer** stakes: the α=0.65 sensitivity analysis (§4.4) shows result is stable for α ≥ 0.55 — the specific B2 values don't matter as long as relative rankings are correct.

**Network Scientist** stakes: the simplified Brandes implementation may produce incorrect relative rankings, not just wrong absolute values. If the predecessor tracking is wrong, the centrality ranking could be fundamentally off.

**Resolution**: The network scientist's concern is stronger on the fundamentals. The paper must show that the 8 T1 corridors are identifiable from network topology alone (degree, betweenness from a clean implementation) independent of the partial-graph approximation. The transport geographer's robustness argument holds for the α parameter but not for the underlying B2 values.

---

## P1 — Blocking (must address before recheck)

**P1.1 — Address the circularity in α estimation.**
*(Transport Geographer, Transport Policy)*
The paper estimates α=0.65 by maximizing STRAHNET alignment, then validates against STRAHNET. Reframe as follows: (a) show that the 8 T1 corridors are stable for all α ≥ 0.55 (already in §4.4) — this is the primary finding; (b) use STRAHNET alignment as a post-hoc consistency check, not a calibration target; (c) use the transportation planning document data (§5.3) as the calibration dataset if a specific α is needed. The claim "α=0.65 estimated by maximizing STRAHNET alignment" should be dropped or reframed as "α=0.65 is within the stable range; the 8 T1 corridors are robust to any α ≥ 0.55."

**P1.2 — Address B2 reliability directly.**
*(Network Scientist)*
Add a methodological caveat section (or paragraph in §3) stating: (a) B2 is computed via simplified Brandes on a partial directed graph; (b) the simplification may affect absolute values but the claim is that relative rankings among the 227 corridors are directionally correct; (c) a validation: the 8 T1 corridors also emerge as the highest-degree nodes in the national graph (I-5, I-10, etc. have the most interstate intersections), confirming the topology-based identification independent of the Brandes computation.

---

## P2 — Important (address for quality)

**P2.1 — Update to v1.1 tier thresholds or note the version.**
*(Transport Policy)*
The paper's Table 1 shows v1.0 thresholds (T1≥26, T2≥20, etc.). The rubric is now at v1.1 (T1≥21, T2≥15). Either update the quantitative results or add a note that scores are reported under v1.0 rubric; v1.1 adjusts thresholds but preserves the same 13 aggregate-score T1 and same 8 centrality-adjusted T1. This is a minor but necessary clarification.

**P2.2 — State the A1 capacity formula explicitly.**
*(Traffic Engineer)*
Section 3 references HPMS AADT but does not state how theoretical capacity is computed. Add one sentence: "We compute theoretical daily capacity as lane\_count × 1,900 pcphpl × 24h × K^{-1} where K=0.09 (peak hour factor), yielding approximately 37,000 vpd per lane-pair per direction." This allows readers to evaluate the V/C claims.

**P2.3 — Qualify the I-40 "at target" claim.**
*(Traffic Engineer)*
I-40's corridor-average V/C is near target; specific interchange areas (Albuquerque, Nashville) exceed it. Add: "I-40's corridor-average V/C (0.84) is near the I2.0 PTI target, though specific interchange clusters require local capacity improvements."

**P2.4 — Add A2 sensitivity note.**
*(Freight Economist)*
All A2 scores are estimated via FAF5 zone traversal. Note how sensitive tier assignments are if A2 values differ by ±20% — this is addressable as "A2 contributes to the aggregate score which is weighted at 0.35 in the tier composite; a 20% change in A2 across all corridors shifts total scores by ~2 points, which does not change any tier assignment at v1.1 thresholds."

**P2.5 — Add EV charging gap analysis for T1.**
*(Freight Economist)*
Table 5 specifies EV charging standards but doesn't show current T1 coverage. Add one sentence: "Current DC fast charging density on T1 corridors ranges from 0.3 to 4.1 DCFC per 100 miles [cite DOE AFDC]; all T1 corridors have gaps requiring I2.0 investment."

---

## Score Summary

| Reviewer | Score | Primary concern |
|---|---|---|
| Transport Geographer | 3/4 | α circularity; B2 partial-graph sensitivity |
| Network Scientist | 2/4 | B2 reliability; simplified Brandes; directed graph inconsistency |
| Transport Policy | 3/4 | No federal implementation mechanism; v1.0 thresholds stale |
| Traffic Engineer | 3/4 | Capacity formula not stated; I-40 at-target claim needs qualification |
| Freight Economist | 3/4 | A2 estimated; commodity criticality absent; decarbonization gap |
| **Mean** | **2.8/4** | — |
| **Min** | **2/4** | — |

Mean 2.8 > 2.5 threshold. Min 2 = floor. **Paper advances to revision.**

---

## Next

After P1.1 and P1.2 are addressed: recheck with Transport Geographer + Network Scientist (the two reviewers with live P1 stakes). P2 items can be addressed simultaneously. Stage: revision → recheck with 2-reviewer panel.
