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
# I-80 recorded source checklist

Accepted WP-WEB-SOURCE-001. Pulse 01 captures existing local cache metadata
without acquisition, exposes it through a typed Rust/WASM filter and retains
all source/traffic/design holds. Prior I-80 waves remain closed and the packet
Hold and narrow. The original packet's HPMS 2023 label and the acquisition
contract's 2018 vintage remain an explicit unresolved discrepancy.

Exit: selected native checks, captured-row/contract parity, legacy/new actual
WASM bytes, exact metadata custody, existing/new browser tests, twelve-role
exit, hosted child release and verified TRACKER snapshot.

## Cross-target regression boundary

Four recorded Windows-native results remain byte-identical locally. Existing
native/WASM logarithm/exponential calculations can differ in their last bit;
no math or dependency is changed to suppress that difference. Build validation
compares all structure/non-numeric values exactly and numeric values with both
absolute/relative tolerance 1e-12 across targets. Four pre-change published WASM
results captured at a055dbfd3f8dfd1a5c020a67cb56c8fda652063c are compared byte-for-byte
to the new WASM. The four new checklist native/WASM results have exact canonical
string parity. Do not describe the legacy cross-target check as byte parity.
