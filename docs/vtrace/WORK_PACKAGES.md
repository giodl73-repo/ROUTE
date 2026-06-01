# Work Packages

## Scope

Repo: ROUTE

VTRACE adoption scope: define executable packages that turn mission,
requirements, specifications, code-rigor constraints, verification, validation,
evidence, and role review into procedural implementation. These packages do not
authorize code changes by themselves; each package must be selected and closed
with evidence before it can claim implementation progress.

## Work Package Table

| ID | Objective | Parent IDs | Affected Surfaces | Entry Criteria | Exit Criteria | L0 / L1 / L2 | Status |
|---|---|---|---|---|---|---|---|
| WP-001 | Establish the ROUTE VTRACE control spine and keep IDs stable. | REQ-001 / REQ-002 / REQ-003 / REQ-011 / SPEC-001 / SPEC-002 / SPEC-003 / SPEC-011 / SPEC-012 / SPEC-013 | `docs/vtrace/`, `docs/SYSTEM_PLAN.md`, `docs/SPEC_INDEX.md`, git state | Mission, CONOPS, requirements, spec baseline, and trace exist. | VTRACE docs pass inspection; evidence records open gaps; child/portfolio separation is visible. | L0: docs diff check / L1: VTRACE inspection / L2: not required until release claim | closed_with_risk |
| WP-002 | Close bundle-first identity governance for segment-bearing changes. | REQ-004 / REQ-005 / SPEC-004 / SPEC-005 / SPEC-NF-003 | `route-network`, bundle schemas, segment-bearing generated artifacts, `docs/route-architecture.md` | Work touches segment identity or promotes identity claims. | Identity change cites stable keys, forbidden mutable-key usage is absent or held, and architecture review is recorded. | L0: targeted crate tests / L1: architecture inspection / L2: downstream artifact inspection | closed |
| WP-003 | Close stop-first SLA/map implementation slices. | REQ-006 / REQ-007 / SPEC-006 / SPEC-007 / SPEC-NF-004 / SPEC-013 | `route-cli`, `route-map`, route data, stop/SLA diagnostics, generated map artifacts | Work touches stops, services, service classes, map geometry, transfers, or SLA promises. | Oversized stop gaps, endpoint/contact policy defects, and map/SLA mismatches are checked or explicitly held. | L0: targeted tests / L1: generated CSV or command gate / L2: `npm run check:l2` when release/public claim is affected | closed_with_risk |
| WP-004 | Govern role-review and public-claim promotion. | REQ-002 / REQ-003 / REQ-008 / REQ-009 / REQ-010 / SPEC-002 / SPEC-003 / SPEC-008 / SPEC-009 / SPEC-010 / SPEC-NF-002 / SPEC-NF-005 | `.roles/`, `reviews/`, `docs/reviews/`, README/release/report/public docs | Work promotes, downgrades, holds, publishes, or uses a claim downstream. | Required roles are named; dissent and required changes are recorded; editorial gates run only after substance review. | L0: docs inspection / L1: role review / L2: release/review gate | closed_with_risk |
| WP-005 | Close implementation verification and evidence for any selected work package. | REQ-001 through REQ-011 / SPEC-001 through SPEC-013 / CR-001 through CR-010 | selected implementation surfaces, `VERIFICATION.md`, `VALIDATION.md`, `EVIDENCE.md`, `REVIEW.md` | A concrete work slice is selected from WP-001 through WP-004 or a new package is added. | Verification commands, evidence rows, validation impact, V closure, and git execution are recorded. | L0: selected fast checks / L1: repo confidence checks / L2: integration/readiness checks when claim requires | closed_with_risk |

## Work Package Details

### WP-001: VTRACE Control Spine

Objective: make ROUTE's VTRACE package a usable source of truth for existing
repo work.

Parent requirement IDs: REQ-001, REQ-002, REQ-003, REQ-011

Parent specification IDs: SPEC-001, SPEC-002, SPEC-003, SPEC-011, SPEC-012,
SPEC-013

