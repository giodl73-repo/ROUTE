---
name: I-80 Clean-Clone Source Reproducibility Research
slug: i80-clean-clone-source-reproducibility
type: report
status: reviewed
rubric_version: v1.4
author: copilot
created: 2026-07-11
updated: 2026-07-11
sources:
  - data/manifest.json
  - data/sources.md
  - data/source-fetch-policy.csv
  - data/fletch-registry.json
  - crates/route-cli/src/main.rs
  - crates/route-data/src/census.rs
  - crates/route-data/src/hpms_fetch.rs
  - crates/route-data/src/fema.rs
  - https://www2.census.gov/geo/tiger/TIGER2025/PRIMARYROADS/
  - https://www2.census.gov/geo/docs/maps-data/data/gazetteer/2025_Gazetteer/
  - https://api.census.gov/data/missing_key.html
  - https://www.ers.usda.gov/data-products/rural-urban-continuum-codes
  - https://data.transportation.gov/Roadways-and-Bridges/Highway-Performance-Monitoring-System-HPMS-/jc5k-rzm8
  - https://developer.nlr.gov/docs/transportation/alt-fuel-stations-v1/
  - https://hazards.fema.gov/gis/nfhl/rest/services/public/NFHL/MapServer
  - https://www.fhwa.dot.gov/bridge/nbi/ascii2025.cfm
  - https://crashviewer.nhtsa.dot.gov/CrashAPI
---

# I-80 Clean-Clone Source Reproducibility Research

## Research Question

Which I-80 report inputs can ROUTE acquire and normalize from a clean clone
today, and which require credentials, adapters, manual source handling, or a
different source contract?

## Decision Supported

This research defines the implementation order for the
`i80-clean-clone-source-reproducibility` wave. The next pulse should orchestrate
only the no-credential paths whose parsers already exist, while credential and
adapter work remains explicit.

## Findings

### ROUTE-SRC-01 - Manifest fetch covers only two required report families

**Sources:** `data/manifest.json`; `route fetch`.

**Observed constraint:** TIGER primary roads and the Census county Gazetteer
have live manifest URLs. Most other reviewed-report inputs have blank manifest
URLs or are not represented.

**Implication:** `route fetch` cannot be presented as the clean-clone report
preparation command.

**Confidence:** High.

### ROUTE-SRC-02 - Gazetteer download and report readiness are different states

**Sources:** `data/manifest.json`; `load_acs_counties_for_scoring` in
`crates/route-cli/src/main.rs`.

**Observed constraint:** The manifest downloads a Gazetteer ZIP, while the
report loader searches for an extracted `*counties_national.txt` file.

**Implication:** The orchestrator must extract and validate the text file before
marking the source ready.

**Confidence:** High.

### ROUTE-SRC-03 - Current Census API access requires a key

**Sources:** https://api.census.gov/data/missing_key.html;
`route fetch-acs`; `route fetch-acs-income`.

**Observed constraint:** A direct 2024 ACS county request redirected to the
official Census missing-key page. The local commands do not expose an API key
or configurable year.

**Implication:** ACS population and income are credential-blocked until ROUTE
supports an environment-provided key. Credentials must never be committed.

**Confidence:** High for the tested current endpoint.

### ROUTE-SRC-04 - HPMS acquisition is automated but not current-national

**Sources:** `crates/route-data/src/hpms_fetch.rs`; `route fetch-hpms`; FHWA
HPMS portal:
https://data.transportation.gov/Roadways-and-Bridges/Highway-Performance-Monitoring-System-HPMS-/jc5k-rzm8.

**Observed constraint:** ROUTE fetches state-hosted 2018 ArcGIS services into
`hpms_2018.csv`. The command is usable and supports scoped state merges, but the
artifact name must not be interpreted as current national HPMS.

**Implication:** Use the existing command for reproducibility while keeping the
2018 vintage explicit. Research a current endpoint separately.

**Confidence:** High.

### ROUTE-SRC-05 - RUCC has a join but no acquisition/conversion path

**Sources:** `data/manifest.json`; `join_rucc` in
`crates/route-data/src/census.rs`; USDA RUCC source page:
https://www.ers.usda.gov/data-products/rural-urban-continuum-codes.

**Observed constraint:** The manifest names a 2023 workbook, while the loader
expects `rucc_2013.csv`. No command reconciles the vintage or converts the
workbook.

**Implication:** RUCC requires a version decision before automation.

**Confidence:** High.

### ROUTE-SRC-06 - AFDC has a current API but no ROUTE adapter

**Sources:** NLR Alternative Fuel Stations API:
https://developer.nlr.gov/docs/transportation/alt-fuel-stations-v1/;
`load_dcfc_stations`.

**Observed constraint:** ROUTE can read `dcfc_stations.csv` but cannot fetch or
normalize it. The API is credentialed and moved from the retired NREL developer
domain to `developer.nlr.gov`.

