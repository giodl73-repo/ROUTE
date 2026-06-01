# Review Checklists

## Scope

Repo: ROUTE

VTRACE adoption scope: operational checklists for specification review, design
review, implementation readiness, work-package closeout, test readiness, and
release/transition readiness.

## Checklist

| Gate | Item | Required | Decision | Evidence / Notes |
|---|---|---|---|---|
| Specification Review | Mission, CONOPS, requirements, specs, unknowns, and validation path are clear. | yes | pass_with_risk | `MISSION.md`, `CONOPS.md`, `REQUIREMENTS.md`, `SPECIFICATION_BASELINE.md`; evidence recorded with browser tooling risk. |
| Trace Review | Every accepted requirement maps to parent needs/scenarios, specs, work packages, verification, and evidence IDs. | yes | pass_with_risk | `TRACE.md`; work packages and evidence IDs are closed or accepted with risk. |
| Design Review | Architecture, package boundaries, interfaces, and code-rigor constraints are acceptable. | yes | pass_with_risk | `SPECIFICATION_BASELINE.md`, `CODE_RIGOR.md`; no product code diff was made in this pass. |
| Implementation Readiness Review | Work packages have parent IDs, entry/exit criteria, profiles, and L0/L1/L2 checks. | yes | pass | `WORK_PACKAGES.md`; WP-001 through WP-005 were executed or accepted with risk. |
| Role Review Readiness | Required role lanes are selected by work type and review records have a required schema. | yes | pass_with_risk | `REVIEW.md`, `CODE_RIGOR.md`, `.roles/ROLE.md`. |
| Test Readiness Review | Procedures, fixtures, expected results, and environment assumptions are ready. | conditional | pass_with_risk | `VERIFICATION.md`; concrete fixture selection deferred to selected work package. |
| Work Package Close Review | Implementation surfaces, trace rows, evidence, validation impact, and role lanes are closed. | yes before closure | pass_with_risk | WP-001 through WP-005 are closed or accepted with risk; browser Playwright tooling remains blocked. |
| Security/Privacy Review | Unsafe, FFI, network, filesystem, shell, credential, and source-ingestion changes are reviewed. | conditional | not_required | No current docs-only change triggers this lane. |
| Source Custody Review | External source claims, citations, caches, and generated evidence can be traced. | conditional | pass_with_risk | Required for data/source ingestion or quantitative public claims. |
| Release/Transition Readiness Review | Evidence supports the readiness claim and open risks are accepted or blocked. | yes before release/public claim | not_required | No release, construction-readiness, or public proof claim is promoted by this docs-only VTRACE pass. |

## Work Package Close Checklist

- [x] Parent `REQ-*`, `SPEC-*`, `CR-*`, and `IF-*` IDs are named.
- [x] Entry criteria were met before implementation started.
- [x] Exit criteria are satisfied or explicitly blocked/deferred.
- [x] L0 checks passed or have accepted waivers.
- [x] L1 checks passed or have accepted waivers.
- [x] L2 checks passed when release, public claim, downstream generated artifact, or integration readiness is affected, or the browser-tooling blocker is explicitly accepted with risk.
- [x] Required `.roles` lanes are recorded.
- [x] Review changed a claim, label, docket, artifact, code surface, command, or next evidence step when it found a gap.
- [x] Evidence rows record actual command/review results.
- [x] Git status and commit scope were inspected before commit/push.

## Role Review Checklist

- [ ] Selected roles match the work-type triggers in `REVIEW.md`.
- [ ] Substance roles run before editorial form gates when `validated` status is sought.
- [ ] Dissent or incompatible stakes are preserved.
- [ ] Rejected or held alternatives are recorded when optimization, identity, map, or SLA decisions are made.
- [ ] Stakeholder lanes are not treated as generic approval; each lane states the concern it governs.
- [ ] Editorial lanes check scope, citations, and numeracy only.

## Gate

Decision: pass_with_risk

Rationale: the checklists are executable and WP-001 through WP-005 have closeout
evidence. The remaining accepted risk is local browser Playwright tooling for
L2 browser/game validation; release readiness is not required because this pass
does not promote a release or public proof claim.
