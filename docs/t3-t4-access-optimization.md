# T3/T4 Access Optimization

## Purpose

T3 and T4 are the access tiers. They make the national and regional promise
system usable by production zones, ports, border regions, terminals, warehouses,
smaller metros, and rural communities.

This document owns the doctrine for T3/T4 zone optimization, access-gap
detection, map treatment, and upward pressure into T2/T1.

## Tier Roles

| Tier | Promise horizon | Primary job | Map scope |
|---|---|---|---|
| T3 | 6h | Regional feeder and zone access into T1/T2/T3 hubs | Zone or regional inset |
| T4 | 1h | Terminal, local freight district, port, yard, warehouse, and last-mile access | Local scenario or zone inset |

T3 should not be treated as "almost T2." T3 is a complete regional access
treatment. T4 should not be treated as leftover roads. T4 is where terminal
service either works or fails.

## Inputs

Current T3/T4 work consumes:

| Artifact | Use |
|---|---|
| `data/lower-tier-pressure-witnesses.csv` | T2 demotions plus near-threshold T3/T4 upgrade pressure |
| `data/t3-t4-pressure-intake.csv` | Thin intake classifying pressure into T3 intake, T4 intake, or T2 review |
| `data/t2-bubble-up-review.csv` | T3 rows that may reopen T2 only through contact proof |
| `data/t1-feedback-docket.csv` | Guardrail preventing lower-tier pressure from becoming T1 without named dependency |
| `data/map-atlas.csv` | Current T3 zone map hooks |

Future T3/T4 optimization should add zone access obligations, terminal ledgers,
and 6h/1h promise-pair or access-promise artifacts.

## Zone-First Rule

T3/T4 must be solved by zone, not as a national route list.

```text
accepted T1/T2 graph
  -> define service zone
  -> identify regional access obligations
  -> generate T3 route/stop columns
  -> generate T4 terminal/local access columns
  -> validate contacts and gaps
  -> emit regional map and repair witnesses
```

The national map may show a T3/T4 summary, but the real optimizer unit is the
zone.

## Access Obligations

A T3 zone should identify obligations such as:

- production/agricultural access to T1/T2 within a 6h promise;
- smaller metro access to a qualified regional hub;
- port and border approach access;
- mountain, desert, delta, or rural interior feeder coverage;
- regional hospital, evacuation, or resilience access where relevant;
- intermodal or rail terminal approach access when source-backed.

A T4 local treatment should identify obligations such as:

- port gate to T1/T2/T3 access;
- rail yard or intermodal terminal to regional spine;
- warehouse district to nearest qualified freight route;
- border crossing approach and staging access;
- local truck parking/rest/charging access;
- first/last-mile incident or closure alternative.

## Access Gap Classes

| Gap class | Meaning | Typical repair |
|---|---|---|
| `missing-regional-feeder` | A zone lacks a credible T3 path to T1/T2 | select T3 route/stop column |
| `missing-terminal-access` | Terminal/local freight district lacks T4 access | select T4 local access column |
| `unstopped-contact` | Route crosses or nears higher tier without selected stop/contact | add stop or repair contact |
| `one-ended-feeder` | Feeder reaches system at one end only | terminal exception or demotion |
| `overpromoted-local-spur` | Local service was treated as T2/T3 | demote to T4 or local inset |
| `regional-upgrade-pressure` | T3 route is near T2 threshold and may serve broader region | send to T2 contact review |
| `unserved-zone` | No feasible lower-tier treatment satisfies access obligation | bubble up repair witness |

Each gap row must point to a next artifact. Naming a problem is not enough.

## Stops And Contacts

T3/T4 stops are operational nodes, not map labels.

T3 stops should include:

- higher-tier contacts;
- regional hubs;
- production or logistics anchors;
- selected endpoints;
- spacing stops needed for 6h access;
- bend/transfer nodes needed for truthful zone maps.

T4 stops should include:

- terminals, gates, yards, warehouses, and freight districts;
- local access points to T3/T2/T1;
- truck parking, rest, charging, or staging nodes when relevant;
- local incident reroute contacts.

If a zone map bends, branches, or transfers at a place, that place must be a
selected stop or explicit repair row.

## Bubble-Up Rules

T3/T4 can reopen higher tiers only through proof:

| Upward target | Required witness |
|---|---|
| T2 | T3 route has T2 contact proof plus source-backed regional service value |
| T1 | T3/T4 pressure names a selected T1 SLA, stop, or topology dependency |

