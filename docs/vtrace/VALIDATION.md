# Validation Plan

## Scope

Repo: ROUTE

VTRACE adoption scope: define the right-side validation checks that prove the
implemented ROUTE work still satisfies the operating scenarios and stakeholder
intent from `MISSION.md` and `CONOPS.md`.

## Validation Matrix

| Validation ID | Scenario / Need | Method | Required Evidence | Applies To | Result |
|---|---|---|---|---|---|
| VAL-001 | OPS-001 / NEED-001 / NEED-002 | operator workflow review | regeneration path, evidence posture labels, source gaps, and deferred gates are visible to a maintainer | REQ-001 / REQ-002 / REQ-003 | pass_with_risk |
| VAL-002 | OPS-002 / NEED-004 | architecture/use-case review | segment-bearing artifacts preserve stable bundle/member/stitch identity across downstream use | REQ-004 / REQ-005 | passed |
| VAL-003 | OPS-003 / NEED-006 | stop-first SLA acceptance check | stops, services, service classes, schematic geometry, and SLA promises agree or carry explicit holds | REQ-006 / REQ-007 | pass_with_risk |
| VAL-004 | OPS-004 / NEED-003 / NEED-005 | role-review acceptance check | affected claims have substance review, stakeholder representation, editorial form checks, and evidence labels before promotion | REQ-002 / REQ-003 / REQ-008 / REQ-009 / REQ-010 | pass_with_risk |
| VAL-005 | OPS-005 / CON-005 | portfolio workflow review | ROUTE child repo work is separate from TRACKER pointer updates | REQ-011 | passed |

## Acceptance Rules

- Validation is scenario-based, not just command-based.
- A passing test can support validation but cannot replace stakeholder or
  operator review when the requirement is about claim use, public framing, or
  review governance.
- A validation row may pass with risk only when the accepted risk names an
  owner and revisit trigger in `REVIEW.md` or `EVIDENCE.md`.
- A requirement may be marked `validated` only after the corresponding `EVID-*`
  row records actual evidence.

## Stakeholder Validation Triggers

| Trigger | Required Validation Lanes | Evidence |
|---|---|---|
| Public or downstream claim promotion | affected parliament/stakeholder roles plus editorial form gates | EVID-008 / EVID-009 / EVID-010 |
| Stop/SLA/map artifact readiness | Schematic Cartographer, Traffic Engineer, Transit-Dependent Traveler | EVID-006 / EVID-007 / EVID-008 |
| Bundle/segment identity change | Optimization Methodologist, Traffic Engineer, Schematic Cartographer | EVID-004 / EVID-005 / EVID-008 |
| Delivery, funding, or agency-readiness language | State DOT Planner, Scope Keeper, Citation Auditor | EVID-009 / EVID-010 |
| Freight, rural, or environmental impact claim | Freight Economist or Freight Industry, Rural Advocate, Environmental Community as applicable | EVID-009 / EVID-010 |

## Validation Levels

| Level | Purpose | Evidence | Required When |
|---|---|---|---|
| L0 | confirms the artifact or code slice is locally coherent | selected tests, inspections, or docs diff check | every work package |
| L1 | confirms repo-level behavior and trace alignment | full repo checks appropriate to touched surfaces plus VTRACE inspection | before push/PR or work-package close |
| L2 | confirms integration/readiness for downstream or public use | `npm run check:l2`, release/review gate, generated artifact inspection, role review | before release, public claim, downstream generated-artifact use, or validated status |

## Validation Gaps

| Gap | Impact | Disposition |
|---|---|---|
| Browser Playwright tooling is blocked. | Browser/game release claims cannot be fully validated yet. | Accepted with risk for VTRACE docs; repair before public browser/game/map release claim. |
| Future implementation changes still need package-specific validation. | New code/data claims cannot inherit this docs package validation automatically. | Require selected work-package closeout before new implementation claims. |
| Stakeholder validation depends on claim type. | Generic stakeholder approval would be too vague to govern implementation. | Use trigger table above and `REVIEW.md` work-type role triggers. |

## Gate

Decision: pass_with_risk

Rationale: validation scenarios and stakeholder triggers are defined and closed
for this VTRACE documentation execution. Browser/game release validation remains
accepted with risk because Playwright tooling is blocked.
