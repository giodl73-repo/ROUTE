# Trace Matrix

## Scope

Repo: ROUTE

VTRACE adoption scope: connect the accepted ROUTE VTRACE requirements to
mission needs, CONOPS scenarios, controlled specification items, design
surfaces, work packages, verification, validation, evidence, and review gates.
This matrix is the routing spine for implementation and closeout.

## Requirement Trace

| Requirement ID | Parent Need | Requirement | Specification Item | Design Element | Code Rigor Constraint | Work Package | Implementation Surface | Verification Method | Validation Method | Evidence Pointer | Status |
|---|---|---|---|---|---|---|---|---|---|---|---|
| REQ-001 | NEED-001 / CON-003 / OPS-001 | Maintain a documented regeneration path for active score, map, SLA, gate, and evidence artifacts. | SPEC-001 / SPEC-007 / SPEC-012 / SPEC-013 | `docs/SYSTEM_PLAN.md`, `GOAL.md`, `package.json`, release docs, command ledgers | CR-005 / CR-008 / CR-010 | WP-001 / WP-005 | scripts, `route-cli`, generated score/map/SLA/gate/evidence artifacts | VER-001 / inspection / command review | VAL-001 / OPS-001 | EVID-001 | accepted |
| REQ-002 | NEED-002 / NEED-003 / CON-001 / CON-004 / OPS-001 / OPS-004 | Label material claims with evidence posture. | SPEC-002 / SPEC-003 / SPEC-010 / SPEC-012 / SPEC-013 | `docs/SYSTEM_PLAN.md`, `docs/SPEC_INDEX.md`, ledgers, closeouts, release manifests, reviews | CR-008 / CR-010 | WP-001 / WP-004 / WP-005 | claim text, generated artifacts, review records | VER-002 / artifact inspection / review | VAL-001 / VAL-004 / OPS-001 / OPS-004 | EVID-002 | accepted |
| REQ-003 | NEED-002 / CON-004 / OPS-001 / OPS-004 | Keep source gaps, heuristic assumptions, simulated evidence, and owner/human review holds visible. | SPEC-003 / SPEC-012 / SPEC-013 | ledgers, closeouts, release manifests, review dockets | CR-008 / CR-010 | WP-001 / WP-004 / WP-005 | evidence ledgers, hold records, review notes | VER-003 / ledger inspection / review | VAL-001 / VAL-004 / OPS-001 / OPS-004 | EVID-003 | accepted |
| REQ-004 | NEED-004 / CON-002 / OPS-002 | Preserve bundle-first identity for segment-bearing artifacts. | SPEC-004 / SPEC-005 / SPEC-012 | `docs/route-architecture.md`, `route-network`, bundle registry semantics | CR-001 / CR-002 / CR-004 / CR-005 / CR-007 / CR-009 | WP-002 / WP-005 | segment-bearing schemas, bundle/member registries, generated network artifacts | VER-004 / architecture gate / artifact inspection | VAL-002 / OPS-002 | EVID-004 | accepted |
| REQ-005 | NEED-004 / CON-002 / OPS-002 | Reject or hold segment-bearing artifacts that rely only on mutable labels as primary identity. | SPEC-004 / SPEC-005 / SPEC-NF-003 / SPEC-012 | `docs/route-architecture.md`, `route-network`, schema review gates | CR-004 / CR-005 / CR-009 | WP-002 / WP-005 | route labels, tiers, map ids, zones, segment rows | VER-005 / architecture gate / data inspection | VAL-002 / OPS-002 | EVID-005 | accepted |
| REQ-006 | NEED-006 / CON-002 / CON-003 / OPS-003 | Keep stop-first SLA work traceable across visible stops, route services, service classes, schematic geometry, and SLA promises. | SPEC-006 / SPEC-007 / SPEC-012 / SPEC-013 | `GOAL.md`, stop/SLA sources, service standards, T2 diagnostics, route map surfaces | CR-001 / CR-002 / CR-003 / CR-004 / CR-005 / CR-008 / CR-010 | WP-003 / WP-005 | route data, `route-cli`, `route-map`, generated diagnostics and SLA artifacts | VER-006 / command gate / artifact inspection | VAL-003 / OPS-003 | EVID-006 | accepted |
| REQ-007 | NEED-006 / CON-001 / CON-004 / OPS-003 | Gate oversized stop gaps, endpoint/contact policy defects, and map/SLA mismatches before release-ready treatment. | SPEC-006 / SPEC-007 / SPEC-NF-004 / SPEC-012 / SPEC-013 | stop gap diagnostics, endpoint/contact checks, map/SLA consistency gates | CR-003 / CR-004 / CR-005 / CR-008 / CR-010 | WP-003 / WP-005 | `route-cli`, `route-map`, generated validation artifacts | VER-007 / command gate / test | VAL-003 / OPS-003 | EVID-007 | accepted |
| REQ-008 | NEED-005 / CON-001 / OPS-004 | Expose transportation tradeoffs through parliament, stakeholder, editorial, or panel review records when claims change or are used downstream. | SPEC-008 / SPEC-NF-005 / SPEC-012 | `.roles/ROLE.md`, `.roles/parliament/`, `.roles/stakeholders/`, `.roles/editorial/`, `.roles/panel-reviewer/` | CR-008 / CR-010 | WP-004 / WP-005 | review records, dockets, claim labels, next-evidence notes | VER-008 / review inspection | VAL-004 / OPS-004 | EVID-008 | accepted |
| REQ-009 | NEED-005 / CON-001 / CON-006 / OPS-004 | Represent delivery feasibility, freight, rural/agricultural access, non-driving access, and environmental/community-health concerns before design options are promoted. | SPEC-008 / SPEC-009 / SPEC-012 | `.roles/stakeholders/`, `.roles/parliament/`, review records, design-claim promotion gates | CR-010 | WP-004 / WP-005 | stakeholder reviews, promotion notes, claim labels | VER-009 / role review / artifact inspection | VAL-004 / OPS-004 | EVID-009 | accepted |
| REQ-010 | NEED-003 / CON-006 / OPS-004 | Keep Interstate 2.0 outputs framed as research, tooling, review, and design analysis rather than construction readiness, compliance, or official endorsement. | SPEC-010 / SPEC-NF-002 / SPEC-012 | `README.md`, release docs, public claim text, editorial reviews | CR-010 | WP-004 / WP-005 | public docs, generated reports, review outputs | VER-010 / editorial review | VAL-004 / OPS-004 | EVID-010 | accepted |
| REQ-011 | CON-005 / OPS-005 | Keep VTRACE adoption changes scoped to ROUTE child-repo artifacts until intentional TRACKER pointer update is requested. | SPEC-011 / SPEC-NF-006 / SPEC-012 | ROUTE child repo, TRACKER submodule pointer, git workflow | CR-005 | WP-001 / WP-005 | child repo commits, TRACKER submodule diff | VER-011 / git status / submodule diff inspection | VAL-005 / OPS-005 | EVID-011 | accepted |

