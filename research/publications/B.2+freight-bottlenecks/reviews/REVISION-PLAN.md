# Revision Plan — B.2+freight-bottlenecks Round 1

Generated from SYNTHESIS.md — 2026-05-07

## P1 — Blocking (required before recheck)

- [ ] **P1.1** Add cost-reduction counterfactual paragraph to Section 8 — cite managed lane freight delay elasticity literature (Small et al. 2006; NCHRP Report 722 or equivalent); bound expected cost reduction under managed lane scenario for I-95 Northeast, I-75 Atlanta, and Donner Pass
- [ ] **P1.2** Cascade multiplier: rename to "T1 cost premium" / "tier cost ratio" OR add minimal cascade propagation model — if renaming, update abstract, Section 5, and all in-text references; if formalizing, add stylized network example or cite Berdica 2002 / Jenelius 2010 on network disruption propagation

## P2 — Important (address for quality)

- [ ] **P2.1** Add PTI ≤ 1.15 standard citation or derivation in Section 8.3 — cite E.2 framework paper, or derive from FHWA FPM thresholds / HCM LOS standards
- [ ] **P2.2** Show Donner Pass cost calculation explicitly in Section 7 — expand to: closures/yr × hours × truck volume × $225/hr = $1.6B/yr; source $225/hr rerouting premium differential; cite Caltrans incident database for closure frequency and mean duration
- [ ] **P2.3** Verify value-at-risk cascade mechanism against FAF5 data, or remove — if verified: add FAF5 freight value per truck-mile comparison (T1 vs. T2); if removed: note that volume and rerouting cost mechanisms are sufficient to explain 1.73×
- [ ] **P2.4** Add spatial autocorrelation note to Section 4.2 — report cluster-robust standard errors for Spearman ρ = 0.67 grouped by geographic region, or add Moran's I test on ATRI density residuals
- [ ] **P2.5** Add I-285 betweenness centrality in T1 subgraph (Atlanta region) as supplementary evidence for T1 reclassification argument — use route CLI diamond analysis infrastructure; report local betweenness vs. national betweenness to illustrate the local/national centrality distinction
