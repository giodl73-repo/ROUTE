---
name: External Rehearsal Technical Demo Script
slug: external-rehearsal-technical-demo-script
type: how-to
status: draft
rubric_version: v1.0
author: codex
created: 2026-06-18
updated: 2026-06-18
sources:
  - docs/how-to/run-route-demo.md
  - docs/evidence/round5-demo-capture.md
  - docs/traces/route-claim-promotion-trace.md
  - docs/reports/requirement-to-refinement-demonstration-report.md
  - docs/reviews/external-rehearsal-packet-candidate-001.md
  - docs/reviews/external-rehearsal-technical-repair-closeout-001.md
  - docs/reviews/communications-external-rehearsal-readiness.md
  - docs/decks/split-deck-presenter-guide.md
---

# External Rehearsal Technical Demo Script

## Purpose

This script compresses the ROUTE technical story into a five-minute internal
demo for the FHWA/USDOT-style technical candidate lane.

The demo shows one loop:

```text
requirement -> source posture -> command -> artifact -> hold -> next evidence
```

It is not a real FHWA, USDOT, state DOT, sponsor, technical reviewer, or agency
review. It does not create external validation, acceptance, endorsement,
official-plan status, construction readiness, guaranteed service, numeric ROI,
eligibility, compliance, public readiness, approval, technical signoff, or
external readiness.

## Setup

| Field | Value |
|---|---|
| Demo length | five minutes |
| Audience | internal FHWA/USDOT-style technical reviewer candidate |
| Presenter stance | show the machine loop, not a finished plan |
| Source posture | artifact-specific; identify source owner, artifact, and access note before using an example |
| Command posture | captured internal command evidence; not release, SLA, or public-readiness proof |
| Map posture | structural visual only with held-claim caption |
| Closing ask | evidence review or demo fixture, not decision, clearance, funding, eligibility, or endorsement |

## Five-Minute Run Of Show

| Time | Presenter Action | Artifact / Evidence | Boundary Sentence |
|---|---|---|---|
| 0:00-0:30 | Open with the loop. | This script; candidate packet. | "This is an internal technical demo candidate, not a real agency review." |
| 0:30-1:10 | State the requirement. | `docs/how-to/run-route-demo.md`; Round 5 fixture. | "A reviewer tightens a planning requirement; ROUTE must show what changes and what stays held." |
| 1:10-1:50 | Show source posture. | Source operations roadmap; claim trace. | "Source posture is artifact-specific; tell me which source owner, artifact, and access note controls the example." |
| 1:50-2:40 | Show the command and captured artifact. | `docs/evidence/round5-demo-capture.md` DEMO-CMD-009 / DEMO-CMD-010. | "Command capture is inspectable internal evidence, not release or operating proof." |
| 2:40-3:25 | Show the artifact change. | 250-mile header-only docket to 225-mile candidate/promotions scaffold. | "The output is a selected artifact under declared constraints, not a final national answer." |
| 3:25-4:10 | Show the hold. | Claim trace rows TRACE-CLAIM-007 / TRACE-CLAIM-008. | "This is a review index and source-needed scaffold, not a recommendation." |
| 4:10-4:45 | Show the next evidence task. | Candidate role review; external readiness checklist. | "A promoted claim needs source custody, role review, prohibited-claim scan, and the selected validation gate." |
| 4:45-5:00 | Close with a narrow ask. | External packet template / evidence review. | "The ask is evidence review or a demo fixture; decisions and approvals are outside this candidate." |

## Speaker Script

### 0:00-0:30 - Open

Use:

> This is a five-minute internal technical demo candidate. It is not a real
> FHWA, USDOT, state DOT, sponsor, or agency review. The point is to show the
> ROUTE machine loop: requirement, source posture, command, artifact, hold, and
> next evidence.

Avoid:

- saying the package is externally ready;
- naming an agency as a reviewer;
- saying the demo proves a plan, project, service, funding path, or release.

### 0:30-1:10 - Requirement

Use:

> The requirement is a planning stress test: what changes if the stop/SLA
> spacing requirement tightens below the current passing threshold?

Point to:

- `docs/how-to/run-route-demo.md`
- `docs/evidence/round5-demo-capture.md`

Boundary:

> A requirement is an input to the evidence machine, not an approval.

### 1:10-1:50 - Source Posture

Use:

> Source posture is artifact-specific; tell me which source owner, artifact,
> and access note controls the example. If we cannot answer that, the claim
> becomes source-needed or held.

Point to:

