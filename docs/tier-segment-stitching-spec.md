# Tier Segment Stitching Spec

## Purpose

T1 and T2 analysis must not assume that an entire route designation is the
service line. `I80`, `I95`, or `US287` are labels over many physical graph
segments. ROUTE must choose segment members, then stitch those members into a
bundle only when the selector evidence justifies that extent.

## Rule

For T1 and T2, the optimizer works in this order:

1. Promise, stop, contact, and budget analysis nominates a service need.
2. The highway graph decomposes that service into edge-level segment
   candidates.
3. Segment candidates are scored and pruned between selected stops, contacts,
   bottlenecks, and promise-relevant terminals.
4. Only the retained ordered members become a stitched service bundle.
5. Route labels remain aliases on the stitched bundle, not the identity of the
   bundle itself.

Whole-route selection is allowed only as a transitional selector surface. It is
not sufficient to bind maps, incidents, upgrades, or game levers.

## Segment Candidate Artifact

`route tier-segment-candidates --gate` emits
`data/tier-segment-candidates.csv`.

Each row represents one graph edge candidate for a T1 or T2 service. The row
carries:

- `national_segment_id`: stable identity for the physical edge candidate;
- `segment_bundle_id`: provisional stitched-service candidate id;
- `stitch_group_id`: continuity claim for the candidate service;
- `edge_id`, `edge_sequence`, `state`, `length_miles`, `aadt`, and
  `lane_count`: graph-member evidence used before bundle promotion;
- selector lineage showing whether the candidate came from T1 SLA selection or
  T2 service selection.

The artifact is intentionally pre-registry. A candidate bundle must still be
pruned, ordered by stops/contacts, and validated before it becomes a committed
service bundle in `data/national-segment-registry.csv` and
`data/national-segment-bundles.csv`.

## T1 Standard

A T1 bundle is valid only after segment members are bounded by:

- selected T1 promise pairs;
- selected T1 stops and METIS split boundaries;
- top-city or national-transfer stops that define where the service should bend
  or terminate;
- SLA feasibility between those stops.

The selector may say that `I80` is needed, but the bundle must say which I-80
segments are part of the national promise spine.

## T2 Standard

A T2 bundle is valid only after segment members are bounded by:

- at least two T1/T2 contact or transfer nodes unless an endpoint exception
  explicitly allows a terminal;
- regional service class and parent-trunk lineage;
- duplicate/parallel-service checks;
- T2 stop placement and restitch rules.

The selector may say that `US287` is a connector, but the bundle must say which
US-287 segments connect which transfer/service nodes.

## Gate Meaning

The first gate proves decomposition, not final promotion. Passing
`route tier-segment-candidates --gate` means selected T1/T2 services have graph
edge candidates. It does not mean the full route is accepted into T1/T2, and it
does not allow downstream consumers to target route labels instead of bundle
ids.
