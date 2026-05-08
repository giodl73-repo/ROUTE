---
reviewer: Lada Adamic
paper: C.2+national-max-flow
review_type: recheck
round: 1
date: 2026-05-07
pp_items_rechecked:
  - PP1.1: Single-commodity sensitivity analysis
verdict: PASS-WITH-NOTE
score: 3/4
---

> **Note:** AI-generated simulated recheck review.

## Items Rechecked

### PP1.1 — Single-commodity sensitivity analysis

**Original concern:** The single-commodity max-flow formulation treats all freight as fungible — it cannot distinguish between commodity classes with different route-choice behavior (time-sensitive manufactured goods, bulk freight, rail-eligible intermodal). In a multi-commodity freight network, the binding constraint for one commodity class may not be the binding arc in the aggregate single-commodity model. The paper's investment recommendations follow from the bottleneck identification; if the bottleneck identification is model-artifact rather than real-world constraint, the recommendations are not grounded in the analysis. I required either a modal substitution sensitivity (removing 15–25% of Donner demand to simulate rail diversion) or a two-commodity max-flow formulation for the Donner Pass corridor at minimum, to demonstrate that the binding bottleneck ranking is robust to the single-commodity assumption.

**Revision:** Section 3.6 ("Single-Commodity Sensitivity Analysis") implements a two-class sensitivity check using FAF5 SCTG commodity codes. The authors split demand into high-value freight (SCTG 7, 8, 9, 17, 30, 34, 38 — manufactured goods and chemicals, 62% of value / 28% of weight) and bulk freight (remaining SCTG codes, 38% / 72%), with differentiated V/C rerouting thresholds (0.85 and 1.05 respectively). The two-class sensitivity produces ranges for the three headline findings: Donner closure −23% (single-commodity) sits within −19% to −27% (two-class range); I-69 gain +18% sits within +14% to +21%; and I-40 compound saturation V/C 1.11 sits within the two-class range of 0.98–1.18, with network failure threshold (V/C > 1.0) crossed in the two-class central case. The I-69 NPV sensitivity table (Table 5) crosses three discount rates (5%, 7%, 10%) against three commodity scenarios (single-commodity, two-class central, two-class high), producing a 3×3 grid with explicit breakeven characterization.

**Verdict:** The two-class analysis is a bounding exercise, not a full multi-commodity network flow — but for this paper's claims it is sufficient. The Donner −23% being within the two-class range of −19% to −27% confirms the finding is directionally robust, not an artifact of the single-commodity assumption. The ±4 percentage point uncertainty range is modest relative to the magnitude of the finding; a 19% decline and a 27% decline are both policy-significant at any reasonable threshold. The I-40 compound saturation finding — which is the paper's most important resilience result — crosses the network failure threshold (V/C > 1.0) under the two-class central case, which means it is not a single-commodity artifact. The I-69 NPV sensitivity table correctly characterizes the investment as marginal at 7% discount rate and frames it as a policy judgment conditioned on demand growth and rate assumptions, not a max-flow conclusion.

One note remains as a P3 suggestion for future work: the two-class check is a bounding exercise, not a proper multi-commodity max-flow. A full multi-commodity formulation — in which commodity classes have separate source-sink demands and share edge capacity — is the methodologically correct successor. This is a meaningful research extension, not a minor replication; it would require implementing the multi-commodity network flow LP and re-running the full analysis. I note it as the natural next step in the research program, not as a condition of acceptance.

## Verdict

The revision directly addresses my blocking concern by providing a two-class commodity sensitivity that confirms all three headline findings are directionally robust within a ±4 percentage point range. The I-69 NPV table correctly conditions the investment recommendation on discount rate and commodity assumptions. Score rises from 2/4 to 3/4. The remaining point reflects the rail-arc absence (intermodal mode-shift is still not modeled, only approximated through the rerouting threshold) and the full multi-commodity extension noted above; neither is a blocker for acceptance at this stage.
