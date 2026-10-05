---
name: Browser freight workbench
slug: browser-freight-workbench
type: plan
status: done
rubric_version: v1.0
author: Codex with Gio Della-Libera
created: 2026-10-04
updated: 2026-10-04
sources: []
wave: browser-freight
date_open: 2026-10-04
date_close: 2026-10-04
---

# Browser Freight Workbench

## Mission
Make ROUTE's Rust calculations usable on GitHub Pages with bounded, synthetic scenarios.

## Opening rule
The user authorized implementation, publication, and TRACKER snapshotting. A teaching sandbox must not promote held highway proposals or imply delivery guarantees.

## Inputs inherited
- Existing BPR, relay staffing, and outage sensitivity calculations in route-sim.
- Native source fetching remains native; public I-80 decision remains hold and narrow.
- TRACKER Rust/WASM Pages guidance; the prior closed reproducibility wave remains intact.

## Pulses
| Pulse | Status | Evidence |
|---|---|---|
| 01 — Portable kernel and bounded adapter | done | route-kernel; route-web; native re-exports |
| 02 — Interactive page and browser checks | done | web; tools/build-pages.py; tests/browser |
| 03 — Review, publish, and snapshot handoff | done | PR #21; hosted workflow 37255171471; CLOSE.md |

## Done criteria
- Native and WASM use one implementation of the extracted calculations.
- Baseline comparison, keyboard controls, sharing, and JSON export work with real WASM.
- Rust and browser checks pass; review findings are resolved.
- Master is published and verified; its pushed SHA and receipt are available for the TRACKER-owned snapshot.

## Non-goals
No national routing, new empirical traffic data, capital recommendations, coupled delivery simulation, or research-claim promotion.

## Close evidence

See [CLOSE.md](CLOSE.md). Child implementation and publication are complete; TRACKER owns the final clean default-branch snapshot. No next wave is opened automatically.
