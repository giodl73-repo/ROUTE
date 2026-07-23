---
name: ROUTE Local Adaptation Worksheet
slug: route-local-adaptation-worksheet
type: worksheet
status: draft
rubric_version: v1.0
author: codex
created: 2026-07-22
updated: 2026-07-22
sources:
  - docs/adoption/README.md
  - docs/briefs/route-first-client-wedge-package.md
  - docs/reports/route-evidence-posture.md
  - docs/map-publication-scope.md
---

# ROUTE Local Adaptation Worksheet

## Purpose

Use this worksheet to scope a bounded ROUTE adaptation before creating an issue,
pull request, map fixture, diagnostic readout, or local review packet.

The goal is not to prove a finished plan. The goal is to make the network job,
source posture, evidence holds, and next review question visible.

## Scope

| Field | Answer |
|---|---|
| State, region, corridor, port, terminal district, or freight network |  |
| Primary service question |  |
| Places that must connect reliably |  |
| Unacceptable failure modes |  |
| Intended audience |  |
| What this worksheet must not claim |  |

## Source Surfaces

| Source Surface | Owner / Publisher | Path Or Link | Status |
|---|---|---|---|
| GIS or route inventory |  |  | source-needed |
| Traffic or reliability data |  |  | source-needed |
| Asset condition |  |  | source-needed |
| Incident, closure, or weather history |  |  | source-needed |
| Freight, port, toll, terminal, or logistics data |  |  | source-needed |
| Safety, environmental, or community constraint |  |  | source-needed |
| Existing plans or project lists |  |  | source-needed |

Use status values such as `source-backed`, `heuristic`, `held`,
`source-needed`, or `confidence-limited`.

## Candidate Roles

| Link, Node, Corridor, Or Area | Candidate Role | Why It Might Matter | Evidence Status |
|---|---|---|---|
|  | T1 national / statewide spine |  | source-needed |
|  | T2 regional connector |  | source-needed |
|  | T3 feeder / access mesh |  | source-needed |
|  | T4 terminal / local access |  | source-needed |
|  | R resilience overlay |  | source-needed |
|  | M maintenance-only / asset hold |  | source-needed |
|  | X explicit non-promotion |  | source-needed |

## Claim Boundary

| Claim Type | Current Disposition | Notes |
|---|---|---|
| Official plan or agency endorsement | held |  |
| Guaranteed service or legal SLA | held |  |
| Construction or engineering readiness | held |  |
| Numeric ROI, cost, or benefit claim | held |  |
| Funding, compliance, or procurement eligibility | held |  |
| External validation | held |  |
| Map publication readiness | held |  |

## First Readout

Write the first readout in five parts:

1. **Service question:** what should this scoped network promise?
2. **Candidate hierarchy:** which links, nodes, or areas might play which roles?
3. **Failure modes:** what breaks the promise before any solution is claimed?
4. **Evidence holds:** which claims are not safe yet?
5. **Next source asks:** what source, reviewer, or fixture would improve the
   claim posture?

## Review Lenses

Ask at least three lenses to review the worksheet before promoting a public
claim.

| Lens | Reviewer Question | Disposition |
|---|---|---|
| Source custody | Are the sources named, stable, and interpreted narrowly? |  |
| Traffic / operations | Do the roles match plausible network function? |  |
| Freight / logistics | Are operating pain points represented without fake guarantees? |  |
| State / MPO / port planning | Does this preserve authority and delivery boundaries? |  |
| Community / environmental | Are harms, exclusions, and local constraints visible? |  |
| Numeracy | Are all numbers sourced, unit-labeled, dated, and uncertainty-aware? |  |
| Public language | Would a reader mistake this for an official plan or endorsement? |  |

## Gate

Decision: **worksheet_only_until_reviewed**

Rationale: This worksheet can support adaptation, review, and source gathering.
It does not create official-plan, endorsement, guaranteed-SLA, construction,
ROI, funding, compliance, procurement, public-readiness, or validation claims.
