---
name: ROUTE Media Discovery Stress Test 001
slug: route-media-discovery-stress-test-001
type: review
status: draft
rubric_version: v1.0
author: codex
created: 2026-06-18
updated: 2026-06-18
sources:
  - docs/media/README.md
  - docs/media/media-fact-sheet.md
  - docs/media/media-claim-guide.md
  - docs/media/media-q-and-a.md
  - docs/media/media-source-index.md
  - docs/media/media-visual-assets.md
  - docs/reports/route-evidence-posture.md
  - docs/reports/industry-stakeholder-evidence-lane-matrix.md
  - docs/reports/industry-stakeholder-fixture-closeout-report.md
  - docs/reviews/communications-pressure-test-run-003.md
  - docs/reviews/external-rehearsal-technical-prevenue-closeout-001.md
  - docs/vtrace/COMMUNICATIONS_STRATEGY.md
  - docs/vtrace/VERIFICATION.md
---

# ROUTE Media Discovery Stress Test 001

## Scope

This stress test simulates a media reader finding ROUTE before a briefing and
trying to write, edit, fact-check, or challenge a short description of the
project.

It is not a press release, media launch, publication approval, public-readiness
gate, external rehearsal, agency review, endorsement, official-plan status,
construction readiness, guaranteed service, numeric ROI, eligibility,
compliance, external validation, or external readiness.

## Scenario

Assumption: a reporter or editor arrives at the repo/site with no live briefing
and starts from `docs/media/README.md`.

The package passes this internal stress test only if the reader can quickly find:

| Need | Required Source |
|---|---|
| One-sentence project description | `docs/media/media-fact-sheet.md` |
| What can and cannot be said | `docs/media/media-claim-guide.md` |
| Deadline Q&A | `docs/media/media-q-and-a.md` |
| Claim-to-source map | `docs/media/media-source-index.md` |
| Visual/caption boundary | `docs/media/media-visual-assets.md` |
| Controlling evidence boundary | `docs/reports/route-evidence-posture.md` |
| Stakeholder representation boundary | `docs/reports/industry-stakeholder-evidence-lane-matrix.md` |
| Current external-rehearsal hold | `docs/reviews/external-rehearsal-technical-prevenue-closeout-001.md` |

## Simulated Participants

| Participant | Pressure Applied | Pass Standard |
|---|---|---|
| Deadline transportation reporter | Needs a short accurate description and one usable quote line. | Uses "evidence-bounded research and tooling project" or equivalent, not "official plan." |
| Editor / fact-checker | Needs a source for each concrete claim. | Uses the source index and evidence posture before publishing a stronger claim. |
| DOT public affairs reviewer | Looks for implied agency approval or federal/state adoption. | Finds explicit non-endorsement and no-agency-review language. |
| Local/community reader | Looks for whether maps imply local construction or impact decisions. | Finds maps-not-proof and visual-caption boundaries. |
| Freight/industry reader | Looks for whether the package claims industry validation. | Finds representation-versus-validation language and fixture boundaries. |
| Technical skeptic | Looks for whether the internal technical lane is being described as outside review. | Finds pre-venue closeout and external-rehearsal holds. |

## Test Run

| Test | Reader Question | Expected Path | Result | Notes |
|---|---|---|---|---|
| T-001 | "What is ROUTE in one sentence?" | Media README to fact sheet. | pass | One-sentence description is concise and bounded. |
| T-002 | "Is Interstate 2.0 an official government plan?" | Fact sheet and claim guide. | pass | Multiple files say no official-plan or agency-adoption claim. |
| T-003 | "Can I say this proves a 48-hour freight network?" | Fact sheet, claim guide, evidence posture. | pass_with_risk | Safe wording exists, but the reader must distinguish planning target from guaranteed service. |
| T-004 | "Can I use the maps?" | Media visual assets and maps-not-proof report. | pass | Required caption pattern is explicit. |
| T-005 | "Can I say stakeholders validated this?" | Media source index and industry/stakeholder matrix. | pass | Representation is separated from validation. |
| T-006 | "Can I cite Port NOLA as a supporter?" | Media Q&A and source index. | pass | Public source use is bounded; endorsement is blocked. |
| T-007 | "Can I say the package passed DOT review?" | Pressure-test run 003, pre-venue closeout, external readiness gate. | pass | Internal dry run and external held status are visible. |
| T-008 | "Where is the strongest source for a claim?" | Media source index. | pass_with_risk | Source index is strong but long; readers should use the most specific row. |
| T-009 | "Can I publish a public launch story from this?" | Media README, Q&A, evidence posture. | pass | Public-readiness and launch claims remain held. |
| T-010 | "What should I verify outside the repo?" | Media README, Q&A, source-use rules. | pass | Agency positions, numbers, current sources, and readiness claims require outside verification. |

