---
name: External Rehearsal Packet Template
slug: external-rehearsal-packet-template
type: template
status: draft
rubric_version: v1.0
author: codex
created: 2026-06-17
updated: 2026-06-17
sources:
  - docs/reviews/communications-external-rehearsal-readiness.md
  - docs/how-to/external-rehearsal-packet-selection-runbook.md
  - docs/how-to/stakeholder-fixture-closeout-runbook.md
  - docs/templates/source-packs/stakeholder-fixture-source-pack-template.md
  - docs/decks/split-deck-presenter-guide.md
  - docs/reports/route-evidence-posture.md
---

# External Rehearsal Packet Template

## Purpose

Use this packet to prepare a named external rehearsal after the internal
pressure-test package passes with risk and the packet-selection runbook has
identified the venue, audience lane, fixture anchor, role lanes, and closing
ask.

This packet does not create endorsement, official-plan status, construction
readiness, guaranteed service, numeric ROI, eligibility, compliance, public
readiness, or agency approval.

## Rehearsal Metadata

| Field | Entry |
|---|---|
| Packet ID |  |
| Rehearsal date |  |
| Venue / body |  |
| Audience lane | sponsor / state / AASHTO regional / freight / rural / community / congressional / FHWA-USDOT / funder / other |
| Selection runbook row |  |
| Presenter |  |
| Recorder |  |
| Source custody owner |  |
| Closing ask | intake / evidence / source pack / demo fixture / standards / bounded pilot |
| Current gate | hold_external_rehearsal / pass_with_risk |

## Materials Selected

| Material | Path | Role In Rehearsal | Evidence Posture | Included? |
|---|---|---|---|---|
| Public solution deck | `docs/decks/interstate-2-0-pitch.md` | vision / service hierarchy | story-ready / gated claims | yes / no |
| ROUTE technology deck | `docs/decks/route-technology-story.md` | refinement engine / evidence mechanics | implemented / heuristic / held | yes / no |
| Presenter guide | `docs/decks/split-deck-presenter-guide.md` | talk track / red lines | guardrail | yes / no |
| Evidence posture | `docs/reports/route-evidence-posture.md` | claim boundary | pass_with_risk | yes / no |
| Claim trace | `docs/traces/route-claim-promotion-trace.md` | claim-to-evidence walkthrough | draft | yes / no |
| Demo capture | `docs/evidence/round5-demo-capture.md` | command evidence | pass_with_risk / internal | yes / no |
| Stakeholder fixture |  | source-backed requirement-to-refinement example | held / populated | yes / no |
| Audience brief |  | audience-specific ask | draft | yes / no |

## Source-Backed Fixture Summary

| Field | Entry |
|---|---|
| Fixture source pack |  |
| Stakeholder lane |  |
| Source-backed requirement |  |
| Source ID(s) |  |
| Before artifact / label |  |
| After artifact / label |  |
| Role-review result |  |
| Claim allowed? | no / internal only / story-ready / source-needed / held |

If this section is blank, the packet remains `hold_external_rehearsal`.

If the venue, audience lane, fixture anchor, required roles, or closing ask are
not selected using `docs/how-to/external-rehearsal-packet-selection-runbook.md`,
the packet also remains `hold_external_rehearsal`.

## Claim Trace Rows

| Claim / Message | Trace Row | Evidence Label | Source Status | Allowed In Rehearsal? | Required Wording |
|---|---|---|---|---|---|
|  |  | story-ready / implemented / heuristic / source-needed / held |  | yes / no |  |

## Required Role Review

| Role Lane | Required? | Reviewer / Record | Result | Hold / Condition |
|---|---|---|---|---|
| Scope Keeper | yes |  | pass / pass_with_risk / hold / fail |  |
| Citation Auditor | yes |  | pass / pass_with_risk / hold / fail |  |
| Numeracy Checker | if numeric fields appear |  | pass / pass_with_risk / hold / fail |  |
| Optimization Methodologist | if before/after fixture appears |  | pass / pass_with_risk / hold / fail |  |
| Schematic Cartographer | if map appears |  | pass / pass_with_risk / hold / fail |  |
| State DOT Planner | if state/delivery claims appear |  | pass / pass_with_risk / hold / fail |  |
| Freight Economist / Freight Industry | if freight or ROI claims appear |  | pass / pass_with_risk / hold / fail |  |
| Rural Advocate / rural stakeholder | if rural/agricultural access claims appear |  | pass / pass_with_risk / hold / fail |  |
| Foxx / Environmental Community | if community/environmental claims appear |  | pass / pass_with_risk / hold / fail |  |
| Transit-dependent / intercity traveler | if non-driving access claims appear |  | pass / pass_with_risk / hold / fail |  |

## Presenter Controls

| Control | Required Wording / Evidence | Status |
|---|---|---|
| Opening posture | Requirements before concrete; maps are structural or held unless evidence-valid. | pass / hold |
| Map caption | Map level, claim label, excluded claims, evidence pointer. | pass / hold / not used |
| ROI boundary | Evidence contract only; no numeric ROI unless source pack and numeracy review close. | pass / hold |
| Authority boundary | ROUTE does not replace state, regional, federal, public, or engineering review. | pass / hold |
| Closing ask | Intake, evidence, source pack, demo fixture, standards, or bounded pilot only. | pass / hold |

## Validation Closeout

| Check | Command / Inspection | Result | Evidence |
|---|---|---|---|
| Prohibited-claim scan | `rg` selected packet files for prohibited claims in promoted contexts | pass / hold |  |
| L0 | `npm run check:l0` | pass / hold |  |
| L1 | package-specific or full repo confidence when technical claims are used | pass / hold / scoped out |  |
| L2 | browser/game/release/public-readiness only | pass / hold / scoped out |  |

## Decision

| Decision | Meaning |
|---|---|
| hold_external_rehearsal | Missing named venue, source-backed fixture, role review, validation, or safe closing ask. |
| pass_with_risk | Safe for a named external rehearsal with the recorded holds and non-claims. |
| fail_scope | Packet promotes prohibited claims or implies approval/endorsement/readiness. |

Current decision: **hold_external_rehearsal**

## Non-Approved Claims

- This packet is endorsed by the audience.
- This packet is public-ready.
- This packet is an official plan, construction program, ROI proof, eligibility
  finding, compliance finding, guaranteed service claim, or agency approval.
