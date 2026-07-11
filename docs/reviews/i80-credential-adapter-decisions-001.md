---
name: I-80 Credential And Adapter Decisions 001
slug: i80-credential-adapter-decisions-001
type: review
status: reviewed
rubric_version: v1.4
author: copilot
created: 2026-07-11
updated: 2026-07-11
sources:
  - data/i80-report-source-contract.csv
  - crates/route-data/src/census.rs
  - tools/prepare_i80_report_sources.py
  - https://www.ers.usda.gov/media/5768/2023-rural-urban-continuum-codes.csv?v=66892
  - https://developer.nlr.gov/docs/transportation/alt-fuel-stations-v1/
  - https://www.fhwa.dot.gov/bridge/nbi/ascii2025.cfm
  - https://static.nhtsa.gov/nhtsa/downloads/FARS/2022/National/FARS2022NationalCSV.zip
---

# I-80 Credential And Adapter Decisions 001

## Adopt Now

| Source | Decision | Boundary |
|---|---|---|
| ACS population | Environment-key support implemented | Reviewed report remains fixed at 2022 |
| ACS income | Environment-key support implemented | National comparison baseline remains fixed at 2022 |
| RUCC | Official 2023 CSV download and normalization implemented | 3,233 rows; output is `GEOID,RUCC,POP,DENSITY` |

## Exclude Pending Reviewed Adapter

| Source | Affected claim | Reason |
|---|---|---|
| AFDC DCFC | D2 charger density | API is credentialed and no fixture-tested normalization exists |
| NBI | D3 bridge count/condition | Raw national data exists, but route-summary normalization is not reviewed |
| FARS | A5 fatal crash rate | Bulk data exists, but route normalization and VMT denominator are not reviewed |
| FEMA | D1 SFHA exposure | Legacy endpoint and tile plan are not an I-80 coverage adapter |

Excluded means unavailable, not zero.

## Remove Unwired Citations

| Source | Decision |
|---|---|
| FAF5 | Remove unconditional report citation; current A2 is an HPMS proxy |
| BEA CAINC4 | Remove unconditional report citation; no BEA cache produced current C3 |

## Security Decision

- Credentials are read only from environment variables.
- Credentials are not accepted as CLI values.
- Request errors do not include credentialed URLs.
- No credential value is logged, persisted, or written to readiness output.

Current environment result: the all-source gate remains blocked only on the
two ACS artifacts because no `CENSUS_API_KEY` was supplied.
