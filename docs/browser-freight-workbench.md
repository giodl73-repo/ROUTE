---
name: Browser freight workbench
slug: browser-freight-workbench
type: spec
status: draft
rubric_version: v1.0
author: Codex with Gio Della-Libera
created: 2026-10-04
updated: 2026-10-04
sources: []
---

# Browser freight workbench

[Open the workbench](https://giodl73-repo.github.io/ROUTE/).

`route-kernel` is ROUTE-owned: pure BPR travel time and the existing relay staffing/outage models. `route-sim` re-exports the same types and functions, preserving native call sites. Native fetching, graph construction, equilibrium assignment, and research evidence stay in their existing crates. This is an internal portability boundary, not a new cross-repository primitive.

`route-web` supplies three explicitly synthetic fixed-corridor fixtures and validates JSON (maximum 4 KB). Demand/capacity are 25–200%, outage 0–24 hours, reserve/absorption 0–100%, target 1–48 hours. No file or network access is needed by the WASM dependency tree. No traffic model calculations live in JavaScript.

Fixtures have three fixed legs, 65 mph free-flow speed, and directional BPR volumes/capacities. Demand scales traffic and 1,200 baseline daily relay swaps together; that relationship is invented for demonstration. Capacity only changes travel; outage only changes relay swap outcomes. Target gaps concern driving time alone. Retention is among outage-affected swaps. These calculations do not estimate delivery times, investment returns, official staffing requirements, or equilibrated route choice. Research holds are unchanged.

## Reproduce

```sh
rustup show
cargo install wasm-bindgen-cli --version 0.2.127 --locked
cargo test --locked -p route-kernel -p route-web -p route-sim
python tools/build-pages.py
npm ci
npx playwright install chromium
npm run test:pages
```

For Windows with installed Chromium, set `ROUTE_BROWSER_PATH` to its executable. Pages CI validates the native shared model and bounded adapter, compiles real WASM, runs Chromium checks, and deploys only master. Output must be inside dist and is capped at 5 MB. The toolchain, npm dependency, and wasm-bindgen versions are pinned.

The page includes the repository LICENSE notice: software is MIT; original non-software content is CC BY-NC 4.0 unless otherwise stated. Synthetic fixtures are software test/demo inputs, not copied external datasets.
