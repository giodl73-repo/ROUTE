# ROUTE Pitfalls

## ROUTE-PF-01: Render-valid maps become proof-valid maps

**Pattern:** A generated map passes file, geometry, or browser checks and then
gets described as evidence-valid, SLA-valid, transit-ready, upgrade-ready, or
publication-ready.

**Domain:** Map generation, reports, decks, media packets, and public claims.

**Why it is hard to catch:** The visual artifact looks complete and often
compresses several unresolved source and claim holds into a clean image.

**Structural solution:** Keep map publication scope separate from map rendering
and require claim labels, publication inventories, and VTRACE evidence rows
before stronger claims.

**Status:** MITIGATED

**Evidence:** `docs/map-publication-scope.md`, `README.md`, and
`docs/vtrace/REQUIREMENTS.md` REQ-002, REQ-003, REQ-006, REQ-007, and REQ-010.

## ROUTE-PF-02: Route labels become hidden IDs

**Pattern:** Route names, tiers, map layers, or zones become convenient joins
for segment-bearing data even though they are presentation attributes.

**Domain:** Network architecture, generated data, maps, simulations, reports,
and game objects.

**Why it is hard to catch:** Labels are human-legible and stable-looking in
small examples.

**Structural solution:** Use bundle/member identity for service and physical
surfaces, and hold rows that cannot attach identity yet.

**Status:** MITIGATED

**Evidence:** `docs/route-architecture.md`, `docs/vtrace/REQUIREMENTS.md`
REQ-004 and REQ-005, and `docs/vtrace/TRACE.md`.

## ROUTE-PF-03: Held evidence is cleaned up as stale documentation

**Pattern:** Source gaps, heuristic assumptions, rejected alternatives, or
review holds are removed during cleanup because they look like obsolete work.

**Domain:** Evidence ledgers, review records, release manifests, wave closeouts,
and public-scope documents.

**Why it is hard to catch:** Removing holds can make documentation shorter and
more confident without changing the underlying evidence.

**Structural solution:** Treat held evidence as a first-class result until a
later source, command, or review record explicitly closes or supersedes it.

**Status:** MITIGATED

**Evidence:** `docs/vtrace/REQUIREMENTS.md` REQ-003,
`docs/vtrace/REVIEW.md`, and `docs/vtrace/EVIDENCE.md`.

## ROUTE-PF-04: Review roles become decorative

**Pattern:** A claim mentions role review, but no selected lane, dissent,
required change, or evidence pointer is recorded.

**Domain:** Parliament, stakeholder, editorial, panel, assurance, and V&V review
records.

**Why it is hard to catch:** The repo has many role files, so the existence of a
panel can be mistaken for execution of a review.

**Structural solution:** Require executable review records with selected roles,
role rationale, dissent or tension, required change, decision, and evidence
pointer.

**Status:** SOLVED

**Evidence:** `docs/vtrace/REVIEW.md` finding FIND-001 and `.roles/ROLE.md`.
