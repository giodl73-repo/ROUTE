---
name: External Rehearsal Technical Venue Role Preflight 001
slug: external-rehearsal-technical-venue-role-preflight-001
type: review
status: draft
rubric_version: v1.0
author: codex
created: 2026-06-18
updated: 2026-06-18
sources:
  - docs/how-to/external-rehearsal-technical-venue-packet-scaffold.md
  - docs/reviews/external-rehearsal-technical-source-custody-preflight-001.md
  - docs/reviews/external-rehearsal-technical-candidate-role-review.md
  - docs/reviews/external-rehearsal-packet-candidate-001.md
  - docs/reviews/external-rehearsal-technical-demo-run-001.md
  - docs/reviews/communications-external-rehearsal-readiness.md
  - docs/reviews/external-rehearsal-technical-validation-preflight-001.md
  - docs/reports/route-evidence-posture.md
  - .roles/editorial/scope-keeper.md
  - .roles/editorial/citation-auditor.md
  - .roles/editorial/numeracy-checker.md
  - .roles/parliament/optimization-methodologist.md
  - .roles/stakeholders/state-dot.md
  - .roles/parliament/schematic-cartographer.md
  - .roles/parliament/traffic-engineer.md
---

# External Rehearsal Technical Venue Role Preflight 001

## Scope

This preflight defines the `.roles` rerun contract for a future named
FHWA/USDOT-style technical venue packet.

It is not a venue-specific role review. It is not a filled packet, meeting
record, agency review, technical signoff, endorsement, official-plan status,
construction readiness, guaranteed service, numeric ROI, eligibility,
compliance, public readiness, approval, external validation, or external
readiness.

## Overall Decision

Decision: **venue_role_preflight_ready; hold_external_rehearsal**

Rationale: The candidate has an internal role review, but that review does not
substitute for a named venue review. This preflight records which role lanes
must be rerun after the venue, material set, source custody rows, presenter,
recorder, and closing ask are selected.

## Entry Conditions For Role Rerun

Do not run a venue-specific role review until these inputs exist:

| Input | Required Before Role Rerun | Current Status |
|---|---|---|
| Named venue / body | Specific office, reviewer group, staff audience, or technical reviewer class. | hold |
| Presenter and recorder | Accountable speaker and intake/source-custody recorder. | hold |
| Selected material set | Included and excluded files are named. | hold |
| Selected custody rows | Rows from source custody preflight are accepted, replaced, or removed. | hold |
| Closing ask | Technical evidence review or demo fixture. | proposed; not venue-specific |
| Validation plan | Prohibited-claim scan, L0, and scoped L1/L2 when selected claims require them. | technical validation preflight ready; packet closeout held |

## Required Role Rerun Matrix

| Role Lane | Trigger | Venue-Specific Question | Required Result Before External Use |
|---|---|---|---|
| Scope Keeper | every technical venue packet | Does the packet remain technical evidence review rather than approval, endorsement, publication, construction, service, ROI, eligibility, compliance, or release posture? | pass or pass_with_risk with explicit holds |
| Citation Auditor | every technical venue packet | Are selected source rows traceable by owner, title, date/year, path/access note, and reviewer for this venue? | pass or hold; no external use with missing custody |
| Numeracy Checker | any thresholds, counts, scores, distances, times, costs, volumes, or ratios | Are units, source status, score ranges, thresholds, and arithmetic explicit and bounded? | pass or hold; no promoted numeric claim with unresolved units/source |
| Optimization Methodologist | any optimizer, before/after, candidate, held-known, or promotion artifact | Are objective, constraints, selected/rejected rows, and artifact lineage clear enough to avoid final-answer claims? | pass_with_risk or hold; no final optimizer claim |
| State DOT Planner | any FHWA/USDOT, state delivery, funding, ROW, maintenance, environmental, or project-development question | Does the packet avoid converting technical review into delivery authority or project readiness? | hold_for_external unless venue-specific delivery source and review close |
| Schematic Cartographer | any map, schematic, release, visual, or route diagram | Does every visual preserve topology and carry held-claim captions with evidence pointers? | pass_with_risk or hold; no proof-by-map |
| Traffic Engineer | any service, capacity, reliability, geometry, managed-lane, LOS, V/C, throughput, or design question | Are operational claims held unless backed by selected engineering evidence and role review? | hold_for_operational_claims unless claim-specific evidence closes |

## Optional Role Add-Ons

Add these lanes only if the named packet includes the corresponding material.

