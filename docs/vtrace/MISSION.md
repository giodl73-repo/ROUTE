# Mission

## Scope

Repo: ROUTE

VTRACE adoption scope: establish the mission baseline for ROUTE before
creating requirements, specification baselines, trace rows, or new work
packages. This file is the leftmost VTRACE artifact for the repo and should
anchor later `REQ-*`, `SPEC-*`, `WP-*`, verification, and validation records.

## Mission Need

| ID | Need | Success Criteria | Status |
|---|---|---|---|
| NEED-001 | ROUTE shall turn public transportation data, local source ledgers, and generated artifacts into a reproducible analysis system for the US interstate network. | A maintainer can regenerate the active score, map, SLA, gate, and evidence artifacts from documented commands, with source/proxy/heuristic labels preserved. | accepted |
| NEED-002 | ROUTE shall identify and explain national highway network gaps, bottlenecks, resilience holes, access gaps, and upgrade candidates without overstating the evidence. | Every material claim is tied to a data artifact, command, source label, confidence label, review record, or explicit hold. | accepted |
| NEED-003 | ROUTE shall convert analysis into defensible Interstate 2.0 design options, not construction promises or advocacy briefs. | Proposed standards, corridors, feature packages, and game/research claims are marked as implemented, heuristic, planned, held, or deprecated before publication or downstream use. | accepted |
| NEED-004 | ROUTE shall keep route identity stable as analysis moves from raw corridors to bundles, maps, simulations, SLA promises, game overlays, and reports. | Segment-bearing artifacts join through `segment_bundle_id`, `national_segment_id`, or an explicitly named transitional surface. | accepted |
| NEED-005 | ROUTE shall expose transportation tradeoffs through review roles instead of hiding them behind a single score. | Parliament, stakeholder, editorial, and panel reviews can change claims, labels, next evidence steps, dockets, or work-package status. | accepted |
| NEED-006 | ROUTE shall support current stop-first SLA network work where visible stops, routes, service classes, schematic geometry, and SLA promises agree. | Stop/SLA gates catch oversized gaps, endpoint/contact policy defects, and map/SLA mismatches before generated artifacts are treated as release-ready. | accepted |

## Users

| User | Need | Success Signal |
|---|---|---|
| ROUTE maintainer | Know which commands, artifacts, and review gates define the current truthful repo state. | A clean gate bundle can be run and the resulting artifacts match the documented claims. |
| Transportation analyst | Inspect scored corridors, gaps, standards, and evidence labels without reverse-engineering the Rust workspace. | Scores, maps, dockets, and reports cite their source surfaces and confidence posture. |
| Infrastructure planner or reviewer | Understand why a corridor, standard, or feature package is supported, held, or downgraded. | Each claim names the data, simulation, role review, and next evidence step that governs it. |
| State DOT planner | See delivery, funding, maintenance, right-of-way, and feasibility implications before a proposal reads as buildable. | Feature packages and design options separate analytical merit from delivery readiness and lifecycle burden. |
| Long-haul trucker and freight operator | Understand how ROUTE handles reliability, rest, grade, bridge, parking, and closure risks that affect daily freight operations. | SLA, stop, standards, and pressure-test artifacts expose operational constraints instead of only aggregate scores. |
| Rural and agricultural user | See whether access, redundancy, farm logistics, emergency reach, and rural service are represented as first-class value. | Rural connectivity and agricultural access claims remain visible even when strict volume or NPV metrics understate them. |
| Transit-dependent traveler | See whether Interstate 2.0 corridors support non-driving intercity access, park-and-ride, coach stops, and first/last-mile links. | Multimodal and shared-transit claims point to actual stops, facilities, or held evidence rather than map proximity alone. |
| Environmental/community health reviewer | See climate, runoff, habitat, noise, air-quality, and environmental-justice exposure before a design option is promoted. | Resilience and mitigation records name affected communities/ecosystems and keep evidence level explicit. |
| Game/system designer | Reuse ROUTE network and standard outputs in Interstate Tycoon without breaking evidence boundaries. | Game-facing artifacts identify which mechanics are implemented, heuristic, simulated, or held for owner review. |
| Coding agent | Make scoped changes without drifting claims, generated artifacts, or review obligations. | Work packages name parent IDs, affected crates/data/docs, validation commands, and evidence rows before closure. |

