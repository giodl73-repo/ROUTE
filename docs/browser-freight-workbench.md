---
name: Browser freight workbench
slug: browser-freight-workbench
type: spec
status: draft
rubric_version: v1.0
author: Codex with Gio Della-Libera
created: 2026-10-04
updated: 2026-10-05
sources: []
---

# Browser freight workbench

[Open the workbench](https://giodl73-repo.github.io/ROUTE/).

`route-kernel` is ROUTE-owned: pure BPR travel time, existing relay staffing/outage models, and the OD trip simulation. `route-sim` re-exports the same types and functions, preserving native call sites. Native fetching, graph construction, equilibrium assignment, and research evidence stay in their existing crates. This is an internal portability boundary, not a new cross-repository primitive.

`route-web` supplies three explicitly synthetic fixed-corridor fixtures and validates JSON (maximum 4 KB). Demand/capacity are 25–200%, outage 0–24 hours, reserve/absorption 0–100%, target 1–48 hours. No file or network access is needed by the WASM dependency tree. No traffic model calculations live in JavaScript.

Fixtures have three fixed legs, 65 mph free-flow speed, and directional BPR volumes/capacities. Demand scales traffic and 1,200 baseline daily relay swaps together; that relationship is invented for demonstration. Capacity only changes travel; outage only changes relay swap outcomes. Target gaps concern driving time alone. Retention is among outage-affected swaps. The steady-state travel/outage cards do not estimate delivery times, investment returns, official staffing requirements, or equilibrated route choice. Research holds are unchanged.

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

The broader native release checks are `cargo test --locked --workspace` and `./scripts/check-mileposts.ps1 -SkipTests`. The latter builds ROUTE once and runs every data gate against Cargo's reported native executable.

## Whole-trip comparison (2026-10-05)

The native OD implementation is now in route-kernel::od; route-sim::od retains native TOML loading and re-exports the portable model. Existing native APIs and tests are preserved.
For each driver mode, the browser samples 2,000 seeded trips and shows representative median and 95th-percentile elapsed times. Components reconcile driving, rest/swaps, incidents and 2-hour fixed overhead. Bars encode component duration and omit zero-hour parts; they are time budgets, not chronological schedules.
The existing native trip model varies traffic 80-120%, with 10% surge probability; BPR V/C is capped at 1.3. Each synthetic leg has invented incident probability 0.1, mean delay 1h and standard deviation 0.5h. Two relay hubs assume fresh crews and 20-25 minute handoffs. Solo rest and team swap rules are simplified, not a legal HOS compliance engine. The separate hub outage calculation does not change trip distributions. No empirical reliability or delivery guarantee is claimed.
