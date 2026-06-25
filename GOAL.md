# Current Goal: Milestone 10

## Stop-First SLA Network

Turn the Beck/T1/T2/T3 map system into a validated connection network where stops,
routes, SLA promises, and schematic geometry all agree.

The selector doctrine is: tier equals promise horizon. T1 is the national 48h/36h
timed-freight spine, T2 is the regional 24h/12h timed-freight connector layer, T3
is the 6h feeder/access mesh, and T4 is the 1h local/terminal layer.

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
- `route stop-sla-summary --gate-max-gap 250` is wired into L2 CLI e2e.
- Shared `StopServiceClass` and `StopNodeClass` primitives live in
  `route-network`.
- The Beck SLA graph now uses deterministic Dijkstra pathing.
- The national SLA surface has no `needs-intermediate-stops` or
  `long-gap-review` rows; current max stop gap is at or under 250 miles.
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
- `route stop-sla-candidates --gate-no-algorithmic` now gates that inspected
  recurring gaps promote to named candidate rows rather than midpoint fallbacks.
- The strongest >250-mile named candidates are now promoted into the Beck stop
  graph; the regenerated SLA surface has a 248-mile max stop gap and no current
  >250-mile promotion docket.
- Added a T2-only Beck service map so local connector geometry can be inspected
  without the bold T1 trunk layer.
- Added a T2 diagnostics CSV to rank dense labels, transfer complexity, and long
  connectors on the T2-only service map.
- T2 service lines now carry explicit start/end parent trunks; bridge connectors
  render as split-parent colors while one-ended feeders remain single-parent.
- T2 geometry diagnostics now prefer real interchange split anchors, flag T2
  lines that pass near T1 stops without stopping, and identify close parallel
  T2 corridors that need bend/spacing cleanup.
- T2 cleanup promoted multi-line drawn stops as transfer candidates, falls back
  split-color breaks to real service stops, and added Meridian/Memphis/Tupelo
  rhythm on southern T2 services so the stop SLA surface stays below 250 miles.
- T2 parallel cleanup widened the Arkansas/Old South lane offsets so I-30 and
  US80 no longer trigger close-parallel review while preserving their stop paths.
- T2 diagnostics now separate compact short services from true dense/transfer
  reviews, and the Capital Beltway loop is expanded so it no longer reads as a
  cramped label knot.
- T2 service standards now expose a `service_class` in diagnostics and SVG
  paths: compact services are accepted, transfer spines are weighted, and long
  connectors keep a dashed center convention over continuous parent colors.
- T2 diagnostics gate now fails only structural layout defects while service
  classes such as transfer spines and long connectors remain standards-backed
  review categories rather than release blockers.
- T2 service classes now export to `data/beck-t2-service-standards.csv` via
  `route beck-t2-service-standards`, giving diagnostics, maps, and game overlays
  one machine-readable service-class contract.
- T2 qualification actions now export to `data/beck-t2-qualification-actions.csv`
  via `route beck-t2-qualification-actions`, separating duplicate-service
  keep, merge, and demotion review rules from visual service classes.
- T2 qualification actions now name covered diagnostic bases, and
  `route beck-t2-qualification-actions --gate` rejects uncovered service-action
  plus qualification-basis pairs before duplicate-service decisions can flow
  downstream.
- `route t2-service-selection` now consumes the T2 qualification-action ledger
  and exports qualification map treatment, gate policy, and game-use semantics
  into `data/t2-service-selection.csv`.
- `route t2-bundle-overlays` now carries qualification map treatment, gate
  policy, and game-use semantics into `data/game/t2-bundle-overlays.csv`, and its
  gate requires those fields for bound overlays.
- `route t2-game-ops-binding-decisions` now preserves qualification gate policy
  and game-use semantics in `data/t2-game-ops-binding-decisions.csv`, and its
  gate requires them for bound or bundle-bound-review decisions.
- `route t2-bundle-overlay-repair-targets` now preserves qualification gate
  policy and game-use semantics in `data/t2-bundle-overlay-repair-targets.csv`
  for bundle-bound-review repair targets.
- `route t2-game-ops-bundle-evidence-review` now preserves qualification gate
  policy and game-use semantics in
  `data/t2-game-ops-bundle-evidence-review.csv` for bundle-bound-review rows.