**Implication:** Add an environment-key adapter behind a source boundary.

**Confidence:** High for API existence and local adapter absence.

### ROUTE-SRC-07 - FEMA is closest to a complete no-credential adapter

**Sources:** `route fetch-fema-d1`; `crates/route-data/src/fema.rs`; FEMA NFHL
service:
https://hazards.fema.gov/gis/nfhl/rest/services/public/NFHL/MapServer.

**Observed constraint:** Fetch and CSV builder logic exist, but endpoint health,
layer assumptions, and I-80 coverage are not recorded in a readiness artifact.

**Implication:** Add health and nonempty-coverage gates rather than a new
fetcher.

**Confidence:** Medium-high.

### ROUTE-SRC-08 - NBI is publicly downloadable but lacks a raw-to-summary adapter

**Sources:** FHWA 2025 NBI ASCII page:
https://www.fhwa.dot.gov/bridge/nbi/ascii2025.cfm; `load_nbi_bridges`.

**Observed constraint:** ROUTE loads a route-level summary CSV, while FHWA
publishes raw delimited bridge files. No command downloads, parses, normalizes,
and summarizes those rows.

**Implication:** NBI is adapter-missing, not credential-blocked.

**Confidence:** High.

### ROUTE-SRC-09 - FARS source selection remains unresolved

**Sources:** NHTSA CrashAPI:
https://crashviewer.nhtsa.dot.gov/CrashAPI; `load_fars_safety`.

**Observed constraint:** ROUTE only loads a prepared route summary. No fetch or
route-normalization command exists, and the official API returned access
restrictions in this environment.

**Implication:** Choose between CrashAPI and annual national downloads before
implementation.

**Confidence:** Medium.

### ROUTE-SRC-10 - Source years are part of the claim contract

**Sources:** `data/manifest.json`; `data/sources.md`;
`corpus/existing/i80.md`.

**Observed constraint:** The reviewed report combines TIGER/Gazetteer 2023,
ACS 2022, HPMS 2018 acquisition code, NBI 2023/2024 references, and FARS 2022.

**Implication:** The orchestrator must emit exact source years and must not
rename old data as current.

**Confidence:** High.

### ROUTE-SRC-11 - FAF5 is cited but not wired into the current I-80 result

**Sources:** `corpus/existing/i80.md`; `crates/route-data/src/faf5.rs`;
`join_a2_freight_proxy` in `crates/route-cli/src/main.rs`.

**Observed constraint:** The report frontmatter cites FAF5 v5.6, but the current
I-80 A2 value is an HPMS cargo-value proxy. The FAF5 parser exists while its
zone table remains incomplete.

**Implication:** FAF5 must have a blocker row. Either remove the unconditional
report citation or implement a real source-backed join.

**Confidence:** High.

### ROUTE-SRC-12 - BEA is cited without a clean-clone join

**Sources:** `corpus/existing/i80.md`; `data/manifest.json`;
`crates/route-score/src/score.rs`.

**Observed constraint:** The report cites BEA CAINC4 2022, but the manifest URL
is blank and no clean-clone BEA acquisition and county join path exists.

**Implication:** BEA must remain blocked until a fetch/join exists, or the
unconditional citation must be removed from generated reports.

**Confidence:** High.

## Recommendations

### Adopt Now

| Action | Owner | Validation |
|---|---|---|
| Add a source-contract reader and orchestrator for TIGER, Gazetteer, HPMS, and FEMA | ROUTE | Download, parse, nonempty coverage, and year gates |
| Add deterministic Gazetteer extraction | `route-data` / `route-cli` | Clean-cache extraction test |
| Emit machine-readable blocker rows for unavailable sources | ROUTE | Every required artifact is ready or blocked with next action |
| Remove unconditional FAF5/BEA citations when those sources did not produce the score | `route-report` | Generated source list matches actual dimension provenance |

### Prototype Behind A Compatibility Boundary

| Action | Owner | Validation |
|---|---|---|
| Census API key and year support | `route-data` | Environment-only key, fixture parse, no secret logging |
| AFDC key-backed DCFC adapter | `route-data` | API fixture and normalized CSV gate |
| NBI raw-to-route-summary adapter | `route-data` | Known bridge fixture and I-80 coverage report |

### Reject Or Defer

| Action | Reason |
|---|---|
| Rename HPMS 2018 data as current HPMS | Misrepresents source vintage |
| Auto-upgrade TIGER/Gazetteer to 2025 without parser tests | Current pipeline is proven only against 2023 artifacts |
| Treat download success as reviewed-report readiness | Parse and coverage may still fail |
| Commit credentials or raw national datasets | Violates repository source and security policy |

## Non-Goals

- Regenerate the I-80 report during the inventory pulse.
- Select a new transportation treatment.
- Convert missing source evidence into default values.
