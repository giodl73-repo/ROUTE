---
reviewer: adamic
persona: Lada Adamic — network scientist, Meta Research / University of Michigan
round: 1
date: 2026-05-07
score: 3/4
---

> **Note:** This is an AI-generated simulated review, written by Claude in the voice of a named-expert persona. It is not an actual review by the named person and does not reflect their views or endorsement.

## Overall

This paper makes a credible methodological contribution by documenting an inductive calibration process with honest self-diagnosis of its own errors — the congestion-stress paradox is real and the fix is directionally correct. What holds the score at 3 rather than 4 is the B2 section: the authors flag the partial-graph instability but defer it entirely, leaving the reader to accept correlation scores (A4 r=0.22, B4 r=0.18, C4 r=0.31) that were computed against a B2 that is known to be wrong.

## What Works

**Honest anomaly documentation.** The dimension-by-dimension I-110 vs. I-80 table is exactly what a calibration paper should produce. The authors do not paper over the v1.0 paradox — they show the raw scores and explain mechanistically why IRI proxied the wrong thing. This is the kind of transparency that lets subsequent work build on the rubric without inheriting its failure modes silently.

**Inductive calibration framing.** Framing the rubric as a hypothesis tested by the corpus is methodologically sound and unusual for transportation infrastructure scoring. Most rubric papers derive dimensions from theory and assert validity; this paper allows the corpus to falsify dimensions. The three statistical tests (variance, correlation, independence) are the right battery, even if the implementation has gaps.

**Forward-only protocol rationale.** The three-criterion justification for forward-only versioning (citation stability, calibration analysis validity, score attribution) is logically tight. The protocol correctly distinguishes between corridor facts and rubric measurement — a corridor does not change when the rubric changes; the measurement changes. This is the right epistemic posture.

**Independence correlation table.** Including explicit Pearson r values for A4/B4/C4 against existing dimensions is exactly right. The finding that C4 (Agricultural Export) correlates only r=0.31 with C3 (Economic Opportunity) is nontrivial — one might expect a stronger overlap — and the explanation (agricultural counties are not high-GDP counties) is well-motivated.

## What Doesn't Work

**B2 partial-graph problem is load-bearing and deferred.** The independence tests for A4, B4, C4 compare against the existing dimension set — including B2. But B2 is flagged as unreliable, computed on a 31-state partial graph. This means the correlation values in Section 6.3 are correlating new dimensions against a noisy estimate of B2. If B2 moves substantially when the full 50-state graph is computed, the correlation structure could change. The paper cannot claim A4, B4, C4 are independent of B2 when B2 is acknowledged to be measurement error. The simplest fix: report the B2 correlations with an explicit caveat and exclude B2 from the independence test battery until the full-graph computation is available. What the paper cannot do is leave this as a limitation footnote while citing the independence test as validation evidence.

**Anchor calibration is not validated against held-out data.** The three-point linear interpolation (10th/90th percentile anchors) is described procedurally but not evaluated empirically. With 227 corridors, a leave-one-out or 5-fold cross-validation of the tier classification against independent ground truth (e.g., ATRI bottleneck cost density, or FHWA investment priority rankings) would show whether the anchor choices are stable. Without this, the reader cannot know whether the 10th/90th percentile anchor is the right choice versus, say, 5th/95th or mean ± 1.5σ.

**C4 operationalization relies on hand-curation.** The paper states explicitly that C4 (Agricultural Export Access) uses USDA county-level data "hand-curated in v1.2; data-driven USDA ERS integration planned for v1.3." Hand-curation is reproducibility-opaque: a different analyst replicating this paper would produce different C4 anchor scores without access to the curation log. For a paper claiming TRR-level methodology, this needs at minimum a data appendix showing the curation decisions, or the C4 analysis should be restricted to the anchors with documented USDA sources.

## The Question I'd Push On

The partial-graph centrality problem is deferred to v1.3, but the paper's central claim — that the three new dimensions (A4, B4, C4) are independent of existing dimensions — relies on those independence correlations. If B2 is currently understated for many corridors (lower bound, as the paper acknowledges), what is the expected direction of B2 movement when the full graph is computed, and would that movement change the B4-B2 correlation from r=0.18 to something that fails the independence threshold? This needs either a sensitivity analysis or an explicit acknowledgment that the independence claims are conditional on the partial-graph B2 remaining stable.
