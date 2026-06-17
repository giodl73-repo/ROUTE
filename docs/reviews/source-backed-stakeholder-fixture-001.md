---
name: Source-Backed Stakeholder Fixture 001
slug: source-backed-stakeholder-fixture-001
type: review
status: draft
rubric_version: v1.0
author: codex
created: 2026-06-17
updated: 2026-06-17
sources:
  - docs/how-to/stakeholder-fixture-closeout-runbook.md
  - docs/templates/source-packs/stakeholder-fixture-source-pack-template.md
  - docs/reviews/source-backed-stakeholder-fixture-candidate-001.md
  - data/t4-terminal-contact-accepted-proof-sources.csv
  - data/t4-terminal-access-columns.csv
  - data/t4-terminal-access-proof-review.csv
  - data/t4-terminal-access-proof-intake.csv
  - docs/reports/t3-t4-access-evidence-appendix.md
  - https://portnola.com/business/cargo/road
---

# Source-Backed Stakeholder Fixture 001

## Purpose

This fixture closes one internal source-backed stakeholder example for the
sponsor-to-DOT dry run packet.

It uses an existing accepted terminal-contact proof source for Port NOLA road
access to show how a concrete freight/terminal requirement changes a ROUTE
evidence posture without creating a construction, map-publication, operating
service, numeric ROI, eligibility, compliance, endorsement, agency approval, or
external-readiness claim.

## Fixture Metadata

| Field | Entry |
|---|---|
| Fixture ID | STAKE-FIX-001 |
| Stakeholder lane | freight industry / terminal access |
| Source pack owner | Citation Auditor |
| Meeting / intake artifact | public terminal-access source already captured in `data/t4-terminal-contact-accepted-proof-sources.csv` |
| Source-backed requirement | Terminal-access claims need a source that names route-to-terminal access, not only a terminal district seed. |
| Affected geography / zone | New Orleans Gentilly terminal district; I-510 and US90Z examples |
| Claim posture before fixture | held_template for stakeholder fixture; source-needed for generic T4 terminal-access proof rows |
| Intended ROUTE artifact to change | source pack / evidence label / claim trace row |
| Review lanes required | Scope Keeper, Citation Auditor, Numeracy Checker, Optimization Methodologist, Freight Economist, Freight Industry, Schematic Cartographer, State DOT Planner |

## Source Custody Rows

| Source ID | Source Path / URL | Title | Publisher / Owner | Date / Year | Access Note | Source Type | Units / Field Names | Reviewer |
|---|---|---|---|---|---|---|---|---|
| STAKE-SRC-001 | `https://portnola.com/business/cargo/road` | Road | Port of New Orleans | verified public page 2026-06-17; repo capture row date 2026-05-15 | public URL; repo-local accepted source row in `data/t4-terminal-contact-accepted-proof-sources.csv` | public terminal road directions | route identifiers and terminal names only; no time, distance, volume, cost, or service measure used | Citation Auditor |

## Requirement-To-Refinement Rows

| Row ID | Requirement | Source ID | Before Artifact / Label | Change Applied | After Artifact / Label | Role Hold / Dissent | Claim Allowed? | Next Evidence Step |
|---|---|---|---|---|---|---|---|---|
| STAKE-FIX-001 | For terminal-access examples, cite a route-to-terminal source before using a row as a concrete freight/terminal fixture. | STAKE-SRC-001 | `docs/reviews/source-backed-stakeholder-fixture-candidate-001.md`: `held_template`; generic T4 proof rows remain source-needed until accepted route-to-terminal proof exists. | Populated a source-backed fixture with Port NOLA source custody and tied it to the accepted proof-source rows for I-510 and US90Z in `data/t4-terminal-contact-accepted-proof-sources.csv`. | This file becomes the populated internal fixture; `docs/traces/route-claim-promotion-trace.md` can add a source-backed terminal-access example with map/publication/upgrade holds preserved. | State DOT authority, operations performance, local impact, and map publication remain held. | internal only | Add DOT/port meeting context, verify any geometry or operating claim separately, and preserve map/publication/upgrade holds until role review and release gates close. |