Design/interface/code-rigor IDs: IF-001, IF-006, IF-007, CR-005, CR-008,
CR-010

Validation scenario IDs: OPS-001, OPS-005

Affected files/modules: `docs/vtrace/*`, `docs/SYSTEM_PLAN.md`,
`docs/SPEC_INDEX.md`, `package.json`, ROUTE git state, TRACKER submodule state

Entry criteria:

- Mission, CONOPS, requirements, specification baseline, and trace files exist.
- Existing unrelated worktree changes are not mixed into VTRACE claims.

Exit criteria:

- VTRACE source files pass whitespace/diff validation.
- Evidence rows identify generated-artifact, command, and role-review gaps.
- TRACKER pointer work remains explicitly separate unless requested.

Verification commands:

```powershell
git diff --check -- docs\vtrace
git status --short
```

Validation levels:

| Level | Required | Commands / Evidence | Result |
|---|---|---|---|
| L0 | yes | `git diff --check -- docs\vtrace` | passed |
| L1 | yes | VTRACE artifact inspection against `MISSION.md`, `CONOPS.md`, `REQUIREMENTS.md`, `SPECIFICATION_BASELINE.md`, `TRACE.md`; VTRACE validator run | passed |
| L2 | no | Full ROUTE integration not required for docs-only control-spine changes unless a release/public claim is promoted. | not_required |

V closure:

| V Area | IDs / Evidence | Status | Notes |
|---|---|---|---|
| Need / CONOPS | NEED-001 through NEED-006 / OPS-001 / OPS-005 | accepted | Covered by mission and CONOPS. |
| Requirements | REQ-001 / REQ-002 / REQ-003 / REQ-011 | accepted | Requirements exist and are traced. |
| Specification | SPEC-001 / SPEC-002 / SPEC-003 / SPEC-011 / SPEC-012 / SPEC-013 | accepted | Baseline exists. |
| Architecture / Interface | IF-001 / IF-006 / IF-007 | pass_with_risk | Command scripts, VTRACE docs, and child/portfolio separation are inspected; future pointer update evidence remains deferred. |
| Design / Code Rigor | CR-005 / CR-008 / CR-010 | pass_with_risk | Docs diff and VTRACE validation passed; generated-artifact and claim-promotion rigor remain deferred to later packages. |
| Implementation | `docs/vtrace/*` | passed | Docs authored and scoped to untracked `docs/vtrace/`. |
| Verification | VER-001 / VER-002 / VER-003 / VER-011 / VER-012 / VER-013 | pass_with_risk | WP-001 verification evidence is recorded; implementation-specific command evidence remains deferred. |
| Validation | OPS-001 / OPS-005 | pass_with_risk | Control-spine workflow is usable; portfolio pointer update validation remains deferred until requested. |
| Trace | `TRACE.md` | accepted | Trace rows now cite work packages and code rigor. |
| Gate | `REVIEW.md` | pass_with_risk | Required WP-001 review lanes are closed with residual implementation risk. |

Review gate:

| Lane | Required | Reviewer / Role | Decision | Evidence / Rationale |
|---|---|---|---|---|
| Systems engineering | yes | VTRACE maintainer | pass_with_risk | Control spine is internally consistent and validator-passing; later packages still need execution evidence. |
| Requirements traceability | yes | Optimization Methodologist | pass_with_risk | IDs cover mission through evidence; rejected/held implementation alternatives remain package-specific. |
| V&V | yes | V&V reviewer | pass_with_risk | L0 and VTRACE validation passed; command evidence for implementation packages remains deferred. |
| Software assurance | no | n/a | not_required | Docs-only package. |
| Security/privacy | no | n/a | not_required | No code, network, credentials, or data ingestion changes. |
| Safety/mission impact | yes | Scope Keeper | pass | WP-001 docs preserve research/tooling/review/design-analysis framing and do not imply construction/compliance readiness. |
| Source custody | no | n/a | not_required | No external source ingestion. |
| Configuration/change control | yes | ROUTE maintainer | pass_with_risk | `git status --short` shows only `?? docs/vtrace/`; no TRACKER pointer update was requested. |

