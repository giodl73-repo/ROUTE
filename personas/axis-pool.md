---
name: ROUTE Axis Pool v1.2
slug: axis-pool
type: spec
status: draft
rubric_version: v1.2
author: human
created: 2026-05-06
updated: 2026-05-07
sources: []
---

# ROUTE Axis Pool v1.2

The **15** candidate dimensions for scoring interstate corridors (12 original + 3 added in v1.2).

**v1.2 additions**: A4 (International Trade Corridor), B4 (Military/Strategic Designation), C4 (Agricultural Export Access). These address gaps identified when US-287 (central plains Mexico-Canada corridor) scored only 14.0/120 despite B1=10.0 — the rubric was blind to strategic value not reflected in current traffic.

**Score range**: 0–150 (was 0–120). Tier thresholds scaled proportionally:
- v1.2: T1≥26, T2≥19, T3≥11, T4<11 (≈same percentiles as v1.1 21/15/9/9)

**A3 fix (v1.2)**: IRI proxy replaced by BPR-estimated PTI when V/C ratio is computable from HPMS AADT + lane count. IRI fallback cap remains 5.0 but BPR-PTI path is now available.

**Status**: Candidate — all 15 dimensions live until the first calibration pass.

**Forward-only**: v1.1 scores locked at 120-point scale. v1.2 scores use 150-point scale.

---

## Live Dimensions

### Band A — Flow

| Dim | Name | Definition | Retirement risk |
|---|---|---|---|
| A1 | Throughput Gap | Current volume vs. designed capacity; congestion severity across route miles | Low — high variance expected |
| A2 | Freight Intensity | Average daily truck volume; commodity value density per corridor mile | Low — high variance expected |
| A3 | Speed Reliability | Travel time reliability. **v1.2: BPR-estimated PTI from V/C ratio when HPMS AADT+lanes available; IRI fallback capped 5.0.** PTI = 1 + 0.15×(V/C×1.15)^4 | Medium — BPR estimate better than IRI proxy; still estimated without real PTI data |
| **A4** | **International Trade Corridor** | USMCA trade corridor designation. Does this corridor serve primary US-Mexico or US-Canada freight flows? Scored from: FHWA border crossing AADT, USMCA corridor designations, O-D distance reduction vs. alternatives. | Low — highly variable; US-287 (B1=10, but missed without A4) is primary motivation |

### Band B — Network

| Dim | Name | Definition | Retirement risk |
|---|---|---|---|
| B1 | Redundancy | Count and quality of parallel interstate-quality alternatives within 50 miles | Low — high geographic variance |
| B2 | Network Centrality | Brandes betweenness centrality on national graph. **Marked estimated (†) on partial graph — only stable after `route score-all` on full 227-corridor network. Do not use B2 for inter-corridor comparison until score-all completes.** | Medium — partial-graph scores are misleading; requires full national graph |
| B3 | Port/Border Access | Connectivity to top-tier ports or major border crossings at termini | Low — binary-ish; will differentiate clearly |
| **B4** | **Military/Strategic Designation** | STRAHNET designation + proximity to major military installations. Captures strategic importance not reflected in commercial traffic metrics. STRAHNET = 5.0 baseline; nuclear missile command, major Army/Navy/Air Force bases within 30 miles = +2.0–5.0. | Low — STRAHNET designation is stable federal data |

### Band C — People

| Dim | Name | Definition | Retirement risk |
|---|---|---|---|
| C1 | Population Reach | Total population within 50 miles of corridor centerline | Low — high variance |
| C2 | Rural Connectivity | % of corridor through agricultural/rural land; rural communities with no close alternative | Low — high variance |
| C3 | Economic Opportunity Access | Buffer GDP per capita relative to national average. **Descriptive only — measures current economic conditions, not access-caused outcomes. Correlation with economic disadvantage is strong (r = 0.41 in B.1 corpus) but not causal; do not use C3 scores as evidence that corridors cause economic improvement.** | Low — high variance; BEA data reliable |
| **C4** | **Agricultural Export Access** | Does this corridor serve a major agricultural production zone with access to export infrastructure? Scored from: USDA county agricultural production value in buffer, proximity to grain export terminals (Houston, Portland OR, New Orleans, Baltimore, Long Beach). Captures why US-287 matters even with low population: Great Plains wheat/beef export route to Gulf ports AND Pacific Northwest ports. | Low — USDA county agricultural data available; high variance across corridors |

### Band D — Future

| Dim | Name | Definition | Retirement risk |
|---|---|---|---|
| D1 | Climate Resilience | Composite exposure score: flood zone miles, wildfire risk miles, extreme heat days per year | Low — high geographic variance |
| D2 | Multimodal Integration | Adjacent freight rail, intermodal hub count, transit connection potential | Low — highly variable by region |
| D3 | Infrastructure Vintage | Weighted average construction decade; % bridges in poor condition; deferred maintenance backlog | Low — high variance; well-documented data |