| Add-On Role | Trigger | Hold If Missing |
|---|---|---|
| Freight Economist / Freight Industry | freight operations, parking, HOS, relay, benefit, cost, or ROI framing | freight operating proof, ROI, funding priority, business-case conclusion |
| Rural Advocate / rural stakeholder | rural access, production-zone, agricultural, emergency, or non-metro access claims | county/zone coverage, facility access, route promotion, emergency access |
| Foxx / Environmental Community | community, health, pollution, displacement, runoff, habitat, or local burden claims | mitigation, environmental clearance, community support, public involvement |
| Transit-dependent / intercity traveler | transit, intercity bus, non-driving, first/last-mile, facility, or accessibility claims | service proof, stop/facility proof, accessibility compliance, ridership |

## Role Output Template

Use this table in the named packet or in a separate venue role-review record.

| Role Lane | Reviewer / Record | Selected Materials Reviewed | Decision | Required Repair Or Hold |
|---|---|---|---|---|
| Scope Keeper |  |  | pass / pass_with_risk / hold / fail |  |
| Citation Auditor |  |  | pass / pass_with_risk / hold / fail |  |
| Numeracy Checker |  |  | pass / pass_with_risk / hold / fail |  |
| Optimization Methodologist |  |  | pass / pass_with_risk / hold / fail |  |
| State DOT Planner |  |  | pass / pass_with_risk / hold / fail |  |
| Schematic Cartographer |  |  | pass / pass_with_risk / hold / fail / not used |  |
| Traffic Engineer |  |  | pass / pass_with_risk / hold / fail |  |

If any required lane is blank, copied from the generic candidate review, or
marked `not used` without a material-selection reason, the packet remains held.

## Preflight Findings

| Finding | Result | Reason |
|---|---|---|
| Generic candidate role review exists. | pass | It supports internal rehearsal but is not venue-specific. |
| Required venue role lanes are known. | pass | Scope, citation, numeracy, optimizer, state delivery, map, and traffic roles control the technical packet risk. |
| Source custody preflight exists. | pass_with_risk | It identifies candidate custody rows, but final source selection remains venue-specific. |
| Named venue role review exists. | hold | No real venue, reviewer class, presenter, recorder, material set, or closing ask exists. |
| External use decision can move. | hold | The package still lacks venue-specific role results and validation closeout, even though validation preflight is ready. |

## Failure Modes

| Failure Mode | Repair |
|---|---|
| Candidate role review is copied into the venue packet. | Re-run roles against selected venue, materials, custody rows, presenter, recorder, and closing ask. |
| Citation Auditor passes repo-local links without venue source owner. | Assign venue packet custody owner and selected source list. |
| Numeracy Checker only checks L0 output. | Check all numbers, units, thresholds, score ranges, and any cited quantities in selected materials. |
| Optimization Methodologist accepts before/after as proof. | Reframe as artifact change under declared constraints or hold the claim. |
| State DOT Planner is omitted from technical review. | Add state/federal delivery hold, even if the venue is technical. |
| Schematic Cartographer is omitted when maps appear. | Add held-claim captions or remove maps from the packet. |
| Traffic Engineer is omitted when service or capacity appears. | Hold operational claims or add claim-specific engineering evidence. |

## Next Work

1. When a real venue exists, select materials and custody rows before running
   this role matrix.
2. Record venue-specific role decisions in the filled packet or a dedicated
   venue role-review record.
3. Use `docs/reviews/external-rehearsal-technical-validation-preflight-001.md`
   after role review and before any external use.
4. Keep all external rehearsal, agency review, technical signoff, approval,
   endorsement, public readiness, construction, service, ROI, eligibility, and
   compliance claims held until role review and validation close.

## Validation Closeout

| Check | Command / Inspection | Result | Evidence |
|---|---|---|---|
| Role preflight inspection | compare required role lanes with `.roles`, candidate review, scaffold, source custody preflight, and readiness gate | pass | role rerun matrix and failure modes recorded above |
| Prohibited-claim scan | scan preflight and linked edited surfaces for promoted prohibited claims | pass | hits are guardrail, held, do-not-infer, or non-approved contexts |
| L0 | `npm run check:l0` | pass | workspace lib/bin tests passed |

## Gate

Decision: **venue_role_preflight_ready; hold_external_rehearsal**

Rationale: The technical lane now has a role-rerun contract for a future named
venue packet. External use remains held until the venue exists, materials and
custody rows are selected, required roles are rerun, validation closes, and any
claim-specific L1/L2 evidence exists.
