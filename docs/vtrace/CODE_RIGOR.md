# Code Rigor

## Scope

Repo: ROUTE

Risk level: high for generated transportation evidence, public claim surfaces,
and stop/SLA/map gates; medium for repo-local process documentation.

Language/toolchain: Rust workspace, CSV/data artifacts, Markdown review
records, npm script wrappers, Playwright browser checks.

## Coding Constraints

| ID | Constraint | Applies To | Verification | Exception Rule |
|---|---|---|---|---|
| CR-001 | Hand-authored Rust functions that affect identity, gates, scoring, simulation, or generated artifacts should stay below 60 logical lines or carry a review rationale. | `route-network`, `route-cli`, `route-map`, `route-score`, `route-data`, `route-sim` | size/complexity inspection and code review | Larger functions require a work-package note naming why the split would reduce clarity or safety. |
| CR-002 | Complex control flow must be bounded by explicit states, deterministic ordering, tests, or a documented invariant. | graph building, bundle identity, stop/SLA gates, optimizer/simulation logic | design inspection, targeted tests, and review | Waive only when control flow mirrors a source format or external API and has fixture coverage. |
| CR-003 | Public CLIs, schemas, file readers, and generated-artifact writers must handle invalid input and errors explicitly. | `route-cli`, `route-data`, CSV/JSON/HTML artifact paths | tests, command review, and artifact inspection | Impossible states must name the upstream invariant that prevents them. |
| CR-004 | Critical identity and service-truth invariants must have assertions, tests, gates, or inspection evidence. | bundle/member identity, stop order, transfer/contact truth, SLA promises, route labels | unit/integration tests, command gates, and role review | If enforced by data generation instead of code, cite the generator and evidence row. |
| CR-005 | Formatter, compiler warnings, tests, and configured lint/static checks must be clean before a work package can close. | whole Rust workspace and script wrappers | `cargo fmt --check`, `cargo test`, configured npm checks | Waivers require owner, reason, and revisit trigger in `EVIDENCE.md`. |
| CR-006 | Unsafe Rust, FFI, network fetching, filesystem writes outside declared artifact paths, and shell execution are prohibited in VTRACE implementation work unless explicitly approved in the work package. | all crates and automation | code inspection and security review | Requires Security/privacy lane approval and a source-custody evidence row. |
| CR-007 | Recursion or unbounded iteration in graph, route, optimizer, parser, or simulation logic must have an explicit bound or proof of termination. | graph traversal, route selection, data parsing, simulation | code review and tests with edge-case fixtures | Waive only for library calls whose bounds are controlled by input preconditions named in the work package. |
| CR-008 | Generated artifacts used as evidence must be reproducible from a named command and must not be manually edited after generation unless the edit is itself reviewed. | score, map, SLA, gate, report, evidence artifacts | command evidence and artifact diff inspection | Manual repair requires a hold label, reviewer, and replacement command plan. |
| CR-009 | Package boundaries must preserve ownership: `route-network` owns stable identity; `route-map` owns rendering; `route-cli` orchestrates commands; docs/reviews govern claims. | package/crate changes | package-boundary inspection | Cross-boundary changes require integration notes in the work package. |
| CR-010 | Public or downstream claims must not be promoted from heuristic, simulated, planned, or source-needed evidence without review and label update. | README, docs, reports, release notes, generated claim surfaces | role review and evidence inspection | Emergency publication requires explicit `pass_with_risk` with owner and revisit trigger. |

## Tailoring

| Area | Rule | Rationale |
|---|---|---|
| Rust identity code | Prefer small pure functions and typed identifiers where existing patterns support them. | Bundle/member/stitch identity failures contaminate downstream maps, reports, and simulations. |
| Stop/SLA/map gates | Treat visible stops, contacts/transfers, geometry, and SLA promises as service-truth surfaces. | Schematic convenience must not invent service. |
| CLI and generated artifacts | Every artifact used as evidence must cite the exact command, expected outputs, and deferred gates. | ROUTE depends on reproducible generated evidence. |
| Role-governed claims | Parliament and stakeholder reviews are substance gates; editorial reviews are form gates before validated status. | `.roles/ROLE.md` separates adversarial substance review from editorial checks. |
| Git execution | Work-package commits should be scoped to ROUTE; TRACKER submodule pointer updates are separate. | TRACKER is the portfolio snapshot repo. |

## Required Role Lanes By Work Type

| Work Type | Required Role Lanes | Additional Conditional Lanes |
|---|---|---|
| VTRACE docs/process only | Scope Keeper, Citation Auditor, Numeracy Checker, Optimization Methodologist | Configuration/change control when commit or submodule pointer work is requested |
| Rust crate or package-boundary change | Optimization Methodologist, software assurance, V&V, configuration/change control | Security/privacy if filesystem, network, FFI, unsafe, or shell behavior changes |
| Bundle/segment identity change | Optimization Methodologist, Traffic Engineer, Schematic Cartographer, software assurance | Freight Economist, Rural Advocate, State DOT when promoted claims affect network priorities |
| Stop/SLA/map change | Schematic Cartographer, Traffic Engineer, Transit-Dependent Traveler, V&V | State DOT, Environmental Community, Freight Industry when claims touch delivery, impacts, or freight operations |
| Public claim, release, or publication change | Parliament lane selected by claim, affected stakeholder lanes, Citation Auditor, Numeracy Checker, Scope Keeper | Panel reviewer lane when paper/publication claims are promoted |
| Data/source ingestion change | Source custody, Citation Auditor, software assurance | Security/privacy when fetching, caching, credentials, or external URLs are touched |

## Exceptions / Waivers

| ID | Constraint | Exception | Rationale | Owner | Revisit Trigger |
|---|---|---|---|---|---|
| none | n/a | n/a | No active waivers for ROUTE VTRACE adoption. | n/a | n/a |

## Verification Evidence

| Evidence ID | Constraint IDs | Command / Review | Result | Evidence Pointer |
|---|---|---|---|---|
| EVID-CR-001 | CR-001 / CR-002 / CR-004 / CR-007 | code inspection for affected work-package diff | pending | `EVIDENCE.md` |
| EVID-CR-002 | CR-003 / CR-008 / CR-010 | artifact and claim inspection | pending | `EVIDENCE.md` |
| EVID-CR-003 | CR-005 | `cargo fmt --check`, `cargo test`, and selected npm checks | pending | `EVIDENCE.md` |
| EVID-CR-004 | CR-006 | security/privacy review for unsafe, FFI, network, filesystem, or shell changes | pending | `EVIDENCE.md` |
| EVID-CR-005 | CR-009 | package-boundary inspection | pending | `EVIDENCE.md` |

## Gate

Decision: pass_with_risk

Rationale: code-rigor constraints are now selectable by work package, but no
implementation package has run the checks yet.
