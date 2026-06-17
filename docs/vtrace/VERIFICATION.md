# Verification Plan

## Scope

Repo: ROUTE

VTRACE adoption scope: define verification methods and command levels for
ROUTE work packages. Command evidence is not closed until `EVIDENCE.md` records
actual results.

## Verification Matrix

| Requirement ID | Method | Command / Inspection | Expected Evidence | Result | Evidence Pointer |
|---|---|---|---|---|---|
| REQ-001 | inspection / demonstration | inspect `package.json`, `docs/SYSTEM_PLAN.md`, `GOAL.md`, generated-artifact ledgers | regeneration path and selected command bundle are named | passed | EVID-001 |
| REQ-002 | inspection / review | inspect claim labels in docs, ledgers, reviews, generated artifacts | material claims have evidence posture labels | pass_with_risk | EVID-002 |
| REQ-003 | inspection / review | inspect ledgers, closeouts, release manifests, review records | source gaps, holds, blockers, and next evidence steps remain visible | pass_with_risk | EVID-003 |
| REQ-004 | architecture inspection / test | inspect `docs/route-architecture.md`, `route-network`, segment-bearing schemas | stable bundle/member/stitch identity is preserved | passed | EVID-004 |
| REQ-005 | architecture inspection / data inspection | inspect segment-bearing rows and schema joins | mutable labels are not sole primary keys or are explicitly held | passed | EVID-005 |
| REQ-006 | command gate / artifact inspection | selected stop/SLA/map command gate and artifact inspection | stops, services, classes, geometry, and SLA promises are synchronized | pass_with_risk | EVID-006 |
| REQ-007 | command gate / test | selected diagnostics, tests, and `npm run check:l2` when required | oversized gaps, endpoint/contact defects, and map/SLA mismatches are blocked or held | pass_with_risk | EVID-007 |
| REQ-008 | review inspection | `REVIEW.md` or dated review record | claims affected by work package have substance review and action record | pass_with_risk | EVID-008 |
| REQ-009 | role review / artifact inspection | required stakeholder lanes from `CODE_RIGOR.md` | delivery, freight, rural/ag, non-driving, and environmental/community-health concerns are represented when applicable | pass_with_risk | EVID-009 |
| REQ-010 | editorial review | inspect public claims and editorial review | outputs remain framed as research/tooling/review/design analysis | pass_with_risk | EVID-010 |
| REQ-011 | git inspection | `git status --short`, TRACKER submodule diff when pointer update is requested | ROUTE child changes and TRACKER pointer changes are separated | passed | EVID-011 |
| REQ-CODE-001 | static analysis / inspection | code-rigor checks selected from `CODE_RIGOR.md` | critical code constraints have evidence or waivers | pass_with_risk | EVID-CR-001 / EVID-CR-002 / EVID-CR-003 / EVID-CR-004 / EVID-CR-005 |

## Commands

```powershell
git diff --check -- docs\vtrace
cargo fmt --check
cargo test -q --workspace --lib --bins
cargo test -q
npm run check:l2
git status --short
```

## Validation Levels

| Level | Purpose | Commands / Evidence | Result |
|---|---|---|---|
| L0 | Fast local sanity for active work package. | docs-only: `git diff --check -- docs\vtrace`; Rust/package work: targeted `cargo test -q -p <crate>` or `cargo test -q --workspace --lib --bins`; review work: role matrix inspection | passed |
| L1 | Full repo confidence before push or PR. | `cargo test -q`; VTRACE artifact inspection; role-review evidence for claim changes | pass_with_risk |
| L2 | Integration/readiness proof before merge, release, or public claim. | `npm run check:l2`; release/review gate; generated artifact inspection | pass_with_risk |

## Round 5 Communications Readiness Gate

This gate is required before the communications package can claim an internal
FHWA/USDOT-style technical review pass. It does not replace L2 for browser,
game, release, or public-readiness claims.

| Gate Item | Required Artifact / Check | Current Result |
|---|---|---|
| Claim-promotion trace exists. | `docs/traces/route-claim-promotion-trace.md` | draft |
| Demo command capture exists. | `docs/evidence/round5-demo-capture.md` | draft; command bundle and 225-mile before/after fixture captured |
| Source-pack templates exist. | `docs/templates/source-packs/` | draft |
| Stakeholder fixture template exists. | `docs/templates/source-packs/stakeholder-fixture-source-pack-template.md` | draft; real populated fixture pending |
| Stakeholder fixture closeout runbook exists. | `docs/how-to/stakeholder-fixture-closeout-runbook.md` | draft; makes remaining blocker executable |
| Pass-artifacts role review exists. | `docs/reviews/communications-role-review-pass-artifacts.md` | draft; pass_with_risk for internal rehearsal |
| Second pressure-test run exists. | `docs/reviews/communications-pressure-test-run-002.md` | draft; internal five-round pass_with_risk; external/public readiness held |
| External rehearsal readiness checklist exists. | `docs/reviews/communications-external-rehearsal-readiness.md` | draft; hold_external_rehearsal |
| External rehearsal packet template exists. | `docs/templates/external-rehearsal-packet-template.md` | draft; venue-specific packet pending |
| Crate coverage audit exists. | `docs/reviews/communications-crate-coverage-audit.md` | draft; partial_coverage and appendix backlog identified |
| Map caption pattern exists. | `docs/decks/split-deck-presenter-guide.md` | draft |
| Technical non-claims block exists. | `docs/decks/split-deck-presenter-guide.md` | draft |
| Prohibited-claim scan passes. | Search for official-plan, construction-ready, guaranteed-SLA, positive ROI, eligibility, compliance, endorsement claims in promoted contexts. | required at closeout |
| L0 command gate passes. | `npm run check:l0` | required at closeout |

