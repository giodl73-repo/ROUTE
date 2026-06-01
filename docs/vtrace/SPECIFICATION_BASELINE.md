# Specification Baseline

## Scope

Repo: ROUTE

Baseline type: mixed

Baseline date: 2026-06-01

VTRACE adoption scope: classify ROUTE's existing contracts and VTRACE target
contracts before implementation planning. This baseline is a control surface:
future work packages should cite `SPEC-*` IDs instead of making unanchored
changes.

## Specification Sources

| Source | Evidence | Status | Notes |
|---|---|---|---|
| `README.md` | ROUTE thesis, method, role model, corpus, dimension pool, source posture. | current | Public-facing repo intent. |
| `GOAL.md` | Active stop-first SLA network goal, success criteria, delivered slice. | current | Current operating focus. |
| `docs/SYSTEM_PLAN.md` | Milepost lifecycle, truth labels, crate ownership, review system. | current | Primary operating plan. |
| `docs/route-architecture.md` | Bundle-first identity invariant and crate direction. | current | Governs segment-bearing artifacts. |
| `docs/SPEC_INDEX.md` | Claim ownership, status vocabulary, key data/spec surfaces. | current | Helps prevent spec drift. |
| `package.json` | L0/L1/L2 script names including `check:l2`. | current | Validation profile entry point. |
| `.roles/` | Parliament, stakeholder, editorial, and panel-review lenses. | current | Review-lane source. |
| `docs/vtrace/MISSION.md` | VTRACE mission needs and constraints. | current | VTRACE source-of-truth adoption artifact exists. |
| `docs/vtrace/CONOPS.md` | VTRACE operating scenarios `OPS-*`. | current | VTRACE scenario source exists. |
| `docs/vtrace/REQUIREMENTS.md` | VTRACE requirements `REQ-*`. | current | VTRACE requirement source exists. |
| `docs/vtrace/TRACE.md` | Requirement-to-spec-to-work-package trace. | current | VTRACE trace source exists. |
| `docs/vtrace/CODE_RIGOR.md` | ROUTE code-rigor constraints `CR-*`. | current | Code rigor source exists. |
| `docs/vtrace/WORK_PACKAGES.md` | Execution packages `WP-*`. | current | Work-package source exists. |
| `docs/vtrace/VERIFICATION.md` | Verification matrix `VER-*` and evidence IDs. | current | Verification source exists. |
| `docs/vtrace/VALIDATION.md` | Scenario validation matrix `VAL-*`. | current | Validation source exists. |
| `docs/vtrace/EVIDENCE.md` | Evidence ledger `EVID-*`. | current | Evidence IDs record command, inspection, and review outcomes; browser L2 tooling risk remains open. |
| `docs/vtrace/REVIEW.md` | Review gate and executable role-review schema. | current | Review source exists. |

## Controlled Specification Items

