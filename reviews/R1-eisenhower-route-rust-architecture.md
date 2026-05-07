---
name: Parliament Review — Eisenhower — route-rust-architecture
slug: R1-eisenhower-route-rust-architecture
type: review
artifact: specs/2026-05-06-route-rust-architecture.md
voice: eisenhower
round: 1
status: draft
rubric_version: v1.0
author: eisenhower
created: 2026-05-06
---

> **Note:** This is an AI-generated simulated review, written by Claude in
> the voice of a named-expert persona. It is not an actual review by the
> named person and does not reflect their views or endorsement.

# Review — General Eisenhower

## Overall

A sound starting architecture. The NHS shapefile is the correct data source — it is the national strategic network as officially designated, which is what I care about. The pipeline is disciplined: fetch, build, score, output. The anchor-first principle is the right instinct; we built the interstate system by completing routes, not by drawing maps. My concern is that B2 (Network Centrality) as described may not capture what I mean by strategic network value, and the treatment of proposed corridors is too thin for what the project ultimately needs to answer.

## What works

**NHS as the data source (§4.1)**: The National Highway System shapefile is the authoritative federal designation. Using it as the corpus foundation means we are analyzing the actual strategic network, not some academic approximation of it. Every corridor in scope earned its designation. This is the right starting point.

**`route build` → `route score` separation (§6)**: Building the graph once and scoring from the cache is operationally correct. In WWII logistics, we built the supply network first and then routed through it — you don't rebuild the road every time a truck crosses it. The same discipline applies here.

**Betweenness centrality via petgraph (§5, §4.2)**: Computing B2 (Network Centrality) from actual graph topology is correct. A corridor that carries traffic because it is geometrically unavoidable — not because planners said it was important — is the one that matters strategically. The graph tells you what the map obscures.

**METIS deferred with a clear future path (§3)**: Deferring METIS until the gap analysis phase is correct. We did not build the entire interstate system before we drove the first truck on it.

**`route gap --type bottleneck|resilience|equity|intermodal` (§6)**: Having discrete gap types that produce named findings is operationally useful. A general "something is wrong" finding is not actionable. Named gap types are.

## What doesn't work

**B2 centrality computation timing (§4.2, §5)**: "Computed once after graph construction; stored per-edge and aggregated per-corridor." This is correct for the final national graph but the spec is silent on what happens when the national graph is incomplete — which it will be during the corpus build phase when only some corridors have been fetched. Betweenness centrality on a partial graph is misleading; a corridor that appears central in a 20-route graph may not be central in a 70-route graph. The spec needs to distinguish Phase 1 centrality estimates from final national centrality.

**Proposed corridor geometry from ArcGIS (§9, §6)**: "Geometry comes from ArcGIS project exports (GeoJSON) or hand-drawn alignments." This is the weakest part of the spec. Proposed corridors are where the strategic argument is most important — and their geometry determines everything about how they interact with the existing network. A hand-drawn alignment that misses a mountain pass, avoids a river crossing, or ignores an existing rail corridor is not a strategic proposal; it is a sketch. The spec needs at minimum a stated standard for how proposed alignments are sourced and validated before scoring.

**No defense or military logistics dimension**: The original interstate system had explicit defense criteria — lane widths, grade limits, and clearances set so military vehicles could use every route. The data model has no field for this. For a project claiming to design Interstate 2.0, the defense utility of proposed corridors is not captured. I would add an `oversize_clearance` flag or a `military_route_flag` to `HighwayEdge` at minimum.

## The question I'd push on

B2 Network Centrality is my primary stake, and the spec says it is computed by Brandes algorithm over `HighwayGraph`. But `HighwayGraph` as defined uses `petgraph::Undirected` — undirected edges. Real highway traffic is directional: freight moves differently east vs. west, the peak-direction load matters. Is undirected the right model for strategic network analysis? If a corridor carries 80% of its load in one direction (farm-to-port, not port-to-farm), undirected centrality understates its strategic importance in that direction. I want this addressed before we score B2 on the anchor.
