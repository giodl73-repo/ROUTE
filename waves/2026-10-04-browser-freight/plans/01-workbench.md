---
name: Browser freight workbench
slug: browser-freight-workbench
type: plan
status: draft
rubric_version: v1.0
author: Codex with Gio Della-Libera
created: 2026-10-04
updated: 2026-10-04
sources: []
---

# Workbench pulse

- [x] Move pure BPR and relay model functions to a ROUTE-owned portable crate; preserve native API paths.
- [x] Add bounded synthetic scenario adapter with baseline comparisons.
- [x] Add responsive UI, worker, sharing, export, and explicit model limits.
- [x] Validate native regression tests, adapter, WASM, keyboard, export, sharing, and mobile.
- [ ] Run release/optimizer gates and milepost bundle; record any existing blockers without erasing them.
- [ ] Resolve code review and publish master; verify live deployment.

The CLI workspace package is `route` in crates/route-cli. Formatting/test scope includes route-kernel, route-web, and route-sim; the existing milepost bundle exercises the CLI release gates.

## Validation before publication

- 22 native route-sim regression tests + 3 route-web tests pass; shared kernel builds natively and as WASM.
- Format checks for changed Rust packages and clippy for route-kernel/route-web pass with warnings denied.
- Three real Chromium/WASM tests pass, covering keyboard, JSON export, shared reload, alternate corridor, mobile, invalid target/link, and engine-load failure.
- Desktop/mobile visual inspection passes; output initially 157,262 bytes. Baseline driving time 7.2 hours; capacity 95% yields 7.3 hours.
- `codex review --uncommitted` exited 0 with no actionable findings after reviewing the final npm-script preservation and worker initialization fix.
- The unmodified milepost bundle fails compiling the existing CLI: `optimizer_constraint_ledger_rows` is unused in non-test builds under warnings denied. Full-workspace formatting also has existing CLI drift. Both are outside the shared simulation changes. Public baseline CI run [32401429643](https://github.com/giodl73-repo/ROUTE/actions/runs/32401429643) already failed at origin/master `98445a1c5d678e54c8ce14b82e1d5d6d4d5bdeb5`. Do not describe the broad native release gate as passing.
- A separate Pages workflow validates the changed model and browser path. Existing npm scripts, original Playwright 1.61.1 dependency, legacy workflows, and research evidence are preserved.
