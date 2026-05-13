# ROUTE Architecture

## Purpose

This document owns the core architecture invariant for ROUTE:

> Route labels are presentation attributes. Stable segment identity is the
> system join key.

ROUTE may display, score, render, promote, rename, or bundle a corridor using
labels such as `I-65`, `US2`, or `T3 Southeast`, but internal artifacts must not
use those labels as primary identity once a row describes a physical route
segment or a service built from segments.

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
  -> bundles / stitch groups
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
4. A service line can contain many physical segments through a bundle.
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
| `data/national-segment-registry.csv` | Current machine-readable identity registry. |
| `data/national-segment-bundles.csv` | Current machine-readable service/corridor bundle registry derived from segment identity. |
| `data/t3-zone-render-board.csv` | First renderer-facing artifact carrying stable segment identity. |
| `data/t3-zone-stop-placement.csv` | First stop-placement artifact carrying stable segment identity and evidence state scope. |
| `data/tier-optimizer-runs.csv` | Run certificate proving identity and bundle artifacts participate in the optimizer bundle. |

## Crate Direction

The current implementation still creates several identity rows in `route-cli`.
That is acceptable as an implementation slice, but the architecture target is:

| Crate | Identity responsibility |
| --- | --- |
| `route-network` | Own segment ids, geometry state scope, and graph-to-segment mapping. |
| `route-score` | Score segments and bundles without treating route labels as identity. |
| `route-map` | Consume selected segments, bundles, stops, and stitch groups; never infer topology from labels. |
| `route-sim` | Attach incidents, detours, and SLA outcomes to segment or bundle ids. |
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