- `docs/traces/route-claim-promotion-trace.md`
- `docs/reports/source-operations-evidence-roadmap.md`

Boundary:

> Source workflow is not source completeness.

### 1:50-2:40 - Command

Use:

> The captured command bundle shows what an internal reviewer can inspect:
> command, output path, observed status, row count or size, pass/hold status,
> and non-claim label.

Show:

```powershell
cargo run -q -p route -- stop-sla-candidates --input target\demo\beck-stop-sla-demo.csv --output target\demo\beck-stop-sla-candidates-225-demo.csv --target-gap 225 --top 5 --gate
cargo run -q -p route -- stop-sla-promotions --input target\demo\beck-stop-sla-candidates-225-demo.csv --output target\demo\beck-stop-sla-promotions-225-demo.csv --gate
```

Boundary:

> Command capture is inspectable internal evidence, not release, SLA, or
> public-readiness proof.

### 2:40-3:25 - Artifact Change

Use:

> At the current 250-mile planning threshold, the candidate docket is
> header-only. At the 225-mile stress threshold, the system surfaces candidate
> rows and a promotion scaffold. That is the change: the requirement alters the
> artifact and the evidence posture.

Point to:

- `target\demo\beck-stop-sla-candidates-demo.csv`
- `target\demo\beck-stop-sla-candidates-225-demo.csv`
- `target\demo\beck-stop-sla-promotions-225-demo.csv`

Boundary:

> No final national answer is claimed; this is a selected artifact under
> declared constraints.

### 3:25-4:10 - Hold

Use:

> The promotion scaffold is not a recommendation. It is a review index and a
> source-needed scaffold. The next step is not "build it"; the next step is
> source custody, role review, and validation selection.

Point to:

- TRACE-CLAIM-007
- TRACE-CLAIM-008
- `docs/reports/route-evidence-posture.md`

Boundary:

> This is a review index, not a recommendation.

### 4:10-4:45 - Next Evidence

Use:

> If a reviewer challenges a row, ROUTE should downgrade, split, hold, or
> convert it into a source, command, role-review, or validation task. Rejection
> is a review record, not a problem to hide.

Point to:

- `docs/reviews/external-rehearsal-technical-candidate-role-review.md`
- `docs/reviews/communications-external-rehearsal-readiness.md`

Boundary:

> Venue-specific source custody and role review are still the bottleneck.

### 4:45-5:00 - Close

Use:

> The safe ask is technical evidence review or a demo fixture. Decisions,
> clearances, funding, eligibility, compliance, official policy, public
> readiness, and external readiness remain outside this candidate.

## What The Demo Shows

| Claim | Status |
|---|---|
| ROUTE can turn a planning requirement into a generated artifact and evidence posture. | internal / implemented with heuristic and source-needed labels |
| ROUTE can show a before/after stress threshold artifact. | internal / pass_with_risk |
| ROUTE can expose holds instead of hiding them. | internal / pass_with_risk |
| ROUTE has a final optimizer answer. | held |
| ROUTE has an agency-reviewed plan. | held |
| ROUTE has a construction, SLA, ROI, eligibility, compliance, public-readiness, or external-readiness proof. | held |

## Failure Responses

| Reviewer Push | First Sentence |
|---|---|
| "Are the sources complete?" | "Source posture is artifact-specific; tell me which source owner, artifact, and access note controls the example." |
| "Did the optimizer find the answer?" | "No final national answer is claimed; this is a selected artifact under declared constraints." |
| "Does this tell us what to fund?" | "This is a review index, not a recommendation." |
| "Can we show this publicly?" | "Not from this candidate; public readiness needs its own scoped gate." |
| "Can FHWA/USDOT decide from this?" | "No; the safe ask is evidence review or a demo fixture." |

## Validation Closeout

| Check | Command / Inspection | Result | Evidence |
|---|---|---|---|
| Script boundary inspection | requirement, source posture, command, artifact, hold, and next-evidence loop reviewed | pass | script sections preserve first-sentence claim boundaries |
| Prohibited-claim scan | scan touched demo script, candidate, readiness, media, posture, verification, strategy, and presenter files | pass | hits are red-line, held, do-not-infer, or non-approved contexts |
| L0 | `npm run check:l0` | pass | workspace lib/bin tests passed |

## Gate

Decision: **demo_script_ready_for_internal_technical_rehearsal; hold_external_rehearsal**

Rationale: The script gives the technical candidate a short, inspectable
requirement-to-evidence loop with first-sentence claim boundaries. It does not
close venue-specific source custody, role review, external packet selection,
public readiness, or agency-facing review.
