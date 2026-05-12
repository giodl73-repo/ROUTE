# National Segment Identity Spec

Date: 2026-05-12

## Purpose

Route designations are labels, not identities. `I-275`, `I-295`, or `I-610`
can refer to different physical corridors in different parts of the country,
and a corridor can change tier, service name, or promoted role without becoming
a different physical segment.

This spec defines the identity grammar used by optimizer artifacts, renderers,
and game overlays so route rows can be joined, promoted, renamed, bundled, and
stitched without relying on route label alone.

## Identity Fields

Every route-segment surface should separate these concepts:

| Field | Meaning |
| --- | --- |
| `national_segment_id` | Stable identity for a physical corridor segment. This should survive tier promotion, label changes, and renderer treatment changes. |
| `segment_bundle_id` | Stable identity for a corridor/service made from one or more ordered segment members. A bundle may contain one segment today and many later. |
| `stitch_group_id` | Join key for rows that describe the same physical segment across board layers, gap ledgers, stop placement, geometry, incidents, and game overlays. |
| `segment_aliases` | Semicolon-separated labels that may change over time: old route labels, new promotion names, local names, board layers, and source labels. |
| `state_scope` | Sorted semicolon-separated state or state-like jurisdiction codes touched by validated stop/geometry evidence. Multi-state segments stay plural. |

## ID Grammar

IDs are dot-delimited and hierarchical, following the same spirit as GEOID-style
prefix testing:

```text
US.HWYSEG.<zone_or_region>.<segment_code>
US.HWYBUNDLE.<zone_or_region>.<bundle_code>
US.HWYSEG.<zone_or_region>.<segment_code>.STITCH
```

Examples:

```text
US.HWYSEG.T3SOUTHEAST.I65
US.HWYBUNDLE.T3SOUTHEAST.I65
US.HWYSEG.T3SOUTHEAST.I65.STITCH
```

The prefix supports coarse tests:

- `US.*` means national United States surface.
- `US.HWYSEG.*` means physical route segment.
- `US.HWYBUNDLE.*` means a service/corridor bundle.
- `US.HWYSEG.T3SOUTHEAST.*` means a segment currently attached to the
  Southeast T3 zone surface.

## Aliases And Promotion

Aliases are not identity. They are labels attached to the identity.

Allowed alias examples:

```text
route:I65
zone-route:t3-southeast:I65
old-route:US66
new-route:I40
promotion:T3-to-T2-candidate
service-name:South-Central-Relay
```

When a corridor is promoted from T3 to T2, the optimizer should keep the same
`national_segment_id` when the physical segment is unchanged and add promotion
or service aliases. If promotion changes the physical extent, create or update
a `segment_bundle_id` whose ordered members point at the physical segment ids.

## Bundles

A bundle is an ordered service/corridor made from one or more segment ids.

Bundle rows should eventually carry:

| Field | Meaning |
| --- | --- |
| `segment_bundle_id` | Bundle identity. |
| `member_segment_ids` | Ordered semicolon-separated member segment ids. |
| `bundle_role` | `single-segment`, `stitched-corridor`, `promoted-service`, or `detour-service`. |
| `bundle_aliases` | Old/new service names, route labels, and promotion labels. |
| `state_scope` | Union of member segment state scopes. |

Single-segment bundles are valid. They give us a stable place to grow when a
future corridor is made by stitching multiple segments together.

## Stop And State Scope

`state_scope` must come from evidence-bearing stops or geometry, not from route
number guesses. A route with no validated in-zone stops may have a blank
`state_scope` and must remain in review until stop/geometry evidence fills it.

Multi-state examples:

```text
AL;TN
IL;IN
TX;MX
```

## Current Implementation

`route t3-zone-render-board --gate` emits the segment identity, bundle, stitch,
and alias fields for T3 zone board rows.

`route t3-zone-stop-placement --gate` carries those fields forward and adds
zone-bounded `state_scope` from selected stop chains. This is the first guard
against duplicate route labels being treated as the same physical corridor.