## Cross-Stage Coverage

| Source Stage | IDs Covered | Downstream Stage | Coverage Status | Notes |
|---|---|---|---|---|
| Mission needs | NEED-001 through NEED-006 | REQ-001 through REQ-010 | covered | Mission needs are represented by one or more requirements except portfolio isolation, which derives from `CON-005`. |
| Mission constraints | CON-001 through CON-006 | REQ-001 through REQ-011 | covered | Constraints are attached to requirements where they affect evidence posture, identity, scope, or repo operations. |
| CONOPS scenarios | OPS-001 through OPS-005 | REQ-001 through REQ-011 | covered | Every scenario drives at least one accepted requirement. |
| Requirements | REQ-001 through REQ-011 | SPEC-001 through SPEC-013 / SPEC-NF-001 through SPEC-NF-006 | covered | Specification coverage is accepted in `SPECIFICATION_BASELINE.md`. |
| Specifications | SPEC-001 through SPEC-013 | `VER-*` and `EVID-*` rows | covered_with_risk | Verification and evidence IDs now record command, inspection, and review outcomes; browser L2 tooling remains accepted risk. |
| Requirements and specs | REQ-* / SPEC-* | WP-001 through WP-005 | covered | Work packages now allocate execution, role review, evidence, and closeout responsibilities. |

## Unknowns Trace

