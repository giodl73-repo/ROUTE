# Requirements

## Scope

Repo: ROUTE

VTRACE adoption scope: derive initial repo-level requirements from
`docs/vtrace/MISSION.md` and `docs/vtrace/CONOPS.md`. These requirements
describe what ROUTE must preserve as VTRACE is applied file by file; they do
not authorize new implementation work by themselves.

## Requirement Table

| ID | Requirement | Parent Need / Constraint / Scenario | Rationale | Priority | Owner | Verification Method | Status |
|---|---|---|---|---|---|---|---|
| REQ-001 | ROUTE shall maintain a documented regeneration path for active score, map, SLA, gate, and evidence artifacts. | NEED-001 / CON-003 / OPS-001 | Reproducibility is the minimum condition for trusting generated claims. | must | ROUTE maintainer | inspection / command review | accepted |
| REQ-002 | ROUTE shall label material claims with evidence posture such as implemented, heuristic, planned, held, deprecated, source-needed, or confidence-limited. | NEED-002 / NEED-003 / CON-001 / CON-004 / OPS-001 / OPS-004 | Evidence labels prevent planned or proxy work from reading as proof. | must | ROUTE maintainer | artifact inspection / review | accepted |
| REQ-003 | ROUTE shall keep source gaps, heuristic assumptions, simulated evidence, and owner/human review holds visible in ledgers or review records. | NEED-002 / CON-004 / OPS-001 / OPS-004 | Hidden evidence debt makes downstream design and publication claims unsafe. | must | ROUTE maintainer | ledger inspection / review | accepted |
| REQ-004 | ROUTE shall preserve bundle-first identity for segment-bearing artifacts. | NEED-004 / CON-002 / OPS-002 | Stable physical/service identity is required before maps, stops, simulations, and reports can be compared. | must | route-network owner | architecture gate / artifact inspection | accepted |
| REQ-005 | ROUTE shall reject or hold segment-bearing artifacts that rely only on route labels, tiers, map ids, or zones as primary identity. | NEED-004 / CON-002 / OPS-002 | Mutable labels cannot safely join generated artifacts across analysis stages. | must | route-network owner | architecture gate / data inspection | accepted |
| REQ-006 | ROUTE shall keep stop-first SLA work traceable across visible stops, route services, service classes, schematic geometry, and SLA promises. | NEED-006 / CON-002 / CON-003 / OPS-003 | Current work depends on map and SLA surfaces agreeing before release claims are credible. | must | route-cli / route-network owners | command gate / artifact inspection | accepted |
| REQ-007 | ROUTE shall gate oversized stop gaps, endpoint/contact policy defects, and map/SLA mismatches before generated artifacts are treated as release-ready. | NEED-006 / CON-001 / CON-004 / OPS-003 | The stop/SLA network must fail visibly when map or service promises drift. | must | route-cli / route-map owners | command gate / test | accepted |
| REQ-008 | ROUTE shall expose transportation tradeoffs through parliament, stakeholder, editorial, or panel review records when a claim is promoted, held, downgraded, or used downstream. | NEED-005 / CON-001 / OPS-004 | ROUTE's review system is part of the evidence model, not decoration. | must | review steward | review inspection | accepted |
| REQ-009 | ROUTE shall represent State DOT delivery feasibility, freight operations, rural/agricultural access, non-driving access, and environmental/community-health concerns in requirements, reviews, or claim labels before design options are promoted. | NEED-005 / CON-001 / CON-006 / OPS-004 | Role review found these stakeholder classes must remain first-class mission users. | should | review steward | role review / artifact inspection | accepted |
| REQ-010 | ROUTE shall keep Interstate 2.0 outputs framed as research, tooling, review, and design analysis rather than construction readiness, statutory compliance, or official agency endorsement. | NEED-003 / CON-006 / OPS-004 | Scope control protects ROUTE from overclaiming public authority or delivery status. | must | ROUTE maintainer | editorial review | accepted |
| REQ-011 | ROUTE shall keep VTRACE adoption changes scoped to ROUTE child-repo artifacts until an intentional TRACKER submodule pointer update is requested. | CON-005 / OPS-005 | The portfolio uses TRACKER as a snapshot repo and child repos as implementation owners. | must | ROUTE maintainer / portfolio maintainer | git status / submodule diff inspection | accepted |

## Requirement Quality Checklist

- [x] Each requirement is clear.
- [x] Each requirement is feasible.
- [x] Each requirement is verifiable.
- [x] Each requirement has an owner.
- [x] Each requirement links to a mission need, constraint, or CONOPS scenario.
- [x] Each requirement avoids implementation detail unless the detail is itself required.

## Role Review Notes

| Role Lens | Requirement Impact | Disposition |
|---|---|---|
| Scope Keeper | Requirements stay at repo/system contract level and do not score a corridor, propose a gap, or specify a construction design. | pass |
| Citation Auditor | Requirements introduce no new numeric claims and rely on repo-local mission/source links for context. | pass |
| Numeracy Checker | Requirements contain no calculations, units, scores, or cost/volume claims. | pass |
| Optimization Methodologist | Requirements protect deterministic regeneration, hard identity constraints, rejected/held visibility, and command-gated artifacts. | pass |
| Schematic Cartographer | Requirements make stop/SLA/map agreement and false-service prevention explicit. | pass |
| Traffic Engineer / Freight Economist / Rural Advocate | Requirements require operational, freight, and rural/access concerns to remain represented before claim promotion. | pass |
| State DOT / Transit-Dependent / Environmental Stakeholders | Requirements require delivery feasibility, non-driving access, and environmental/community-health concerns before design promotion. | pass |

## CONOPS Trace Review

| Scenario ID | Requirements Derived |
|---|---|
| OPS-001 | REQ-001, REQ-002, REQ-003 |
| OPS-002 | REQ-004, REQ-005 |
| OPS-003 | REQ-006, REQ-007 |
| OPS-004 | REQ-002, REQ-003, REQ-008, REQ-009, REQ-010 |
| OPS-005 | REQ-011 |

## Deferred Requirements

| ID | Reason Deferred | Revisit Trigger |
|---|---|---|
| none | n/a | n/a |
