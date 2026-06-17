# Communications Rude Q&A Drill Scorecard

## Purpose

This scorecard turns `docs/reviews/communications-rude-qa.md` into a repeatable
internal rehearsal.

The drill is intentionally adversarial. A reviewer asks blunt questions from the
rude Q&A, the presenter answers in real time, and the recorder scores whether
the answer preserved evidence posture, named the supporting artifact, blocked
unsupported claims, and identified the next evidence step.

This scorecard is internal prep. It does not represent a real sponsor, state,
regional, congressional, FHWA, USDOT, stakeholder, or agency review. It does not
create an official plan, construction recommendation, service guarantee, ROI
finding, eligibility finding, compliance finding, endorsement, approval, or
public-readiness claim.

## Roles

| Role | Job |
|---|---|
| Presenter | Answers each hostile question without checking notes unless the drill is explicitly marked open-book. |
| Hostile reviewer | Reads questions, interrupts vague answers, and asks one follow-up when an answer overclaims or hides a blocker. |
| Scope Keeper | Flags official-plan, construction, guarantee, public-readiness, approval, or endorsement drift. |
| Numeracy Checker | Flags ROI, cost, benefit, score, ranking, flow, allocation, or quantified value drift. |
| Citation Auditor | Flags missing source, command, artifact path, or provenance. |
| Recorder | Captures score, repair note, owner, and required evidence step. |

## Scoring Rubric

| Score | Meaning | Required Action |
|---|---|---|
| pass | Answer names posture, artifact, blocked claim, and next evidence step without overclaiming. | Keep answer; optionally add to presenter guide. |
| pass_with_risk | Answer stays safe but misses one artifact, role, or evidence step. | Repair wording before external-readiness packet use. |
| hold | Answer is safe only because it refuses the claim, but it cannot name the next artifact or owner. | Add evidence-step note or route to source-pack/runbook backlog. |
| fail | Answer implies adoption, construction, guaranteed service, numeric ROI, eligibility, compliance, endorsement, approval, public readiness, or unsupported proof. | Remove or rewrite before any further rehearsal. |

## Required Drill Set

Each rehearsal must cover at least these questions:

| ID | Question | Required Artifact To Name |
|---|---|---|
| RQ-001 | Is this an actual official Interstate 2.0 plan? | `docs/reports/route-evidence-posture.md` |
| RQ-002 | Are you saying these corridors should be built? | `docs/traces/route-claim-promotion-trace.md` |
| RQ-003 | Where are the real ROI numbers? | `docs/reports/route-roi-cost-framework.md` |
| RQ-004 | Why should anyone trust the score? | `docs/reports/graph-scoring-measurement-appendix.md` and `docs/DIMENSIONS.md` |
| RQ-005 | Why not just publish this? | `docs/reviews/communications-external-rehearsal-readiness.md` |
| RQ-006 | Can we reproduce and challenge the artifacts? | `docs/evidence/round5-demo-capture.md` and `docs/vtrace/VERIFICATION.md` |
| RQ-007 | Are your sources complete? | `docs/reports/source-operations-evidence-roadmap.md` |
| RQ-008 | Did the simulation or game prove the system works? | `docs/reports/simulation-game-evidence-boundary.md` |

Optional add-ons should be selected by venue:

| Venue | Add Questions From |
|---|---|
| Local / regional | Local impacts, objections, rural access, map bypass concerns. |
| State | STIP/LRTP, ROW, maintenance, environmental process, state authority. |
| AASHTO region | cross-border scope, bundle identity, regional sensitivity, governance non-claims. |
| Congressional | federal funding ask, district examples, ROI, community harms. |
| DOT technical | command provenance, source custody, scoring assumptions, L1/L2 readiness. |

## Score Sheet

| Question ID | Score | Failure Mode If Any | Repair Note | Owner | Evidence Step |
|---|---|---|---|---|---|
| RQ-001 | pending |  |  | Scope Keeper |  |
| RQ-002 | pending |  |  | Scope Keeper |  |
| RQ-003 | pending |  |  | Numeracy Checker |  |
| RQ-004 | pending |  |  | route-score owner / Numeracy Checker |  |
| RQ-005 | pending |  |  | review steward |  |
| RQ-006 | pending |  |  | route-cli owner / Citation Auditor |  |
| RQ-007 | pending |  |  | route-data owner / Citation Auditor |  |
| RQ-008 | pending |  |  | game/system designer / V&V |  |

## Failure Modes

| Failure Mode | Example | Required Repair |
|---|---|---|
| proof jump | "The map shows the corridor is ready." | Replace with structural/held map posture and evidence gate. |
| authority jump | "This is the plan states should follow." | Replace with intake, evidence, and review framing. |
| funding jump | "This package should receive construction funding." | Replace with source pack, standards, evidence, demo, or bounded pilot ask. |
| ROI jump | "The corridor has positive ROI." | Replace with ROI evidence-contract language. |
| score jump | "The score proves the rank." | Replace with dimension/confidence/sensitivity review language. |
| generated-artifact jump | "The generated report proves it." | Replace with provenance and draft-inspection language. |
| readiness jump | "We can publish this now." | Replace with external-readiness and L1/L2 gate language. |
| consensus jump | "Stakeholders agree." | Replace with role review, dissent, and source-backed fixture language. |

## Pass Threshold

The drill passes for internal rehearsal when:

- every required question scores `pass` or `pass_with_risk`;
- no answer scores `fail`;
- no answer implies official adoption, construction, guaranteed service,
  numeric ROI, eligibility, compliance, endorsement, approval, or public-readiness claims;
- all `pass_with_risk` rows have owners and repair notes;
- all `hold` rows identify the missing source, command, role review, or fixture.

The drill remains held for external use until the external rehearsal readiness
checklist, source-backed fixture, affected role review, prohibited-claim scan,
and required validation gates close for the named venue.

## Closeout Note Template

Use this note after each drill:

```text
Rude Q&A drill date:
Venue simulated:
Presenter:
Hostile reviewer:
Recorder:

Overall decision: pass / pass_with_risk / hold / fail

Fail rows:
Hold rows:
Pass-with-risk repairs:
New source-pack or fixture needs:
Claims removed from script:
Next rehearsal condition:
```
