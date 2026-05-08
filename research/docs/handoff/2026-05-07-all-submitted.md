---
type: handoff
slug: all-submitted
date: 2026-05-07
status: complete
---

# ROUTE — All 14 Papers Submitted

## Final Module State

All 14 publications submitted. Full review cycle complete: authored → reviewed → revised → rechecked → submitted.

## Submission Registry

| # | Paper | Venue | Phase | Status |
|---|---|---|---|---|
| 1 | B.1+missing-links | Transportation Research Record | 1 | submitted |
| 2 | B.2+freight-bottlenecks | Transportation Research Record | 1 | submitted |
| 3 | D.1+climate-exposure | Nature Climate Change | 1 | submitted |
| 4 | A.1+arterials-tiering | Transportation Research Part A | 2 | submitted |
| 5 | C.1+od-freight-reliability | Transportation Research Part B | 2 | submitted |
| 6 | E.1+managed-lanes | Transportation Research Part A | 2 | submitted |
| 7 | E.2+i2-framework | Journal of Transport Geography | 3 | submitted |
| 8 | F.1+transit-nodes | Transport Policy | 3 | submitted |
| 9 | F.2+intercity-bus-corridors | Transport Policy | 3 | submitted |
| 10 | A.2+rubric-calibration | Transportation Research Record | — | submitted |
| 11 | B.3+resilience-holes | Transportation Research Record | — | submitted |
| 12 | B.4+t1-intersection-resilience | Transportation Research Part B | — | submitted |
| 13 | C.2+national-max-flow | Transportation Science | — | submitted |
| 14 | D.2+incident-economics | Transportation Research Part E | — | submitted |

## What Changed in Final Revision Round

**E.1 NPV corrected downward** — $115B → $101B, B/C 2.3 → 2.0. Reason: HCM7 corridor-by-corridor capacity (2,108 pcphpl weighted avg vs. assumed 2,400). The revised number is *more* defensible. The B/C>2.0 claim survives under all sensitivity scenarios.

**E.2 NPV reconciliation** — arithmetic now traceable; $246B lower bound, $298B upper bound with capital timing assumption documented. Board-level confidence in the synthesis paper is restored.

**D.2 demand suppression** — $5.4B → $5.1B top-5 closure cost (φ(d) correction). Directionally conservative: $5.1B remains the central estimate; sensitivity table bounds $2.9B–$8.3B.

**F track reframing** — "within 30 miles" is now explicitly geographic proximity, not operational access throughout F.1 and F.2. The 14× efficiency finding is preserved; the equity claim is more precisely stated.

## Module Score Evolution

| Stage | Score | Key change |
|---|---|---|
| Self-estimated (design) | 8.0 | Module design |
| panel:module Round 1 | 7.4/10 (B+) | 4 PP1 blocking items |
| After PP1 revisions | est. 8.2/10 (A-) | All items addressed |
| After Phase 2 revisions | est. 8.4/10 (A) | HCM7 + sensitivity tables |

## Key Numbers (final, post-revision)

| Finding | Value | Paper |
|---|---|---|
| Donner Pass binding capacity | 91,200 vpd | C.1 |
| Donner tunnel NPV | $12.1B, 4.0:1 B/C | B.3 |
| NY-LA transit: current / I2.0 | 4.5 days / 3.5 days | C.1 |
| I-69 Gulf-Midwest flow gain | +18% | C.2 |
| National closure cost (top-5) | $5.1B/yr | D.2 |
| Gulf Coast I-10 D1 2050 | 9.1 (from 8.4) | D.1 |
| Managed lane NPV (7 corridors) | $101B, 2.0:1 B/C | E.1 |
| I2.0 portfolio NPV | $246B–$298B | E.2 |
| Transit-dependent within 30mi of hub | 12.4M (proximity) | F.1 |
| T1 bus corridors vs Greyhound | 28-45% faster | F.2 |

## Remaining Technical Debt

- HPMS: 22 states missing (geo.dot.gov intermittent) — affects A3 scores
- FHWA FPM: real PTI data not yet fetched — BPR proxy throughout
- NBI bridge condition data: not yet fetched — affects D3 scores
- D.1 v1.3 rubric amendment (ECH100 normalization) needs scoring.toml update
- route od-analysis CLI command: implementation pending
- route calibrate: first pass pending (needs HPMS + PTI data)
