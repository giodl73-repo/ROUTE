# Concept of Operations

## Scope

Repo: ROUTE

VTRACE adoption scope: describe the operating scenarios that requirements and
specification baselines must preserve. CONOPS is not a design proposal; it is
the bridge from mission needs to observable repo workflows.

## Actors

| Actor | Responsibility | Needs |
|---|---|---|
| ROUTE maintainer | Own repo truth, active goals, generated artifacts, and release posture. | Clear commands, evidence labels, review gates, and scoped child-repo changes. |
| Coding agent | Make bounded changes to docs, data, commands, tests, and generated artifacts. | Parent IDs, affected surfaces, validation commands, and stop conditions. |
| Review steward | Run `.roles` review lanes and record changes to claims, holds, or next evidence steps. | Mission/requirement IDs, review scope, and artifacts to inspect. |
| Transportation analyst | Inspect score, gap, SLA, map, and standards outputs. | Reproducible artifacts with source, confidence, and evidence posture. |
| Route-network owner | Preserve bundle-first identity across segment-bearing artifacts. | Identity gates, bundle/member data, and migration holds for transitional surfaces. |
| Route-map / schematic owner | Keep visible stops, route services, bends, transfers, labels, and service classes truthful. | Map/SLA consistency checks and false-transfer prevention. |
| Stakeholder reviewer | Apply State DOT, freight, rural, transit, and environmental/community-health lenses. | Claims and feature options that name delivery, access, operations, and mitigation posture. |

## Scenarios

### OPS-001: Regenerate And Verify Active Artifacts

Trigger: a maintainer or agent changes source data, Rust logic, generated CSVs,
maps, SLA surfaces, docs, or claim labels.

Inputs: repo-local source data, Rust workspace, current active goal, documented
commands, existing generated artifacts, and review/gate expectations.

Normal path:

1. Identify the authoritative source files and generated outputs affected by
   the change.
2. Run the smallest relevant regeneration command or gate bundle.
3. Confirm generated artifacts either match the intended change or remain
   unchanged.
4. Record command evidence and any claim-label changes in the appropriate
   VTRACE evidence, verification, or future trace artifact.

Failure or degraded path: if a command cannot run, data is missing, generated
artifacts drift unexpectedly, or a source is blocked, the work remains held with
an explicit blocker, next evidence step, and affected requirement IDs.

Outputs: regenerated artifacts, validation command results, evidence rows, and
updated claim posture where needed.

Handoffs: maintainer to reviewer for evidence/claim changes; maintainer to
portfolio owner only when a child commit and TRACKER pointer update are
requested.

Validation evidence: local command output, generated artifact diff, future
`EVID-*` rows.

### OPS-002: Preserve Bundle-First Identity

Trigger: a change touches route services, national segments, bundles, stops,
SLA rows, maps, simulations, incidents, reports, game overlays, or optimizer
surfaces.

Inputs: `docs/route-architecture.md`, segment/bundle data, affected crates,
affected generated artifacts, and identity-related requirements.

Normal path:

1. Determine whether each affected row is segment-level, bundle-level,
   stitch-level, or transitional.
2. Require `segment_bundle_id`, `national_segment_id`, `stitch_group_id`, or a
   declared transitional surface for segment-bearing rows.
3. Run the relevant architecture, data, or artifact gate.
4. Hold rows that depend only on mutable route labels, tiers, map ids, or zones.

Failure or degraded path: if identity cannot be attached yet, the row remains
review/held and points to the artifact or work package that will attach stable
identity.

Outputs: identity-safe artifact changes, held rows, repair dockets, or gate
failures.

Handoffs: route-network owner to route-map, route-sim, route-report, route-cli,
or review steward depending on affected surface.

Validation evidence: architecture gate output, artifact inspection, future
`TRACE.md` links.

### OPS-003: Close A Stop-First SLA Work Slice

Trigger: a work package changes stop placement, SLA graph rows, T1/T2/T3
service definitions, route endpoint/contact rules, service classes, schematic
geometry, or generated Beck/T2/T3 artifacts.

Inputs: `GOAL.md`, stop/SLA data, route service standards, map diagnostics,
affected Rust crates, generated maps/CSVs, and `.roles` map/review lenses.

Normal path:

1. Name the parent `REQ-*` and affected stop/SLA/map surfaces.
2. Update source rows or Rust logic before generated artifacts.
3. Regenerate the affected SLA, candidate, standards, diagnostics, and map
   outputs.
4. Run L0/L1/L2 gates appropriate to the affected crates and artifacts.
5. Use schematic and stakeholder review lenses when visual service meaning,
   transfer truth, or user access changes.

