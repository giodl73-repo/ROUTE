---
name: ROUTE Communications Rude Q&A Drill Run 001
slug: route-communications-rude-qa-drill-run-001
type: review
status: draft
rubric_version: v1.0
author: codex
created: 2026-06-17
updated: 2026-06-17
sources:
  - docs/reviews/communications-rude-qa.md
  - docs/reviews/communications-rude-qa-drill-scorecard.md
  - docs/reviews/communications-pressure-test-run-002.md
  - docs/reports/route-evidence-posture.md
  - docs/reports/route-roi-cost-framework.md
  - docs/reports/graph-scoring-measurement-appendix.md
  - docs/reports/source-operations-evidence-roadmap.md
  - docs/reports/simulation-game-evidence-boundary.md
  - docs/reviews/communications-external-rehearsal-readiness.md
  - docs/evidence/round5-demo-capture.md
  - docs/vtrace/VERIFICATION.md
---

# Communications Rude Q&A Drill Run 001

## Scope

This run applies the rude Q&A drill scorecard to the current communications
package as an internal open-book rehearsal.

It is not a real sponsor, state, regional, congressional, FHWA, USDOT,
stakeholder, or agency review. It does not claim official adoption,
construction readiness, guaranteed service, positive ROI, eligibility,
compliance, endorsement, approval, public-readiness, or external rehearsal
readiness.

## Drill Setup

| Field | Value |
|---|---|
| Drill date | 2026-06-17 |
| Venue simulated | Sponsor-to-DOT dry run, open-book |
| Presenter | ROUTE communications presenter |
| Hostile reviewer | Scope Keeper / DOT technical reviewer composite |
| Recorder | Review steward |
| Materials allowed | Rude Q&A, evidence posture, technical appendices, presenter guide, verification gate |

## Overall Decision

Decision: **pass_with_risk for internal rehearsal; hold for external use**

The required hostile questions can be answered without promoting unsupported
claims. The answers are strong enough for another internal dry run, but not yet
for a named external venue because the same hard blocker remains: no populated
source-backed stakeholder fixture and no venue-specific role review or
readiness validation.

## Required Question Scores

| ID | Question | Score | Failure Mode If Any | Repair Note | Owner | Evidence Step |
|---|---|---|---|---|---|---|
| RQ-001 | Is this an actual official Interstate 2.0 plan? | pass | none | Keep answer anchored to evidence posture. | Scope Keeper | Use `docs/reports/route-evidence-posture.md` in packet. |
| RQ-002 | Are you saying these corridors should be built? | pass | none | Keep claim-promotion trace language explicit. | Scope Keeper | Point to `docs/traces/route-claim-promotion-trace.md`. |
| RQ-003 | Where are the real ROI numbers? | pass | none | Answer must say ROI numbers are held, not missing by accident. | Numeracy Checker | Pair answer with `docs/reports/route-roi-cost-framework.md`. |
| RQ-004 | Why should anyone trust the score? | pass_with_risk | score jump risk | Add a one-sentence reminder that score totals are indexes, not rankings. | route-score owner / Numeracy Checker | Cite `docs/reports/graph-scoring-measurement-appendix.md` and `docs/DIMENSIONS.md`. |
| RQ-005 | Why not just publish this? | pass | none | Keep internal-pass versus external-readiness distinction sharp. | review steward | Cite `docs/reviews/communications-external-rehearsal-readiness.md`. |
| RQ-006 | Can we reproduce and challenge the artifacts? | pass_with_risk | provenance gap risk | Name command capture and verification, but avoid implying full L1/L2 release evidence. | route-cli owner / Citation Auditor | Cite `docs/evidence/round5-demo-capture.md` and `docs/vtrace/VERIFICATION.md`. |
| RQ-007 | Are your sources complete? | pass | none | Use artifact-specific source posture, not a global yes/no. | route-data owner / Citation Auditor | Cite `docs/reports/source-operations-evidence-roadmap.md`. |
| RQ-008 | Did the simulation or game prove the system works? | pass | none | Keep simulation/game as teaching and stress-test surfaces. | game/system designer / V&V | Cite `docs/reports/simulation-game-evidence-boundary.md`. |

## Venue Add-On Scores

| Venue Question | Score | Repair Note | Evidence Step |
|---|---|---|---|
| Are you using national freight language to steamroll local impacts? | pass_with_risk | Add a short answer that names local intake, environmental/community health lanes, and dissent preservation together. | Link local/regional intake template and role review. |
| Does a state value brief imply state commitment? | pass | Keep "intake and framing surface" wording. | Cite state value brief and state intake payload. |
| Who owns cross-border commitments? | pass | Keep governance non-claim explicit. | Cite state-to-AASHTO regional packet. |
| Can an elected official claim local funding is coming? | pass | Keep funding/eligibility claims held. | Cite political and funder briefs. |
| What happens when a DOT reviewer rejects a claim? | pass_with_risk | Add "the rejection becomes a review record and claim trace update" to presenter language. | Cite claim-promotion trace and verification gate. |

## Repairs Before Next Internal Dry Run

| Repair ID | Issue | Required Change | Owner |
|---|---|---|---|
| REPAIR-RQ-004 | Score answer could still sound like "trust the score." | Add presenter wording: "The total is an index for review; dimensions, confidence, and sensitivity decide whether a claim can move." | Numeracy Checker / route-score owner |
| REPAIR-RQ-006 | Reproducibility answer could imply full release evidence. | Add presenter wording: "We can inspect current command capture; L1/L2 release evidence is still held where public/browser/readiness claims are made." | route-cli owner / Citation Auditor |
| REPAIR-LOCAL-001 | Local impact answer needs one compact line. | Add presenter wording tying local intake, environmental/community health, non-driving access, delivery, rural access, and dissent rows together. | Scope Keeper / affected roles |
| REPAIR-DOT-001 | DOT rejection answer should state the artifact consequence. | Add presenter wording: "Rejected claims are revised, downgraded, held, or split into source-needed tasks." | review steward |

## Claims Removed Or Kept Out

The drill kept these claims out of the presenter answer set:

- Interstate 2.0 is an adopted plan.
- Any corridor, hub, interchange, or standard should be built.
- Any map proves service, asset, terminal, environmental, or construction
  readiness.
- Any ROI, benefit-cost, funding, eligibility, or program claim is ready.
- Any source-backed stakeholder fixture already exists.
- Any simulation/game result proves operational readiness.
- Any internal pass means external/public readiness.

## New Source-Pack Or Fixture Needs

No new source-pack category was created by this drill. The same priority need
remains:

| Need | Why It Matters |
|---|---|
| Populated source-backed stakeholder fixture | It is the blocker that would let the package demonstrate a real requirement-to-refinement loop instead of only a template/runbook path. |
| Venue-specific role review | Required before the rude Q&A answers can be used in a real sponsor, state, regional, congressional, or DOT setting. |
| L1/L2 readiness evidence or explicit exclusion | Required before any public, browser, release, game, or publication-readiness claim is made. |

## Closeout

Overall decision: **pass_with_risk**

Fail rows: none.

Hold rows: none in the required drill set; external use remains held by the
external readiness gate.

Pass-with-risk repairs: RQ-004, RQ-006, local impacts, DOT rejection handling.

Next rehearsal condition: apply the four repair lines above, then run a
closed-book sponsor-to-DOT dry run or a venue-specific rude Q&A drill using the
external rehearsal packet template.
