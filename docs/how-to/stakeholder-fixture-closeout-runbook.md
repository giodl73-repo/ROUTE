---
name: Stakeholder Fixture Closeout Runbook
slug: stakeholder-fixture-closeout-runbook
type: how-to
status: draft
rubric_version: v1.0
author: codex
created: 2026-06-17
updated: 2026-06-17
sources:
  - docs/templates/source-packs/stakeholder-fixture-source-pack-template.md
  - docs/reviews/communications-role-review-pass-artifacts.md
  - docs/reviews/communications-pressure-test-run-001.md
  - docs/evidence/round5-demo-capture.md
  - docs/traces/route-claim-promotion-trace.md
  - docs/vtrace/VERIFICATION.md
---

# Stakeholder Fixture Closeout Runbook

## Purpose

This runbook turns a real stakeholder requirement into a source-backed
requirement-to-refinement fixture.

It exists because the current 225-mile Round 5 fixture is useful threshold
sensitivity, but not a state, industry, rural, local, transit, or community
requirement backed by an external source.

This runbook does not create endorsement, official-plan status, construction
readiness, guaranteed service, numeric ROI, eligibility, compliance, or public
readiness.

## Entry Criteria

| Item | Required Condition |
|---|---|
| Stakeholder requirement | A real requirement exists in an intake note, public plan, meeting record, dataset, letter, or other source artifact. |
| Source custody | The source can be named by owner, title, date/year, path or access note, and reviewer. |
| Affected ROUTE artifact | The requirement can change a candidate row, hold label, source pack, map caption, demo command, or evidence label. |
| Role lanes | At least one affected stakeholder lane and the editorial gates are named. |
| Non-claims | Presenter and review materials keep endorsement, official-plan, construction, guaranteed-SLA, numeric ROI, eligibility, compliance, and public-readiness claims held. |

If any entry criterion is missing, the fixture stays `held_template`.

## Step 1: Capture Source Custody

Create or copy a fixture source pack from:

```text
docs/templates/source-packs/stakeholder-fixture-source-pack-template.md
```

Fill only fields that are backed by the source. Do not normalize uncertain
claims into stronger language.

Minimum source custody:

| Field | Rule |
|---|---|
| Source Path / URL | Use a repo path for archived material, a URL for public material, or an access note for restricted material. |
| Title | Use the source title, meeting title, dataset name, or record label. |
| Publisher / Owner | Name the agency, organization, meeting body, or data owner. |
| Date / Year | Use the specific date when available; otherwise use the year and mark precision. |
| Access Note | Say public, provided, restricted, meeting notes, or unavailable. |
| Units / Field Names | Required for any number, threshold, quantity, or measurement. |
| Reviewer | Name the role lane or reviewer responsible for custody. |

## Step 2: Convert Requirement To Artifact Change

The fixture must show a before/after change. Acceptable changes are limited to:

| Change Type | Acceptable Output |
|---|---|
| Candidate row | A new, changed, rejected, or held candidate row with source status. |
| Hold label | A claim moves from story-ready or heuristic to source-needed or held, or from source-needed to source-backed. |
| Source pack | A blank evidence field becomes source-backed, or a missing source is explicitly recorded as a blocker. |
| Map caption | A map caption gains a stricter label, excluded-claim note, or artifact pointer. |
| Demo command | A command, threshold, input, or output path is captured as a fixture. |
| Evidence label | A trace or evidence row changes label with a reason and next step. |

Rejected or held rows are valid fixture outcomes. A fixture does not need to
produce a positive candidate.

## Step 3: Record Before / After Evidence

Use this minimum evidence shape:

| Field | Required Entry |
|---|---|
| Fixture ID | Stable identifier. |
| Requirement | Plain-English requirement, quoted or paraphrased from the source. |
| Source ID | Source row from the fixture source pack. |
| Before artifact / label | Path, row, label, or claim posture before the source-backed requirement. |
| Change applied | Threshold, field, label, command, source pack row, or caption change. |
| After artifact / label | Path, row, label, or claim posture after the change. |
| Role hold / dissent | Any objection from affected role lanes. |
| Claim allowed? | `no`, `internal only`, `story-ready`, `source-needed`, or `held`. |
| Next evidence step | What would be required to strengthen the claim. |

## Step 4: Run Role Review

At minimum, run:

| Role | Required For |
|---|---|
| Scope Keeper | Every fixture. |
| Citation Auditor | Every source-backed row. |
| Numeracy Checker | Every numeric field, threshold, unit, cost, benefit, volume, or service window. |
| Optimization Methodologist | Every before/after artifact change. |

Then add affected lanes:

| Fixture Type | Add Roles |
|---|---|
| State or delivery | State DOT Planner, Traffic Engineer. |
| Freight or industry | Freight Economist, Freight Industry, Long-Haul Trucker when applicable. |
| Rural or agricultural | Rural Advocate, Rural Farmer, Rural Resident when applicable. |
| Community or environmental | Foxx, Environmental Community, Local Official when applicable. |
| Transit or non-driving access | Transit-Dependent Traveler, Intercity Traveler when applicable. |
| Map or stop visibility | Schematic Cartographer. |
| Resilience or hazard | Climate Resilience Engineer. |

Role review can pass, pass_with_risk, hold, or fail. A hold is acceptable when
the fixture preserves the reason and next evidence step.

## Step 5: Close Or Hold The Fixture

| Decision | Meaning |
|---|---|
| pass_internal | Fixture has source custody, before/after evidence, and role review; usable in internal rehearsal. |
| pass_with_risk | Fixture is usable internally but has named evidence or role holds. |
| held_source | Source custody is incomplete or restricted. |
| held_artifact | No reproducible artifact or label change exists. |
| held_role | A required role lane blocks promotion. |
| fail_scope | Fixture promotes endorsement, official-plan, construction, guaranteed-SLA, numeric ROI, eligibility, compliance, or public-readiness claims. |

Only `pass_internal` or `pass_with_risk` can support another internal Round 5
rehearsal. Neither decision supports public or external readiness by itself.

## Closeout Checklist

| Item | Pass / Hold | Evidence |
|---|---|---|
| Source custody row filled. |  |  |
| Requirement row filled. |  |  |
| Before/after artifact or label captured. |  |  |
| Editorial roles reviewed. |  |  |
| Affected stakeholder lanes reviewed. |  |  |
| Dissent or hold preserved. |  |  |
| Prohibited-claim scan passes. |  |  |
| `docs/traces/route-claim-promotion-trace.md` updated if claim posture changes. |  |  |
| `docs/evidence/round5-demo-capture.md` or successor evidence record updated if command evidence changes. |  |  |
| `docs/vtrace/VERIFICATION.md` updated if Round 5 gate status changes. |  |  |

## Gate

Decision: held_until_populated_fixture

Rationale: This runbook makes the remaining Round 5 blocker executable. It does
not close that blocker until a real source-backed requirement is captured,
converted into an artifact or label change, reviewed through `.roles`, and
recorded without prohibited claims.
