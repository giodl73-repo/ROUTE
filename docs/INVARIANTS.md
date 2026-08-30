# ROUTE Invariants

## ROUTE-I-01: Segment-bearing artifacts preserve stable identity

**Claim:** Segment-bearing artifacts do not rely only on mutable route labels,
tiers, zones, or map IDs as primary identity.

**Why it matters:** Mutable labels cannot safely join generated artifacts across
maps, SLA surfaces, simulations, reports, and reviews.

**Test:** `docs/vtrace/VERIFICATION.md` VER-004 and VER-005; selected
architecture inspections over `docs/route-architecture.md`, `route-network`,
and segment-bearing schemas.

**Status:** VERIFIED

## ROUTE-I-02: Public claims carry evidence posture

**Claim:** Material public claims remain labeled with evidence posture such as
implemented, heuristic, planned, held, source-needed, or confidence-limited.

**Why it matters:** ROUTE has public-facing maps, reports, decks, and buyer
materials; unlabeled claims can read as official, construction-ready, or
externally validated.

**Test:** `docs/vtrace/VERIFICATION.md` VER-002, VER-003, and VER-010; role
review by Scope Keeper, Citation Auditor, and Numeracy Checker before validated
claim promotion.

**Status:** VERIFIED

## ROUTE-I-03: Map rendering is not map proof

**Claim:** A rendered map artifact cannot by itself prove SLA validity, transit
readiness, upgrade readiness, asset-condition readiness, public readiness, or
official endorsement.

**Why it matters:** ROUTE can generate many maps before all source, optimizer,
review, and publication gates are satisfied.

**Test:** `docs/map-publication-scope.md`, `docs/TESTING.md` L2 map checks, and
`docs/vtrace/VERIFICATION.md` VER-006 and VER-007.

**Status:** VERIFIED

## ROUTE-I-04: Validation levels remain claim-scoped

**Claim:** L0, L1, and L2 validation prove only the surfaces named by the active
work package or evidence row.

**Why it matters:** ROUTE has a large mixed Rust, Python, generated-artifact,
browser, research, and review surface. A passing command must not overclaim
unrelated readiness.

**Test:** `docs/vtrace/VERIFICATION.md`, `docs/vtrace/EVIDENCE.md`, and
`docs/TESTING.md`.

**Status:** VERIFIED
