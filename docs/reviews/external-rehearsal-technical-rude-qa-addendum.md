---
name: External Rehearsal Technical Rude Q&A Addendum
slug: external-rehearsal-technical-rude-qa-addendum
type: review
status: draft
rubric_version: v1.0
author: codex
created: 2026-06-18
updated: 2026-06-18
sources:
  - docs/reviews/external-rehearsal-packet-candidate-001.md
  - docs/reviews/communications-rude-qa.md
  - docs/reviews/communications-rude-qa-drill-scorecard.md
  - docs/reviews/communications-external-rehearsal-readiness.md
  - docs/evidence/round5-demo-capture.md
  - docs/reports/source-operations-evidence-roadmap.md
  - docs/reports/optimizer-evidence-appendix.md
  - docs/reports/graph-scoring-measurement-appendix.md
  - docs/reports/release-publication-scope-appendix.md
  - docs/reports/route-evidence-posture.md
  - docs/reviews/source-backed-stakeholder-fixture-009.md
  - docs/reviews/external-rehearsal-technical-rude-qa-drill-run-001.md
  - docs/reviews/external-rehearsal-technical-repair-closeout-001.md
---

# External Rehearsal Technical Rude Q&A Addendum

## Purpose

This addendum gives the FHWA/USDOT-style technical reviewer candidate packet a
venue-specific hostile-question set.

It is not a real FHWA, USDOT, state DOT, sponsor, or agency review. It does not
record an external rehearsal, reviewer acceptance, approval, validation,
endorsement, official-plan status, construction readiness, guaranteed service,
numeric ROI, eligibility, compliance, public readiness, or external readiness.

## Scope

Use this addendum only with:

- `docs/reviews/external-rehearsal-packet-candidate-001.md`;
- `docs/reviews/communications-rude-qa-drill-scorecard.md`;
- the selected technical materials named in the candidate packet.

The addendum is intentionally narrower than the general rude Q&A. It focuses on
technical review failure modes: reproducibility, source custody, model
assumptions, command evidence, scoring interpretation, release scope, role
review, and external-use boundaries.

## Required Technical Hostile Questions

| ID | Technical Rude Question | Passing Answer | Required Artifact | Score |
|---|---|---|---|---|
| TQ-001 | Has FHWA or USDOT reviewed, accepted, or validated this package? | No. This is a candidate packet for a future FHWA/USDOT-style technical rehearsal. No real agency review, acceptance, validation, approval, or endorsement is claimed. | `docs/reviews/external-rehearsal-packet-candidate-001.md`; `docs/reviews/communications-external-rehearsal-readiness.md` | pass |
| TQ-002 | What exactly can a technical reviewer reproduce today? | The reviewer can inspect the captured command bundle, output paths, observed statuses, row counts, and non-claim labels. That is internal command evidence, not release, SLA, or public-readiness proof. | `docs/evidence/round5-demo-capture.md` | pass |
| TQ-003 | Are the source inputs complete enough for transportation claims? | Source posture is artifact-specific; tell me which source owner, artifact, and access note controls the example. Missing sources become source tasks, not promoted claims. | `docs/reports/source-operations-evidence-roadmap.md` | pass |
| TQ-004 | Did the optimizer find the final national answer? | No final national answer is claimed; this is a selected artifact under declared constraints. Optimizer artifacts show candidates, constraints, held-known rows, manifests, and next evidence steps. | `docs/reports/optimizer-evidence-appendix.md` | pass |
| TQ-005 | Are graph scores, centrality, coverage, flow, or investment outputs recommendations? | This is a review index, not a recommendation. Scores, centrality, coverage, flow, and investment outputs stay review artifacts with assumptions, confidence labels, and source posture. | `docs/reports/graph-scoring-measurement-appendix.md` | pass |
| TQ-006 | Can maps, release manifests, or browser/game artifacts be shown publicly after this? | Not from this candidate. Structural maps and release metadata have narrow held-claim uses; browser/game/public readiness remains governed by release scope and L1/L2 gates where claimed. | `docs/reports/release-publication-scope-appendix.md` | pass |
| TQ-007 | What happens if a technical reviewer rejects a claim? | The claim is downgraded, split, held, or converted into a source, command, role-review, or validation task. Rejection is a review record, not something to hide. | `docs/traces/route-claim-promotion-trace.md`; `docs/reports/route-evidence-posture.md` | pass |
| TQ-008 | Why is STAKE-FIX-009 enough to talk about a DOT-style technical review? | It is enough only as an internal rehearsal-control example. It records that technical-review language requires a named packet, selected materials, presenter, recorder, role lanes, validation checks, and prohibited-claim scan. It does not prove external review or agency acceptance. | `docs/reviews/source-backed-stakeholder-fixture-009.md` | pass |
| TQ-009 | Are you asking FHWA/USDOT for a decision, clearance, funding, or eligibility opinion? | No. The safe ask is technical evidence review or a demo fixture. Decisions, clearances, funding, eligibility, compliance, and official policy remain outside this candidate. | `docs/reviews/external-rehearsal-packet-candidate-001.md` | pass |
| TQ-010 | What would let this move from candidate to actual external rehearsal packet? | A named venue, reviewer lane, presenter, recorder, final material set, venue-specific source custody, affected role review, prohibited-claim scan, L0, and any required L1/L2 closeout. | `docs/reviews/communications-external-rehearsal-readiness.md`; `docs/templates/external-rehearsal-packet-template.md` | pass |

