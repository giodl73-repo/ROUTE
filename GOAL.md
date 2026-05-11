# Current Goal: Milestone 10

## Stop-First SLA Network

Turn the Beck/T1/T2/T3 map system into a validated connection network where stops,
routes, SLA promises, and schematic geometry all agree.

## Why This Matters

The maps now look and behave more like a metro system, and the stop-to-stop SLA
surface exists. The next step is making that surface operational: every route
promise should be traceable through visible stops, and every visible stop/line
should mean something in the SLA graph.

## Success Criteria

- Reduce oversized stop gaps in `data/beck-stop-sla.csv`, especially rows marked
  `needs-intermediate-stops`.
- Promote stop classes into a shared vocabulary, similar to `RouteTier`.
- Enforce endpoint/contact rules for T1, T2, and T3 routes.
- Ensure Beck/T2/T3 map bends happen at named stops that exist in the SLA graph.
- Add gates that catch visible map/SLA mismatches before they reach generated
  artifacts.

## Rules To Encode

- T1 endpoints must be national terminals or major transfer hubs.
- T2 routes should connect at least two meaningful system-contact nodes unless
  explicitly marked as one-ended.
- T3 routes should connect regional chains into local T1/T2 context.
- Long schematic edges must either have intermediate stops or carry an explicit
  review/blocker status.
- SLA claims remain heuristic until supported by source-backed reliability data.

## Target Artifacts

- `route stop-sla-summary`
- Updated `data/beck-stop-sla.csv`
- Shared stop/service-class primitives
- L2 gates for map/SLA consistency
- Regenerated Beck/T2/T3 maps and board CSVs as needed

## First Task List

1. Add a stop SLA summary command.
2. Identify worst stop gaps by route and route segment.
3. Add shared stop class/service class types.
4. Encode T1/T2/T3 endpoint rules.
5. Add map-to-SLA consistency tests.
6. Reduce the highest-impact `needs-intermediate-stops` gaps.
7. Regenerate artifacts and run `npm run check:l2`.

## Delivered Slice

- `route stop-sla-summary` reports pair counts, SLA windows, gap status,
  evidence labels, mode-comparison notes, and worst gaps.
- `route stop-sla-summary --gate-max-gap 300` is wired into L2 CLI e2e.
- Shared `StopServiceClass` and `StopNodeClass` primitives live in
  `route-network`.
- The Beck SLA graph now uses deterministic Dijkstra pathing.
- The national SLA surface has no `needs-intermediate-stops` rows; current max
  stop gap is at or under 300 miles.
- Added visible intermediate stops for the largest western, mountain, and
  Appalachian gaps.
- T1/T2 route endpoint/contact policy is now shared through `route-network` and
  gated by Beck map tests.
- Washington and Spokane are classified as transfer/contact stops where the
  schematic topology already used them as route contacts.
- `route stop-sla-summary` now reports recurring stop-to-stop gap segments so
  oversized OD rows point back to concrete stop investments.
- Added Wichita Falls, Show Low, and Daytona Beach as stop-first anchors; the
  national SLA surface max stop gap is now 382 miles.
- Added Wheeling, Iowa City, Hattiesburg, Fort Smith, Pueblo, Texarkana, Macon,
  and Wichita as route-rhythm stops; the national SLA surface max stop gap is
  now 352 miles.
- `route stop-sla-candidates` joins recurring SLA gaps to the stop-candidate
  ledger and draft city seeds, scoring spacing gain plus route/intersection
  contact value before new stops are promoted onto the Beck map.
- Empty candidate gaps now receive `draft-algorithmic-midpoint` rows and can be
  exported as a CSV docket, so every oversized segment has a review target even
  before source-backed stop promotion.
- `route stop-sla-promotions` converts the SLA candidate docket into
  `tier-stop-candidates`-shaped source-needed rows and gates that the promotion
  scaffold is structurally append-ready.
- The city seed layer now covers the current >300-mile recurring gaps, so the
  default promotion docket recommends named review targets instead of pure
  spacing midpoints.
- Named city-seed targets for the >300-mile queue are now promoted into the Beck
  stop graph, regenerating the national SLA surface and shifting the next
  candidate docket to the >250-mile queue.
- The current top >250-mile candidate and promotion dockets now resolve to named
  service-city or stop-ledger candidates instead of algorithmic midpoint
  placeholders.
