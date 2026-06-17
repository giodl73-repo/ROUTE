---
name: T3/T4 Access Evidence Appendix
slug: t3-t4-access-evidence-appendix
type: report
status: draft
rubric_version: v1.0
author: copilot
created: 2026-06-16
updated: 2026-06-16
sources:
  - docs/t3-t4-access-optimization.md
  - docs/reports/rural-access-national-service-network-report.md
  - docs/optimizer-constraint-ledger-spec.md
  - docs/optimizer-artifact-manifest.md
  - docs/national-segment-identity-spec.md
  - docs/bundle-registry-spec.md
  - data/t3-t4-pressure-intake.csv
  - data/t3-zone-access-obligations.csv
  - data/t3-zone-route-columns.csv
  - data/t4-terminal-access-columns.csv
  - data/t3-t4-access-gaps.csv
  - data/t3-zone-map-diagnostics.csv
  - data/t3-zone-render-board.csv
  - data/t3-zone-stop-placement.csv
  - data/t4-terminal-access-evidence-review.csv
  - data/t4-terminal-access-proof-acquisition.csv
  - data/t4-terminal-access-proof-artifact-source-access.csv
  - data/t4-terminal-access-map-exclusion.csv
---

# T3/T4 Access Evidence Appendix

## Purpose

This appendix makes the lower-tier access machinery visible for reviewers who
ask whether the rural, production-zone, small-metro, port, warehouse, terminal,
and local freight story is only narrative.

The answer is: ROUTE has a real access-ledger chain, but many lower-tier claims
remain held. The current artifacts can show pressure, obligations, selected
zone columns, held access gaps, map-board rows, stop-placement status, terminal
proof tasks, and terminal-overlay exclusions. They do not prove terminal access
everywhere, promote local rows into higher tiers, or authorize map publication
claims for unresolved terminal-access overlays.

## Reviewer Takeaway

T3 and T4 are not leftover roads.

T3 is the regional feeder and zone-access layer. T4 is the local terminal,
port, rail-yard, warehouse, border, and last-mile freight-access layer. ROUTE
treats them as zone-first service obligations and then records where the lower
tier can pass, stay held, or pressure T2/T1 only through named proof.

```text
lower-tier pressure
  -> zone obligation
  -> T3 feeder column or T4 terminal column
  -> access gap / proof task / render-board row
  -> constraint ledger and next artifact
```

## Current Artifact Inventory

These counts are local artifact counts from the current repo snapshot, not
coverage claims about the national network.

| Artifact | Rows | What It Shows | Boundary |
|---|---:|---|---|
| `data/t3-t4-pressure-intake.csv` | 108 | Lower-tier pressure classified into T3 intake, T4 intake, or T2 review. | Intake does not promote a route. |
| `data/t3-zone-access-obligations.csv` | 10 | Five zone-map obligations across 6h feeder access and 24h upgrade review. | Obligations need source and contact evidence before stronger claims. |
| `data/t3-zone-route-columns.csv` | 43 | Selected T3 feeder columns, upward-review connectors, held below-threshold rows, and constraint pressure. | Selected columns are map/optimizer inputs, not operating promises. |
| `data/t4-terminal-access-columns.csv` | 69 | T4 local terminal-review rows with 1h access obligations and terminal districts. | Rows remain held until route-to-terminal proof is attached and accepted. |
| `data/t3-t4-access-gaps.csv` | 75 | Below-threshold feeder gaps, terminal-evidence gaps, repair actions, next artifacts, and blocked upward pressure. | Gap visibility is not gap closure. |
| `data/t3-zone-map-diagnostics.csv` | 5 | Zone-map readiness diagnostics for the current T3 atlas zones. | Diagnostics do not prove local access or terminal readiness. |
| `data/t3-zone-render-board.csv` | 123 | Renderer/game board rows for zone summaries, selected routes, review connectors, held gaps, and backlog. | Render-board rows carry review status; maps are not proof. |
| `data/t3-zone-stop-placement.csv` | 20 | Zone-bounded stop-placement status for selected T3 render-board routes. | Stop placement is a readiness surface, not a service guarantee. |
| `data/t4-terminal-access-evidence-review.csv` | 69 | Review decisions preserving source-needed terminal-contact blockers. | Seed terminal districts cannot be laundered into route-contact proof. |
| `data/t4-terminal-access-proof-acquisition.csv` | 69 | Proof acquisition tasks for held terminal-access rows. | Tasks are worklists, not accepted evidence. |
| `data/t4-terminal-access-proof-artifact-source-access.csv` | 69 | Manual/cache source-access policy for terminal proof artifacts. | Live fetch remains unsupported for these proof rows. |
| `data/t4-terminal-access-map-exclusion.csv` | 1 | Accepted scope exclusion for unresolved T4 terminal-access overlay publication claims. | Exclusion preserves evidence work but blocks map/publication claims. |

## Evidence Chain

### 1. Pressure Intake

Lower-tier pressure starts in `data/t3-t4-pressure-intake.csv`. The row can say
that a route, score band, or demotion belongs in T3 intake, T4 intake, or T2
review. The row cannot promote a route by itself.

Safe claim: "ROUTE records lower-tier pressure and routes it to a review path."

Held claim: "This pressure proves a route deserves higher-tier treatment."

### 2. Zone Obligations

`data/t3-zone-access-obligations.csv` groups pressure into zone obligations.
This is the step that makes rural, production-zone, small-metro, port, border,
and terminal access visible without pretending that a national line map is local
proof.