| Spec ID | Parent REQ IDs | Type | Current / Target / Deprecated / Unknown | Specification Statement | Verification Method | Validation Method | Owner | Risk | Status |
|---|---|---|---|---|---|---|---|---|---|
| SPEC-001 | REQ-001 | ops | current | ROUTE documents a command-oriented operating model for regenerating and checking active artifacts through `docs/SYSTEM_PLAN.md`, `GOAL.md`, `package.json`, release docs, and command-specific ledgers. | inspection / command review | OPS-001 | ROUTE maintainer | medium | accepted |
| SPEC-002 | REQ-001 / REQ-002 / REQ-003 | product | current | ROUTE uses truth/evidence status vocabulary to distinguish implemented, heuristic, planned, held, deprecated, source-needed, and confidence-limited claims. | artifact inspection | OPS-001 / OPS-004 | ROUTE maintainer | medium | accepted |
| SPEC-003 | REQ-003 | evidence | current | Source gaps, holds, blockers, and next evidence steps are represented in ledgers, closeouts, release manifests, and review records rather than erased from the public claim surface. | ledger inspection / review | OPS-001 / OPS-004 | ROUTE maintainer | high | accepted |
| SPEC-004 | REQ-004 / REQ-005 | architecture | current | Segment-bearing artifacts must join through `segment_bundle_id`, `national_segment_id`, `stitch_group_id`, or an explicitly transitional surface; route labels, tiers, map ids, and zones are mutable presentation/classification fields. | architecture inspection / gate review | OPS-002 | route-network owner | high | accepted |
| SPEC-005 | REQ-004 / REQ-005 | package | current | `route-network` owns bundle membership, stable segment identity, graph-to-segment mapping, and bundle registry semantics; downstream crates consume bundle/member identity instead of redefining it. | crate-boundary inspection | OPS-002 | route-network owner | high | accepted |
| SPEC-006 | REQ-006 / REQ-007 | ops | current | Stop-first SLA work must keep visible stops, route services, service classes, schematic geometry, and SLA promises synchronized through source rows, generated artifacts, diagnostics, and command gates. | command gate / artifact inspection | OPS-003 | route-cli / route-map owners | high | accepted |
| SPEC-007 | REQ-007 | test | current | ROUTE has L0/L1/L2 validation entry points, including `npm run check:l2`, for system smoke/e2e validation; exact VTRACE gate selection remains a later verification decision. | script inspection / later command run | OPS-003 | ROUTE maintainer | medium | accepted |
| SPEC-008 | REQ-008 / REQ-009 | review | current | ROUTE review is governed by `.roles/`, including parliament, stakeholder, editorial, and panel-review lanes that can change claims, labels, dockets, or next evidence steps. | role inspection / review record inspection | OPS-004 | review steward | medium | accepted |
| SPEC-009 | REQ-009 | review | target | VTRACE work packages that promote design, map, SLA, release, or publication claims must name applicable stakeholder lanes, including delivery feasibility, freight operations, rural/agricultural access, non-driving access, and environmental/community-health impact. | role review / artifact inspection | OPS-004 | review steward | medium | accepted |
| SPEC-010 | REQ-010 | product | current | ROUTE outputs are framed as research, tooling, review, game/simulation, and design analysis; they are not construction drawings, statutory compliance claims, official agency endorsements, or predictions of what will be built. | editorial inspection | OPS-004 | ROUTE maintainer | high | accepted |
| SPEC-011 | REQ-011 | ops | current | ROUTE implementation and VTRACE artifacts belong in the ROUTE child repo; TRACKER should only record intentional submodule pointer updates after child work is committed. | git status / submodule diff inspection | OPS-005 | ROUTE maintainer / portfolio maintainer | medium | accepted |
| SPEC-012 | REQ-001 / REQ-002 / REQ-003 / REQ-004 / REQ-005 / REQ-006 / REQ-007 / REQ-008 / REQ-009 / REQ-010 / REQ-011 | process | target | ROUTE VTRACE adoption uses `docs/vtrace/` as the source-of-truth package from mission through trace, evidence, work packages, verification, validation, and review. | VTRACE artifact inspection | OPS-005 | ROUTE maintainer | medium | accepted |
| SPEC-013 | REQ-001 / REQ-002 / REQ-003 / REQ-006 / REQ-007 | evidence | target | VTRACE evidence and verification artifacts shall record the selected ROUTE command bundle, generated-artifact expectations, and any deferred gates caused by dirty local state or expensive commands. | verification/evidence inspection | OPS-001 / OPS-003 / OPS-005 | ROUTE maintainer | medium | accepted |

## Public Contracts

| Contract ID | Spec IDs | Surface | Compatibility Rule | Change-Control Trigger | Verification Evidence |
|---|---|---|---|---|---|
| IF-001 | SPEC-001 / SPEC-007 | CLI / scripts | Script and command names used by VTRACE artifacts must match repo-local entry points or be marked deferred/unknown. | Renaming validation scripts or command gates. | `package.json`, future `VERIFICATION.md` |
| IF-002 | SPEC-004 / SPEC-005 | data schema / architecture | Segment-bearing artifacts must use stable bundle/member/stitch identity or name their transitional surface. | New segment-bearing artifact, schema change, or identity migration. | `docs/route-architecture.md`, data artifact inspection |
| IF-003 | SPEC-006 | generated map/SLA artifacts | Stop/SLA/map artifacts may be release-ready only when diagnostics and gates show no blocking mismatch or the hold is explicit. | Stop, service, SLA, schematic, or service-class change. | `VERIFICATION.md`, generated artifact diff |
| IF-004 | SPEC-008 / SPEC-009 | `.roles` review | Claim promotion must cite applicable review lanes or explain why no lane applies. | Claim promotion, downgrade, publication, or release-surface change. | review record inspection |
| IF-005 | SPEC-010 | public claims | Public-facing ROUTE text must not imply construction readiness, legal compliance, official endorsement, or proof-grade evidence when the evidence label is weaker. | README, release, research, design, or game-publication claim change. | editorial review |
| IF-006 | SPEC-011 | TRACKER submodule | TRACKER pointer updates are separate from child repo implementation changes. | Child repo commit intended for portfolio snapshot. | TRACKER submodule diff |
| IF-007 | SPEC-012 / SPEC-013 | VTRACE docs | VTRACE artifacts under `docs/vtrace/` must preserve stable IDs once downstream artifacts reference them. | New VTRACE stage, ID rename, or requirement/spec change. | VTRACE trace inspection |

## Package / Language Allocation

