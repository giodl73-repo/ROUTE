---
reviewer: Lada Adamic
paper: B.4+t1-intersection-resilience
review_type: recheck
round: 1
date: 2026-05-07
pp_items_rechecked:
  - PP1.1: Manual validation of k-connectivity for top-5 intersections
verdict: PASS-WITH-NOTE
score: 3/4
---

> **Note:** AI-generated simulated recheck review.

## Items Rechecked

### PP1.1 — Manual validation of k-connectivity for top-5 intersections

**Original concern:** The k-connectivity results for the 9-of-15 SPF intersections rested entirely on TIGER/Line junction snapping, with two intersections noted as unresolved and no information on whether those two appeared in the top-five priority sites. For a paper whose core contribution is k-connectivity classification, unquantified snapping artifacts constitute a threat to the primary result, not a minor caveat. I required either manual validation of the top-five sites with documented methodology, or a snapping-tolerance sensitivity analysis across ±10m to ±100m.

**Revision:** Section 3.5 ("Manual Validation of Top-5 Intersections") is present in the revised manuscript. The authors validate the five highest-B2_product intersections — Atlanta I-75/I-85, Jacksonville I-10/I-95, Toledo I-75/I-90, Richmond I-95/I-85, and Sacramento I-5/I-80 — against aerial imagery (Google Maps satellite, 2024) and FHWA interchange inventory data. Table 4 reports graph-computed k versus verified k for all five sites; all five confirm. The Atlanta, Jacksonville, and Toledo k=1 classifications are each given a brief physical narrative: Atlanta's I-285 ring road correctly represented as a single merge corridor at k=1; Jacksonville's T-intersection confirmed with no parallel ramp; Toledo's partial cloverleaf confirmed with only two of four quadrants built. The five confirmed sites cover 63% of portfolio NPV ($8.8B of $14.0B). A single-misclassification sensitivity is stated: any one k=1 → k=2 reclassification changes portfolio NPV by less than 5%.

**Verdict:** I accept this. Aerial imagery cross-referenced against FHWA interchange inventory is not a formal proof of k-connectivity in the graph-theoretic sense — it is an informed engineering judgment that the graph correctly represents the physical interchange geometry. But for a transportation paper (not a graph theory proof), this level of validation is appropriate and sufficient. The physical narratives for Atlanta, Jacksonville, and Toledo are specific enough that a peer reviewer can independently assess the claim. The 63%/$8.8B NPV coverage figure means the portfolio ranking survives even if the unvalidated tail is partially misclassified. The authors have addressed my blocking concern.

One note remains: I would suggest as a P3 future-work item that a formal graph validation using OpenStreetMap as a secondary source be undertaken in the extended version of this analysis. OSM's junction tagging is typically more complete than TIGER/Line for complex urban interchanges (Atlanta and Boston especially), and an OSM crosscheck would establish whether the snapping artifacts in the two unresolved intersections (not in the top five) affect any k=1 designations in the lower-priority sites. This is not a condition of acceptance for the current paper, but it would strengthen the methodology section if the authors intend to extend the corpus beyond 15 intersections.

## Verdict

The revision directly and specifically addresses my blocking concern: manual validation is present for the five sites that matter most to the investment portfolio, coverage is stated quantitatively, and single-misclassification sensitivity is bounded. Score rises from 2/4 to 3/4. The remaining point reflects the undirected k-connectivity formulation (P2.7 from the original round, not addressed in this recheck pass) and the OSM crosscheck noted above; neither is a blocker for acceptance.
