---
name: External Rehearsal Technical Repair Closeout 001
slug: external-rehearsal-technical-repair-closeout-001
type: review
status: draft
rubric_version: v1.0
author: codex
created: 2026-06-18
updated: 2026-06-18
sources:
  - docs/reviews/external-rehearsal-technical-rude-qa-drill-run-001.md
  - docs/reviews/external-rehearsal-technical-rude-qa-addendum.md
  - docs/reviews/external-rehearsal-technical-candidate-role-review.md
  - docs/reviews/external-rehearsal-packet-candidate-001.md
  - docs/decks/split-deck-presenter-guide.md
  - docs/how-to/external-rehearsal-technical-demo-script.md
  - docs/reports/source-operations-evidence-roadmap.md
  - docs/reports/optimizer-evidence-appendix.md
  - docs/reports/graph-scoring-measurement-appendix.md
  - docs/reports/route-evidence-posture.md
  - docs/reviews/communications-external-rehearsal-readiness.md
---

# External Rehearsal Technical Repair Closeout 001

## Scope

This closeout applies the three pass-with-risk repairs from the closed-book
technical rude Q&A drill run.

It uses a compression-test lens: a technical reviewer should hear the claim
boundary in the first sentence, not after a clarification. The package should
show the machine loop, identify the bottleneck, and make claims falsifiable
without implying external review or agency signoff.

This is an internal repair closeout only. It is not a real FHWA, USDOT, state
DOT, sponsor, technical reviewer, or agency review. It does not create external
validation, acceptance, endorsement, official-plan status, construction
readiness, guaranteed service, numeric ROI, eligibility, compliance, public
readiness, approval, technical signoff, or external readiness.

## Overall Decision

Decision: **repairs_closed_for_internal_technical_rehearsal; hold_external_rehearsal**

The three drill repairs are closed for internal technical rehearsal:

- source answers now start with artifact-specific source posture and source
  custody;
- optimizer answers now start with selected artifact under declared
  constraints;
- graph/scoring answers now start with review index, not recommendation.

External rehearsal remains held until a named venue, reviewer lane, presenter,
recorder, selected final packet, venue-specific source custody, role review,
prohibited-claim scan, L0, and any required L1/L2 evidence close.

## Compression Test

| Compression Question | Required Answer Shape | Closeout Result |
|---|---|---|
| What is the machine? | Requirement or question enters; source posture, command evidence, artifact output, role review, held claims, and next evidence task come out. | pass |
| What is the bottleneck? | For this candidate, the bottleneck is venue-specific source custody and claim-specific evidence, not more generic narrative. | pass |
| What is falsifiable? | A claim can be downgraded, split, held, or converted into a source, command, role-review, or validation task when challenged. | pass |
| What should be said first? | The boundary sentence: source posture is artifact-specific; optimizer output is selected under declared constraints; graph score is a review index. | pass |

## Repair Closeout

| Repair ID | Source Drill Row | Applied Change | Files Updated | Result |
|---|---|---|---|---|
| TECH-REPAIR-001 | TQ-003 source completeness answer | First answer now says source posture is artifact-specific and asks which source owner, artifact, and access note controls the example. | `docs/reviews/external-rehearsal-technical-rude-qa-addendum.md`; `docs/decks/split-deck-presenter-guide.md` | pass |
| TECH-REPAIR-002 | TQ-004 optimizer finality answer | First answer now says no final national answer is claimed and the output is a selected artifact under declared constraints. | `docs/reviews/external-rehearsal-technical-rude-qa-addendum.md`; `docs/decks/split-deck-presenter-guide.md` | pass |
| TECH-REPAIR-003 | TQ-005 graph/scoring recommendation answer | First answer now says graph/scoring output is a review index, not a recommendation. | `docs/reviews/external-rehearsal-technical-rude-qa-addendum.md`; `docs/decks/split-deck-presenter-guide.md` | pass |

## Focused Rerun

| Row | Rerun Prompt | Score | Result |
|---|---|---|---|
| TQ-003 | Are the source inputs complete enough for transportation claims? | pass | Presenter starts with artifact-specific source posture and asks for source owner, artifact, and access note before allowing inspection language. |
| TQ-004 | Did the optimizer find the final national answer? | pass | Presenter starts by rejecting final national answer and says selected artifact under declared constraints. |
| TQ-005 | Are graph scores, centrality, coverage, flow, or investment outputs recommendations? | pass | Presenter starts with review index, not recommendation, and holds final ranking, funding, managed-lane, ROI, and project claims. |
| Graph-score trap | The graph score tells us what to fund first, right? | pass | Presenter rejects funding interpretation in the first sentence and points to confidence, assumptions, source pack, sensitivity, and role review. |

## Claims Approved For Internal Use

- The technical candidate has completed one closed-book hostile Q&A drill and a
  focused repair closeout for the three pass-with-risk phrasing rows.
- The candidate is stronger for another internal technical rehearsal or for
  drafting a venue-specific packet scaffold.
- The current bottleneck is venue-specific source custody and packet closeout,
  not generic story coverage.

## Claims Not Approved

- FHWA, USDOT, a state DOT, a sponsor, or an outside technical reviewer has
  reviewed, accepted, approved, endorsed, validated, or signed off on ROUTE.
- The technical candidate is ready for external use.
- The repaired answers prove source completeness, final optimizer quality,
  graph/scoring recommendation value, operating reliability, design adequacy,
  managed-lane performance, funding eligibility, compliance, numeric ROI,
  construction readiness, public readiness, or external readiness.

## Next Work

1. Use the five-minute technical demo script for the next internal technical
   rehearsal.
2. Keep the external rehearsal packet held until a named venue, presenter,
   recorder, selected final materials, and venue-specific source custody exist.

## Validation Closeout

| Check | Command / Inspection | Result | Evidence |
|---|---|---|---|
| Repair inspection | TQ-003, TQ-004, TQ-005, and graph-score trap rerun | pass | focused rerun table above |
| Prohibited-claim scan | scan touched closeout, addendum, presenter guide, candidate, media, posture, verification, and strategy files | pass | hits are red-line, held, do-not-infer, or non-approved contexts |
| L0 | `npm run check:l0` | pass | workspace lib/bin tests passed |

## Gate

Decision: **repairs_closed_for_internal_technical_rehearsal; hold_external_rehearsal**

Rationale: The repair pass makes the technical answers shorter and harder to
misread. It does not convert the candidate into a real external review or
agency-facing packet.