| Spec IDs | Package / Crate / Module / Language | Responsibility | Forbidden Responsibility | Validation Profile |
|---|---|---|---|---|
| SPEC-001 / SPEC-007 / SPEC-013 | `route-cli`, `package.json`, scripts | Orchestrate gates, commands, and generated artifact checks. | Own stable identity semantics that belong in library crates. | L0: script inspection / L1: cargo tests / L2: `npm run check:l2` candidate |
| SPEC-004 / SPEC-005 | `route-network` / Rust | Own bundle identity, segment membership, graph-to-segment mapping, and registry semantics. | Infer stable identity from mutable labels alone. | L0: crate tests / L1: architecture gate / L2: downstream artifact inspection |
| SPEC-006 | `route-map`, `route-cli`, route data / Rust + CSV | Generate and gate stop/SLA/map/diagnostic surfaces. | Treat visually convenient map geometry as service truth. | L0: unit tests / L1: generated CSV gate / L2: e2e map/SLA gate |
| SPEC-008 / SPEC-009 / SPEC-010 | `.roles/`, `reviews/`, `docs/` / Markdown | Govern claim review, stakeholder lenses, and scope/editorial gates. | Promote claims without recorded evidence posture. | L0: docs inspection / L1: role review / L2: release/review gate |
| SPEC-011 | TRACKER submodule + ROUTE child repo / git | Keep child implementation and portfolio pointer updates separate. | Mix unrelated TRACKER work with ROUTE VTRACE changes. | L0: `git status` / L1: submodule diff |
| SPEC-012 / SPEC-013 | `docs/vtrace/` / Markdown | Hold VTRACE source-of-truth adoption artifacts and future evidence. | Override existing ROUTE specs without trace or review. | L0: `git diff --check` / L1: VTRACE artifact inspection |

## Nonfunctional Constraints

| Constraint ID | Parent Spec IDs | Constraint | Threshold / Rule | Verification Method | Status |
|---|---|---|---|---|---|
| SPEC-NF-001 | SPEC-001 / SPEC-013 | Reproducibility | Generated artifacts used as evidence must name their regeneration command or remain deferred. | inspection | accepted |
| SPEC-NF-002 | SPEC-002 / SPEC-003 / SPEC-010 | Evidence honesty | Weaker-than-proof evidence must remain visibly labeled before downstream use. | review inspection | accepted |
| SPEC-NF-003 | SPEC-004 / SPEC-005 | Identity stability | Mutable labels must not be the sole primary key for segment-bearing rows. | architecture gate / data inspection | accepted |
| SPEC-NF-004 | SPEC-006 | Map/SLA truthfulness | Schematic geometry must not create false stops, false transfers, or untracked service promises. | schematic review / command gate | accepted |
| SPEC-NF-005 | SPEC-008 / SPEC-009 | Review usefulness | Review must change a claim, label, docket, artifact, or next evidence step when it finds a gap. | review inspection | accepted |
| SPEC-NF-006 | SPEC-011 | Portfolio isolation | Child repo implementation changes and TRACKER pointer updates remain separate commits unless explicitly directed otherwise. | git inspection | accepted |

## Assumptions And Unknowns

| ID | Item | Impact | Disposition | Owner |
|---|---|---|---|---|
| SPEC-UNK-001 | Exact ROUTE VTRACE L2 command bundle for each future stop-first SLA work slice. | Work-package validation must select whether `npm run check:l2` is required for the concrete slice. | select in `WORK_PACKAGES.md` closeout and record in `VERIFICATION.md` / `EVIDENCE.md` | ROUTE maintainer |
| SPEC-UNK-002 | Actual command and review results for VTRACE `EVID-*` rows. | Evidence rows exist but cannot close until commands or reviews run. | close in `EVIDENCE.md` during work-package execution | ROUTE maintainer |
| SPEC-UNK-003 | Dirty local worktree and detached/branch state may block full command validation during VTRACE authoring. | VTRACE adoption should not accidentally validate or commit unrelated local work. | accept risk with scoped doc validation until commit requested | ROUTE maintainer |

## Requirement-To-Spec Coverage

| Requirement ID | Spec IDs | Coverage Status | Notes |
|---|---|---|---|
| REQ-001 | SPEC-001, SPEC-007, SPEC-013 | covered | Regeneration path exists; VTRACE gate bundle selection remains a verification task. |
| REQ-002 | SPEC-002, SPEC-003, SPEC-010, SPEC-013 | covered | Evidence posture is current ROUTE doctrine and future VTRACE evidence work. |
| REQ-003 | SPEC-003, SPEC-013 | covered | Holds/source gaps are current ROUTE behavior; VTRACE evidence rows are target. |
| REQ-004 | SPEC-004, SPEC-005 | covered | Bundle-first identity is current architecture. |
| REQ-005 | SPEC-004, SPEC-005, SPEC-NF-003 | covered | Mutable labels are explicitly forbidden as sole stable identity. |
| REQ-006 | SPEC-006, SPEC-007, SPEC-013 | covered | Stop-first SLA work has current artifacts and target VTRACE verification. |
| REQ-007 | SPEC-006, SPEC-007, SPEC-NF-004 | covered | Gate surfaces exist; exact L2 requirement is selected by work package. |
| REQ-008 | SPEC-008, SPEC-NF-005 | covered | `.roles` review model exists. |
| REQ-009 | SPEC-009, SPEC-008 | covered | Stakeholder promotion rule is target VTRACE discipline over current role model. |
| REQ-010 | SPEC-010, SPEC-NF-002 | covered | Scope/public-claim control exists and remains review-gated. |
| REQ-011 | SPEC-011, SPEC-NF-006 | covered | Portfolio snapshot discipline applies. |