Failure or degraded path: if oversized gaps, endpoint/contact defects, false
transfers, dense labels, or map/SLA mismatches remain, the slice stays held or
creates a follow-on repair docket instead of being treated as release-ready.

Outputs: closed stop/SLA work package, regenerated artifacts, gate results,
review notes, and evidence rows.

Handoffs: route-cli / route-network owners to route-map owner and review
steward.

Validation evidence: command gates, generated artifact diffs, review notes,
future `EVID-*` rows.

### OPS-004: Promote Or Hold A Design Claim

Trigger: a corridor, standard, feature package, game mechanic, research claim,
or publication-facing statement is proposed for downstream use.

Inputs: evidence labels, generated artifacts, simulation output, source ledgers,
review records, stakeholder lenses, and non-goal constraints.

Normal path:

1. Confirm the claim has implemented, heuristic, planned, held, deprecated,
   source-needed, or confidence-limited status.
2. Verify that review roles have either challenged the claim or explicitly
   accepted its current posture.
3. Check that delivery, freight, rural/access, transit, and environmental
   concerns are represented when the claim affects those users.
4. Promote only the bounded claim; keep construction readiness, compliance, and
   agency endorsement out of scope.

Failure or degraded path: if evidence, review, or scope is insufficient, the
claim remains held, downgraded, or redirected to a next evidence step.

Outputs: promoted claim, held claim, downgraded claim, review record, or next
evidence docket.

Handoffs: review steward to maintainer, research/design owner, or game/system
designer.

Validation evidence: review record, evidence ledger row, gate output, future
trace link.

### OPS-005: Apply VTRACE One File At A Time

Trigger: a maintainer asks to adopt VTRACE for ROUTE stage by stage.

Inputs: existing ROUTE docs, local `.roles`, VTRACE templates, active ROUTE
worktree state, and current `docs/vtrace/` artifacts.

Normal path:

1. Create or update one VTRACE artifact at a time.
2. Use prior VTRACE IDs as parent IDs for the new artifact.
3. Review the artifact against relevant ROUTE `.roles`.
4. Run whitespace/document validation before moving to the next stage.
5. Avoid commits or TRACKER pointer updates unless explicitly requested.

Failure or degraded path: if repo state is dirty, detached, or contains
unrelated changes, keep edits scoped to the requested VTRACE artifact and report
the local status.

Outputs: one reviewed VTRACE artifact with stable IDs for later trace.

Handoffs: maintainer to next-stage artifact author.

Validation evidence: `git diff --check` for touched VTRACE docs and role review
notes.

## Operational Assumptions

- ROUTE already has substantial implementation, generated artifacts, and review
  history; VTRACE adoption is a retrofit, not a greenfield process.
- The active VTRACE sequence starts with mission, CONOPS, requirements, and then
  specification baseline before implementation planning.
- Some ROUTE commands may be expensive or blocked by unrelated local worktree
  state; VTRACE docs should record intended validation commands even when a
  full gate is deferred.
- `.roles` review is part of ROUTE operations and must change evidence posture,
  claim labels, dockets, or next steps when it finds a gap.
- TRACKER remains the portfolio snapshot repo; ROUTE owns repo-local
  implementation and VTRACE artifacts.

## Role Review Notes

| Role Lens | CONOPS Impact | Disposition |
|---|---|---|
| Scope Keeper | CONOPS describes operating workflows and avoids prescribing a specific corridor, gap, or design proposal. | pass |
| Citation Auditor | CONOPS introduces no new quantitative claims; it names repo-local artifacts and future evidence paths. | pass |
| Numeracy Checker | CONOPS contains no arithmetic, units, score ranges, cost figures, or traffic volumes. | pass |
| Optimization Methodologist | Scenarios require source/generated separation, deterministic gates, held rows, and explicit identity constraints. | pass |
| Schematic Cartographer | Stop-first SLA and map workflow preserves transfer truth, stop order, service classes, and map/SLA consistency. | pass |
| Traffic Engineer / Freight Economist / Rural Advocate | Claim promotion flow requires operational, freight, and rural/access evidence before downstream use. | pass |
| State DOT / Transit-Dependent / Environmental Stakeholders | Claim promotion flow keeps delivery feasibility, non-driving access, and environmental/community-health effects in scope. | pass |

## Open Questions

| ID | Question | Disposition |
|---|---|---|
| OQ-001 | Which exact command bundle should become ROUTE's VTRACE L2 gate for the current stop-first SLA work? | Defer to `VERIFICATION.md` or `WORK_PACKAGES.md` after specification baseline. |
| OQ-002 | Which existing ROUTE ledgers should become the first evidence sources for VTRACE `EVID-*` rows? | Defer to `EVIDENCE.md` after trace and verification artifacts exist. |