Safe claim: "A zone obligation names the access problem to test."

Held claim: "The access problem is solved across the zone."

### 3. T3 Feeder Columns

`data/t3-zone-route-columns.csv` selects or holds route-level feeder columns.
Rows carry the promise horizon, contact requirement, map treatment, constraint
budget pressure, source obligation, next artifact, and validation status.

This is the main bridge from rural-access story to inspectable route work. It
lets reviewers ask why a route is selected, held, or sent upward for review.

### 4. T4 Terminal Columns

`data/t4-terminal-access-columns.csv` classifies local terminal pressure into
zone-scoped rows. It names terminal obligations and terminal districts, but
terminal district seed evidence is not enough. The rows remain held until a
separate route-to-terminal contact proof artifact exists.

This protects the communications package from the common failure mode where a
terminal exists near a route and the story quietly treats proximity as access.

### 5. Access Gaps And Constraint Ledger

`data/t3-t4-access-gaps.csv` collects unresolved T3/T4 pressure into explicit
gap classes such as below-threshold feeder and terminal-evidence-needed. The
optimizer constraint ledger then carries those blockers as shared claim debt.

Safe claim: "ROUTE can preserve lower-tier blockers and show the next repair
artifact."

Held claim: "ROUTE has closed the blocker."

### 6. Render Board And Stop Placement

`data/t3-zone-map-diagnostics.csv`, `data/t3-zone-render-board.csv`, and
`data/t3-zone-stop-placement.csv` turn selected feeders and held gaps into
renderer-facing and game-board-facing rows. These rows carry stable segment,
bundle, stitch, alias, state-scope, route status, and placement status fields.

That makes lower-tier maps reviewable, but it does not make maps proof of local
access, terminal service, or public readiness.

### 7. Terminal Proof Worklist

`data/t4-terminal-access-evidence-review.csv`,
`data/t4-terminal-access-proof-acquisition.csv`, and
`data/t4-terminal-access-proof-artifact-source-access.csv` preserve 69
terminal-access rows as source-needed proof work. The current source-access
policy requires manual or cached non-seed proof metadata before these rows can
be accepted.

The important communications point is that the proof debt is visible and
counted. It is not closed.

### 8. Map Exclusion

`data/t4-terminal-access-map-exclusion.csv` accepts a narrow exclusion:
unresolved T4 terminal-access overlay publication claims are excluded from
current maps while upgrade/evidence work remains preserved.

This is a strong evidence-boundary artifact. It lets ROUTE keep working on
terminal access without implying terminal-access publication readiness.

## How This Supports Rural Access

| Rural / Access Concern | ROUTE Surface | What Reviewers Can Inspect |
|---|---|---|
| Production-zone or farm-region access | T3 zone obligations and route columns | Zone, obligation class, selected/held feeder route, contact requirement, next artifact. |
| Smaller-metro attachment | T3 feeder columns and stop placement | Whether the route has selected stops and T1/T2/T3 contact context. |
| Port, rail-yard, warehouse, and terminal access | T4 terminal columns and proof worklists | Whether the terminal obligation has accepted route-to-terminal proof. |
| Emergency or evacuation prompt | Access-gap and scenario hooks | Whether the scenario remains heuristic or source-backed. |
| Lower-tier pressure on higher tiers | T2 bubble-up / T1 feedback surfaces and constraint ledger | Whether the row has contact proof or a named higher-tier dependency. |
| Map/game representation | T3 diagnostics, render board, stop placement, map exclusion | Whether the output carries held status, identity, gaps, and exclusions. |

## Reviewer Pressure Questions

- Which zone obligation owns the access concern?
- Is the route a selected T3 feeder, a held review connector, or a T4 local
  terminal-access row?
- Which gap class applies, and what next artifact repairs it?
- Does upward pressure have contact proof, or is it blocked from T2/T1?
- Does the row carry stable bundle/member/stitch identity before map, game, or
  report use?
- Is terminal evidence based on a non-seed route-to-terminal proof artifact?
- Are unresolved terminal-access overlays excluded from publication claims?
- Does the communications surface say "visible access obligation" instead of
  "solved access"?

## Safe Language

| Use This | Avoid This |
|---|---|
| "T3/T4 make lower-tier access visible and reviewable." | "T3/T4 prove every rural or terminal access need is solved." |
| "Zone obligations create an inspectable access worklist." | "The national map proves local access." |
| "Terminal rows remain held until non-seed proof is attached and accepted." | "Nearby terminal seed data proves route-to-terminal access." |
| "Lower-tier pressure can feed the optimizer ledger." | "Lower-tier pressure automatically promotes a route." |
| "Current maps exclude unresolved T4 terminal-access publication claims." | "The map is complete for terminal access." |

## Non-Goals

- This appendix does not select new rural routes, terminal links, hubs, or
  construction projects.
- This appendix does not close the 69 source-needed terminal-access proof tasks.
- This appendix does not claim operating SLA, emergency-response performance,
  positive ROI, agency endorsement, legal eligibility, public release readiness,
  or compliance.
- This appendix does not turn T3/T4 map rows into publication-ready local access
  proof.

## Gate

Decision: pass_with_risk for internal communications review.

Rationale: The lower-tier access story now has a communications-facing evidence
chain tied to current artifacts and explicit holds. Stronger rural, terminal,
emergency, local-access, map-publication, game-readiness, or promotion claims
remain gated by zone-specific sources, accepted route-to-terminal proof, role
review, scenario evidence, map scope decisions, and L1/L2 validation.
