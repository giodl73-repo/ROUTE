---
name: HPMS historical custody recovery
slug: hpms-historical-custody-recovery
type: plan
status: draft
rubric_version: v1.4
author: Codex
created: 2026-10-30
updated: 2026-10-30
sources:
  - docs/hpms-vintage-audit.md
  - docs/handoff/2026-05-07-post-b1-review.md
  - https://www.fhwa.dot.gov/policyinformation/hpms/shapefiles.cfm
wave: 2026-10-30-hpms-custody-recovery
pulse: 01
depends_on: []
governing_roles: [traffic-engineer, citation-auditor, scope-keeper, numeracy-checker]
---

# Pulse 01: inventory and prospective capture custody

## Mission and scope

Inventory available normalized candidates without modifying them; pin the tracked
history of the 11,344 AADT and 24% truck figures. Record the handoff's claimed
160,969 rows / 28 states separately from today's 165,873-row / eleven-state
local cache. Neither is attested historical publisher input. Preserve first-party
history snapshots for shallow/offline verification, with complete blob hashes.

Capture exactly five fixed public URLs (publisher documentation, California layer
metadata, server-side count, complete distinct-year query, five-row sample).
Record query predicates and normalization scope differences, raw-body SHA-256,
response metadata and UTC retrieval time. Raw publisher bodies stay in ignored
.cache custody; tracked receipts contain metadata, counts and hashes only.
The query is state-wide, pre-normalization, not I-80-only and not a historic replay.

## Deliverables

- [ ] data/hpms-custody-investigation.json and immutable first-party snapshots.
- [ ] tools/hpms_custody.py: offline receipt/history checks, optional raw-body
  verification and bounded capture to a new directory inside .cache only.
- [ ] Focused tests for HTTP-200 ArcGIS errors, incomplete year responses,
  malformed values, sample completeness and absence of historical inference.
- [ ] docs/hpms-custody-recovery.md, README navigation and role disposition.
- [ ] Existing packet/source verifiers and full ten-case actual-WASM suite;
  hosted default and artifact identity before a separate TRACKER pointer update.

## Non-goals and reviews

No Rust/parser/join/normalizer/scoring/model or dependency change, old packet or
cache regeneration, new credentials, raw row publication, new public Pages file,
traffic freshness, source-year admission, engineering or fiscal promotion.
Traffic Engineer passes the distinction between documentation/candidate/current
source populations; historical execution remains unverified. Citation Auditor
requires exact Git blobs and capture hashes/URLs/times, and explicit distinction
between local verification and offline receipt validation. Scope Keeper passes
metadata/custody tooling only. Numeracy requires no comparison of unlike counts
or row medians with replicated graph means. Other source/domain holds remain.
Implementing-agent lenses only, no independent panel or cleared historical hold.
CR-006: five allowlisted HTTPS reads, bounded response sizes/timeouts, new ignored
output directory only; no overwrite or mutation of data/cache or arbitrary URL.

## Gates

Offline verification; supplied raw-body verification; focused unit tests; all
original packet/checklist/label-audit checks; clean build and existing ten browser
cases. Old public bundle bytes except clean provenance must remain identical.
