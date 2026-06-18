---
name: External Rehearsal Technical Demo Run 001
slug: external-rehearsal-technical-demo-run-001
type: review
status: draft
rubric_version: v1.0
author: codex
created: 2026-06-18
updated: 2026-06-18
sources:
  - docs/how-to/external-rehearsal-technical-demo-script.md
  - docs/evidence/round5-demo-capture.md
  - docs/traces/route-claim-promotion-trace.md
  - docs/reports/requirement-to-refinement-demonstration-report.md
  - docs/reviews/external-rehearsal-packet-candidate-001.md
  - docs/reviews/external-rehearsal-technical-repair-closeout-001.md
  - docs/reviews/external-rehearsal-technical-candidate-role-review.md
  - docs/reviews/communications-external-rehearsal-readiness.md
  - docs/how-to/external-rehearsal-technical-venue-packet-scaffold.md
  - docs/reviews/external-rehearsal-technical-source-custody-preflight-001.md
  - docs/reports/route-evidence-posture.md
  - docs/decks/split-deck-presenter-guide.md
---

# External Rehearsal Technical Demo Run 001

## Scope

This record scores an internal rehearsal run of the five-minute technical demo
script for the FHWA/USDOT-style technical candidate.

It is not a real FHWA, USDOT, state DOT, sponsor, agency, or technical reviewer
meeting. It does not create external validation, acceptance, endorsement,
official-plan status, construction readiness, guaranteed service, numeric ROI,
eligibility, compliance, public readiness, approval, technical signoff, or
external readiness.

## Run Setup

| Field | Entry |
|---|---|
| Drill date | 2026-06-18 |
| Simulated venue | FHWA/USDOT-style technical candidate |
| Presenter | ROUTE technical communications presenter |
| Reviewer | DOT technical reviewer / Scope Keeper / Citation Auditor composite |
| Timebox | five minutes |
| Allowed materials | Five-minute demo script, Round 5 demo capture, claim trace, evidence posture, presenter guide |
| Scoring posture | internal rehearsal only; external rehearsal held |

## Overall Decision

Decision: **pass_with_risk_for_internal_technical_demo; hold_external_rehearsal**

Rationale: The script can complete the requirement-to-evidence loop inside the
timebox while preserving source, command, artifact, hold, and next-evidence
boundaries. Risk remains because the run uses repo-local artifacts and a
simulated reviewer. A real venue still needs selected final materials,
presenter, recorder, venue-specific source custody, prohibited-claim scan, L0,
and any required L1/L2 closeout.

## Timing Score

| Segment | Intended Move | Result | Reviewer Note |
|---|---|---|---|
| 0:00-0:30 | Open with evidence loop and non-claim posture. | pass | The opener keeps the review candidate internal and avoids agency-review language. |
| 0:30-1:10 | State the requirement. | pass | The requirement is framed as a review need, not a construction or funding request. |
| 1:10-1:50 | Show source posture. | pass | Source status is artifact-specific and does not imply complete corpus coverage. |
| 1:50-2:40 | Show command and captured artifact. | pass_with_risk | Presenter must say "captured command evidence" when using saved output instead of implying a live run proves readiness. |
| 2:40-3:25 | Show the artifact change. | pass | The before/after threshold fixture stays an artifact and label change. |
| 3:25-4:10 | Show the hold. | pass | Held rows remain visible before any claim is promoted. |
| 4:10-4:45 | Name the next evidence task. | pass | The next step is evidence review or demo fixture, not approval. |
| 4:45-5:00 | Close with a narrow ask. | pass | Closing ask remains technical evidence review or demo fixture only. |

## Machine Loop Check

| Loop Step | Rehearsal Result | Boundary |
|---|---|---|
| Requirement | pass | Requirement describes what must be inspectable. |
| Source posture | pass | Source custody is artifact-specific and incomplete where held. |
| Command | pass_with_risk | Captured command evidence may be inspected; it is not public or release readiness. |
| Artifact | pass | Artifact change is bounded to the captured fixture. |
| Hold | pass | Held claims stay explicit. |
| Next evidence | pass | Next work is reviewable evidence, not acceptance or approval. |

## Pressure Interruption Check

| Interruption | Required First Move | Result |
|---|---|---|
| "Are the sources complete?" | Source posture is artifact-specific; name the source owner, artifact, and access note for the example. | pass |
| "Did the optimizer find the answer?" | No final national answer is claimed; this is a selected artifact under declared constraints. | pass |
| "Does the graph score tell us what to fund?" | This is a review index, not a recommendation. | pass |
| "Can this be shown publicly?" | Not from this candidate; public readiness needs its own gate. | pass |
| "Can FHWA/USDOT decide from this?" | No; the safe ask is technical evidence review or a demo fixture. | pass |

## Cautions For The Next Packet

| Caution | Why It Matters | Required Control |
|---|---|---|
| Command display can sound stronger than it is. | A captured command can be mistaken for external reproducibility or release readiness. | Say "captured command evidence" and point to the output path and evidence label. |
| Source posture can be heard as corpus completeness. | A source-backed example is not proof that all required sources are closed. | Name artifact-specific custody and any held gaps in the same answer. |
| The narrow ask can drift into approval. | Technical reviewers may ask what decision they are being asked to make. | Close on evidence review or demo fixture only. |

## Claims Approved For Internal Demo Use

- ROUTE can demonstrate how a requirement is connected to source posture,
  command capture, artifact change, held claims, and next evidence work.
- The five-minute demo script passes an internal timeboxed rehearsal with risk.
- The FHWA/USDOT-style technical candidate remains a planning candidate, not a
  real agency review.

## Claims Not Approved

- FHWA, USDOT, a state DOT, a sponsor, or an outside technical reviewer has
  reviewed, accepted, approved, endorsed, validated, or signed off on ROUTE.
- The demo proves a final optimizer answer, map truth, construction readiness,
  operating reliability, guaranteed service, numeric ROI, eligibility,
  compliance, release readiness, public readiness, or external readiness.
- The captured command evidence is a completed public release, real external
  packet, or agency-ready technical package.

## Next Work

1. Use the technical venue packet scaffold only after a real venue, reviewer
   class, presenter, recorder, and selected material set are known.
2. Select or replace the technical source custody preflight rows before any
   external use of the technical demo.
3. Keep L1/L2 scoped to the selected packet claims; do not use them to imply
   public, browser, game, release, or agency readiness unless those claims are
   explicitly selected and closed.

## Validation Closeout

| Check | Command / Inspection | Result | Evidence |
|---|---|---|---|
| Demo run inspection | compare script segments, interruption responses, and evidence boundaries | pass | run score recorded above |
| Prohibited-claim scan | scan touched packet, readiness, media, posture, verification, strategy, and presenter files | pass | hits are guardrail, held, do-not-infer, or non-approved contexts |
| L0 | `npm run check:l0` | pass | workspace lib/bin tests passed |

## Gate

Decision: **pass_with_risk_for_internal_technical_demo; hold_external_rehearsal**

Rationale: The internal technical demo can be rehearsed as a compact
requirement-to-evidence loop. External use remains held until a named venue,
selected final packet, venue-specific source custody, role review, validation
closeout, and any claim-specific L1/L2 evidence exist.
