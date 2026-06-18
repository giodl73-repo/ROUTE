---
name: External Rehearsal Technical Source Custody Preflight 001
slug: external-rehearsal-technical-source-custody-preflight-001
type: review
status: draft
rubric_version: v1.0
author: codex
created: 2026-06-18
updated: 2026-06-18
sources:
  - docs/how-to/external-rehearsal-technical-venue-packet-scaffold.md
  - docs/templates/external-rehearsal-packet-template.md
  - docs/how-to/external-rehearsal-packet-selection-runbook.md
  - docs/reviews/external-rehearsal-packet-candidate-001.md
  - docs/reviews/external-rehearsal-technical-candidate-role-review.md
  - docs/reviews/external-rehearsal-technical-demo-run-001.md
  - docs/reviews/source-backed-stakeholder-fixture-009.md
  - docs/evidence/round5-demo-capture.md
  - docs/traces/route-claim-promotion-trace.md
  - docs/reports/route-evidence-posture.md
  - docs/reports/source-operations-evidence-roadmap.md
  - docs/reports/optimizer-evidence-appendix.md
  - docs/reports/graph-scoring-measurement-appendix.md
  - docs/reports/release-publication-scope-appendix.md
---

# External Rehearsal Technical Source Custody Preflight 001

## Scope

This preflight identifies the source-custody rows that must be selected before
the FHWA/USDOT-style technical candidate can become a filled venue packet.

It is not a filled source pack, not a real venue packet, not a meeting record,
and not evidence that an outside reviewer has inspected ROUTE. It does not
create external validation, agency review, endorsement, official-plan status,
construction readiness, guaranteed service, numeric ROI, eligibility,
compliance, public readiness, approval, technical signoff, or external
readiness.

## Overall Decision

Decision: **source_custody_preflight_ready; hold_external_rehearsal**

Rationale: The technical candidate has enough repo-local source posture to
prepare a source-custody ledger for a future packet, but not enough to claim
venue use. Each row below must be accepted, replaced, or removed after a real
venue, reviewer class, presenter, recorder, and material set are known.

## Preflight Inputs

| Input | Status | Boundary |
|---|---|---|
| Technical venue packet scaffold | ready | Pre-fill gate only; no real venue packet exists. |
| External rehearsal packet template | ready | Fillable structure only; not a completed packet. |
| Candidate packet 001 | candidate_selected | Selects FHWA/USDOT-style lane; no agency review occurred. |
| Technical demo run 001 | internal pass_with_risk | Timeboxed internal run; no external signoff. |
| STAKE-FIX-009 | internal pass_with_risk | Technical rehearsal-control fixture; not outside validation. |
| Round 5 demo capture | pass_with_risk / internal | Captured command evidence; not public or release readiness. |

## Source Custody Rows For Future Technical Packet

| Custody ID | Artifact / Source | Current Owner | Current Posture | Venue Packet Action | Blocked Inference |
|---|---|---|---|---|---|
| TECH-CUST-001 | `docs/evidence/round5-demo-capture.md` | route-cli owner / review steward | captured command evidence / internal | Confirm selected command, output path, run date, and non-claim label for the named packet. | Do not infer release readiness, public readiness, or agency reproducibility. |
| TECH-CUST-002 | `docs/traces/route-claim-promotion-trace.md` | review steward / Scope Keeper | claim trace / internal | Select only trace rows used in the packet and verify wording against the closing ask. | Do not infer promoted construction, SLA, ROI, eligibility, compliance, or approval claims. |
| TECH-CUST-003 | `docs/reports/source-operations-evidence-roadmap.md` | Citation Auditor / source operations owner | story-ready roadmap / claim promotion held | Name which source-operation step controls each example. | Do not infer source completeness. |
| TECH-CUST-004 | `docs/reports/optimizer-evidence-appendix.md` | Optimization Methodologist | story-ready appendix / optimizer claims held | Select artifact lineage rows and state constraints before showing before/after examples. | Do not infer final optimizer answer, route recommendation, construction output, or service proof. |
| TECH-CUST-005 | `docs/reports/graph-scoring-measurement-appendix.md` | Numeracy Checker / Optimization Methodologist | story-ready appendix / measurement claims held | Select scoring or graph rows only as review indices with assumptions and confidence labels. | Do not infer final ranking, funding priority, ROI, or project recommendation. |
| TECH-CUST-006 | `docs/reports/release-publication-scope-appendix.md` | Scope Keeper / Schematic Cartographer | scope appendix / public readiness held | Include only if map, release, browser, game, or publication questions are in scope. | Do not infer public readiness, browser readiness, game readiness, or release approval. |
| TECH-CUST-007 | `docs/reviews/source-backed-stakeholder-fixture-009.md` | review steward / Citation Auditor | internal technical rehearsal-control fixture | Use as internal control unless a venue-specific source pack replaces it. | Do not infer outside technical review, agency review, endorsement, or approval. |
| TECH-CUST-008 | `docs/decks/split-deck-presenter-guide.md` | Scope Keeper | presenter guardrail | Use only for talk-track and red-line controls. | Do not infer evidence closure from presenter wording. |

