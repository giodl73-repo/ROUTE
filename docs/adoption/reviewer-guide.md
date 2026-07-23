---
name: ROUTE External Reviewer Guide
slug: route-external-reviewer-guide
type: guide
status: draft
rubric_version: v1.0
author: codex
created: 2026-07-22
updated: 2026-07-22
sources:
  - docs/adoption/README.md
  - docs/adoption/local-adaptation-worksheet.md
  - docs/reports/route-evidence-posture.md
  - docs/media/media-claim-guide.md
  - research/REVIEWERS.md
---

# ROUTE External Reviewer Guide

## Purpose

ROUTE benefits from skeptical review. The most useful review does not ask
whether Interstate 2.0 is exciting. It asks which claim is supported, which
claim is premature, which source is missing, and which affected perspective has
not been heard.

Use this guide when reviewing a README section, paper, map, diagnostic fixture,
local adaptation worksheet, or public-language claim.

## Good Review Targets

| Target | Useful Review Question |
|---|---|
| Public claim | Is the wording safe, sourced, and bounded? |
| Map or schematic | Does the caption make clear what the map does and does not prove? |
| Source interpretation | Does the cited source support the exact claim being made? |
| Service role | Does T1/T2/T3/T4/R/M/X match the network job, or is the role overstated? |
| Local adaptation | Are the scope, source surfaces, held claims, and next review step clear? |
| Research paper | Are the method, references, quantities, and panel objections addressed? |
| Contribution | Does it preserve source custody and evidence labels? |

## Review Lenses

Pick the lens you can actually defend.

| Lens | What To Look For |
|---|---|
| Freight / logistics | Commodity sensitivity, dwell, reliability, staging, backhaul, terminal access, and operating realism. |
| State DOT / MPO / port planning | Planning authority, delivery limits, project-list interpretation, public process, and source ownership. |
| Traffic engineering / operations | Bottlenecks, incident behavior, detours, access constraints, truck routing, and signal/signage realism. |
| Resilience / climate | Shared hazards, alternate-route independence, recovery windows, weather exposure, and adaptation limits. |
| Community / environmental | Displacement, safety, pollution, noise, local access, environmental review, and distributional effects. |
| Finance / funding / procurement | Eligible funding claims, procurement posture, cost basis, ROI gates, and sponsor realism. |
| Labor / safety | Driver conditions, relay assumptions, duty constraints, safety review, and workforce claims. |
| Numeracy | Units, dates, uncertainty, denominator choice, price year, arithmetic closure, and unsupported totals. |
| Public language | Overclaiming, agency-endorsement drift, implied construction readiness, and map-as-proof risk. |

## Review Output Format

Use this compact format for an issue, PR review, or review note:

```text
Lens:
Location:
Claim or artifact:
Concern:
Evidence or rationale:
Suggested disposition: pass / revise / hold / source-needed
Safer wording or next source ask:
```

## Dispositions

| Disposition | Meaning |
|---|---|
| `pass` | The claim or artifact is safe within its current evidence boundary. |
| `revise` | The idea may be usable, but wording, source interpretation, scope, or evidence label needs repair. |
| `hold` | The claim is premature and should not be repeated externally. |
| `source-needed` | The claim might be true, but ROUTE does not yet carry the source needed to say it. |
| `confidence-limited` | The claim can be made only with explicit uncertainty, scope, or method limits. |

## Red Flags

Flag any contribution that says or implies:

- an agency, DOT, port, operator, or stakeholder endorsed ROUTE;
- a map proves an official network or construction program;
- a service window is guaranteed;
- a corridor, hub, or project has proven ROI;
- a local adaptation is externally validated;
- a planning target is an engineering recommendation;
- public repository visibility creates procurement, funding, compliance, or
  approval status.

## Gate

Decision: **review_guidance_only**

Rationale: This guide helps external reviewers produce actionable feedback. It
does not certify any review, claim, map, paper, local adaptation, or public
release.