## Operating Context

ROUTE is a Rust workspace, data corpus, review system, and research/design
process for Interstate 2.0. It combines generated CSVs, map artifacts, source
ledgers, pressure-test simulations, stop/SLA diagnostics, review records, and
documentation. Work usually happens inside a dirty portfolio checkout, so
repo-local changes must stay scoped and must not depend on TRACKER-relative
paths for build correctness.

The current active goal is the stop-first SLA network: stops, routes, SLA
promises, service classes, and schematic geometry should agree across T1/T2/T3
surfaces. This mission file does not replace the existing Milepost plan; it
creates the VTRACE anchor that later requirements and work packages can trace
back to.

## Constraints

| ID | Constraint | Rationale | Status |
|---|---|---|---|
| CON-001 | ROUTE public claims must stay bounded by implemented commands, generated artifacts, source labels, confidence labels, and review records. | Prevents planned, heuristic, or simulated work from reading as proof-grade evidence. | accepted |
| CON-002 | Segment-bearing artifacts must preserve the bundle-first architecture: route labels, tiers, map ids, and zones are not stable primary keys. | Keeps maps, simulations, stops, upgrades, and reports tied to stable physical/service identity. | accepted |
| CON-003 | Generated artifacts must name the source-of-truth data and commands that regenerate them. | Keeps the repo reproducible and prevents hand-edited generated outputs from becoming hidden state. | accepted |
| CON-004 | Source gaps, heuristic rows, simulated evidence, and owner/human review holds must remain visible status, not missing prose. | Keeps evidence debt actionable and traceable. | accepted |
| CON-005 | ROUTE implementation changes belong in the child repo; TRACKER should only receive intentional submodule pointer updates. | Preserves portfolio snapshot discipline. | accepted |
| CON-006 | ROUTE must not claim construction readiness, statutory compliance, or official agency endorsement. | Keeps the project framed as research, tooling, review, and design analysis. | accepted |

## Non-Goals

- ROUTE is not a construction drawing set or official engineering study.
- ROUTE is not an advocacy brief for a specific corridor or policy outcome.
- ROUTE does not predict what federal, state, or local governments will build.
- ROUTE does not treat visual maps, game mechanics, or heuristic simulations as
  proof-grade evidence unless their evidence level says so.
- ROUTE does not let a route designation alone define physical identity.

## Success Criteria

| Criterion | Validation Method | Evidence Pointer |
|---|---|---|
| VTRACE mission needs are explicit enough to derive requirements. | Inspect this file before writing `REQUIREMENTS.md`. | future `EVID-*` |
| Mission needs cover analysis, evidence posture, design boundaries, identity, review roles, and active stop/SLA work. | Cross-check against `README.md`, `GOAL.md`, `docs/SYSTEM_PLAN.md`, and `docs/route-architecture.md`. | future `EVID-*` |
| Later VTRACE artifacts can reference stable parent IDs. | `REQ-*` rows should cite `NEED-*` and `CON-*` IDs from this file. | future `TRACE.md` |

## Role Review Notes

| Role Lens | Mission Impact | Disposition |
|---|---|---|
| Scope Keeper | Mission stays at repo/system intent and avoids corridor scoring, gap findings, or design proposals. | pass |
| Citation Auditor | Mission makes no new quantitative claims; source links are repo-local context artifacts. | pass |
| Numeracy Checker | Mission contains no arithmetic, score, cost, volume, mileage, or percentage claims. | pass |
| Optimization Methodologist | Mission needs and constraints require reproducible commands, explicit evidence labels, stable identity, and visible held/rejected states. | pass |
| Schematic Cartographer | Mission covers stop/SLA/map agreement and false-transfer prevention through stop-first and identity constraints. | pass |
| Traffic Engineer / Freight Economist / Rural Advocate | Mission now includes operational reliability, freight value, rural/agricultural access, and evidence-bounded tradeoffs. | pass |
| State DOT / Transit-Dependent / Environmental Stakeholders | Mission now names delivery feasibility, non-driving access, and environmental/community-health review as first-class user needs. | pass |

## Source Links

- `README.md`
- `GOAL.md`
- `docs/SYSTEM_PLAN.md`
- `docs/route-architecture.md`
- `specs/2026-05-06-route-design.md`