| Unknown ID | Trace Disposition | Closure Path | Status |
|---|---|---|---|
| SPEC-UNK-001 | Exact L2 command selection is work-package-specific. | L2 was selected in `WORK_PACKAGES.md`; Rust e2e passed, while browser Playwright tooling is blocked and accepted with risk. | accepted risk |
| SPEC-UNK-002 | Actual command and review results were deferred until package execution. | `EVIDENCE.md`, `VERIFICATION.md`, `VALIDATION.md`, and `REVIEW.md` now record actual command/review outcomes for WP-001 through WP-005. | closed_with_risk |
| SPEC-UNK-003 | Dirty local worktree can affect validation scope. | `git status --short` is recorded during closeout and this pass remains scoped to ROUTE `docs/vtrace/`. | accepted risk |

## Trace Rules

- Stable IDs in `MISSION.md`, `CONOPS.md`, `REQUIREMENTS.md`, and
  `SPECIFICATION_BASELINE.md` must not be renamed after this file references
  them unless the rename is captured here and in downstream evidence.
- A work package must cite at least one `REQ-*` and one `SPEC-*` before
  implementation begins.
- An evidence row must cite at least one `VER-*`, the command or review
  that produced it, and the exact artifact inspected.
- A requirement may move from `accepted` to `implemented`, `verified`, or
  `validated` only when a downstream work package and evidence pointer exist.
- A requirement may be waived or retired only with a role review note and an
  explicit replacement, deferral, or non-applicability rationale.

## Trace Risks

| Risk ID | Risk | Impact | Disposition |
|---|---|---|---|
| TRACE-RISK-001 | Work packages were authored before execution. | Implementation closure could not be claimed until selected packages recorded command, review, and evidence results. | Closed with WP-001 through WP-005 closeout. |
| TRACE-RISK-002 | Evidence IDs existed before actual results. | Accepted requirements needed VTRACE evidence before verification/validation closure. | Closed or accepted with risk in `EVIDENCE.md`. |
| TRACE-RISK-003 | Code rigor constraints are selected but no product code diff was made in this pass. | Language/package-specific constraints cannot be marked satisfied for future code/data changes without package-specific evidence. | Accepted with risk; future implementation diffs must close through `CODE_RIGOR.md`, `VERIFICATION.md`, and `EVIDENCE.md`. |
| TRACE-RISK-004 | ROUTE has unrelated local worktree changes outside `docs/vtrace/`. | Full command validation could mix VTRACE documentation with unrelated local work. | Limit this stage to documentation trace inspection until validation scope is selected. |

## Role Review Notes

| Role Lens | Trace Impact | Disposition |
|---|---|---|
| Scope Keeper | Trace maps process and system-contract obligations without scoring corridors or proposing construction work. | pass |
| Citation Auditor | Trace introduces no new external factual claims and points to repo-local source artifacts. | pass |
| Numeracy Checker | Trace uses IDs and status values only; no quantitative claims or calculations are introduced. | pass |
| Optimization Methodologist | Trace preserves regeneration, identity, command gate, and evidence-posture obligations as future work-package gates. | pass |
| Schematic Cartographer | Trace keeps stop/SLA/map consistency tied to visible-stop and false-service checks. | pass |
| Traffic Engineer / Freight Economist / Rural Advocate | Trace keeps operational, freight, rural, and access lenses attached to claim promotion. | pass |
| State DOT / Transit-Dependent / Environmental Stakeholders | Trace keeps delivery feasibility, non-driving access, and community/environmental review visible before design promotion. | pass |

## Trace Gate

Decision: pass_with_risk

Required before implementation execution:

- [x] Every accepted `REQ-*` maps to mission, constraint, or CONOPS parentage.
- [x] Every accepted `REQ-*` maps to one or more controlled specification items.
- [x] Every accepted `REQ-*` has a verification and validation method.
- [x] Every accepted `REQ-*` has an assigned work package.
- [x] Every accepted `REQ-*` has evidence pointer coverage.
- [x] Code rigor constraints are selected for affected language/package surfaces.
- [x] Actual command and review evidence is recorded for selected work packages or accepted with risk.

Rationale: the trace is strong enough to drive implementation packages and
review gates. WP-001 through WP-005 now have command, inspection, review, and
evidence outcomes. The remaining accepted risk is the local browser Playwright
tooling blocker for L2 browser/game validation and the need for future
package-specific evidence when product code or generated data changes.
