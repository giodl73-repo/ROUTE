---
reviewer: Network Scientist
persona: Lada Adamic (Meta / University of Michigan) — network analysis, graph algorithms
round: 1
date: 2026-05-07
score: 2/4
---

> **Note:** This is an AI-generated simulated review, written by Claude in
> the voice of a named-expert persona. It is not an actual review by the
> named person and does not reflect their views or endorsement.

## Overall

The paper correctly identifies betweenness centrality as the right primary signal for arterial classification — this is consistent with a large body of network science literature on backbone detection. My concern is that the B2 computation described in the paper is methodologically insufficient for the claim being made. A partial-graph Brandes implementation with simplified predecessor tracking on a directed graph does not produce reliable betweenness centrality estimates. The central finding (that centrality-adjusted T1 outperforms aggregate-score T1) may be correct, but the evidence base for it is weaker than the paper claims.

## What Works

**The theoretical motivation for betweenness centrality is correct.** Betweenness centrality measures the fraction of shortest paths passing through a node or edge, which is exactly what "strategic national importance" means for a highway corridor — how many routes depend on it. The paper cites Brandes (2001) appropriately and the conceptual argument is sound.

**The natural break analysis** (Section 4.1): Jenks natural breaks is the correct algorithm for this type of clustering problem. The thresholds (21/15/9 in v1.1) are derived from the data rather than imposed. The approach is defensible.

**Aggregate-score T1 as a diagnostic for the paradox**: Showing that aggregate scoring produces I-110 and I-880 at the top is the paper's strongest concrete result, because it doesn't depend on B2 at all — it only requires the A1/A3 scores from HPMS, which are relatively reliable.

## What Doesn't Work

**The B2 scores are unreliable by the paper's own admission.** Section 3 and the axis-pool documentation both note that B2 is "estimated" on a "partial graph" and should not be used for inter-corridor comparison. But the paper's central claim — that centrality-adjusted classification outperforms aggregate-score classification — depends directly on B2 values being in the right rank order. If the partial-graph Brandes gives systematically incorrect rankings (e.g., due to the graph being disconnected in ways that create artificial high-centrality corridors), the tier assignments are wrong.

**The Brandes implementation has known simplifications.** The code comment in centrality.rs notes "TODO: implement full Brandes predecessor-based dependency for accuracy." The current implementation uses a uniform contribution heuristic rather than proper dependency accumulation. For a national highway graph with 12,000+ edges, this approximation may introduce substantial error. The paper presents the results as if full Brandes was run.

**Directed vs. undirected graph.** The paper uses a directed graph for centrality computation (correct in principle — freight flows have directionality). But the all-or-nothing assignment in the Frank-Wolfe algorithm treats the graph as if travel is symmetric. If a directed edge from A to B has very different flow characteristics than B to A, the centrality computed from the directed graph may not match the assignment model's assumptions. This inconsistency should be addressed.

**The α=0.65 estimation uses STRAHNET, then validates against STRAHNET.** The transport geographer has flagged this. From a network science perspective, the additional concern is that STRAHNET is a binary designation, not a continuous measure. Fitting a linear weight to maximize agreement with a binary outcome on a 227-sample dataset is low-statistical-power. The sensitivity range (α ≥ 0.55 gives the same result) suggests the finding is robust, but the estimation procedure is not rigorous.

## The Question I'd Push On

If the B2 scores are unreliable (partial graph, simplified Brandes), how do you know the 8 T1 corridors are correct rather than an artifact of the approximation? Specifically: can you show that the 8 corridors are also identifiable from the STRAHNET highway network topology alone, using only network structure (degree, betweenness) and no HPMS traffic data? If the 8 corridors emerge from pure topology — even before any traffic data is joined — that would be the strongest possible evidence that they are the true structural backbone, independent of any scoring system.
