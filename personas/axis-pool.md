---
name: ROUTE Axis Pool v1.0
slug: axis-pool
type: spec
status: draft
rubric_version: v1.0
author: human
created: 2026-05-06
updated: 2026-05-06
sources: []
---

# ROUTE Axis Pool v1.0

The 12 candidate dimensions for scoring interstate corridors. This is the scoring instrument plus the ledger that tracks how each dimension performs across the corpus.

**Status**: Candidate — all 12 are live until the first calibration pass (after 20+ existing corridors scored). The calibration pass may retire correlated or low-variance dimensions and bump the rubric version. Prior scores are locked at the version they were scored under.

---

## Live Dimensions

### Band A — Flow

| Dim | Name | Definition | Retirement risk |
|---|---|---|---|
| A1 | Throughput Gap | Current volume vs. designed capacity; congestion severity across route miles | Low — high variance expected |
| A2 | Freight Intensity | Average daily truck volume; commodity value density per corridor mile | Low — high variance expected |
| A3 | Speed Reliability | Average speed vs. design speed; day-to-day variance | Medium — may correlate strongly with A1 |

### Band B — Network

| Dim | Name | Definition | Retirement risk |
|---|---|---|---|
| B1 | Redundancy | Count and quality of parallel interstate-quality alternatives within 50 miles | Low — high geographic variance |
| B2 | Network Centrality | Betweenness centrality proxy: how many routes depend on this corridor; cascade effect if closed | Medium — hard to operationalize; may collapse into B1 |
| B3 | Port/Border Access | Connectivity to top-tier ports or major border crossings at termini | Low — binary-ish; will differentiate clearly |

### Band C — People

| Dim | Name | Definition | Retirement risk |
|---|---|---|---|
| C1 | Population Reach | Total population within 50 miles of corridor centerline | Low — high variance |
| C2 | Rural Connectivity | % of corridor through agricultural/rural land; rural communities with no close alternative | Low — high variance |
| C3 | Equity Access | % of 50-mile buffer population in low-income or tribal census tracts | Medium — may correlate with C2 |

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
| A3 | Speed Reliability | 0 | — | — | — | A1? | — | candidate |
| B1 | Redundancy | 0 | — | — | — | — | — | candidate |
| B2 | Network Centrality | 0 | — | — | — | — | B1? | candidate |
| B3 | Port/Border Access | 0 | — | — | — | — | — | candidate |
| C1 | Population Reach | 0 | — | — | — | — | — | candidate |
| C2 | Rural Connectivity | 0 | — | — | — | — | C3? | candidate |
| C3 | Equity Access | 0 | — | — | — | — | C2? | candidate |
| D1 | Climate Resilience | 0 | — | — | — | — | — | candidate |
| D2 | Multimodal Integration | 0 | — | — | — | — | — | candidate |
| D3 | Infrastructure Vintage | 0 | — | — | — | — | — | candidate |

**Amendment status**: `candidate` = live but not yet validated by corpus variance · `validated` = confirmed informative after calibration pass · `retired` = retired after calibration pass

---

## Retired Dimensions

None yet.

---

## Changelog

| Date | Change | Rubric version |
|---|---|---|
| 2026-05-06 | Initial 12-dimension pool established | v1.0 |

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
| Foxx | C3 Equity Access | C1 Population Reach | C2 Rural Connectivity | — |
| Freight Economist | A2 Freight Intensity | B3 Port/Border | A1 Throughput Gap | B2 Network Centrality |
| Traffic Engineer | A1 Throughput Gap | A3 Speed Reliability | D3 Infrastructure Vintage | B1 Redundancy |
| Climate Engineer | D1 Climate Resilience | D2 Multimodal Integration | B1 Redundancy | — |
| Rural Advocate | C2 Rural Connectivity | C3 Equity Access | B1 Redundancy | C1 Population Reach |

Designed contentions:
- **A1 (Throughput Gap)**: Moses, Freight Economist, Traffic Engineer all want it → high-conflict dimension
- **B1 (Redundancy)**: Eisenhower, Climate Engineer, Rural Advocate, Traffic Engineer → every band represented
- **C3 (Equity Access)**: Foxx, Rural Advocate → direct collision likely
