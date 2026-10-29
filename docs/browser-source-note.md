---
name: Browser I-80 source checklist scope
slug: browser-i80-source-checklist-scope
type: reference
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
# Recorded I-80 source checklist

The independent Rust/WebAssembly inspector replays one 2026-10-29 local-cache
observation: 12 source entries, 4 ready, 2 blocked, 6 excluded. The unchanged
Python readiness function produced this observation without acquisition,
credentials, cache writes or network attempts. Browser/CI builds use only the
committed JSON. They do not inspect or reacquire a cache. The explicit capture
tool refuses to overwrite a recorded snapshot.

Ready means dated local artifact checks. It does not prove current reader/CI
readiness, raw-source hash verification, report regeneration or traffic quality.
Evidence counts describe unlike units (ZIP members, lines, CSV records); never
sum them or interpret them as traffic/population/coverage. Contract gaps and
observed blockers remain separate: ready can coexist with schema/vintage work.
Current/candidate source years and publisher URLs are frozen contract fields,
not claims of today's latest data, endpoint health or current law.

**Unresolved HPMS vintage:** the unchanged historical reviewed packet labels
traffic HPMS 2023; the source contract and acquisition code identify 2018.
Neither original artifact nor its values were relabeled or recomputed. The
packet's traffic values have not been relinked to acquired source bytes.
I-80 remains **Hold and narrow**.

Three downloads retain exact committed LF bytes: recorded observation JSON,
original source contract CSV and historical reviewed packet Markdown.
source-custody.json identifies their repository paths, byte lengths and SHA-256.
These hashes cover repository artifacts, not newly verified primary-source bytes.
Raw publisher artifacts are absent. No current refresh, national traffic
provenance, approved design, guaranteed SLA, positive ROI, fiscal savings or
agency endorsement is established. Current AADT, intervention effect and matched
packet-traffic vintage remain null. The synthetic corridor/relay/trip engine is
independent; no source metadata supplies its invented traffic assumptions.

The native replay is `cargo run --locked -p route-web --bin
route-source-checklist -- fixtures/source-checklist-all.json`. Only optional
status null/ready/blocked/excluded is accepted. Unknown fields/states and queries
above 64 KiB fail closed. Build validation independently matches every original
CSV contract row, three original-artifact hashes, full captured-row projection,
four native/WASM query strings and four pre-change synthetic canonical results.
