# Revision Plan — A.2+rubric-calibration Round 1

Generated from SYNTHESIS.md — 2026-05-07

## P1 — Blocking (required before recheck)

- [ ] **P1.1** Add external validation paragraph to Section 6 — cite ATRI ρ=0.72 from B.2 or run leave-one-out test of 8 T1 corridors against ATRI bottleneck rank
- [ ] **P1.2** Add B2-conditioned caveat to Section 6.3 independence test table — make explicit that r values are conditional on partial 31-state B2 and require revalidation when full-graph B2 is available
- [ ] **P1.3** Add BPR-to-PTI evaluation paragraph to Section 3.2 — explain why IRI was used as fallback rather than BPR-estimated V/C-based PTI; cite HPMS coverage gap if that is the reason

## P2 — Important (address for quality)

- [ ] **P2.1** Add corpus construction description (1-2 paragraphs or citation) — 227-corridor definition criteria, corridor boundary decisions, spur inclusion, data sources
- [ ] **P2.2** Add bootstrap confidence intervals for 10th/90th percentile anchor values in Section 6.3 — 1,000-iteration bootstrap is computationally feasible at N=227
- [ ] **P2.3** Add B4 two-component documentation note — STRAHNET baseline (peacetime logistics) vs. installation-proximity bonus (strategic nuclear/STRATCOM); clarify that investment implications differ between the two components
- [ ] **P2.4** Resolve C4 hand-curation reproducibility gap — either add USDA NASS/ERS data appendix for anchor derivations, or add explicit "provisional pending v1.3" language that meets TRR reproducibility standards
- [ ] **P2.5** Add geographic coverage analysis for A4/B4/C4 — identify which regions cannot score above B4 baseline regardless of actual strategic function; address Southeast, Appalachia, and Pacific Northwest explicitly
