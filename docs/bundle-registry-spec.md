# Bundle Registry Spec

## Purpose

The bundle registry is the shared resolver for ROUTE service/corridor identity.
`data/national-segment-bundles.csv` is the portable artifact; `route_network::BundleRegistry`
is the in-process index that renderers, simulations, game overlays, reports, and
optimizer repair ledgers should use.

Bundles are the core abstraction. Route labels are display aliases. Segment ids
are physical members.

## Registry Inputs

The registry indexes `route_network::SegmentBundle` rows. Those rows are
normally produced by:

1. `route_network::build_segment_bundles`;
2. `route national-segment-bundles --gate`;
3. CSV loading into `SegmentBundle` when a consumer starts from
   `data/national-segment-bundles.csv`.

## Required Indexes

`BundleRegistry` must support these lookups:

| Lookup | Use |
|---|---|
| `segment_bundle_id` | Canonical service/corridor identity. |
| route label | Compatibility lookup for route-shaped inputs such as `I-80` or `I80`. |
| bundle alias | Compatibility lookup for old route names, promoted names, and service names. |
| member segment id | Resolve physical graph/geometry members to their owning bundle. |
| stitch group id | Resolve continuity and restitch claims. |
| current tier | Filter T1/T2/T3/T4 service sets. |
| current zone | Filter regional or zone maps. |

Route-label lookup normalizes punctuation and case. For example, `I-80`, `I80`,
and `i_80` resolve to the same key.

Alias lookup uses the same normalization. A typed alias such as `old-route:I80`
must also index the value after the colon, so compatibility callers can resolve
either `old-route:I80` or `I80` without knowing the alias type.

## Ambiguity Rules

The registry may return multiple bundles for a route label, alias, tier, zone,
member segment, or stitch group. Consumers must not silently pick an arbitrary
row when multiple bundles are returned unless their product explicitly accepts
an aggregate view.

Multi-match lookups must return registry rows in artifact order. That gives
renderers and scenario builders deterministic output while still forcing the
consumer to choose an explicit disambiguation rule.

Consumers should prefer:

1. exact `segment_bundle_id`;
2. exact member segment id when processing graph/geometry;
3. exact stitch group id when processing continuity or restitch work;
4. alias lookup only as a migration bridge for renamed or promoted services;
5. route-label lookup only as a compatibility bridge;
6. tier/zone lookup only for filtering candidate sets.

## Snapshot Contract

Registry consumers should build the registry once per artifact snapshot and pass
that instance through the render/simulation/report operation. A product must not
mix bundle rows from one `data/national-segment-bundles.csv` snapshot with member
segments, overlays, or gates generated from another snapshot unless the artifact
declares itself as a migration comparison.

## Consumer Contract

Renderers:

- store `segment_bundle_id` in output metadata;
- use member segments for physical geometry;
- do not infer topology from route labels.

Simulations:

- attach incidents and upgrades to bundle ids;
- expand bundles to member segments/edges before capacity changes;
- preserve bundle id in scenario output.

Game overlays:

- bind levers, incidents, upgrades, and restitches to bundle ids;
- mark rows as `bundle-binding-pending` rather than targeting route labels when
  a bundle is not yet available.

Reports:

- publish bundle ids in frontmatter;
- list member segment ids as evidence, not as the primary service identity.

## Gates

The current gates are:

- `route national-segment-bundles --gate`: verifies bundle rows and member ids.
- `route bundle-architecture --gate`: verifies crate-level bundle-facing APIs.
- `route t2-bundle-overlays --gate`: verifies game/ops T2 overlay rows either
  bind to bundles or explicitly declare pending bundle binding.
