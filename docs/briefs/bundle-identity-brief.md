# Bundle Identity Brief

## Purpose

This brief explains why ROUTE treats a service or corridor as a bundle instead
of trusting a route label such as `I-80`, `US-287`, or `I-275` as the primary
identity.

Route labels are useful for human recognition, but they are not enough for
reviewable planning software. A label can be reused in different regions,
renamed, promoted, shortened, extended, split, or attached to only part of a
service. ROUTE therefore separates display labels from stable identities before
maps, optimizer rows, simulation incidents, reports, or game overlays can make
claims about the same object.

## Core Message

ROUTE does not ask reviewers to trust a route name. It asks them to inspect the
bundle, its ordered members, its continuity claim, its aliases, and its state
scope.

That distinction is what lets ROUTE say "this service candidate is still under
review" without losing the rows needed to price evidence debt, test a map layer,
or record a stakeholder requirement. It also prevents a map, scenario, or report
from silently treating a familiar route label as if it proved the underlying
service extent.

## Identity Layers

| Layer | Field | Reviewer Question |
|---|---|---|
| Physical member | `national_segment_id` | Which stable physical corridor segment is being referenced? |
| Service object | `segment_bundle_id` | Which ordered service or corridor bundle owns the claim? |
| Continuity claim | `stitch_group_id` | Why may these rows be stitched for service, geometry, detour, or promotion review? |
| Human labels | `route_label`, `segment_aliases`, `bundle_aliases` | What names help people recognize the object, and are any old/new names preserved? |
| Scope evidence | `evidence_state_scope`, `geometry_state_scope`, `state_scope` | Which jurisdictions are supported by evidence or validated geometry? |

The service object is the bundle. Member segments are the physical evidence
under the bundle. Labels are aliases. State scope is evidence-sensitive and may
remain blank or held when the supporting evidence is not ready.

## Why Labels Are Not Enough

Route labels fail as primary keys because they are mutable and ambiguous:

- the same label can exist in different places;
- a physical corridor can keep its identity while tier, zone, route label, or
  service name changes;
- a promoted service can use multiple old and new names;
- a detour or relay service may use only part of a labeled route;
- a multi-state corridor needs explicit state scope rather than inferred scope
  from a route name.

The safe reviewer posture is therefore:

1. Start with `segment_bundle_id` for service claims.
2. Expand to `member_segment_ids` when physical graph, pavement, bridge,
   geometry, or stop evidence is being inspected.
3. Use `stitch_group_id` only for continuity or restitch questions.
4. Treat aliases and route labels as lookup aids, not proof.
5. Hold the claim when a row cannot attach the required identity yet.

## What The Bundle Registry Does

`data/national-segment-bundles.csv` is the portable service/corridor registry.
`route_network::BundleRegistry` is the in-process resolver that consumers should
use when they render maps, attach incidents, simulate upgrades, publish reports,
or bind game overlays.

The registry supports lookups by bundle id, member segment id, stitch group,
alias, route label, tier, and zone. Exact bundle id is the preferred lookup.
Alias and route-label lookup are compatibility bridges and may return multiple
rows. A consumer that gets multiple bundle rows must disambiguate explicitly or
declare that it is producing an aggregate view.

## Claim Boundary

Bundle identity makes a claim inspectable. It does not make the claim true.

| Safe To Say | Do Not Say |
|---|---|
| "This artifact carries a stable service/corridor bundle id." | "This bundle proves the corridor should be built." |
| "Member segments make the physical extent reviewable." | "The route label proves the physical extent." |
| "A stitch group records the continuity claim to inspect." | "The stitch group proves operational continuity." |
| "Aliases preserve old, new, and display names." | "A familiar route name is enough evidence." |
| "State scope must come from geometry or evidence." | "The route name proves all affected jurisdictions." |
| "Held rows remain visible until identity or evidence closes." | "A visible row is ready for publication, funding, or service promises." |

The brief does not claim an official plan, construction readiness, guaranteed
service, positive ROI, legal eligibility, agency compliance, public release
readiness, or stakeholder endorsement.

## Reviewer Pressure Questions

Use these questions in technical reviews:

- Which `segment_bundle_id` owns the service claim?
- Which ordered `member_segment_ids` define the physical extent?
- Does the bundle overlap another bundle, and is that overlap explicit?
- Which aliases are display labels, old labels, promoted names, or source names?
- Is `state_scope` geometry-backed, evidence-backed, or still held?
- If a route-label lookup returns more than one bundle, what disambiguation rule
  did the consumer use?
- Does a map, report, simulation, or game overlay preserve the bundle id in its
  output metadata?
- If the row is held, which next artifact can attach or repair identity?

## Current Evidence Surface

The current local documentation and gates that support this brief are:

| Surface | Role |
|---|---|
| `docs/route-architecture.md` | Names bundle-first identity as the ROUTE architecture rule. |
| `docs/national-segment-identity-spec.md` | Defines segment, bundle, stitch, alias, and state-scope grammar. |
| `docs/bundle-registry-spec.md` | Defines registry lookup order, ambiguity rules, snapshot contract, consumer contract, and gates. |
| `docs/tier-segment-stitching-spec.md` | Defines the selector-to-segment-to-bundle workflow for T1/T2 stitching. |
| `data/national-segment-registry.csv` | Machine-readable bundle/member relationship surface. |
| `data/national-segment-bundles.csv` | Machine-readable service/corridor bundle rollup. |

This is enough for an internal reviewer to inspect the identity model. It is not
enough to promote any corridor, service, map, scenario, ROI, or construction
claim without the separate evidence gates named by the relevant surface.