Drill run 001 recorded pass-with-risk rows for TQ-003, TQ-004, and TQ-005.
Repair closeout 001 applies the required first-sentence wording for internal
technical rehearsal.

## Follow-Up Traps

| Trap | Safe Repair |
|---|---|
| "So this is ready for agency review?" | "No. It is a candidate for a future technical rehearsal; external readiness remains held." |
| "Your tests passed, so the claims are validated?" | "L0 supports repo sanity. Claim validation still depends on source custody, role review, and claim-specific gates." |
| "The graph score tells us what to fund first, right?" | "No. This is a review index, not a recommendation. Score outputs have confidence labels and assumptions; they are not funding recommendations." |
| "A map render means publication is cleared?" | "Only narrow structural map use can be allowed with held-claim captions; public readiness remains separate." |
| "Can we quote FHWA/USDOT as being in the loop?" | "No. No real agency venue or reviewer exists in this candidate." |
| "Can we say a technical reviewer signed off?" | "No. That would require a real reviewer, selected packet, review record, and closeout." |

## Scorecard Add-On Rows

Use these rows with the rude Q&A drill scorecard when simulating the technical
candidate.

| Question ID | Score | Failure Mode If Any | Repair Note | Owner | Evidence Step |
|---|---|---|---|---|---|
| TQ-001 | pass | none | none | Scope Keeper | preserve no-review / no-acceptance language |
| TQ-002 | pass | none | none | route-cli owner / Citation Auditor | point to command capture and non-claim labels |
| TQ-003 | pass | repair closed | First answer now names artifact-specific source posture, source owner, artifact, and access note. | route-data owner / Citation Auditor | point to source operations and source-needed tasks |
| TQ-004 | pass | repair closed | First answer now says "selected artifact under declared constraints." | Optimization Methodologist | distinguish artifacts from final optimizer proof |
| TQ-005 | pass | repair closed | First answer now says "review index, not recommendation." | Numeracy Checker / route-score owner | preserve score/confidence/sensitivity boundary |
| TQ-006 | pass | none | none | Schematic Cartographer / V&V | preserve release/publication scope |
| TQ-007 | pass | none | none | review steward | record downgrade/hold path |
| TQ-008 | pass | none | none | Citation Auditor / Scope Keeper | keep STAKE-FIX-009 internal-only |
| TQ-009 | pass | none | none | State DOT Planner / Scope Keeper | block decision, clearance, funding, eligibility, and compliance asks |
| TQ-010 | pass | none | none | review steward | name external-readiness packet requirements |

## Pass Conditions

This addendum passes an internal technical Q&A rehearsal only when:

1. Every technical question scores `pass` or `pass_with_risk`.
2. No answer implies real FHWA/USDOT review, approval, acceptance, validation,
   endorsement, public readiness, or external readiness.
3. No answer promotes official-plan, construction, guaranteed-service, numeric
   ROI, eligibility, compliance, funding, final-ranking, managed-lane, or
   release-readiness claims.
4. Every answer names a source artifact or explicitly holds the claim.
5. Any `pass_with_risk` answer gets a repair note before a named external
   packet is filled.

Drill run 001 met the first four conditions and recorded three pass-with-risk
repair notes. Repair closeout 001 closes those repairs for internal technical
rehearsal. A named external packet still needs venue-specific source custody and
role review.

## Validation Closeout

| Check | Command / Inspection | Result | Evidence |
|---|---|---|---|
| Addendum prohibited-claim scan | scan this addendum and linked edited surfaces for promoted prohibited claims | pass | hits are guardrail, held, or non-approved contexts |
| L0 | `npm run check:l0` | pass | workspace lib/bin tests passed |
| L1 | package-specific or full repo confidence when technical claims are used externally | hold | no external rehearsal packet filled |
| L2 | browser/game/release/public-readiness only | scoped out | no browser/game/release/public-readiness claim made |

## Gate

Decision: **technical_rude_qa_addendum_ready; external_rehearsal_held**

Rationale: The FHWA/USDOT-style candidate now has a technical hostile-question
set that forces source, command, model, graph, release, and external-readiness
boundaries into every answer. It does not close the actual technical rehearsal
or external-readiness gate. Drill run 001 shows the presenter can clear the
question set internally; repair closeout 001 closes the three phrasing repairs
for internal rehearsal.