## Findings

| Finding | Severity | Repair / Operating Rule |
|---|---|---|
| The media package is usable as a discovery surface with claim holds. | pass | Keep media users anchored on README, fact sheet, claim guide, Q&A, source index, and visual assets. |
| Freight and service-window language remains the easiest place for a reader to overstate the story. | pass_with_risk | Repeat "planning target, not guarantee" in any media-facing freight answer. |
| The source index is comprehensive but dense. | pass_with_risk | Use quote-ready source pointers for quick navigation and the full index for fact-checking. |
| Industry/stakeholder language is safe if "represented" and "source-gated" are used. | pass | Do not shorten it to "validated." |
| Technical lane language is safe after the pre-venue closeout. | pass | Say internal technical pre-venue stack is complete; external use still needs a real venue packet. |

## Approved Media Discovery Language

| Situation | Safe Language |
|---|---|
| One-line description | "ROUTE is an evidence-bounded research and tooling project exploring an Interstate 2.0 service hierarchy for roads." |
| Maps | "ROUTE maps are structural service visuals with held-claim captions." |
| Freight promise | "The 48-hour freight idea is a planning target for asking what evidence would be needed, not a guaranteed operating commitment." |
| Stakeholder coverage | "The package represents major stakeholder and industry lanes through roles, reports, and bounded internal fixtures; outside validation is not claimed." |
| Current readiness | "The media materials are reference resources with claim holds; external rehearsal and public-readiness claims remain held." |

## Non-Approved Media Discovery Language

- ROUTE is an official federal, state, regional, or agency plan.
- ROUTE has been approved, endorsed, validated, or accepted by USDOT, FHWA, a
  state DOT, Port NOLA, industry, community stakeholders, or external reviewers.
- ROUTE proves construction readiness, guaranteed service, positive ROI,
  eligibility, compliance, environmental clearance, or funding priority.
- ROUTE maps prove routes, upgrades, terminal access, asset condition,
  resilience benefit, or public readiness.
- The internal technical pre-venue stack is a real technical review.

## Next Work

1. If a public site or README landing surface is edited, link the media README,
   fact sheet, claim guide, Q&A, source index, and visual-assets guide together
   as the first media path.
2. If a story, briefing, or press inquiry becomes real, fill a venue/source
   specific media packet and rerun prohibited-claim, source-custody, and L0
   checks.
3. Keep publication, public-readiness, external-rehearsal, agency-review,
   endorsement, construction, SLA, ROI, eligibility, and compliance claims held.

## Validation Closeout

| Check | Command / Inspection | Result | Evidence |
|---|---|---|---|
| Media discovery inspection | compare media README, fact sheet, claim guide, Q&A, source index, visual assets, evidence posture, and stakeholder matrix | pass | stress test wired into media README, source index, strategy, verification, and evidence posture |
| Prohibited-claim scan | scan stress test and linked edited surfaces for promoted prohibited claims | pass | hits are guardrail, held, do-not-infer, or non-approved contexts |
| L0 | `npm run check:l0` | pass | workspace lib/bin tests passed |

## Gate

Decision: **media_discovery_pass_with_risk; public_readiness_held**

Rationale: The media package can support a reporter or editor who discovers the
repo before a briefing, provided they follow the source index, visual captions,
and claim guide. Public-readiness, external-rehearsal, agency-review,
endorsement, construction, service, ROI, eligibility, and compliance claims
remain held.
