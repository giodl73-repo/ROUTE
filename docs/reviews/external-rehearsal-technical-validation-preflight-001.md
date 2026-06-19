---
name: External Rehearsal Technical Validation Preflight 001
slug: external-rehearsal-technical-validation-preflight-001
type: review
status: draft
rubric_version: v1.0
author: codex
created: 2026-06-18
updated: 2026-06-18
sources:
  - docs/reviews/communications-external-rehearsal-readiness.md
  - docs/reviews/external-rehearsal-technical-venue-role-preflight-001.md
  - docs/reviews/external-rehearsal-technical-source-custody-preflight-001.md
  - docs/how-to/external-rehearsal-technical-venue-packet-scaffold.md
  - docs/templates/external-rehearsal-packet-template.md
  - docs/vtrace/VERIFICATION.md
  - package.json
---

# External Rehearsal Technical Validation Preflight 001

## Scope

This preflight defines validation requirements for a future named
FHWA/USDOT-style technical venue packet.

It is not a validation closeout for a real packet. It is not a filled packet,
agency review, technical signoff, endorsement, official-plan status,
construction readiness, guaranteed service, numeric ROI, eligibility,
compliance, public readiness, approval, external validation, or external
readiness.

## Overall Decision

Decision: **technical_validation_preflight_ready; hold_external_rehearsal**

Rationale: The technical lane now has packet, source-custody, and role-review
preflights. This document makes the validation closeout executable once a real
venue packet exists, while keeping all external-use claims held.

## Entry Conditions

Do not run a packet validation closeout until these inputs exist:

| Input | Required Before Validation Closeout | Current Status |
|---|---|---|
| Named venue packet | Filled from `docs/templates/external-rehearsal-packet-template.md`. | hold |
| Selected material set | Included and excluded files are listed. | hold |
| Selected source custody rows | Rows are accepted, replaced, or removed for the named venue. | hold |
| Venue role review | Required role lanes rerun against selected venue and materials. | hold |
| Closing ask | Technical evidence review or demo fixture, not approval. | proposed only |
| Claim scope | L0/L1/L2 needs are selected based on actual claims in the packet. | hold |

## Required Validation Matrix

| Validation Item | When Required | Command / Inspection | Pass Condition | If Missing |
|---|---|---|---|---|
| Prohibited-claim scan | every named technical packet | `rg` selected packet and material set for promoted official-plan, construction, SLA, ROI, eligibility, compliance, endorsement, approval, public-readiness, external-readiness, and agency-review claims | Hits are only red-line, held, do-not-infer, or non-approved contexts. | hold_external_rehearsal |
| L0 | every named technical packet after edits | `npm run check:l0` | Workspace lib/bin tests pass after packet edits. | hold_external_rehearsal |
| Stale-language scan | every named technical packet | scan selected packet for copied candidate-only, pending, or unfilled placeholder language | No stale candidate-only or placeholder text remains except explicit holds. | hold_external_rehearsal |
| Source-custody inspection | every named technical packet | compare selected custody rows with packet materials and role review | Every concrete example has owner, title, date/year, path/access note, and reviewer. | hold_external_rehearsal |
| Role-review inspection | every named technical packet | compare filled role rows with venue role preflight | Required roles have venue-specific results and holds. | hold_external_rehearsal |
| L1 | if packet uses implementation, command reproducibility, optimizer, graph/scoring, or technical artifact claims beyond docs-only posture | `npm run check:l1` or recorded package-specific confidence | Full repo or package-specific confidence is recorded and scoped to the claim. | hold or downgrade claim |
| L2 | only if packet claims browser, game, release, publication, public-readiness, or external-readiness behavior | `npm run check:l2` or scoped release/readiness evidence | Integration/readiness evidence passes and claim remains scoped. | remove or hold the claim |

## Prohibited-Claim Scan Pattern

Use a selected-material scan equivalent to:

```powershell
rg -n "official[- ]plan|construction-ready|construction readiness|guaranteed[- ]SLA|guaranteed service|positive ROI|numeric ROI|eligibility|compliance|endorsement|approval|public readiness|external readiness|technical signoff|agency review|external validation" <selected packet files>
```

The scan can pass only if every hit is a guardrail, explicit hold,
do-not-infer statement, or non-approved claim.

## L1 / L2 Escalation Rules

| Claim In Named Packet | Minimum Gate | Boundary |
|---|---|---|
| Docs-only technical evidence review | prohibited-claim scan + L0 | Does not prove external readiness. |
| Command reproducibility or implementation behavior | L1 or package-specific confidence | Must name exact command/artifact and claim scope. |
| Optimizer or graph/scoring behavior beyond story posture | L1 or package-specific confidence plus role review | Does not prove final optimizer answer or funding priority. |
| Map/publication/release/browser/game readiness | L2 or scoped release/readiness evidence | Do not include unless the packet explicitly selects those claims. |
| Public readiness, external readiness, agency approval, or technical signoff | not allowed from this preflight | Requires real venue packet and separate closeout evidence; still not assumed. |

## Validation Output Template

Use this table in the named packet.

| Check | Command / Inspection | Result | Evidence |
|---|---|---|---|
| Prohibited-claim scan |  | pass / hold / fail |  |
| Stale-language scan |  | pass / hold / fail |  |
| Source-custody inspection |  | pass / hold / fail |  |
| Venue role-review inspection |  | pass / hold / fail |  |
| L0 | `npm run check:l0` | pass / hold / fail |  |
| L1 | `npm run check:l1` or scoped confidence | pass / hold / scoped out |  |
| L2 | `npm run check:l2` or scoped readiness evidence | pass / hold / scoped out |  |

If any required row is blank, the packet remains `hold_external_rehearsal`.

## Failure Modes

| Failure Mode | Required Repair |
|---|---|
| Validation is run before the venue packet is filled. | Keep result as internal only; rerun after final packet edits. |
| L0 passes and is treated as claim validation. | Reframe L0 as repo sanity only; use source custody and role review for claims. |
| L1/L2 is skipped while technical or release claims are promoted. | Run the scoped gate or remove the claim. |
| Scan hits are ignored because they appear in a table. | Classify every hit as guardrail, held, do-not-infer, non-approved, or promoted. |
| Selected materials change after validation. | Rerun prohibited-claim scan and L0; rerun L1/L2 if selected claims changed. |

## Next Work

1. When a real venue packet exists, run this validation matrix after final
   material, custody, and role-review edits.
2. Record validation results in the filled packet before any external use.
3. Keep all external rehearsal, agency review, technical signoff, approval,
   endorsement, public readiness, construction, service, ROI, eligibility, and
   compliance claims held until selected validation closes.

## Validation Closeout

| Check | Command / Inspection | Result | Evidence |
|---|---|---|---|
| Validation preflight inspection | compare readiness checklist, verification plan, package scripts, source custody preflight, and role preflight | pass | validation matrix and escalation rules recorded above |
| Prohibited-claim scan | scan preflight and linked edited surfaces for promoted prohibited claims | pass | hits are guardrail, held, do-not-infer, or non-approved contexts |
| L0 | `npm run check:l0` | pass | workspace lib/bin tests passed |

## Gate

Decision: **technical_validation_preflight_ready; hold_external_rehearsal**

Rationale: The technical lane now has an executable validation preflight for a
future named packet. External use remains held until a real venue packet exists,
selected materials, custody rows, venue role review, validation closeout, and
any claim-specific L1/L2 evidence are recorded.
