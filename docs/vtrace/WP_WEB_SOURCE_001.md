---
name: I-80 recorded source checklist
slug: i80-recorded-source-checklist
type: plan
status: reviewed
rubric_version: v1.4
author: Codex
created: 2026-10-29
updated: 2026-10-29
sources:
  - data/i80-report-source-contract.csv
  - tools/prepare_i80_report_sources.py
  - docs/packets/i80-flagship-review-packet.md
---
# WP-WEB-SOURCE-001: I-80 recorded source checklist

Status: accepted for implementation. Parents: REQ-001/002/003/008/009/010/011,
SPEC-001/007/013 reproducibility/evidence surfaces, CR-001..006, OPS-001/004/005.
This is source-metadata inspection, not a corridor score or design promotion.

## Contract and source grain

Use the unchanged source contract and prepare_i80_report_sources.readiness_rows
with attempts={} against the existing canonical local cache. Capture once,
without acquisition, credentials or cache writes, as a dated immutable JSON
observation. Preserve all original contract/observation fields, script/contract/
packet canonical Git hashes, capture time and origin commit. No raw cached
publisher files, key values, full machine paths or network-attempt diagnostics
enter the snapshot. A capture tool runs only when explicitly invoked; Pages
builds replay the committed snapshot, never inspect CI/browser caches or fetch.

Expose a pure Rust typed adapter in route-web. Query optional status null/all,
ready, blocked or excluded; reject unknown states/fields and >64 KiB. Validate
the frozen schema, IDs, duplicated contract/observation identity, fixed source
partition and recorded provenance/false acquisition flags. Count source entries
only; never sum heterogeneous ZIP members, CSV records and lines or reinterpret
them as traffic, population, coverage quality or primary-byte verification.
Status selection filters the recorded observation; it is not current readiness.
Do not port or silently change the Python acquisition/readiness policy.

Keep contract years, candidate years, acquisition/parser status, contract gap,
observed blocker, next action, evidence count and its original detail separate.
A ready cache observation can retain unresolved schema/vintage work. Candidate
years and URLs are recorded contract fields, not claims of today's latest source
or endpoint health. Cached counts are not proof that packet quantities reproduce.

Explicit unresolved conflict: the reviewed packet labels traffic HPMS 2023;
the source contract and fetcher identify 2018. Preserve both original artifacts
and values, publish the discrepancy and keep packet-value provenance unobserved.
Do not relabel/recompute corpus/packet traffic or scores in this task. Retain
Hold and narrow: no approved design, positive ROI, guaranteed SLA, agency
endorsement, fiscal saving or national traffic provenance follows. Source
readiness does not populate the invented freight scenarios.

Publish exact committed LF bytes for snapshot, original contract and historical
reviewed packet, with SHA custody/source note. Raw source artifacts are not in
the bundle and no primary hash, fresh endpoint, current law or credential is
verified. The source contract's metadata/hash is separate from original bytes.
Keep the existing evaluate/run functions and four captured synthetic canonical
results unchanged. Use an independent worker/UI/export and visible header link;
errors clear stale data; reset, downloads, keyboard/mobile and transportation
roundtrip persist. Seven authorities false; current AADT, intervention effect
and matched packet-traffic vintage remain null. Artifact <5 MB.

## Scope, assurance and validation

ROUTE owns this new wave/pulse; all prior waves and I-80 dispositions stay
closed/held. TAXLANE changes transportation evidence wording/assertions only.
No package/dependency/pin or runtime product coupling, no acquisition/parser/
scoring/kernel change. TRACKER snapshots after child gates/deployment and exact
live custody. Other repos, ICELINES, BISECT and owner state are no-op.

CR-006 assurance approval for this accepted package: existing wasm-bindgen FFI
with bounded JSON only; native CLI replay reads a named query file; Python
capture reads existing local cache without execution/network; build subprocesses
are fixed cargo/git commands and write declared .cache/dist metadata. No unsafe
Rust, secret collection, user-provided command execution or new fetch. Security/
privacy review passes those boundaries. Canonical ignored Cargo overrides remain
untouched through a clean child worktree. No TRACKER-dependent Rust paths.

L0: pinned fmt/check for route-web/kernel/sim and packet --check. L1: locked
selected-package tests, native/WASM strict Clippy, frozen-capture/contract
validation and four legacy canonical regressions. Full ROUTE acquisition/report/
map machinery is unchanged and outside this adapter validation. L2: four query
WASM/native canonical comparisons, original captured-row parity, source hashes,
existing five/new browser cases, invalid/stale/reset/error, keyboard/mobile and
transportation navigation; hosted gates and clean exact default/live artifacts.
CR-001: loader/validation/query/projection stay separate and bounded; JSON
formatting may be verbose but does not justify merged policy/parse logic.

## Role fixed point before code

Implementing-agent lenses, not participation/endorsement by named archetypes:

- Eisenhower: pass; no national-system benefit from local source presence. Defer
  network outcomes to a bounded validated corridor intervention.
- Moses: pass; no buildability or throughput promise. Defer delivery evidence.
- Foxx: pass; absent ACS is not zero need or distributional proof. Defer equity/
  incidence evidence to matched demographic and intervention work.
- Freight Economist: resolved; counts have heterogeneous units, no traffic/NPV/
  commodity-flow inference. Defer economics to sourced intervention validation.
- Traffic Engineer: resolved; flag 2023/2018 conflict without changing source
  values; readiness is not AADT or operational evidence. Defer provenance repair.
- Climate Engineer: pass; FEMA stays excluded, no hazard or resilience conclusion.
  Defer a reviewed replacement/coverage source.
- Rural Advocate: pass; RUCC cache context is not delivered rural access. Defer
  service/household and agricultural access evidence.
- Optimization Methodologist: pass; frozen capture/partition and strict queries,
  no candidate ranking or implicit optimizer objective. Defer selector changes.
- Schematic Cartographer: pass; no new maps, contacts, transfer or service claims.
- Citation Auditor: resolved; exact metadata custody, dated local observation,
  absent raw bytes and unresolved packet-vintage discrepancy explicit.
- Scope Keeper: resolved; checklist only; original corpus/packet and simulations
  independent. No source or design promotion.
- Numeracy Checker: resolved; source-entry totals only, counts not added across
  unlike units, missing years not zero, packet quantities unobserved.

State DOT, freight operations, rural/agricultural, transit-dependent and
environmental/community-health lenses retain their distinct delivery, access,
rights and outcome unknowns with these deferrals. No unresolved blocking
actionable finding remains. Repeat all twelve lenses at exit. Rollback: revert
new adapter/UI/evidence wording and redeploy prior artifacts; retain records.

## Cross-target regression boundary

Four recorded Windows-native results remain byte-identical locally. Existing
native/WASM logarithm/exponential calculations can differ in their last bit;
no math or dependency is changed to suppress that difference. Build validation
compares all structure/non-numeric values exactly and numeric values with both
absolute/relative tolerance 1e-12 across targets. Four pre-change published WASM
results captured at a055dbfd3f8dfd1a5c020a67cb56c8fda652063c are compared byte-for-byte
to the new WASM. The four new checklist native/WASM results have exact canonical
string parity. Do not describe the legacy cross-target check as byte parity.
