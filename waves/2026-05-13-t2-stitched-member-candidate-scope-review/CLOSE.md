---
wave: t2-stitched-member-candidate-scope-review
date_closed: 2026-05-13
status: closed
type: closeout
---

# T2 Stitched Member Candidate Scope Review Closeout

## Decision

Close the wave with two candidate scope review rows, both held. I295 and I664
have route-level stitched candidate evidence, but that evidence spans more bundle
ids and state scopes than the blocked one-member bundle ids. No T2 game,
incident, publication, or upgrade claim is promoted.

## Artifacts

| Artifact | Result |
|---|---|
| `data/t2-stitched-member-candidate-scope-review.csv` | 2 rows, both `held-for-scope-review` |
| `data/tier-optimizer-runs.csv` | Registers the scope review as `held-known` with 2 blockers |
| `data/release-manifest.csv` | Registers the scope review as held public release metadata |
| `waves/2026-05-13-t2-stitched-member-candidate-scope-review/panels/candidate-scope/review.md` | Role review accepting the hold |

## Scope Findings

| Route | Blocked bundle candidates | Route candidates | Candidate bundle count | State scope |
|---|---:|---:|---:|---|
| I295 | 1 | 84 | 9 | FL; GA; MA; ME; NJ; NY; PA; SC; VA |
| I664 | 1 | 8 | 2 | NC; VA |

Both rows preserve `game;incident;publication;upgrade` with `blocker_delta = 0`.

## Gate Record

- `cargo test -p route`
- `route t2-stitched-member-candidate-scope-review --gate`
- `route t2-stitched-member-registry-handoff --gate`
- `route tier-optimize --all-tiers --gate`
- `route optimizer-manifest --gate`
- `route release-manifest --gate`
- `scripts/check-mileposts.ps1 -SkipTests`

## Next Wave Candidate

The next clean slice is a route-family/state-scope decision docket that decides
whether I295 and I664 candidate evidence should split, merge, or expand the
blocked bundle ids.
