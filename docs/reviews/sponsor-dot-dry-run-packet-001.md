---
name: Sponsor To DOT Dry Run Packet 001
slug: sponsor-dot-dry-run-packet-001
type: review
status: draft
rubric_version: v1.0
author: codex
created: 2026-06-17
updated: 2026-06-17
sources:
  - docs/templates/external-rehearsal-packet-template.md
  - docs/reviews/communications-external-rehearsal-readiness.md
  - docs/reviews/communications-rude-qa-drill-run-002.md
  - docs/reviews/communications-rude-qa-repair-closeout.md
  - docs/decks/split-deck-presenter-guide.md
  - docs/reports/route-evidence-posture.md
  - docs/reviews/source-backed-stakeholder-fixture-candidate-001.md
---

# Sponsor To DOT Dry Run Packet 001

## Purpose

This packet turns the current communications package into a named internal
dry-run scenario: a sponsor-facing meeting followed by FHWA/USDOT-style
technical questions.

It is not an external rehearsal. It does not create endorsement, official-plan
status, construction readiness, guaranteed service, numeric ROI, eligibility,
compliance, public-readiness, agency approval, or external-readiness.

## Rehearsal Metadata

| Field | Entry |
|---|---|
| Packet ID | SPONSOR-DOT-DRY-RUN-001 |
| Rehearsal date | not scheduled |
| Venue / body | internal sponsor-to-DOT technical dry run |
| Audience lane | sponsor / FHWA-USDOT |
| Presenter | ROUTE communications presenter |
| Recorder | review steward |
| Source custody owner | Citation Auditor |
| Closing ask | source pack plus demo fixture |
| Current gate | hold_external_rehearsal |

## Decision

Decision: **hold_external_rehearsal**

The rude Q&A package now passes an internal closed-book drill, and the presenter
guide contains the repaired answers. This packet is still held because the
source-backed stakeholder fixture is only a candidate shell, not a populated
fixture with source custody, before/after artifact change, and role review.

## Materials Selected

| Material | Path | Role In Rehearsal | Evidence Posture | Included? |
|---|---|---|---|---|
| Public solution deck | `docs/decks/interstate-2-0-pitch.md` | sponsor vision and service hierarchy | story-ready / gated claims | yes |
| ROUTE technology deck | `docs/decks/route-technology-story.md` | refinement engine and evidence mechanics | implemented / heuristic / held | yes |
| Presenter guide | `docs/decks/split-deck-presenter-guide.md` | talk track, repaired rude Q&A answers, red lines | guardrail | yes |
| Evidence posture | `docs/reports/route-evidence-posture.md` | claim boundary | pass_with_risk | yes |
| Rude Q&A run 002 | `docs/reviews/communications-rude-qa-drill-run-002.md` | internal hostile-question rehearsal result | internal pass / external held | yes |
| Claim trace | `docs/traces/route-claim-promotion-trace.md` | claim-to-evidence walkthrough | draft | yes |
| Demo capture | `docs/evidence/round5-demo-capture.md` | command evidence | pass_with_risk / internal | yes |
| Stakeholder fixture candidate | `docs/reviews/source-backed-stakeholder-fixture-candidate-001.md` | missing real requirement work item | held_template | yes |
| External readiness checklist | `docs/reviews/communications-external-rehearsal-readiness.md` | gate and holds | hold_external_rehearsal | yes |

## Safe Opening

Use this opening posture:

> This is an internal sponsor-to-DOT dry run. The purpose is to test whether
> the story, evidence posture, command capture, and rude Q&A answers stay
> bounded. It is requirements before concrete, not a project list or external
> endorsement request.

## Closing Ask

Close with one ask:

> Help us populate one source-backed stakeholder fixture: a real requirement,
> source custody, before/after artifact or label change, and role review.

Do not close on construction funding, map adoption, ROI, eligibility,
compliance, endorsement, approval, or public-readiness.

## Source-Backed Fixture Summary

