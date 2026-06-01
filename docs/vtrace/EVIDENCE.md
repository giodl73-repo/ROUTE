# Evidence Ledger

## Scope

Repo: ROUTE

VTRACE adoption scope: record command, inspection, review, git, and generated
artifact evidence for ROUTE VTRACE work packages. Pending rows are explicit
debt, not proof.

## Evidence Records

| Evidence ID | Type | Source / Command | Expected Result | Actual Result | Status | Owner | Revisit Trigger |
|---|---|---|---|---|---|---|---|
| EVID-001 | inspection / command | inspect `package.json`, `docs/SYSTEM_PLAN.md`, `GOAL.md` | regeneration paths and command candidates are identifiable | `package.json` defines `check:l0`, `check:l1`, `check:l2`, and `check:game-browser`; `GOAL.md` names active stop/SLA commands and target artifacts; `docs/SYSTEM_PLAN.md` names regenerated artifacts and definition of done. | passed | ROUTE maintainer | closed in WP-001 |
| EVID-002 | inspection | inspect claim labels in `docs/SPEC_INDEX.md`, ledgers, reviews, generated artifacts | material claims carry evidence posture | `docs/SYSTEM_PLAN.md` defines truth labels; `docs/SPEC_INDEX.md` includes claim status vocabulary and source/gate surfaces. Generated artifact claim labels remain package-specific. | pass_with_risk | ROUTE maintainer | revisit in WP-004 or claim-promotion package |
| EVID-003 | inspection | inspect ledgers, closeouts, release manifests, review records | source gaps, holds, blockers, and next evidence steps are visible | `docs/SPEC_INDEX.md` lists hold, blocker, source, gate, review, and generated-evidence surfaces; WP-004 remains responsible for closing claim-specific review rows. | pass_with_risk | ROUTE maintainer | revisit in WP-004 or evidence-closeout package |
| EVID-004 | inspection / test | inspect `docs/route-architecture.md`, `crates/route-network/`, bundle schemas | stable identity contract is preserved | `docs/route-architecture.md`, `docs/bundle-registry-spec.md`, and `docs/national-segment-identity-spec.md` define bundle/member/stitch identity and route-label limits; `cargo test -q -p route-network` passed 44 tests. | passed | route-network owner | closed in WP-002 |
| EVID-005 | inspection | inspect segment-bearing schemas and rows | mutable labels are not sole primary keys or are held | Architecture and identity specs explicitly forbid route labels, tiers, map ids, and zones as primary identity; bundle registry ambiguity rules require explicit disambiguation. | passed | route-network owner | closed in WP-002 |
| EVID-006 | command / artifact | selected stop/SLA/map gate | stops, services, service classes, geometry, and SLA promises agree | `GOAL.md`, `docs/beck-renderer-contract.md`, and `docs/sla-promise-portfolio.md` define stop/SLA/map truth rules; `npm run check:l2` passed Rust e2e CLI tests before the browser-tooling blocker. | pass_with_risk | route-cli / route-map owners | revisit if browser Playwright tooling is restored |
| EVID-007 | command / test | selected diagnostics and `npm run check:l2` when required | stop gaps, endpoint/contact defects, and map/SLA mismatches are blocked or held | `npm run check:l2` ran `cargo test -q -p route --test e2e_cli` successfully, then failed on `playwright test` because no local `node_modules\\.bin\\playwright` exists and the resolved Playwright command reported `unknown command 'test'`. | pass_with_risk | route-cli / route-map owners | install/repair Playwright CLI before release/public map claim |
| EVID-008 | review | `REVIEW.md` or dated review record | review changes claim, label, docket, artifact, or next evidence step when gaps are found | `REVIEW.md` defines executable role-review records and work-type triggers; this pass promotes no new transportation claim, so implementation-specific dissent remains deferred to future claim packages. | pass_with_risk | review steward | revisit when a work package promotes, downgrades, or publishes a claim |
| EVID-009 | review | selected `.roles` lanes | affected stakeholder concerns are represented before claim promotion | `.roles/ROLE.md` and `REVIEW.md` map required parliament, stakeholder, editorial, panel, assurance, security, and V&V lanes by trigger; no new design option is promoted in this docs-only execution. | pass_with_risk | review steward | revisit on claim promotion or public release package |
| EVID-010 | review | editorial/public-claim review | public outputs avoid construction, compliance, endorsement, or proof overclaims | `docs/vtrace/REVIEW.md` and `docs/vtrace/VALIDATION.md` preserve research/tooling/review/design-analysis framing; no public README/release claim was changed in this pass. | pass_with_risk | ROUTE maintainer / review steward | revisit before public claim or release package |
| EVID-011 | git inspection | `git status --short`; TRACKER submodule diff when requested | child repo changes and TRACKER pointer changes are separated | `git status --short` in ROUTE reports only `?? docs/vtrace/`; no TRACKER pointer update was requested or performed. | passed | ROUTE maintainer / portfolio maintainer | revisit before commit or pointer update |
| EVID-012 | inspection | inspect `docs/vtrace/*` | VTRACE package is internally consistent | `cargo run -- ..\..\applied-systems\route` from VTRACE reported `VTRACE validation passed`; `git diff --check -- docs\vtrace` passed in ROUTE. | passed | ROUTE maintainer | closed in WP-001 |
| EVID-013 | inspection | inspect `VERIFICATION.md`, `EVIDENCE.md`, work-package closeout | chosen commands and deferred gates are recorded | `VERIFICATION.md` defines L0/L1/L2 commands and conditional L2 policy; `EVIDENCE.md` records row-level owners and revisit triggers; implementation evidence remains deferred by package. | pass_with_risk | ROUTE maintainer | revisit in WP-005 closeout |
| EVID-CR-001 | code review | affected Rust diff inspection | size, complexity, invariant, and bounded-iteration constraints are satisfied or waived | No Rust implementation diff was introduced by this VTRACE pass; `cargo test -q -p route-network` passed 44 tests for the identity package surface. | passed | work-package owner | revisit on code diff |
| EVID-CR-002 | artifact/review inspection | generated artifact and claim inspection | error handling, generated evidence, and claim-promotion constraints are satisfied | Generated-artifact and claim-promotion constraints are encoded in `CODE_RIGOR.md`, `REVIEW.md`, and `EVIDENCE.md`; no generated artifact or public claim is promoted in this pass. | pass_with_risk | work-package owner / review steward | revisit on artifact or claim change |
| EVID-CR-003 | command | `cargo fmt --check`; `cargo test`; selected npm checks | formatter/tests/checks pass or waivers are recorded | `cargo fmt --check` passed; `cargo test -q -p route-network` passed; the Rust e2e half of `npm run check:l2` passed; browser Playwright tooling failed as recorded in EVID-007. | pass_with_risk | work-package owner | revisit when Playwright CLI is restored |
| EVID-CR-004 | security review | inspect unsafe/FFI/network/filesystem/shell behavior | no unapproved high-risk behavior is introduced | No code, network, credential, FFI, unsafe, shell, or source-ingestion change was introduced by this VTRACE documentation pass. | passed | security/privacy reviewer | revisit on security-triggering change |
| EVID-CR-005 | package-boundary review | inspect changed crate/package responsibilities | package ownership boundaries are preserved | `docs/route-architecture.md` and `docs/vtrace/SPECIFICATION_BASELINE.md` preserve crate ownership boundaries; no package-boundary implementation change was introduced. | passed | work-package owner | revisit on package-boundary change |

## Evidence Rules

- Evidence IDs are stable and referenced from `TRACE.md` and `VERIFICATION.md`.
- Command evidence records the exact command, working directory, result, and
  whether unrelated dirty worktree state was present.
- Review evidence points to `REVIEW.md`, a PR review, or a dated review note.
- Deferred evidence includes an owner and revisit trigger in the evidence row.
- Generated artifacts used as evidence must name the command that produced
  them and the artifact path inspected.
- A requirement cannot move to `verified` or `validated` until the cited
  evidence row has an actual result.

## Deferred Evidence Groups

| Evidence Group | Notes |
|---|---|
| Primary evidence rows | Close as work packages are executed. Row-level owners and triggers are listed in the evidence table. |
| Code-rigor evidence rows | Not required for docs-only changes unless a claim is promoted. Row-level owners and triggers are listed in the evidence table. |

## Gate

Decision: pass_with_risk

Rationale: WP-001 through WP-005 are closed or accepted with explicit risk.
The main residual risk is the L2 browser Playwright tooling blocker.