## Spec-To-Verification Coverage

| Spec ID | Verification IDs / Commands | Expected Result | Evidence Pointer | Status |
|---|---|---|---|---|
| SPEC-001 | VER-001 / inspection | Regeneration sources and script entry points are identified. | EVID-001 | passed |
| SPEC-002 | VER-002 / inspection | Truth/evidence labels are present in ROUTE doctrine and artifacts. | EVID-002 | passed |
| SPEC-003 | VER-003 / inspection | Holds, blockers, and next evidence steps remain visible. | EVID-003 | passed |
| SPEC-004 | VER-004 / architecture inspection | Bundle-first identity rule is present. | EVID-004 | passed |
| SPEC-005 | VER-005 / architecture or command gate | Crate/package responsibilities are allocated. | EVID-004 / EVID-005 | passed |
| SPEC-006 | VER-006 / command gate candidate | Stop/SLA/map consistency can be checked. | EVID-006 | pass_with_risk |
| SPEC-007 | VER-007 / script inspection | L0/L1/L2 script candidates are identified; browser Playwright CLI is blocked locally. | EVID-007 | pass_with_risk |
| SPEC-008 | VER-008 / role inspection | Required role lanes exist and are mapped to package triggers. | EVID-008 | pass_with_risk |
| SPEC-009 | VER-009 / role review | Stakeholder lanes are attached before claim promotion; no new claim was promoted in this docs pass. | EVID-009 | pass_with_risk |
| SPEC-010 | VER-010 / editorial review | Public claims remain within ROUTE scope. | EVID-010 | pass_with_risk |
| SPEC-011 | VER-011 / git inspection | ROUTE child changes are separated from TRACKER pointer updates. | EVID-011 | pass_with_risk |
| SPEC-012 | VER-012 / VTRACE inspection | VTRACE package is internally consistent after validator pass. | EVID-012 | pass_with_risk |
| SPEC-013 | VER-013 / evidence inspection | VTRACE evidence names chosen commands and deferred gates. | EVID-013 | pass_with_risk |

## Role Review Notes

| Role Lens | Baseline Impact | Disposition |
|---|---|---|
| Scope Keeper | Baseline classifies repo/system contracts and does not score a corridor, propose a gap, or specify a construction design. | pass |
| Citation Auditor | Baseline cites repo-local artifacts and introduces no new quantitative transportation claims. | pass |
| Numeracy Checker | Baseline contains dates, IDs, and validation levels but no calculations, traffic volumes, costs, or score totals. | pass |
| Optimization Methodologist | Baseline separates current contracts, target VTRACE work, public interfaces, package allocation, unknowns, and verification coverage. | pass |
| Schematic Cartographer | Baseline treats map/SLA truthfulness and false-transfer prevention as controlled specs and nonfunctional constraints. | pass |
| Traffic Engineer / Freight Economist / Rural Advocate | Baseline preserves operational, freight, and rural/access concerns through evidence labels and role-review requirements. | pass |
| State DOT / Transit-Dependent / Environmental Stakeholders | Baseline requires delivery feasibility, non-driving access, and environmental/community-health concerns before design claim promotion. | pass |

## Specification Gate

Decision: pass_with_risk

Required before implementation planning:

- [x] Every accepted `REQ-*` maps to one or more `SPEC-*` IDs or a recorded deferral.
- [x] Every implementation work package can name parent `SPEC-*` IDs or discovery status.
- [x] Public contracts have owners and change-control triggers.
- [x] Unknowns are resolved, blocked, deferred, or converted to discovery work.
- [x] Verification and validation methods are credible for the controlled claim.

Rationale: ROUTE already has substantial current contracts for regeneration,
evidence posture, bundle identity, stop/SLA/map work, review roles, and public
scope control. VTRACE-specific trace, code-rigor, work-package, verification,
validation, evidence, and review artifacts now exist and WP-001 through WP-005
are closed or accepted with risk. The residual risk is the local browser
Playwright tooling blocker for L2 browser/game validation and the need for
future product code or generated-data changes to carry package-specific
evidence.