| Field | Entry |
|---|---|
| Fixture source pack | held; see `docs/reviews/source-backed-stakeholder-fixture-candidate-001.md` |
| Stakeholder lane | sponsor-to-DOT candidate; exact affected lane pending source |
| Source-backed requirement | held; no real source selected |
| Source ID(s) | none yet |
| Before artifact / label | pending fixture selection |
| After artifact / label | pending fixture selection |
| Role-review result | pending |
| Claim allowed? | no; internal only once populated and reviewed |

Because this section is not populated, the packet remains
`hold_external_rehearsal`.

## Claim Trace Rows

| Claim / Message | Trace Row | Evidence Label | Source Status | Allowed In Rehearsal? | Required Wording |
|---|---|---|---|---|---|
| Interstate 2.0 is a service-network vision. | communications thesis / doctrine surfaces | story-ready | local docs | yes, internal | "vision" and "service hierarchy"; no official-plan language |
| ROUTE makes claims inspectable and refinable. | claim-promotion trace / evidence posture | implemented / pass_with_risk | local docs | yes, internal | "inspectable" and "evidence-bounded"; no proof claim |
| ROI is an evidence contract. | ROI/cost framework | gated | local docs | yes, internal | "numeric ROI held until source pack and review close" |
| Current demo shows internal command capture. | round5 demo capture | pass_with_risk / internal | local docs | yes, internal | "command capture is inspectable; L1/L2 release evidence remains held" |
| Real stakeholder requirement changes an artifact. | stakeholder fixture candidate | held_template | missing real source | no | Say the fixture is the next evidence ask, not completed evidence |

## Required Role Review

| Role Lane | Required? | Reviewer / Record | Result | Hold / Condition |
|---|---|---|---|---|
| Scope Keeper | yes | pending | hold | named packet needs final scope review |
| Citation Auditor | yes | pending | hold | source-backed fixture missing |
| Numeracy Checker | yes | pending | hold | ROI/cost/score language must remain bounded |
| Optimization Methodologist | if before/after fixture appears | pending | hold | fixture not populated |
| Schematic Cartographer | if map appears | presenter guide / maps report | pass_with_risk | map captions required |
| State DOT Planner | yes | pending | hold | sponsor-to-DOT lane needs state/DOT feasibility review |
| Freight Economist / Freight Industry | if freight example appears | pending | hold | freight example source not selected |
| Rural Advocate / rural stakeholder | if rural/access example appears | pending | hold | affected lane pending source |
| Foxx / Environmental Community | if community/environmental example appears | pending | hold | affected lane pending source |

## Presenter Controls

| Control | Required Wording / Evidence | Status |
|---|---|---|
| Opening posture | Requirements before concrete; internal dry run only. | pass |
| Map caption | Map level, claim label, excluded claims, evidence pointer. | pass_with_risk |
| ROI boundary | Evidence contract only; no numeric ROI. | pass |
| Score boundary | Score total is an index for review; dimensions, confidence, and sensitivity decide whether a claim can move. | pass |
| Reproducibility boundary | Current command capture can be inspected; L1/L2 release evidence remains held where relevant. | pass |
| Authority boundary | ROUTE does not replace state, regional, federal, public, or engineering review. | pass |
| Closing ask | Populate one source-backed stakeholder fixture. | pass |

## Validation Closeout

| Check | Command / Inspection | Result | Evidence |
|---|---|---|---|
| Prohibited-claim scan | scan selected packet files for promoted prohibited claims | required at closeout | pending |
| L0 | `npm run check:l0` | required at closeout | pending |
| L1 | package-specific or full repo confidence when technical claims are used | hold | not required for internal packet draft |
| L2 | browser/game/release/public-readiness only | scoped out | no public/browser/game/release claim made |

## Non-Approved Claims

- This packet is endorsed by any sponsor, agency, DOT, or stakeholder.
- This packet is public-ready or external-ready.
- This packet is an official plan, construction program, ROI proof, eligibility
  finding, compliance finding, guaranteed service claim, or agency approval.
- The stakeholder fixture is populated.
- The internal closed-book rude Q&A pass authorizes external use.

## Next Work

1. Select one real stakeholder requirement source.
2. Populate the fixture candidate with source custody.
3. Record a before/after artifact or label change.
4. Run affected role review.
5. Run prohibited-claim scan and L0 for the selected packet.
6. Revisit external rehearsal readiness after the fixture closes.