Git execution:

- Branch/worktree: ROUTE child repo only.
- Commit plan: commit ROUTE VTRACE docs before any TRACKER pointer update.
- Push/PR condition: run selected validation and record evidence first.
- Agent stop condition: stop before committing unrelated local changes.

Wave/pulse execution:

- Active wave: not selected.
- Pulse file: not selected.
- Pulse status: deferred.
- Pulse evidence: future ROUTE or TRACKER pulse record if requested.

Status: closed_with_risk. WP-001 control-spine evidence is recorded; later
implementation, generated-artifact, and public-claim evidence remains deferred
to WP-002 through WP-005.

### WP-002: Bundle-First Identity Governance

Objective: keep segment-bearing implementation slices from drifting away from
stable bundle/member/stitch identity.

Parent requirement IDs: REQ-004, REQ-005

Parent specification IDs: SPEC-004, SPEC-005, SPEC-NF-003

Design/interface/code-rigor IDs: IF-002, CR-001, CR-002, CR-004, CR-005,
CR-007, CR-009

Validation scenario IDs: OPS-002

Affected files/modules: `crates/route-network/`, bundle schemas,
segment-bearing generated artifacts, `docs/route-architecture.md`,
`docs/bundle-registry-spec.md`, `docs/national-segment-identity-spec.md`

Entry criteria:

- Work package identifies whether the change is schema, crate logic, generated
  artifact, or documentation.
- Existing stable-key surfaces are named before implementation.

Exit criteria:

- New or changed segment-bearing rows use stable identity or carry an explicit
  transitional hold.
- Mutable labels, tiers, map ids, and zones are not sole primary keys.
- Review record names any rejected or held identity alternatives.

Validation levels:

| Level | Required | Commands / Evidence | Result |
|---|---|---|---|
| L0 | yes | targeted `cargo test -q -p route-network` or crate-specific equivalent | passed |
| L1 | yes | architecture/data inspection of changed identity surfaces | passed |
| L2 | conditional | downstream artifact inspection when generated maps/reports consume the identity | not_required; no generated report/map identity claim promoted |

V closure required at package closeout:

| V Area | Required IDs / Evidence | Closeout Rule |
|---|---|---|
| Need / CONOPS | NEED-004 / OPS-002 | Confirm the identity work still serves bundle-first continuity. |
| Requirements | REQ-004 / REQ-005 | Confirm both requirements are satisfied or explicitly held. |
| Specification | SPEC-004 / SPEC-005 / SPEC-NF-003 | Confirm stable-key and mutable-label constraints are preserved. |
| Design / Code Rigor | CR-001 / CR-002 / CR-004 / CR-005 / CR-007 / CR-009 | Record code-rigor evidence or waivers. |
| Implementation | changed identity surfaces | List changed files, schemas, generated artifacts, and held alternatives. |
| Verification / Evidence | EVID-004 / EVID-005 / EVID-CR-* | Replace deferred rows with actual results. |
| Validation / Review | VAL-002 / EVID-008 | Record required role-review decision and any dissent. |
| Gate | REVIEW.md or dated review record | Close as pass, pass_with_risk, blocked, deferred, or not_required. |

Status: closed

### WP-003: Stop-First SLA/Map Closure

Objective: make stop/SLA/map slices procedural rather than random by requiring
entry criteria, selected checks, role lanes, and evidence before claim
promotion.

Parent requirement IDs: REQ-006, REQ-007

Parent specification IDs: SPEC-006, SPEC-007, SPEC-NF-004, SPEC-013

Design/interface/code-rigor IDs: IF-001, IF-003, CR-001, CR-002, CR-003,
CR-004, CR-005, CR-008, CR-010

Validation scenario IDs: OPS-003

Affected files/modules: `crates/route-cli/`, `crates/route-map/`, route data,
stop/SLA diagnostics, generated map/SLA artifacts, `docs/beck-renderer-contract.md`,
`docs/sla-promise-portfolio.md`