Score-only pressure may be visible but cannot promote a route. Current
`data/t2-bubble-up-review.csv` enforces this for T2, and
`data/t1-feedback-docket.csv` enforces it for T1.

## Map Treatment

T3 maps should be zone schematics with local T1/T2 context. They are not
standalone horizontal placeholders and should not imply national service.

Current atlas zones:

- `t3-great-lakes`
- `t3-southeast`
- `t3-texas-border`
- `t3-mountain-west`
- `t3-mid-south`

Each zone map should show:

- selected T3 feeder chains;
- local T1/T2 segments needed for context;
- contacts and transfer stops;
- unresolved access gaps;
- held blockers or demotion decisions where useful.

T4 maps should usually appear inside a game scenario, local inset, or terminal
access board rather than the national atlas.

## Game Semantics

T3/T4 are where the game can make access tangible:

- local feeder upgrade;
- terminal access repair;
- port or border queue relief;
- rural production-zone connection;
- rest/charging/staging fix;
- incident detour or local resilience repair.

Game hooks should target access gap classes and selected service columns, not
hand-picked route names.

## Gate Requirements

A T3/T4 gate should fail when:

- a selected T3 route lacks a higher-tier or regional contact;
- a selected T4 local access column lacks a terminal/local obligation;
- a zone map contains bends or transfers at unselected stops;
- a feeder is promoted to T2 without contact proof;
- a pressure row bubbles to T1 without a named T1 dependency;
- an access gap lacks a next artifact;
- a zone map omits the local T1/T2 context needed to understand attachments.

Review rows are allowed if their next artifact is explicit.

## Current Status

Current implementation status:

- `data/lower-tier-pressure-witnesses.csv` records T2 demotions and near-threshold
  lower-tier pressure.
- `data/t3-t4-pressure-intake.csv` classifies 108 pressure rows.
- `data/t3-zone-access-obligations.csv` groups pressure rows into five T3 zone
  map obligations: 6h regional feeder access and 24h upgrade review.
- `data/t3-zone-route-columns.csv` selects route-level T3 feeder columns from
  the obligation table, carries normalized constraint-budget summaries, and
  holds upward/below-threshold rows for review.
- `data/t4-terminal-access-columns.csv` classifies T4 local pressure into
  zone-scoped terminal-review rows with 1h access obligations and normalized
  constraint-budget summaries. Terminal-review rows name zone-specific
  terminal districts from `data/intermodal_terminals.csv`; they remain held
  until route-to-terminal contact proof is authored.
- `data/t3-t4-access-gaps.csv` collects unresolved T3/T4 access pressure into
  below-threshold feeder and terminal-evidence gap classes, inheriting
  constraint pressure from the held selector row.
- `data/optimizer-constraint-ledger.csv` normalizes those access gaps into
  `lower_tier_feeder_gap` and `terminal_access_evidence_gap` claim blockers so
  lower-tier pressure feeds the shared optimizer budget.
- `data/optimizer-constraint-budget.csv` rolls those access blockers back into
  T3/T4 route rows as `claim_blocker_count`, `constraint_penalty_score`, and
  `top_constraint_classes`.
- `data/t3-zone-map-diagnostics.csv` joins selected T3 route columns, access
  gaps, and map atlas ids into zone-map readiness decisions.
- `data/t3-zone-render-board.csv` turns map diagnostics into renderer/game board
  rows: zone summaries, selected T3 route columns, review connectors, and
  held-gap callouts.
- `data/t3-zone-stop-placement.csv` checks selected render-board routes against
  zone-bounded stop candidates, separating render-ready stop chains from
  stop-authoring gaps.
- `docs/national-segment-identity-spec.md` defines the stable segment id,
  route-bundle, stitch-group, alias, and state-scope grammar used by the T3
  render-board and stop-placement outputs.
- `data/national-segment-registry.csv` merges segment-bearing rows into one
  auditable identity surface for downstream geometry, overlays, and promotion
  review.
- `data/t2-bubble-up-review.csv` sends 18 near-threshold T3 rows to T2 contact
  review instead of promoting them directly.
- `data/map-atlas.csv` tracks five T3 zone maps.

Current gaps:

- no full T3 zone optimizer yet;
- county/source enrichment is not yet attached to the zone obligation table;
- T3 geometry still needs bend constraints after zone-bounded stop placement;
- T4 still needs terminal/source enrichment and rendered local inset treatment.
