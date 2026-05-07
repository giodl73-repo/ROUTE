---
reviewer: Transport Geographer
persona: Susan Hanson (Clark University) — spatial access, network geography, mobility
round: 1
date: 2026-05-07
score: 3/4
---

> **Note:** This is an AI-generated simulated review, written by Claude in
> the voice of a named-expert persona. It is not an actual review by the
> named person and does not reflect their views or endorsement.

## Overall

A genuinely novel contribution: the congestion-stress paradox is a real finding that transportation planners need to see. The demonstration that aggregate scoring puts I-110 above I-80 is the paper's sharpest moment — it makes the methodological problem vivid in a way that abstract critique cannot. My concerns are about the α=0.65 estimation procedure (which creates a circularity) and about what the B2 centrality scores actually represent given that they are computed on a partial graph of uncertain quality.

## What Works

**The congestion-stress paradox is the right framing.** The distinction between "where the system is stressed" and "which corridors the system depends on structurally" is an important conceptual contribution that practitioners will recognize from their own experience. The I-110/I-80 contrast in Table 2 makes the argument concretely rather than abstractly. This is the section reviewers will remember.

**STRAHNET validation** (Section 5.1): 100% T1 alignment vs. 85% for aggregate-score T1 is a clean finding. STRAHNET is an independent strategic designation based on different criteria, so alignment with it is meaningful external validation.

**The ATRI correlation** (Section 5.2): Spearman ρ=0.72 vs. 0.61 is a meaningful improvement that transportation practitioners will recognize as significant. ATRI bottleneck density is the closest thing we have to revealed-preference evidence of where the freight network matters most.

**The "metro map" framing** (Section 6): Using transit cartography as the analogy is both pedagogically effective and structurally correct — the hierarchy of transit lines maps onto the highway tier structure in a way that makes the concept immediately legible.

## What Doesn't Work

**The α estimation is circular.** The paper estimates α=0.65 by "maximizing alignment with STRAHNET designation" and then validates the centrality-adjusted classification by showing 100% STRAHNET alignment. But if α was chosen to maximize STRAHNET alignment, the validation is circular — of course the result aligns with STRAHNET, it was estimated to do so. The paper needs either: (a) an independent validation set (transportation planning document data is used in Section 5.3 but not for α estimation — use it for both), or (b) a sensitivity analysis showing that the 8 T1 corridors are robust across a range of α values (0.5–0.8), not just at the estimated 0.65.

**B2 centrality is marked estimated throughout.** Section 3 notes that all B2 scores are estimated because the graph is a partial national network. But the paper's central argument — that betweenness centrality is the right primary tiering signal — depends on B2 being accurate. If the partial-graph B2 is systematically wrong in specific ways (e.g., underestimating centrality of corridors in states with incomplete data), the tier assignments could shift. This needs a sensitivity analysis: how different are the tier assignments when computed on the full 227-corridor graph vs. the partial graph used here?

**The 8 T1 corridors look like prior knowledge.** The centrality-adjusted T1 contains exactly the 8 corridors that practitioners would intuitively identify as primary arteries (I-5, I-10, I-35, I-40, I-75, I-80, I-90, I-95). This could mean the method is working correctly. It could also mean the method is being tuned to recover a pre-existing answer. The paper needs to acknowledge this interpretive ambiguity and be more explicit about whether it is discovering the tier structure or confirming it.

## The Question I'd Push On

The paper proposes α=0.65 as estimated from data, but the sensitivity analysis in Section 4.4 shows the 8 T1 corridors are stable for α ≥ 0.55. This means the result doesn't actually depend on the specific α=0.65 estimate — any value above 0.55 gives the same answer. So why present α estimation as if it matters? The paper would be stronger if it presented the finding as: "for any reasonable weighting of centrality over aggregate score (α ≥ 0.55), the same 8 corridors emerge as T1" — and dropped the STRAHNET-calibrated α entirely. Why not frame it this way?
