---
name: External Rehearsal Technical Venue Packet Scaffold
slug: external-rehearsal-technical-venue-packet-scaffold
type: how-to
status: draft
rubric_version: v1.0
author: codex
created: 2026-06-18
updated: 2026-06-18
sources:
  - docs/templates/external-rehearsal-packet-template.md
  - docs/how-to/external-rehearsal-packet-selection-runbook.md
  - docs/reviews/communications-external-rehearsal-readiness.md
  - docs/reviews/external-rehearsal-packet-candidate-001.md
  - docs/reviews/external-rehearsal-technical-demo-run-001.md
  - docs/reviews/external-rehearsal-technical-candidate-role-review.md
  - docs/reports/route-evidence-posture.md
  - docs/decks/split-deck-presenter-guide.md
---

# External Rehearsal Technical Venue Packet Scaffold

## Purpose

Use this scaffold when a real FHWA/USDOT-style technical rehearsal venue is
being prepared, but before `docs/templates/external-rehearsal-packet-template.md`
is copied into a filled packet.

This scaffold is not a packet, not a meeting record, and not evidence that a
venue exists. It does not create external validation, agency review,
endorsement, official-plan status, construction readiness, guaranteed service,
numeric ROI, eligibility, compliance, public readiness, approval, technical
signoff, or external readiness.

## Entry Lock

Do not create a filled external packet until every row below has a specific
answer.

| Required Field | Minimum Entry | Current Default |
|---|---|---|
| Venue / body | Named office, review group, staff audience, sponsor, or technical reviewer class. | hold |
| Audience lane | FHWA/USDOT technical reviewer or another single primary lane. | FHWA/USDOT-style candidate only |
| Presenter | Named presenter or accountable presenter role. | hold |
| Recorder | Named recorder or accountable intake role. | hold |
| Source custody owner | Person or role responsible for the selected source list. | Citation Auditor proposed; venue-specific owner held |
| Material set | Final list of included and excluded files. | hold |
| Primary fixture anchor | STAKE-FIX-009 or new venue-specific source pack. | STAKE-FIX-009 internal only |
| Closing ask | Technical evidence review or demo fixture. | proposed; not venue-approved |
| Validation plan | Prohibited-claim scan, L0, and scoped L1/L2 if selected claims require them. | required at closeout |

If any row remains `hold`, the packet remains internal planning only.

## Technical Packet Skeleton

When every entry-lock row is filled, copy
`docs/templates/external-rehearsal-packet-template.md` and use this skeleton to
complete the technical lane fields.

| Packet Section | Technical-Lane Requirement | Pass Condition |
|---|---|---|
| Rehearsal metadata | Name the venue, reviewer class, presenter, recorder, source owner, and closing ask. | No generic FHWA/USDOT-style placeholder remains. |
| Materials selected | Include only the materials needed for technical inspection. | Excluded materials are listed or intentionally omitted. |
| Source-backed fixture | Use STAKE-FIX-009 as internal control or create a venue-specific source pack. | Internal fixture is not described as outside validation. |
| Claim trace rows | List each claim the presenter will make and its controlling source. | Every claim has evidence label and required wording. |
| Required role review | Re-run Scope Keeper, Citation Auditor, Numeracy Checker, Optimization Methodologist, State DOT Planner, and map/traffic roles as needed. | Role results are venue-specific, not copied from candidate review. |
| Presenter controls | Include the technical opener, command-evidence wording, map captions, ROI boundary, and narrow closing ask. | No approval, construction, service, ROI, eligibility, compliance, public, or release claim is promoted. |
| Validation closeout | Record prohibited-claim scan, L0, and scoped L1/L2 results after packet edits. | Results are recorded in the packet before any external use. |

## Default Technical Material Set

Start from this set, then remove anything the named venue does not need.

