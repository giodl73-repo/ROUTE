# Graph And Scoring Measurement Appendix

## Purpose

This appendix explains how ROUTE turns graph facts, score dimensions, coverage
outputs, flow checks, and investment allocation experiments into reviewable
questions.

The safe claim is that ROUTE has a measurement layer. It can score corridors,
surface confidence, compute graph metrics, expose coverage gaps, identify
capacity bottlenecks, and run rough allocation experiments. Those outputs are
decision-support artifacts. They are not official route decisions, funding
recommendations, service guarantees, ROI findings, eligibility findings, agency
compliance findings, stakeholder endorsements, or public-release evidence.

## Measurement Stack

| Layer | Current Surface | Reviewer Question |
|---|---|---|
| Graph build | `route build`, `route-network` graph types, cached graph summary | Which edges, routes, attributes, and joins are in the current graph? |
| Centrality | `route score-all`, `route_network::centrality::compute_edge_betweenness` | Which graph edges are structurally important under the current graph model? |
| Scoring | `route-score`, `docs/DIMENSIONS.md`, `data/scores-all.csv` | Which 16 dimensions contributed to a corridor score, and with what confidence? |
| Coverage | `route coverage`, `data/coverage-gaps.csv` | Which counties or grid cells exceed a selected distance threshold from the selected highway network? |
| Flow | `route flow <designation>` | Which corridor segment is the binding capacity bottleneck under the current capacity assumptions? |
| Allocation experiment | `route invest --budget <B>` | What would a simple budget-constrained optimizer select under rough cost and throughput assumptions? |
| Corpus/report output | `route report <designation>` | How are score and graph outputs converted into inspectable draft entries? |

## Scoring Instrument

ROUTE's score is a 16-dimension instrument grouped into four bands:

| Band | Dimensions | What It Helps Ask |
|---|---|---|
| Flow | A1 throughput gap, A2 freight intensity, A3 speed reliability, A4 international trade, A5 safety record | Is the corridor carrying or constraining important movement? |
| Network | B1 redundancy, B2 centrality, B3 port/border access, B4 military/strategic role | Does the corridor matter to the national graph or critical access? |
| People | C1 population reach, C2 rural connectivity, C3 economic opportunity access, C4 agricultural export access | Who is affected, and which access needs might volume-only metrics miss? |
| Future | D1 climate resilience, D2 multimodal integration, D3 infrastructure vintage | Which future-readiness or asset-debt questions remain visible? |

Each dimension carries a score, justification, source list, confidence value,
and estimated flag. The total score is useful only with those fields still
visible. A high total with weak source confidence is not a promoted conclusion;
it is a prompt for source, role, and sensitivity review.

## What Current Ledgers Show

| Ledger | Current Use | Boundary |
|---|---|---|
| `data/scores-all.csv` | Tracked score ledger with route, tier, rubric version, total score, confidence labels, all 16 dimension scores, and dimension confidence columns. | A score row is not a final priority, project, program, or funding claim. |
| `data/coverage-gaps.csv` | Gap ledger from coverage analysis, including county/geography fields, nearest distance, population, land area, gap class, and artifact reason. | A gap row does not prove a project, access promise, or equity outcome. |
| `docs/DIMENSIONS.md` | Public registry for the v1.4 dimensions and current truth labels. | A dimension definition does not prove its source posture is complete. |
| `docs/reports/corpus-report-generation-appendix.md` | Explains generated corpus/report boundaries. | Generated entries remain draft inspection surfaces. |

## Command Boundary

| Command | Safe Interpretation | Do Not Treat As |
|---|---|---|
| `route score <route>` | One corridor scored against the current rubric and available source joins. | A final corridor finding. |
| `route score-all` | National candidate scoring run with centrality and confidence fields. | A final national ranking. |
| `route coverage` | Distance-to-network analysis using county centroid mode when source data exists or grid proxy mode when it does not. | Proof that a specific access investment is required or sufficient. |
| `route flow <route>` | Bottleneck and capacity inspection under current lane/AADT assumptions. | Proof of managed-lane need, throughput benefit, or operating reliability. |
| `route invest --budget <B>` | Rough LP-style allocation experiment using current candidate pool and coarse cost/gain assumptions. | ROI, cost estimate, funding plan, or capital program. |
| `route report <route>` | Generated draft corpus entry for review. | Publication or source truth. |