---

## Rubric Ledger

Tracks how each dimension performs across the scored corpus. Updated after every corridor scoring.

| Dim | Name | Corridors scored | Mean | IQR | Min | Max | Correlated with | Amendment status |
|---|---|---|---|---|---|---|---|---|
| A1 | Throughput Gap | 0 | — | — | — | — | — | candidate |
| A2 | Freight Intensity | 0 | — | — | — | — | — | candidate |
| A3 | Speed Reliability | 0 | — | — | — | A1? | IRI fallback capped 5.0 (v1.1) | candidate |
| B1 | Redundancy | 0 | — | — | — | — | — | candidate |
| B2 | Network Centrality | 0 | — | — | — | — | B1? | partial-graph warning explicit (v1.1) | candidate |
| B3 | Port/Border Access | 0 | — | — | — | — | — | candidate |
| C1 | Population Reach | 0 | — | — | — | — | — | candidate |
| C2 | Rural Connectivity | 0 | — | — | — | — | C3? | candidate |
| C3 | Economic Opportunity Access | 0 | — | — | — | — | C2? | renamed + descriptive-only note (v1.1) | candidate |
| D1 | Climate Resilience | 0 | — | — | — | — | — | candidate |
| D2 | Multimodal Integration | 0 | — | — | — | — | — | candidate |
| D3 | Infrastructure Vintage | 0 | — | — | — | — | — | candidate |

**Amendment status**: `candidate` = live but not yet validated by corpus variance · `validated` = confirmed informative after calibration pass · `retired` = retired after calibration pass

---

## Retired Dimensions

None yet.

---

## Changelog

| Date | Change | Rubric version | Triggered by |
|---|---|---|---|
| 2026-05-06 | Initial 12-dimension pool established | v1.0 | Initial design |
| 2026-05-07 | **A3**: IRI fallback capped at 5.0; **B2**: partial-graph warning explicit; **C3**: renamed + descriptive-only note | v1.1 | score-all run + B.1 peer review |
| 2026-05-07 | **A4** International Trade Corridor: USMCA freight flow potential | v1.2 | US-287 scored 14.0 despite B1=10.0 — rubric blind to strategic value |
| 2026-05-07 | **A3** upgraded: BPR-estimated PTI from V/C ratio when HPMS data available | v1.2 | User request: fix speed data |
| 2026-05-07 | **B4** Military/Strategic Designation: STRAHNET + military base proximity | v1.2 | User request: military access dimension |
| 2026-05-07 | **C4** Agricultural Export Access: grain belt + export terminal proximity | v1.2 | User request: agricultural dimension; US-287 central plains corridor |
| 2026-05-07 | Score range: 12 dims × 10 = 120 → 15 dims × 10 = 150; tier thresholds scaled | v1.2 | 3 new dimensions added |

---

## Calibration Pass Protocol

Run after 20+ existing corridors are scored.

1. Compute variance stats for each dimension (mean, IQR, min, max).
2. Flag for retirement: IQR < 2.0 (not differentiating) OR correlation r > 0.85 with another dimension (redundant).
3. Propose retirement or redefinition. Write rationale here.
4. Bump rubric version. Lock prior scores.
5. Determine Parliament behavior: retired dimensions drop from draft pool; promoted dimensions (if any new ones identified from argument records) enter as candidates.

---

## Preferred Axes by Parliament Voice

Each parliament voice has 3–4 preferred dimensions they're most likely to argue from. When two voices share a preferred dimension, contention is designed.

| Voice | Primary | Secondary | Tertiary | Quaternary |
|---|---|---|---|---|
| Eisenhower | B2 Network Centrality | B1 Redundancy | B3 Port/Border | D1 Climate Resilience |
| Moses | A1 Throughput Gap | A2 Freight Intensity | D3 Infrastructure Vintage | — |
| Foxx | C3 Economic Opportunity | C1 Population Reach | C2 Rural Connectivity | — |
| Freight Economist | A2 Freight Intensity | B3 Port/Border | A1 Throughput Gap | B2 Network Centrality |
| Traffic Engineer | A1 Throughput Gap | A3 Speed Reliability | D3 Infrastructure Vintage | B1 Redundancy |
| Climate Engineer | D1 Climate Resilience | D2 Multimodal Integration | B1 Redundancy | — |
| Rural Advocate | C2 Rural Connectivity | C3 Economic Opportunity | B1 Redundancy | C1 Population Reach |

Designed contentions:
- **A1 (Throughput Gap)**: Moses, Freight Economist, Traffic Engineer all want it → high-conflict dimension
- **B1 (Redundancy)**: Eisenhower, Climate Engineer, Rural Advocate, Traffic Engineer → every band represented
- **C3 (Equity Access)**: Foxx, Rural Advocate → direct collision likely