## Evidence Boundary

| Safe Finding | Held Finding |
|---|---|
| ROUTE can show one source-backed terminal-access fixture for internal rehearsal. | The source proves terminal service quality, one-hour access, construction need, state approval, or map-publication readiness. |
| The fixture demonstrates source custody and label discipline. | The fixture validates all T4 terminal rows or closes regional freight access. |
| Port NOLA road directions can be used as a public source for a bounded terminal-access example. | Port NOLA, Louisiana DOTD, USDOT, FHWA, or any stakeholder endorses Interstate 2.0 or ROUTE. |

## Required Role Review

| Role Lane | Review Question | Result | Finding / Hold |
|---|---|---|---|
| Scope Keeper | Does the fixture remain an evidence artifact rather than a project approval? | pass | The fixture is limited to source custody and claim-label change for internal rehearsal. |
| Citation Auditor | Is the source traceable by title, owner, date/access note, and repo-local artifact? | pass | Public URL, owner, title, access note, and accepted proof-source rows are named. |
| Numeracy Checker | Are numeric claims, units, thresholds, price years, and calculations avoided or explicit? | pass | Route numbers are identifiers only; no time, distance, cost, volume, ROI, or service calculation is used. |
| Optimization Methodologist | Is the before/after change reproducible and not hand-shaped into a desired answer? | pass_with_risk | The before/after is a label/source-pack change, not a rerun optimizer output; stronger optimization claims remain held. |
| Freight Economist | Does the fixture avoid turning terminal access into ROI or freight-value proof? | pass_with_risk | The fixture supports a terminal-access evidence example only; freight value and ROI remain gated. |
| Freight Industry | Does the fixture preserve operating constraints such as dwell, HOS, parking, reliability, weights, and clearances? | pass_with_risk | The source gives road-access directions, not full operating evidence. |
| Schematic Cartographer | Does any map-facing use avoid proof-by-picture? | pass_with_risk | The fixture may support a map caption or inset example only if held claims remain visible. |
| State DOT Planner | Are authority, funding, ROW, maintenance, phasing, and environmental process bounded? | hold | No state DOT authority or delivery review is included. |

## Closeout Checklist

| Item | Pass / Hold | Evidence |
|---|---|---|
| Source custody row filled. | pass | `STAKE-SRC-001` names URL, title, owner, access note, source type, and reviewer. |
| Requirement row filled. | pass | `STAKE-FIX-001` states route-to-terminal source requirement. |
| Before/after artifact or label captured. | pass | Candidate shell changes to this populated fixture and trace-ready source-backed example. |
| Editorial roles reviewed. | pass_with_risk | Scope, citation, numeracy, and optimization findings recorded above. |
| Affected stakeholder lanes reviewed. | pass_with_risk | Freight lanes reviewed; State DOT delivery authority remains held. |
| Dissent or hold preserved. | pass | State DOT, operating-performance, local-impact, map-publication, and stronger freight claims remain held. |
| Prohibited-claim scan passes. | pass | Hits are guardrail, held, or non-approved contexts. |
| `docs/traces/route-claim-promotion-trace.md` updated if claim posture changes. | pass | `TRACE-CLAIM-009` added for terminal-access fixture. |
| `docs/evidence/round5-demo-capture.md` or successor evidence record updated if command evidence changes. | hold | No command evidence changed. |
| `docs/vtrace/VERIFICATION.md` updated if Round 5 gate status changes. | pass | Populated fixture row added. |

## Gate

Decision: **pass_with_risk for internal rehearsal**

Rationale: This fixture satisfies the minimum populated source-backed
requirement-to-refinement shape for an internal sponsor-to-DOT dry run: source
custody, requirement, before/after label change, and role review are recorded.
It does not authorize external use, public release, map publication, operating
service, construction, funding, ROI, eligibility, compliance, endorsement, or
approval claims.
