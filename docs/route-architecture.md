# ROUTE Architecture

## Purpose

This document owns the core architecture invariant for ROUTE:

> Bundles are the core service abstraction. Segment ids are stable physical
> members. Route labels are presentation attributes.

ROUTE may display, score, render, promote, rename, or bundle a corridor using
labels such as `I-65`, `US2`, or `T3 Southeast`, but internal artifacts must not
use those labels as primary identity once a row describes a physical route
segment or a service built from segments. The default join key for a service,
line, incident overlay, upgrade, promise, map row, or game object is
`segment_bundle_id`; `national_segment_id` is used when the row is explicitly
about one physical member inside a bundle.

## Core Abstraction

A bundle is the object ROUTE plans, draws, upgrades, simulates, promises
against, and teaches to the game. It can be:

- a single physical segment;
- a stitched corridor made from multiple physical segments;
- a promoted service that keeps old names and new names as aliases;
- a detour, relief, or special-lane service over known members.

Every segment-bearing artifact should either carry `segment_bundle_id` directly
or point to `data/national-segment-bundles.csv` as the join surface. Segment ids
remain mandatory because bundles need auditable physical membership, but
segments are not the user-facing service unit.

## Core Spine

The architecture has four identity layers:

| Layer | Key | Purpose |
| --- | --- | --- |
| Physical segment | `national_segment_id` | Stable identity for a physical corridor segment. |
| Service/corridor bundle | `segment_bundle_id` | Ordered set of one or more physical segments. |
| Continuity claim | `stitch_group_id` | Claim that rows may be stitched for service, geometry, detour, or promotion review. |
| Mutable labels | aliases and attributes | Tier, zone, route label, map layer, promotion status, source labels. |

Everything else attaches to those layers:

```text
raw network geometry
  -> national segment identity
  -> T1/T2 segment candidates from selector analysis
  -> pavement / asset-condition service floors
  -> bundles as service/corridor objects
  -> stitch groups for continuity claims
  -> tier optimizer columns
  -> stops / contacts / state scopes
  -> Beck geometry and game overlays
  -> incidents, upgrades, promotions, and release claims
```

If an artifact cannot yet produce a segment id, it must remain a transitional
surface and name the next artifact that will attach identity.

## Non-Negotiable Rules

1. `route`, `route_label`, `zone_id`, map id, and tier are never sufficient
   primary keys for segment-bearing artifacts.
2. `national_segment_id` must not encode mutable tier, zone, route label,
   renderer layer, or promotion status.
3. A route designation can appear many times in the registry when it names
   different physical segments.
4. Every service/corridor row should use a bundle as its primary identity.
5. Stops and geometry may refine `state_scope`, but route labels may not infer
   state scope by themselves.
6. Every stable segment must belong to a bundle, even when that bundle contains
   only one segment.
7. Renderers, game overlays, incident ledgers, and promotion dockets should join
   through bundles first, then fall back to individual segment ids only when
   the artifact is explicitly segment-level.

## Core Artifacts

| Artifact | Architectural role |
| --- | --- |
| `docs/national-segment-identity-spec.md` | Defines segment, bundle, stitch, alias, and state-scope grammar. |
| `docs/tier-segment-stitching-spec.md` | Defines the T1/T2 rule that selector outputs nominate service needs, graph edges become segment candidates, and only validated ordered members become bundles. |
| `docs/tier-pavement-standards.md` | Defines the pavement/ride-quality floor that candidate and bundle members must satisfy before SLA, freight, transit, or promotion claims pass. |
| `data/national-segment-registry.csv` | Current machine-readable identity registry. |
| `data/national-segment-bundles.csv` | Current machine-readable service/corridor bundle registry derived from segment identity. |
| `data/tier-segment-candidates.csv` | T1/T2 selector output decomposed into graph-edge bundle members before route labels can become services. |
| `data/tier-pavement-docket.csv` | Segment-member pavement readiness surface consumed by the registry before T1/T2 bundles can claim service readiness. |
| `data/tier-pavement-source-gaps.csv` | Bundle-level action docket for pavement source/repair blockers holding T1/T2 bundles in review. |
| `data/tier-pavement-acquisition-plan.csv` | State-level source acquisition plan for pavement blockers that must feed back into member readiness. |
| `data/tier-pavement-acquisition-docket.csv` | Runnable source-acquisition task list for refreshing pavement evidence and rerunning bundle readiness gates. |
| `data/t3-zone-render-board.csv` | First renderer-facing artifact carrying stable segment identity. |
| `data/t3-zone-stop-placement.csv` | First stop-placement artifact carrying stable segment identity and evidence state scope. |
| `data/tier-optimizer-runs.csv` | Run certificate proving identity and bundle artifacts participate in the optimizer bundle. |

## Crate Direction

Bundle construction is now a shared `route-network` responsibility. New code
should use `route_network::SegmentBundleMember`,
`route_network::SegmentBundle`, `route_network::BundleStatus`,
`route_network::build_segment_bundles`, and `route_network::bundle_action`
instead of rebuilding bundle rollups inside downstream crates.
Efficient lookup semantics are owned by `docs/bundle-registry-spec.md` and
implemented through `route_network::BundleRegistry`.
`route bundle-architecture --gate` emits `data/bundle-architecture.csv` as the
crate-level adoption certificate for this rule.

The current implementation still creates several identity rows in `route-cli`.
That is acceptable as an implementation slice, but the architecture target is:

| Crate | Identity responsibility |
| --- | --- |
| `route-data` | Parse raw source records only; downstream code converts source rows into bundle members after identity is known. |
| `route-network` | Own `SegmentBundleMember`, `SegmentBundle`, `BundleStatus`, bundle rollups, segment ids, geometry state scope, and graph-to-segment mapping. |
| `route-score` | Score bundles through `score_bundle`; keep `score_corridor` as the member/corridor compatibility layer. |
| `route-map` | Render bundle-aware products through `build_bundle_svg`; never infer topology from route labels. |
| `route-sim` | Attach incidents, detours, and SLA outcomes through `BundleIncidentSpec` or bundle ids. |
| `route-report` | Publish bundle-aware corpus entries through `write_bundle_corpus_entry`. |
| `route-cli` | Orchestrate commands and gate artifacts, not own identity policy long-term. |

## Migration Contract

Older artifacts can still expose `route` while they are being migrated. New or
modified optimizer artifacts should include, or explicitly point to, one of:

- `national_segment_id`
- `segment_bundle_id`
- `stitch_group_id`
- `data/national-segment-bundles.csv`
- `data/national-segment-registry.csv`

When a row cannot attach identity yet, it must be labeled as review/held and
point to the identity-producing artifact in `next_artifact`.