- `route t2-game-ops-bundle-evidence-policy` now preserves qualification gate
  policy and game-use semantics in
  `data/t2-game-ops-bundle-evidence-policy.csv` for bundle-bound stop-chain
  policy rows.
- `route t2-game-ops-bundle-evidence-policy-acceptance` now preserves
  qualification gate policy and game-use semantics in
  `data/t2-game-ops-bundle-evidence-policy-acceptance.csv` when accepted source
  policy rows had them.
- `route t2-game-ops-bundle-evidence-blocker-relief` now preserves qualification
  gate policy and game-use semantics in
  `data/t2-game-ops-bundle-evidence-blocker-relief.csv` when source acceptance
  rows had them.
- `route optimizer-constraint-ledger` now preserves T2 qualification gate policy
  and game-use semantics in `optimizer_effect` for replayed game/ops bundle
  relief rows whose source blocker-relief rows had them.
- `route optimizer-constraint-budget` now rolls qualification-bearing
  `optimizer_effect` values into `qualification_effects` so selector-facing
  budget rows retain the T2 gate/game-use contract.
- `T2GameOpsBindingIntakeRow` now accepts and preserves `qualification_effects`
  from budget rollup for future active game/ops binding intake rows while
  remaining compatible with existing intake CSVs.
- `T2GameOpsBindingDecisionRow` now accepts and preserves `qualification_effects`
  from binding intake for future active game/ops binding decisions while
  remaining compatible with existing decision CSVs.
- `T2BundleOverlayRepairTargetRow` now accepts and preserves
  `qualification_effects` from binding decisions for future active repair target
  rows while remaining compatible with existing repair-target CSVs.
- `T2ServiceClassRepairDocketRow` now accepts and preserves
  `qualification_effects` from repair targets for future service-class repair
  docket rows while remaining compatible with existing docket CSVs.
- `T2GameOpsBundleEvidenceReviewRow` now accepts and preserves
  `qualification_effects` from service repair or repair targets for future
  evidence review rows while remaining compatible with existing review CSVs.
- `T2GameOpsBundleEvidencePolicyRow` now accepts and preserves
  `qualification_effects` from evidence review for future evidence policy rows
  while remaining compatible with existing policy CSVs.
- `T2GameOpsBundleEvidencePolicyAcceptanceRow` now accepts and preserves
  `qualification_effects` from evidence policy for future acceptance rows while
  remaining compatible with existing acceptance CSVs.
- `T2GameOpsBundleEvidenceBlockerReliefRow` now accepts and preserves
  `qualification_effects` from evidence acceptance for future blocker-relief rows
  while remaining compatible with existing relief CSVs.
- Optimizer ledger replay now carries non-empty blocker-relief
  `qualification_effects` into `optimizer_effect` without emitting empty
  qualification-effect placeholders in current generated rows.
- `TierCandidateColumnRow` and `T2RegionalizerRow` now accept and preserve
  `qualification_effects` from budget rollup for future T2 candidate/regionalizer
  rows while remaining compatible with existing CSVs.
- `T2ServiceSelectionRow` now accepts and preserves `qualification_effects` from
  regionalizer rows for future service selection rows while remaining compatible
  with existing CSVs.
- `T2ServiceDiagnosticQueueRow` now accepts and preserves `qualification_effects`
  from service selection for future diagnostic queue rows while remaining
  compatible with existing CSVs.
- `T2RouteFamilySplitRow` now accepts and preserves `qualification_effects` from
  service diagnostics for future route-family split rows while remaining
  compatible with existing split CSVs.
- `TierSegmentCandidateRow` now accepts and preserves `qualification_effects`
  from T2 service selection and route-family splits for future segment candidate
  rows while remaining compatible with existing candidate CSVs.
- `TierPavementDocketRow` now accepts and preserves `qualification_effects` from
  tier segment candidates for future pavement docket rows while remaining
  compatible with existing docket CSVs.
- `NationalSegmentRegistryRow` now accepts and preserves `qualification_effects`
  from tier segment candidates and pavement dockets for future registry rows
  while remaining compatible with existing registry CSVs.
- `NationalSegmentBundleRow` now accepts and preserves `qualification_effects`
  from national segment registry members for future bundle rows while remaining
  compatible with existing bundle CSVs.
- `T2BlockerClosureRow` now accepts and preserves `qualification_effects` from
  national segment bundles for future blocker-closure rows while remaining
  compatible with existing closure CSVs.