Entry criteria:

- Work package names the stop/service/service-class/geometry/SLA surface.
- Expected generated artifacts and command gates are named before editing.
- Required role lanes are selected from `CODE_RIGOR.md`.

Exit criteria:

- Oversized stop gaps, endpoint/contact policy defects, and map/SLA mismatches
  are checked.
- False stops, false transfers, and untracked service promises are absent or
  explicitly held.
- `VERIFICATION.md`, `EVIDENCE.md`, and `REVIEW.md` record command and role
  results.

Validation levels:

| Level | Required | Commands / Evidence | Result |
|---|---|---|---|
| L0 | yes | targeted Rust tests for touched crate/package | passed via Rust e2e portion of `npm run check:l2` |
| L1 | yes | generated CSV/diagnostic or command-gate inspection | pass_with_risk |
| L2 | conditional | `npm run check:l2` before release, public claim, or downstream generated-artifact use | pass_with_risk; Rust e2e passed, browser Playwright tooling blocked |

V closure required at package closeout:

| V Area | Required IDs / Evidence | Closeout Rule |
|---|---|---|
| Need / CONOPS | NEED-006 / OPS-003 | Confirm the slice supports stop-first SLA operation. |
| Requirements | REQ-006 / REQ-007 | Confirm traceability and gates are satisfied or explicitly held. |
| Specification | SPEC-006 / SPEC-007 / SPEC-NF-004 / SPEC-013 | Confirm stop/SLA/map truth and selected command evidence. |
| Design / Code Rigor | CR-001 / CR-002 / CR-003 / CR-004 / CR-005 / CR-008 / CR-010 | Record code-rigor evidence or waivers. |
| Implementation | changed stop/SLA/map surfaces | List changed files, generated artifacts, diagnostics, and holds. |
| Verification / Evidence | EVID-006 / EVID-007 / EVID-CR-* | Replace deferred rows with actual command/artifact results. |
| Validation / Review | VAL-003 / EVID-008 | Record schematic, traffic, transit, and conditional stakeholder lanes. |
| Gate | REVIEW.md or dated review record | Close as pass, pass_with_risk, blocked, deferred, or not_required. |

Status: closed_with_risk

### WP-004: Role Review And Claim Promotion

Objective: make `.roles` review executable and auditable before claims are
promoted, held, downgraded, or used downstream.

Parent requirement IDs: REQ-002, REQ-003, REQ-008, REQ-009, REQ-010

Parent specification IDs: SPEC-002, SPEC-003, SPEC-008, SPEC-009, SPEC-010,
SPEC-NF-002, SPEC-NF-005

Design/interface/code-rigor IDs: IF-004, IF-005, CR-008, CR-010

Validation scenario IDs: OPS-004

Affected files/modules: `.roles/`, `reviews/`, `docs/reviews/`, public docs,
release notes, generated reports

Entry criteria:

- Claim or artifact status change is named.
- Applicable parliament, stakeholder, editorial, and panel lanes are selected
  or explicitly marked not required.

Exit criteria:

- Review record includes selected lanes, dissent or incompatible stakes, claim
  effect, label effect, required artifact changes, and next evidence step.
- Editorial gates run after substance review when `validated` status is sought.

Validation levels:

| Level | Required | Commands / Evidence | Result |
|---|---|---|---|
| L0 | yes | docs/review inspection | passed |
| L1 | yes | role-review matrix completed in `REVIEW.md` or a dated review record | pass_with_risk |
| L2 | conditional | release/review gate before public claim, publication, or downstream use | not_required; no public claim promoted |

V closure required at package closeout:

| V Area | Required IDs / Evidence | Closeout Rule |
|---|---|---|
| Need / CONOPS | NEED-003 / NEED-005 / OPS-004 | Confirm claim use matches mission scope and affected stakeholders. |
| Requirements | REQ-002 / REQ-003 / REQ-008 / REQ-009 / REQ-010 | Confirm evidence labels, holds, roles, and public framing are closed. |
| Specification | SPEC-002 / SPEC-003 / SPEC-008 / SPEC-009 / SPEC-010 / SPEC-NF-002 / SPEC-NF-005 | Confirm review usefulness and evidence honesty. |
| Design / Code Rigor | CR-008 / CR-010 | Record claim/evidence review evidence or waivers. |
| Implementation | changed claim/review/public surfaces | List changed docs, reviews, reports, labels, and next evidence steps. |
| Verification / Evidence | EVID-002 / EVID-003 / EVID-008 / EVID-009 / EVID-010 / EVID-CR-002 | Replace deferred rows with actual review results. |
| Validation / Review | VAL-004 | Record role-review decision and editorial gate result. |
| Gate | REVIEW.md or dated review record | Close as pass, pass_with_risk, blocked, deferred, or not_required. |

Status: closed_with_risk

### WP-005: Verification, Evidence, And V Closure

Objective: close the V for each selected package by recording the commands,
evidence, validation impact, review decisions, and residual risks.

Parent requirement IDs: REQ-001 through REQ-011

Parent specification IDs: SPEC-001 through SPEC-013

Design/interface/code-rigor IDs: CR-001 through CR-010

Validation scenario IDs: OPS-001 through OPS-005

Affected files/modules: selected work-package surfaces, `VERIFICATION.md`,
`VALIDATION.md`, `EVIDENCE.md`, `REVIEW.md`

Entry criteria:

- A concrete work slice names parent `REQ-*`, `SPEC-*`, `CR-*`, and role lanes.
- L0/L1/L2 requirements are selected or explicitly not required.

Exit criteria:

- Every changed implementation surface has evidence.
- Every accepted risk has owner and revisit trigger.
- The V closure rows are complete or marked not applicable with rationale.

Validation levels:

| Level | Required | Commands / Evidence | Result |
|---|---|---|---|
| L0 | yes | selected fast local checks | passed |
| L1 | yes | repo-confidence checks appropriate to touched surfaces | pass_with_risk |
| L2 | conditional | integration/readiness checks before merge, release, or public claim | pass_with_risk; Rust e2e passed, browser Playwright tooling blocked |

V closure required at package closeout:

| V Area | Required IDs / Evidence | Closeout Rule |
|---|---|---|
| Need / CONOPS | OPS-001 through OPS-005 | Confirm selected scenario coverage. |
| Requirements | selected `REQ-*` rows | Confirm each selected requirement has actual evidence or an accepted deferral. |
| Specification | selected `SPEC-*` rows | Confirm selected specs map to verification and validation evidence. |
| Design / Code Rigor | selected `CR-*` rows | Confirm selected code-rigor rows are satisfied or waived. |
| Implementation | selected work-package surfaces | List exact files, commands, artifacts, and generated outputs. |
| Verification / Evidence | selected `VER-*` / `EVID-*` rows | Replace deferred rows with actual results. |
| Validation / Review | selected `VAL-*` / role lanes | Record scenario validation and review outcomes. |
| Gate | REVIEW.md or dated review record | Close as pass, pass_with_risk, blocked, deferred, or not_required. |

Status: closed_with_risk

## Orphan Check

Before implementation starts, confirm:

- [x] Every accepted `REQ-*` is assigned to a work package or dispositioned.
- [x] Every accepted `SPEC-*` is assigned to a work package, verification item, or dispositioned.
- [x] Every interface-changing work package names `IF-*` IDs.
- [x] Every package/crate/module-changing work package names package boundaries in `SPECIFICATION_BASELINE.md`.
- [x] Every critical-code work package names `CR-*` IDs.
- [x] Every work package has exit criteria and verification commands or evidence expectations.
- [x] Every work package lists L0/L1/L2 requirements or explicit non-requirement.
- [x] Every work package has V closure rows or explicit package-closeout closure rules.
- [x] Every required assurance/security review lane is named or accepted with rationale.
- [x] No work package is only cleanup without parent IDs or discovery status.