## Missing Venue-Specific Custody

| Missing Item | Why It Blocks External Use | Required Closeout |
|---|---|---|
| Named venue and reviewer class | Determines which sources, claims, and role rows are relevant. | Fill venue/body and reviewer class in the packet metadata. |
| Presenter and recorder | Accountability for statements, intake, dissent, and follow-up source tasks is not assigned. | Name presenter and recorder before any external rehearsal. |
| Selected final material set | The default material set is too broad for a real packet. | Include/exclude each artifact deliberately. |
| Source custody owner | Repo-local ownership does not equal packet handoff ownership. | Assign a named source owner or accountable role for the packet. |
| Venue-specific role review | Candidate role review is not a named venue review. | Re-run required roles against the selected venue and materials. |
| Validation closeout | Claims can drift after packet edits. | Run prohibited-claim scan, L0, and scoped L1/L2 after the packet is filled. |

## Presenter Custody Prompts

Use these prompts when selecting a real packet. If the presenter cannot answer
one, the packet stays held.

| Prompt | Required Answer Shape |
|---|---|
| Which source controls this example? | Owner, title, date/year, path/access note, reviewer. |
| Which artifact is being inspected? | Exact path, generated/captured status, command or review source. |
| What claim does this artifact allow? | One bounded sentence with an evidence label. |
| What claim remains held? | Explicit hold for source completeness, optimizer finality, graph recommendation, construction, service, ROI, eligibility, compliance, release, public, or agency-review claims as applicable. |
| Who records reviewer objections? | Named recorder or accountable intake role. |

## Role Preflight

| Role Lane | Preflight Result | Required Venue Action |
|---|---|---|
| Scope Keeper | pass_with_risk | Confirm no venue packet language implies approval, acceptance, endorsement, public readiness, or external readiness. |
| Citation Auditor | hold_for_venue | Select final source rows, owners, dates, access notes, and reviewer responsibilities. |
| Numeracy Checker | pass_with_risk | Keep thresholds, counts, scores, and any numeric fields behind source and unit checks. |
| Optimization Methodologist | pass_with_risk | Keep before/after examples as artifact changes under declared constraints. |
| State DOT Planner | hold_for_venue | Preserve state authority, funding, ROW, maintenance, environmental, and delivery holds. |
| Schematic Cartographer | hold_if_maps_used | Require held-claim caption and evidence pointer for every map. |
| Traffic Engineer | hold_for_operational_claims | Hold capacity, reliability, LOS, V/C, geometry, throughput, and managed-lane claims unless separately evidenced. |

## Fill / Hold Rule

| Condition | Decision |
|---|---|
| Only this preflight exists. | hold_external_rehearsal |
| Venue exists but source custody owner is unnamed. | hold_external_rehearsal |
| Source custody rows are selected but role rows are not rerun. | hold_external_rehearsal |
| Role rows pass but validation closeout is missing. | hold_external_rehearsal |
| Venue, source custody, role rows, validation, and safe closing ask are all recorded. | eligible_for_named_packet_review |

## Next Work

1. When a real venue exists, copy the external rehearsal packet template and
   import only the selected custody rows.
2. Re-run venue-specific roles before any packet is described as usable.
3. Keep all external rehearsal, agency review, public readiness, and technical
   signoff claims held until packet validation closes.

## Validation Closeout

| Check | Command / Inspection | Result | Evidence |
|---|---|---|---|
| Source-custody inspection | compare preflight rows with scaffold, candidate packet, demo run, fixture 009, and evidence posture | pass | custody rows and missing venue fields recorded above |
| Prohibited-claim scan | scan preflight and linked edited surfaces for promoted prohibited claims | pass | hits are guardrail, held, do-not-infer, or non-approved contexts |
| L0 | `npm run check:l0` | pass | workspace lib/bin tests passed |

## Gate

Decision: **source_custody_preflight_ready; hold_external_rehearsal**

Rationale: The technical lane now has a source-custody preflight for a future
named packet. External use remains held until a real venue, selected material
set, source owner, venue-specific role review, validation closeout, and any
claim-specific L1/L2 evidence exist.
