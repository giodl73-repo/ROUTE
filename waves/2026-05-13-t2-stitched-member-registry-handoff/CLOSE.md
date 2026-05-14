---
wave: t2-stitched-member-registry-handoff
date_closed: 2026-05-13
status: closed
type: closeout
---

# T2 Stitched Member Registry Handoff Closeout

## Decision

Close the wave with two stitched-member handoff rows, both held for member
expansion. I295 and I664 each still have one current registry member in the
blocked bundle, below the two-member minimum for a stitched service, so no T2
game, incident, publication, or upgrade claim is promoted.

## Artifacts

| Artifact | Result |
|---|---|
| `data/t2-stitched-member-registry-handoff.csv` | 2 rows, both `held-for-member-expansion` |
| `data/national-segment-registry.csv` | Regenerated canonical registry, 4054 rows |
| `data/national-segment-bundles.csv` | Regenerated canonical bundles, 196 rows |
| `data/tier-optimizer-runs.csv` | Registers the handoff as `held-known` with 2 blockers |
| `data/release-manifest.csv` | Registers the handoff as held public release metadata |
| `waves/2026-05-13-t2-stitched-member-registry-handoff/panels/stitched-registry/review.md` | Role review accepting the hold |

## Residual Blockers

| Route | Current registry members | Candidate route members | Decision |
|---|---:|---:|---|
| I295 | 1 | 84 | held for member expansion |
| I664 | 1 | 8 | held for member expansion |

Candidate evidence exists, but the blocked bundle still has only one registry
member. Both rows preserve `game;incident;publication;upgrade` with
`blocker_delta = 0`.

## Gate Record

- `cargo test -p route`
- `route t2-stitched-member-registry-handoff --gate`
- `route t2-national-bundle-readiness-audit --gate`
- `route national-segment-registry --gate`
- `route national-segment-bundles --gate`
- `route tier-optimize --all-tiers --gate`
- `route optimizer-manifest --gate`
- `route release-manifest --gate`
- `scripts/check-mileposts.ps1 -SkipTests`

## Next Wave Candidate

The cleanest next slice is a route-family or state-scope split review for the
stitched-member candidates, because route-level candidate evidence is broader
than the currently blocked bundle ids.
