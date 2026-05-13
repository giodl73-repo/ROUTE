# National Segment Identity Spec

Date: 2026-05-12

## Purpose

Route designations are labels, not identities. `I-275`, `I-295`, or `I-610`
can refer to different physical corridors in different parts of the country.
A corridor can also change tier, zone assignment, route label, service name, or
promotion status without becoming a different physical segment.

This spec defines the identity grammar used by optimizer artifacts, renderers,
incident overlays, and game systems so rows can be joined, promoted, renamed,
bundled, and stitched without relying on route label alone.

## Core Rule

Stable identity must not encode mutable classification.

Do not put tier, current zone, route label, renderer layer, or promotion status
inside `national_segment_id`. Those are attributes and aliases. They may change.

## Identity Fields

Every route-segment surface should separate these concepts:

| Field | Meaning |
| --- | --- |
| `national_segment_id` | Opaque stable identity for a physical corridor segment. This survives tier promotion, zone reassignment, label changes, and renderer treatment changes. |
| `current_zone_id` | Current optimizer/map zone assignment, such as `t3-southeast`. Mutable. |
| `current_tier` | Current service tier, such as `T1`, `T2`, `T3`, or `T4`. Mutable. |
| `route_label` | Current displayed route label, such as `I65` or `US2`. Mutable and not globally unique. |
| `segment_bundle_id` | Stable identity for a corridor/service made from one or more ordered segment members. |
| `stitch_group_id` | Continuity group used to test whether segment rows may be stitched into one service, geometry line, detour, or promotion candidate. |
| `segment_aliases` | Semicolon-separated labels attached to the segment: route labels, old/new labels, zone assignments, promotion labels, and source labels. |
| `bundle_aliases` | Semicolon-separated labels attached to the bundle/service rather than only one member segment. |
| `evidence_state_scope` | Sorted state/jurisdiction codes supported by stop, source, or observation evidence. |
| `geometry_state_scope` | Sorted state/jurisdiction codes supported by validated geometry. |
| `state_scope` | Preferred published state scope. Use `geometry_state_scope` when present; otherwise use `evidence_state_scope`. |

## ID Grammar

IDs are dot-delimited and hierarchical for prefix testing, but the stable token
after the type prefix is opaque:

```text
US.HWYSEG.<stable_segment_token>
US.HWYBUNDLE.<stable_bundle_token>
US.HWYSTITCH.<stable_stitch_token>
```

Examples:

```text
US.HWYSEG.8C9A0A46F5A5D5A2
US.HWYBUNDLE.8C9A0A46F5A5D5A2
US.HWYSTITCH.8C9A0A46F5A5D5A2
```

The prefix supports coarse tests:

- `US.*` means national United States surface.
- `US.HWYSEG.*` means physical route segment.
- `US.HWYBUNDLE.*` means corridor/service bundle.
- `US.HWYSTITCH.*` means continuity/stitching group.

Zone and tier tests must use `current_zone_id` and `current_tier`, not the
stable id string.

## Alias Rules

Aliases are labels, not identity. They must use a namespace prefix.

Allowed alias namespaces:

```text
route:<normalized-route-label>
route-label:<normalized-route-label>
old-route:<normalized-route-label>
new-route:<normalized-route-label>
current-zone:<zone-id>
former-zone:<zone-id>
current-tier:<tier>
former-tier:<tier>
promotion:<promotion-label>
service-name:<service-label>
source:<source-artifact-or-system>
layer:<board-or-render-layer>
zone-route:<zone-id>:<normalized-route-label>
```

Route labels are normalized without punctuation for comparison: `I-65` and
`I65` both normalize to `I65`. If an alias changes meaning over time, preserve
the old alias and add a new one rather than rewriting history.

## Promotion

When a corridor is promoted from one tier to another, keep the same
`national_segment_id` if the physical segment is unchanged. Update
`current_tier`, add `former-tier:<old-tier>`, and add a `promotion:*` alias.

If promotion changes the physical extent, create or update a bundle whose
ordered members point at the stable physical segment ids. The bundle receives
the service/promotion aliases; member segments keep their physical identities.

## Bundles

