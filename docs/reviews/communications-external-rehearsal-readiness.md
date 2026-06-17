---
name: ROUTE Communications External Rehearsal Readiness
slug: route-communications-external-rehearsal-readiness
type: review
status: draft
rubric_version: v1.0
author: codex
created: 2026-06-17
updated: 2026-06-17
sources:
  - docs/reviews/communications-pressure-test-run-002.md
  - docs/reviews/communications-role-review-pass-artifacts.md
  - docs/how-to/stakeholder-fixture-closeout-runbook.md
  - docs/templates/source-packs/stakeholder-fixture-source-pack-template.md
  - docs/templates/external-rehearsal-packet-template.md
  - docs/decks/split-deck-presenter-guide.md
  - docs/reports/route-evidence-posture.md
  - docs/vtrace/VERIFICATION.md
  - docs/vtrace/COMMUNICATIONS_STRATEGY.md
---

# ROUTE Communications External Rehearsal Readiness

## Scope

This checklist decides whether the current communications package can be used
in a named external rehearsal with a sponsor, state, regional peer group,
industry stakeholder, community stakeholder, congressional staff, FHWA/USDOT
technical reviewer, or similar audience.

This is not a publication gate and not an approval gate. Passing this checklist
does not create endorsement, official-plan status, construction readiness,
guaranteed service, numeric ROI, eligibility, compliance, public readiness, or
agency approval.

## Current Decision

Decision: **hold_external_rehearsal**

The package passes the internal five-round simulation with risk, but it should
not yet be used in a named external rehearsal because no populated
source-backed stakeholder fixture exists. The package has the process, template,
trace, evidence capture, and role-review infrastructure needed to prepare that
fixture. The external rehearsal packet template now defines where a future
reviewer records the named venue, selected materials, source-backed fixture,
role review, validation, and closing ask.

## Rehearsal Context Gate

| Item | Required Before External Rehearsal | Current Status |
|---|---|---|
| Named venue | Meeting body, sponsor, reviewer group, or stakeholder class is identified. | held |
| Audience lane | State / regional / freight / rural / community / congressional / FHWA/USDOT / funder lane selected. | held |
| Materials list | Deck, brief, report, runbook, trace, source pack, and evidence posture files selected. | held |
| Closing ask | Intake, evidence, source pack, demo fixture, standards, or bounded pilot ask selected. | held |
| Recorder | Person or role responsible for intake, dissent, and source custody named. | held |
| Rehearsal packet | `docs/templates/external-rehearsal-packet-template.md` copied or filled for the named venue. | template exists; packet held |

## Evidence Gate

| Item | Required Before External Rehearsal | Current Status |
|---|---|---|
| Populated stakeholder fixture | Source custody row, requirement row, before/after artifact or label change, and role-review result. | held |
| Source custody | Every concrete external example has owner, title, date/year, path/access note, units when needed, and reviewer. | held |
| Claim trace | Any concrete claim has a row in `docs/traces/route-claim-promotion-trace.md` or is explicitly out of scope. | pass_with_risk |
| Evidence posture | Every artifact shown has story-ready, implemented, heuristic, source-needed, gated, or held label. | pass_with_risk |
| Demo capture | Any command shown has command, output path, observed status, row count/size when applicable, and non-claim label. | pass_with_risk |

## Role Gate

| Role Lane | Required Before External Rehearsal | Current Status |
|---|---|---|
| Scope Keeper | Confirms the selected materials do not drift into approval, construction, or agency-readiness claims. | held for selected venue |
| Citation Auditor | Confirms source custody for every concrete example and quantitative field. | held for selected venue |
| Numeracy Checker | Confirms no uncited or unreviewed quantity, threshold, cost, benefit, ROI, volume, or service claim is promoted. | held for selected venue |
| Affected stakeholder lanes | Review the concrete fixture and any dissent, burden, or held claim. | held for selected venue |
| Optimization Methodologist | Confirms any before/after artifact change is reproducible and not hand-shaped. | held for selected fixture |
| Schematic Cartographer | Confirms any map is captioned by posture and does not imply proof. | pass_with_risk |

## Presenter Gate

| Item | Required Before External Rehearsal | Current Status |
|---|---|---|
| Opening posture | Presenter starts with "requirements before concrete" or equivalent bounded language. | pass |
| Map caption | Every map has level, claim label, excluded claims, and evidence pointer. | pass_with_risk |
| Forbidden phrases removed | Official plan, construction-ready, guaranteed service, positive ROI, stakeholder validated, map proves readiness, statutory compliance, agency endorsement. | pass_with_risk |
| Ask menu | Presenter closes with intake, evidence, source pack, demo fixture, standards, or bounded pilot. | pass_with_risk |
| Escalation path | Any pressure for construction, ROI, eligibility, compliance, SLA, or endorsement redirects to source-pack and role-review gates. | pass_with_risk |

## Validation Gate

| Item | Required Before External Rehearsal | Current Status |
|---|---|---|
| L0 | `npm run check:l0` passes after package edits. | required at closeout |
| Prohibited-claim scan | Scan selected materials for promoted prohibited claims. | required at closeout |
| L1 | Full repo or package-specific confidence evidence recorded if external rehearsal uses technical claims. | held |
| L2 | Required only for browser, game, release, publication, or public-readiness claims. | held / scoped out unless claimed |

## Pass Conditions

External rehearsal readiness can move from `hold_external_rehearsal` to
`pass_with_risk` only when:

1. A named venue and audience lane exist.
2. A populated stakeholder fixture or explicit source-backed concrete example
   exists.
3. Source custody and affected role review are recorded for that fixture.
4. The selected material set has a prohibited-claim scan.
5. L0 passes after edits.
6. The closing ask is intake, evidence, source pack, demo fixture, standards, or
   bounded pilot only.

## Non-Approved Claims

- The package has external endorsement.
- The package is public-ready.
- Any state, regional, congressional, FHWA, USDOT, industry, rural, local,
  transit, environmental, or community participant has approved the plan.
- Any map, fixture, or demo proves construction, SLA, ROI, eligibility,
  compliance, or official-plan readiness.

## Gate

Decision: **hold_external_rehearsal**

Rationale: The internal simulation package is disciplined, but an external
rehearsal needs a concrete venue, selected materials, populated source-backed
fixture, role review, and closeout validation. Until those exist, external use
would risk converting process readiness into implied stakeholder validation.