- Closure-driven `T2RouteFamilySplitRow` rows now preserve
  `qualification_effects` from blocker closures and carry them into optimizer
  effects for future route-family split rows.
- `T2ClosureDisposition` and `TierCandidateColumnRow` now preserve
  `qualification_effects` from blocker closures while merging any optimizer
  budget effects for future candidate-column rows.
- `T2BundleRepairQueueRow` now accepts and preserves `qualification_effects`
  from candidate columns and blocker closures for future bundle-repair queue rows
  while remaining compatible with existing repair CSVs.
- Repair-derived `TierSegmentCandidateRow` rows now preserve
  `qualification_effects` from `T2BundleRepairQueueRow` when bundle repair work
  re-enters segment candidate generation.
- `T2BundleOverlayRow` now accepts and preserves merged `qualification_effects`
  from service selection and national segment bundles for future overlay rows,
  and qualification-effect merging now keeps pipe-delimited effect text.
- `T2GameOpsBindingDecisionRow` generation now merges `qualification_effects`
  from binding intake and bundle overlays so future decisions preserve both
  budget-derived and overlay-derived qualification contracts.
- `T2BundleOverlayRepairTargetRow` coverage now verifies merged
  `qualification_effects` from game/ops binding decisions survive into repair
  targets.
- `T2ServiceClassRepairDocketRow` coverage now verifies repair-target
  `qualification_effects` survive into service repair dockets and optimizer
  effect text.
- `T2GameOpsBundleEvidenceReviewRow` coverage now verifies service-repair
  `qualification_effects` survive into game/ops bundle evidence review rows.
- `T2GameOpsBundleEvidencePolicyRow` coverage now verifies evidence-review
  `qualification_effects` survive into game/ops bundle evidence policy rows.
- `T2GameOpsBundleEvidencePolicyAcceptanceRow` coverage now verifies evidence
  policy `qualification_effects` survive into policy acceptance rows.
- `T2GameOpsBundleEvidenceBlockerReliefRow` coverage now verifies policy
  acceptance `qualification_effects` survive into blocker relief rows.
- `route optimizer-constraint-ledger` coverage now verifies blocker-relief
  `qualification_effects` survive into game/ops bundle relief optimizer effects
  as pipe-delimited text.
- `route optimizer-constraint-budget` now extracts qualification effect clauses
  from ledger optimizer effects and emits pipe-delimited budget
  `qualification_effects` for downstream candidate consumers.
- T2 regionalizer and service-selection coverage now verifies normalized budget
  `qualification_effects` survive through candidate-column and regionalizer
  handoffs.
- `T2ServiceDiagnosticQueueRow` coverage now verifies service-selection
  `qualification_effects` survive into diagnostic queue rows and optimizer
  effect text.
- `T2ParallelServiceQueueRow` now accepts and preserves service-selection
  `qualification_effects` for close-parallel review rows and carries them into
  optimizer effect text.
- `route optimizer-constraint-ledger` coverage now verifies close-parallel
  service qualification effects survive into parallel-service ledger rows.
- `route optimizer-constraint-budget` coverage now verifies close-parallel
  service `qualification_effects` survive from parallel ledger rows into
  route-scoped budget rollups.
- `T2ServiceOverlayDiagnosticDecisionRow` now accepts and preserves
  service-repair `qualification_effects` for service-overlay diagnostic
  decisions.
- `T2LocalZoneOverlayHandoffRow` now accepts and preserves service-repair
  `qualification_effects` for local-zone handoff rows.
- `T2BundleOverlayRepairDeltaRow` now accepts and preserves merged
  decision/repair-target `qualification_effects` for bundle overlay replay
  deltas.
- `T2OverlayOptimizerActionDocketRow` now accepts and preserves repair-delta
  `qualification_effects` before routing held overlay work into priority review
  queues.
- `T2OverlayP2ServiceOverlayReviewRow` now accepts and preserves optimizer-action
  `qualification_effects` for service-overlay review rows.
- `T2OverlayP3LocalZoneOverlayReviewRow` now accepts and preserves
  optimizer-action `qualification_effects` for local-zone overlay review rows.
- `T2OverlayP1StructuralReadinessReviewRow` now accepts and preserves
  optimizer-action `qualification_effects` for structural readiness review rows.
- `T2BundleReadinessDispositionRow` now accepts and preserves repair-target
  `qualification_effects` for bundle readiness disposition rows.