A bundle is an ordered corridor/service made from one or more segment ids.
Bundles are normative, not optional.

Bundle rows carry:

| Field | Meaning |
| --- | --- |
| `segment_bundle_id` | Stable bundle identity. |
| `member_segment_ids` | Ordered semicolon-separated member segment ids. |
| `bundle_role` | `single-segment`, `stitched-corridor`, `promoted-service`, or `detour-service`. |
| `bundle_aliases` | Old/new service names, route labels, and promotion labels attached to the bundle. |
| `state_scope` | Union of member segment state scopes. |

Bundle gates:

- Every member id must exist in the segment registry.
- Member order is meaningful and must be preserved.
- `single-segment` bundles must have exactly one member and that member must be
  the row's `national_segment_id`.
- `stitched-corridor`, `promoted-service`, and `detour-service` bundles must
  have at least two members once they leave review.
- A bundle may overlap another bundle, but overlap must be explicit through
  shared member ids, not duplicated segment ids.
- Bundle `state_scope` must equal the ordered union of member scopes.

## Stitch Groups

A stitch group is a continuity claim. It answers whether rows can be connected
for service, geometry, detour, or promotion review.

For an atomic segment, `stitch_group_id` may be one-to-one with the segment.
For a multi-segment corridor, the bundle owns the continuity claim and should
use a stitch group shared by its member rows or recorded on the bundle row.

Stitch gates:

- Stitch ids use the `US.HWYSTITCH.*` prefix.
- A row must not rely on route label alone to join a stitch group.
- A multi-segment stitch must name ordered member segment ids.

## Stop And State Scope

`evidence_state_scope` comes from evidence-bearing stops, sources, or observed
events. `geometry_state_scope` comes from validated geometry.

`state_scope` is a convenience field:

1. Use `geometry_state_scope` when present.
2. Otherwise use `evidence_state_scope`.
3. Leave blank when neither exists, and keep the row in review.

Multi-state examples:

```text
AL;TN
IL;IN
TX;MX
```

## Current Implementation

`route t3-zone-render-board --gate` emits stable segment identity, bundle,
stitch, and alias fields for T3 zone board rows.

`route t3-zone-stop-placement --gate` carries those fields forward and adds
zone-bounded evidence scope from selected stop chains.

`route national-segment-registry --gate` merges segment-bearing artifacts into
`data/national-segment-registry.csv`, producing auditable bundle-member rows.
The registry is keyed by the bundle/member relationship, not route label alone,
because the same physical segment may participate in more than one service
bundle. Downstream optimizers, renderers, incident overlays, and game systems
should join against this registry before trusting route labels.

`route tier-segment-candidates --gate` decomposes T1/T2 selector outputs into
graph edge-level segment candidates before any route label can be promoted into
a stitched bundle. This is the pre-registry analysis surface for national and
regional service lines.

`route tier-pavement-docket --gate` joins those T1/T2 segment candidates to the
pavement standard. The registry consumes the docket so a T1/T2 bundle member is
`pass` only when pavement readiness passes; source-needed or repair-required
pavement rows keep the owning bundle in review.

`route tier-pavement-source-gaps --gate` aggregates those review members back to
bundle-level source or repair actions. This is the closure surface for
T1/T2 bundles whose only registry blocker is pavement evidence.

`route tier-pavement-acquisition-plan --gate` groups those bundle blockers into
state-level HPMS/DOT pavement source tasks. It is an acquisition planner, not a
readiness proof; member readiness still closes through `data/tier-pavement-docket.csv`.

`route tier-pavement-acquisition-docket --gate` converts acquisition-plan rows
into runnable fetch/rebuild/verify commands. The docket may be filtered by
priority and printed as a script, but it still does not promote readiness by
itself. The rebuild command is `route build --all-roads` because T2 pavement
evidence depends on US-route graph members as well as Interstate members.

`route national-segment-bundles --gate` rolls the registry up to
`data/national-segment-bundles.csv`, producing one row per service/corridor
bundle. Downstream service lines, Beck renderers, game overlays, incidents,
promotion reviews, and stitch decisions should use bundles as their default
join surface. Segment ids remain the lower-level physical members inside those
bundles.