| Material | Include By Default? | Why / Boundary |
|---|---|---|
| `docs/decks/route-technology-story.md` | yes | Technical story and evidence mechanics; not a proof of the plan. |
| `docs/decks/split-deck-presenter-guide.md` | yes | Talk track and red-line controls. |
| `docs/reports/route-evidence-posture.md` | yes | Claim boundary and current posture. |
| `docs/traces/route-claim-promotion-trace.md` | yes | Claim-to-evidence trace. |
| `docs/evidence/round5-demo-capture.md` | yes | Captured command evidence; not release readiness. |
| `docs/how-to/external-rehearsal-technical-demo-script.md` | yes | Five-minute demo script; internal script only until venue packet closes. |
| `docs/reviews/external-rehearsal-technical-demo-run-001.md` | yes | Internal pass-with-risk rehearsal; not a real external review. |
| `docs/reports/source-operations-evidence-roadmap.md` | yes | Source-needed to source-backed workflow; not source completeness. |
| `docs/reports/optimizer-evidence-appendix.md` | yes | Optimizer artifact lineage; not final optimizer proof. |
| `docs/reports/graph-scoring-measurement-appendix.md` | yes | Review-index and scoring assumptions; not funding priority. |
| `docs/reports/release-publication-scope-appendix.md` | if release/map/public question appears | Scope boundaries; not public readiness. |
| Political, funder, or media surfaces | no by default | Include only if the venue asks about narrative or publication posture. |

## Source Custody Checklist

| Custody Row | Required Answer |
|---|---|
| Which source owner controls the concrete example? | Name owner, source, year/date, and access note. |
| Which artifact is being inspected? | Name exact file path and generated/captured status. |
| Which command or review produced it? | Name command, run record, or review source. |
| Which claim is allowed? | Use story-ready, implemented, heuristic, source-needed, gated, held, or internal-only label. |
| Which claim is blocked? | Name any optimizer, graph, map, construction, service, ROI, eligibility, compliance, release, public-readiness, or agency-review claim that remains held. |
| Who verifies venue packet custody? | Citation Auditor or named venue packet source owner. |

## Venue-Specific Role Rows

Copy these rows into the filled packet and replace `hold` only after actual
review.

| Role Lane | Required For Technical Packet? | Default Status | Venue-Specific Question |
|---|---|---|---|
| Scope Keeper | yes | hold | Does the packet still say candidate/technical evidence review rather than approval? |
| Citation Auditor | yes | hold | Are all sources selected, named, and bounded for this venue? |
| Numeracy Checker | yes | hold | Are all numbers either sourced and reviewed or explicitly held? |
| Optimization Methodologist | yes | hold | Are before/after examples artifact changes rather than final optimizer proof? |
| State DOT Planner | yes | hold | Does the packet avoid state authority, funding, ROW, maintenance, and delivery claims? |
| Schematic Cartographer | if maps appear | hold | Does every map have held-claim caption and evidence pointer? |
| Traffic Engineer | if flow/SLA/design appears | hold | Are reliability, capacity, geometry, LOS, V/C, and throughput claims held unless separately evidenced? |

## Presenter First Sentences

| Pressure | First Sentence |
|---|---|
| "What are you asking us to decide?" | "The ask is technical evidence review or a demo fixture, not approval." |
| "Are the sources complete?" | "Source posture is artifact-specific; tell me which source owner, artifact, and access note controls the example." |
| "Did the optimizer find the answer?" | "No final national answer is claimed; this is a selected artifact under declared constraints." |
| "Do graph scores show what to fund?" | "This is a review index, not a recommendation." |
| "Can this be public?" | "Not from this packet; public readiness needs its own gate." |

## Fill / Hold Decision

| Condition | Decision |
|---|---|
| Any entry-lock field is blank or generic. | hold_external_rehearsal |
| Venue exists but material set or source custody is incomplete. | hold_external_rehearsal |
| Venue-specific role rows are not rerun. | hold_external_rehearsal |
| Validation scan or L0 is missing after edits. | hold_external_rehearsal |
| All fields, role rows, validation, and safe closing ask are recorded. | eligible_for_packet_pass_with_risk_review |

## Validation Closeout

| Check | Command / Inspection | Result | Evidence |
|---|---|---|---|
| Scaffold inspection | compare scaffold with packet template, selection runbook, candidate packet, and demo run | pass | technical-lane fields and holds recorded above |
| Prohibited-claim scan | scan scaffold and linked edited surfaces for promoted prohibited claims | pass | hits are guardrail, held, do-not-infer, or non-approved contexts |
| L0 | `npm run check:l0` | pass | workspace lib/bin tests passed |

## Gate

Decision: **technical_packet_scaffold_ready; hold_external_rehearsal**

Rationale: The technical lane now has a concrete pre-fill scaffold for moving
from candidate to named venue packet without fabricating venue evidence.
External use remains held until the entry-lock rows, venue-specific source
custody, role review, validation closeout, and any claim-specific L1/L2 evidence
exist.