## Evidence Strengths

The measurement layer strengthens the communications package because it lets a
reviewer challenge a claim at the right level:

- If the issue is source quality, inspect confidence, estimated flags, source
  labels, and source-roadmap blockers.
- If the issue is graph shape, inspect build inputs, bundle/member identity,
  centrality, coverage mode, and graph-contact evidence.
- If the issue is a proposed investment narrative, inspect the cost basis,
  throughput assumption, constraint ledger, and ROI/cost source pack before
  promoting any number.
- If the issue is rural, terminal, or local access, inspect coverage gaps,
  T3/T4 access appendices, and role review instead of relying on a score total.

## Reviewer Pressure Questions

| Pressure Question | Passing Answer |
|---|---|
| Which rubric version produced this score? | The score ledger or generated entry names the rubric version. |
| Which dimensions drove the score? | The artifact exposes all 16 dimensions and confidence columns, not just the total. |
| Which dimensions are weak, estimated, or proxy-based? | The estimated flag, confidence labels, and `docs/DIMENSIONS.md` truth labels remain visible. |
| Did centrality come from the current graph? | The score run or report points to the current graph build and centrality computation posture. |
| Was coverage county-based or grid-based? | The coverage output states the mode; grid mode remains a rough proxy. |
| Are flow outputs lane-backed or fallback-backed? | The flow output flags missing lane data and names the source command needed to improve it. |
| Does an allocation output include a real cost basis? | If not, it stays an experiment and is paired with the ROI/cost framework. |
| Can this score become a claim? | Only after source pack, role review, sensitivity review, and claim-promotion trace close for that exact claim. |

## Round-Specific Use

| Round | Useful Measurement Artifact | Pass Condition |
|---|---|---|
| Intra-state regional meeting | Coverage gaps, dimension rows, local source gaps | Use scores to ask local questions; do not present local commitments. |
| State meeting | Score confidence, asset and source gaps, graph-contact posture | Show what state evidence would change before any state-facing claim is promoted. |
| Multi-state regional meeting | Centrality, bundle identity, coverage and access ledgers | Keep cross-border claims tied to bundle/member identity and evidence state scope. |
| Congressional hearing | Dimension registry, score ledger posture, ROI/cost framework | Explain why ROUTE prevents premature national rankings and fake ROI. |
| DOT review | Command provenance, graph build, scoring config, flow/coverage assumptions | Let reviewers reproduce, challenge, downgrade, or hold outputs without changing the story. |

## Red Lines

Do not say:

- the score ledger is a final national priority list;
- a coverage gap proves a specific project;
- a flow bottleneck proves a managed-lane or widening obligation;
- an allocation experiment is a cost estimate, ROI result, or funding plan;
- a centrality result proves operational criticality without source and
  sensitivity review;
- a high score overrides `.roles` dissent, state feasibility, environmental
  review, asset condition, or source gaps.

## Safe Language

| Use This | Avoid This |
|---|---|
| "The score total is a review index with confidence and dimension detail." | "The score proves the corridor ranks above another for funding." |
| "Coverage output identifies access questions under a selected threshold." | "Coverage output proves an access project is required." |
| "Flow output identifies a bottleneck under current capacity assumptions." | "Flow output proves the benefit of a managed lane." |
| "Investment allocation is an experiment using rough assumptions." | "The allocation is a budget request or ROI case." |
| "Graph and score outputs feed claim review." | "Graph and score outputs are the claim." |

## Next Evidence Steps

1. Add a small scored-corridor example to the demo capture only when the command,
   config path, score ledger row, and confidence interpretation are recorded.
2. Pair any `route invest` example with the ROI/cost framework and a visible
   statement that costs and gains are rough assumptions until sourced.
3. Record coverage mode and source posture whenever `coverage-gaps.csv` appears
   in a review packet.
4. Add sensitivity notes before comparing corridors by score total in any
   sponsor, state, regional, congressional, or DOT review.
