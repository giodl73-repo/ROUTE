---
name: I-80 External Review Docket
slug: i80-external-review-docket
type: review
status: draft
rubric_version: v1.4
author: copilot
created: 2026-07-11
updated: 2026-07-11
sources:
  - docs/packets/i80-flagship-review-packet.md
  - gaps/i80-flagship.md
  - docs/plans/i80-des-moines-transfer-resilience-validation.md
  - waves/2026-07-11-i80-flagship-stabilization/panels/i80-treatment-review/R1-consolidated.md
---

# I-80 External Review Docket

## Purpose

Test whether ROUTE has framed a credible validation problem. External reviewers
are not being asked to endorse a project, corridor alignment, funding request,
or agency position.

## Shared Packet

- `docs/packets/i80-flagship-review-packet.md`
- `corpus/existing/i80.md`
- `gaps/i80-flagship.md`
- `docs/plans/i80-des-moines-transfer-resilience-validation.md`

## Lane 1 - DOT Or MPO Practitioner

### Reviewer profile

A practitioner with responsibility for freeway operations, freight planning,
incident management, system planning, or project development.

### Questions

1. Is the Des Moines node and analysis boundary operationally recognizable?
2. Which existing paths, restrictions, work zones, or incident procedures are
   missing?
3. Is k-connectivity useful here, or does it obscure the actual operational
   failure?
4. Are the constructability, diversion, safety, climate, and community gates
   sufficient before conceptual design?
5. What evidence would make the hypothesis worth further agency attention?

### Required output

`advance`, `narrow`, or `reject`, plus missing agency data and one concrete
change to the validation plan.

## Lane 2 - Freight Operator Or Economist

### Reviewer profile

A carrier, shipper, logistics operator, freight planner, or transportation
economist familiar with route choice and disruption costs.

### Questions

1. Does an I-35/I-80 closure create a material freight-transfer problem?
2. Which O-D flows, commodities, schedules, and diversion choices must be
   represented?
3. Are operations-only or intermodal responses more plausible than connectors?
4. Which benefit mechanism could be measured without manufacturing ROI?
5. What null result would convince you to stop?

### Required output

`advance`, `narrow`, or `reject`, plus the minimum credible demand/economic
dataset and the correct treatment comparison.

## Lane 3 - Transportation Researcher

### Reviewer profile

A researcher in network science, traffic engineering, resilience, freight,
transport geography, or infrastructure policy.

### Questions

1. Are the graph, node-pair, zone, and freight-eligibility definitions
   reproducible?
2. Is k-connectivity the right metric and target?
3. Which sensitivity and falsification tests are missing?
4. Are the snapshot-history, climate, equity, and shared-failure controls
   methodologically adequate?
5. Does the validation plan distinguish model error from physical deficiency?

### Required output

`advance`, `narrow`, or `reject`, plus methodological corrections and a minimum
replication package.

## Review Decision Matrix

| Result | Meaning | ROUTE action |
|---|---|---|
| Advance | The validation question is credible with listed repairs | Open a bounded evidence/geometry wave |
| Narrow | The question is useful but too broad or incorrectly measured | Amend the validation plan before execution |
| Reject | Des Moines is the wrong hypothesis or metric | Close the candidate and record the null result |

## Review Results

| Lane | Status | Decision | Rationale | Missing data | Required plan change |
|---|---|---|---|---|---|
| DOT or MPO practitioner | pending | — | — | — | — |
| Freight operator or economist | pending | — | — | — | — |
| Transportation researcher | pending | — | — | — | — |

**Roll-up decision:** pending.

The roll-up may be `advance`, `narrow`, `reject`, or `mixed`. A mixed result
must identify which validation work can proceed and which claim or mechanism
remains held.

## Claim Boundary

No external response may be described as approval, endorsement, official-plan
status, construction readiness, eligibility, compliance, positive ROI, or a
guaranteed operating outcome.
