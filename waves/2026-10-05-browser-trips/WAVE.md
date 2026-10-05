---
name: Browser whole-trip comparison
slug: browser-trips
type: plan
status: done
rubric_version: v1.0
author: Codex with Gio Della-Libera
created: 2026-10-05
updated: 2026-10-05
sources: []
---

Mission: expose existing native driver-mode Monte Carlo calculations on Pages.
Portable ROUTE-owned OD module; native filesystem loader remains native.
Three existing synthetic corridors, 2,000 seeded samples per driver mode.
Median and 95th-percentile trips include driving, rest/swaps, incidents and fixed overhead.
No new research promotion, legal HOS certification, or coupling of hub outages into arrival times.

Pulse: extract shared simulation, add comparison and time breakdown, verify reproducibility and mobile use.
Gates: focused and full native tests, strict portable clippy, real WASM browser tests, code review, CI/Pages deployment.

Validation: 482 full-workspace Rust tests, five real-WASM Chromium tests, strict portable clippy. Review caught timeline sizing; fixed zero flex basis and omitted zero durations, verified proportionally in browser. Artifact 196,615 bytes. Publication CI remains final gate.
