---
reviewer: Lada Adamic
paper: A.1+arterials-tiering
review_type: recheck
round: 1
date: 2026-05-07
pp_items_rechecked: [P1.2]
verdict: PASS-WITH-NOTE
score: 3/4
---

> **Note:** AI-generated simulated recheck review.

## Items Rechecked

### P1.2 — B2 reliability and independence of the centrality-adjusted T1 finding

**Concern.** The original paper's central claim — that centrality-adjusted classification outperforms aggregate-score classification — depended on B2 betweenness centrality scores that the paper itself acknowledged were computed on a partial directed graph using a simplified Brandes implementation. My concern was twofold: (a) the paper did not state the reliability limitation candidly in the main text, only in a footnote; and (b) no topology-based validation independent of the Brandes computation was provided. Without such validation, the 8 T1 corridors could be an artifact of the partial-graph approximation rather than a structural feature of the national network.

**Revision.** The revised paper adds a methodological caveat section in Section 3 (B2 Reliability and the Directional-Ranking Claim) that states all three required elements explicitly: (a) B2 is computed via simplified Brandes on a partial directed graph covering 31 states; (b) the simplification may affect absolute values but the claim is that relative rankings among the 227 corridors are directionally correct — the paper explicitly restricts its claim to rank order, not cardinal scores; (c) three independent validations are listed. The three validations are: (i) degree-node agreement — I-5, I-10, I-35, I-40, I-75, I-80, I-90, and I-95 are the eight corridors with the highest intersection count in the national graph, identified from network topology alone without Brandes; (ii) STRAHNET alignment — 100% of the centrality-adjusted T1 matches the STRAHNET strategic designation; (iii) ATRI bottleneck agreement — Spearman ρ=0.72 between T1 classification and ATRI bottleneck cost density.

**Verdict.** The degree-node validation is the key item I was looking for. The claim that the 8 T1 corridors are also the 8 highest-degree nodes in the national graph — identifiable from network structure alone, before any Brandes computation is run — is precisely the topology-independent confirmation that breaks the dependence on the simplified implementation. I checked that the claim is specific: the paper names I-5, I-10, I-35, I-40, I-75, I-80, I-90, and I-95 as the corridors with the most interstate intersections, which matches the degree-ranking result. The directional-ranking-only claim is appropriately hedged. The three-validator structure is defensible for journal submission.

**P3 note.** The simplified Brandes implementation remains a genuine limitation even with the three validations. The degree-node agreement is compelling, but degree and betweenness can diverge on non-planar graphs — a corridor can have many intersections but low betweenness if those intersections are all with low-traffic spurs. Future work should implement a validated full Brandes algorithm on the complete 50-state national graph; until then, the paper's restriction to directional ranking claims is appropriate but should be carried forward explicitly into any downstream analysis that uses B2 scores.

## Verdict

The three independent validations — degree-node agreement, STRAHNET alignment, and ATRI ρ=0.72 — together establish that the 8 T1 corridors are a structural feature of the national network, not an artifact of the simplified Brandes implementation. The directional-ranking-only claim is correctly scoped. PASS-WITH-NOTE at 3/4, rising from the original 2/4; the blocking concern is resolved, with the Brandes implementation limitation carried forward as acknowledged future work.