Decision: pass_with_risk for internal Round 5 rehearsal.

Rationale: The missing artifact classes identified by the pressure-test run now
exist as draft surfaces, the command bundle plus 225-mile threshold fixture are
captured, `.roles` pass-artifacts review is recorded, and the stakeholder
fixture closeout runbook exists. External/public readiness remains held until a
populated source-backed stakeholder fixture, L1/L2 readiness evidence, and any
claim-specific source packs close.

## Evidence Ledger

| Evidence ID | Type | Path / URL / Command | Covers | Result |
|---|---|---|---|---|
| EVID-001 | inspection / command | `package.json`, `docs/SYSTEM_PLAN.md`, `GOAL.md` | REQ-001 / SPEC-001 / SPEC-007 / SPEC-013 | passed |
| EVID-002 | inspection | `docs/SPEC_INDEX.md`, ledgers, reviews, generated artifacts | REQ-002 / SPEC-002 / SPEC-010 | pass_with_risk |
| EVID-003 | inspection | ledgers, closeouts, release manifests, review records | REQ-003 / SPEC-003 | pass_with_risk |
| EVID-004 | inspection / test | `docs/route-architecture.md`, `crates/route-network/` | REQ-004 / SPEC-004 / SPEC-005 | passed |
| EVID-005 | inspection | segment-bearing schemas and rows | REQ-005 / SPEC-NF-003 | passed |
| EVID-006 | command / artifact | selected stop/SLA/map gate | REQ-006 / SPEC-006 / SPEC-013 | pass_with_risk |
| EVID-007 | command / test | selected diagnostics and `npm run check:l2` when required | REQ-007 / SPEC-007 / SPEC-NF-004 | pass_with_risk |
| EVID-008 | review | `REVIEW.md` or dated review record | REQ-008 / SPEC-008 / SPEC-NF-005 | pass_with_risk |
| EVID-009 | review | selected `.roles` lanes | REQ-009 / SPEC-009 | pass_with_risk |
| EVID-010 | review | public claim/editorial review | REQ-010 / SPEC-010 | pass_with_risk |
| EVID-011 | git inspection | `git status --short`, TRACKER submodule diff if applicable | REQ-011 / SPEC-011 | passed |
| EVID-012 | inspection | `docs/vtrace/*` | SPEC-012 | passed |
| EVID-013 | inspection | verification/evidence rows | SPEC-013 | pass_with_risk |

## Code Rigor Verification

| Constraint ID | Method | Command / Review | Result | Evidence Pointer |
|---|---|---|---|---|
| CR-001 | size/complexity inspection | affected Rust diff review | passed | EVID-CR-001 |
| CR-002 | control-flow inspection and tests | affected package tests and design review | passed | EVID-CR-001 |
| CR-003 | interface/error-path tests | CLI/schema/file reader tests | pass_with_risk | EVID-CR-002 |
| CR-004 | invariant tests or evidence | targeted tests/gates for identity and service truth | passed | EVID-CR-001 |
| CR-005 | formatter/test/static checks | `cargo fmt --check`, `cargo test`, selected npm scripts | pass_with_risk | EVID-CR-003 |
| CR-006 | security inspection | unsafe/FFI/network/filesystem/shell review | passed | EVID-CR-004 |
| CR-007 | termination/bounds review | graph/parser/simulation inspection and tests | passed | EVID-CR-001 |
| CR-008 | generated-artifact review | command evidence and artifact diff inspection | pass_with_risk | EVID-CR-002 |
| CR-009 | package-boundary review | affected crate responsibility inspection | passed | EVID-CR-005 |
| CR-010 | claim-promotion review | role review and evidence posture inspection | pass_with_risk | EVID-CR-002 |

## Gaps

| Gap | Impact | Disposition |
|---|---|---|
| Browser half of L2 is blocked by missing/mismatched Playwright CLI. | Release/readiness browser claims cannot close without restoring local Playwright tooling. | Accepted with risk for VTRACE docs; repair before public browser/game/map release claim. |
| Full implementation validation is evidence-based and package-specific. | Future code/data changes still need package-specific evidence before `validated` status. | Require selected `WP-*` closeout before new implementation claims. |
| Existing unrelated worktree changes may affect full-repo validation. | VTRACE doc evidence could be confused with unrelated implementation work. | Scope evidence rows to selected files and record git status. |
