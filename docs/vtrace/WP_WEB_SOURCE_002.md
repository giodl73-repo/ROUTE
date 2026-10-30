---
name: I-80 HPMS label-origin trace
slug: i80-hpms-label-origin-trace
type: plan
status: reviewed
rubric_version: v1.4
author: Codex
created: 2026-10-30
updated: 2026-10-30
sources:
  - docs/vtrace/WP_WEB_SOURCE_001.md
  - crates/route-data/src/hpms_fetch.rs
  - crates/route-report/src/lib.rs
---
# WP-WEB-SOURCE-002: trace HPMS labels without inventing vintage

Accepted before implementation, owner-directed continuation of WASM self-service.
No prior I-80 wave, reviewed packet, corpus or recorded checklist is regenerated.

## Required outcome

Expose the source-label chain beside the existing Rust/WASM checklist and as
independent exact-custody JSON/Markdown downloads. Trace reviewed packet facts to
the corpus, packet builder and hardcoded scorer/report-writer labels; contrast
the explicit 2018 acquisition configuration. Both are code/metadata evidence,
not a matched publisher observation. Record that the normalized cache has no
year column or raw publisher response custody. Dated cache inspection is local
metadata only, not new acquired source data or fresh current traffic.

Prevent future default scorer/report-writer output from asserting HPMS 2023
when CorridorAttributes/HpmsRecord carry no source year. Use explicit unverified
input-year wording, preserving mathematical outputs, confidence and thresholds.
Explain the default route-level median-AADT/mean-truck assignment to graph edges;
edge summaries do not prove independent measured segment variation. Do not replace
2023 with 2018 or silently claim the packet is regenerated. Preserve all old
artifact hashes and four checklist responses and four legacy WASM strings.

## Scope and gates

Allowed: scorer/report citation strings and explicit provenance caveat, focused
native regression tests; one frozen audit manifest, offline custody verifier,
source-note/HTML explanation and independent downloads; Pages copy/verify steps and affected native gates in .github/workflows/pages.yml;
one browser case proving trace downloads/custody and old primary outputs; this
wave/pulse and README/VTRACE acceptance record. No network/source acquisition,
new credentials, parser/join/math changes, dependency/lock/pin changes, cache
writes, corpus/packet regeneration, scoring promotion or TAXLANE calculation.

L0 fmt, offline evidence verifier, original packet --check and source-checklist
custody. L1 route-score/report/network/data and kernel/web/sim tests, cargo test
-p route, strict affected native/WASM Clippy; hosted full existing native/release
and Pages gates. L2 build under /ROUTE/, existing nine actual-WASM cases and the
new trace/custody case, native/WASM canonical checks unchanged, keyboard/mobile
and TAXLANE roundtrip, exact default artifact/live files before TRACKER snapshot.

## Role and assurance acceptance

Implementing-agent review; no named-person participation or independent panel.
The parent WP-WEB-SOURCE-001 twelve-role holds and stakeholder deferrals remain.
Traffic Engineer: pass for tracing software labels and the default route-level
join; matched vintage, spatial variation, present AADT and intervention effects
remain unverified. Citation Auditor: pass with exact committed-blob hashes and
locators, explicit distinction between input configuration, local cache and raw
publisher custody. Scope Keeper: pass for metadata and future label correction;
original reviewed corpus/packet and their dispositions remain immutable here.
Numeracy: pass with unchanged math and like-for-like distinctions; observed
local cache median is not the historical packet's population or estimator.
Other national, construction, freight/ROI, equity, climate, rural, optimization
and map outcomes stay deferred exactly as in the parent contract. No authority
or source-spine gate advances; no critical/major finding remains in this scope.

CR-001: string replacements do not expand existing long generator functions;
new verifier helpers and test cases remain small and separated. CR-002..005:
fixed manifest, deterministic hashes, strict shape/custody checks and regression
gates. CR-006 approval: existing Git-blob reads and fixed build subprocesses only;
no new native execution interface, secret, network acquisition, person data or
unsafe FFI. Local canonical cache/ignored Cargo overrides remain untouched;
implementation uses an isolated child worktree. Rollback: revert the child
release and restore the preceding Pages artifact, retaining historical receipts.
