---
name: HPMS label-origin trace
slug: hpms-label-origin-trace
type: reference
status: reviewed
rubric_version: v1.4
author: Codex
created: 2026-10-30
updated: 2026-10-30
sources:
  - data/browser-hpms-vintage-audit.json
  - docs/vtrace/WP_WEB_SOURCE_002.md
---
# Why the packet label does not establish a source year

The preserved I-80 packet says HPMS 2023. Its builder copies the Key Facts
section from the corpus; the original report writer and scorer hardcoded 2023
without receiving a source year. The fetcher instead configures 2018 state
services. This proves a software mechanism that can emit an unsupported label,
not which bytes a historical run actually consumed. The packet's input vintage
remains unverified. Do not relabel its figures 2018.

The default join first groups HPMS rows by route across states, takes upper-
median AADT and mean truck proportion, and assigns those aggregates to graph
edges. Subsequent graph-edge means/percentiles are not independent matched
segment observations. No spatial traffic variation or corridor-average traffic
is established by replication across graph edges. This release changes no join,
math, values, scores, thresholds or confidence.

A separate dated local normalized-cache inspection records 165,873 CSV rows,
including 42,970 I-80 rows in eleven states, with no year column. Its row-level
upper-median AADT is 22,400, whereas the reviewed packet reports 11,344. The
populations, execution and estimators are not matched. Neither value replaces
the other or proves a source year. No raw publisher response/capture manifest
was verified or published. The normalized-cache hash is a local-file hash,
not publisher capture custody and not a new current-traffic observation.

Future default native reports/citations now state that the input source year
is unverified, rather than inventing 2023 or guessing 2018. A normal report can
still reproduce existing arithmetic; that is not validation of source vintage.
The historical corpus, packet and recorded checklist retain exact original
bytes and their unresolved status. Four checklist canonical responses and four
pre-checklist published WASM canonical results remain unchanged.

The independent JSON audit has nine line-located first-party code excerpts,
complete source-blob hashes and separate excerpt hashes. Two complete original
writer/scorer files are preserved in the repo for offline verification; all
other evidence inputs remain unchanged. Build verification checks those copies
or current committed Git blobs and each excerpt. It never reacquires or reads a
cache. The JSON's dated cache observation is recorded, not independently
re-verified by browser/CI. All source code remains under the existing software
license; publisher materials retain their own rights.

Next: recover a historical normalized input hash plus the publisher response
and retrieval manifest, then reproduce the exact filters, join, population and
estimator. Without that chain, matched packet input vintage stays null.
I-80 remains Hold and narrow; no traffic freshness, engineering, design, ROI,
SLA, fiscal admission or agency endorsement is promoted. The fictional
freight/relay/trip workbench remains independent.
