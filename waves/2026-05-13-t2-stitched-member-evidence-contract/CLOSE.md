---
wave: t2-stitched-member-evidence-contract
date_closed: 2026-05-13
status: closed
type: closeout
---

# T2 Stitched Member Evidence Contract Closeout

## Decision

Close the wave with 11 stitched-member evidence contract rows, all
`source-needed`. The contract defines continuity, state-scope, and source-proof
requirements for every I295 and I664 selection docket row without satisfying
evidence or changing candidate membership.

## Artifacts

| Artifact | Result |
|---|---|
| `data/t2-stitched-member-evidence-contract.csv` | 11 rows, all `evidence_status = source-needed` and `validation_status = review` |
| `data/tier-optimizer-runs.csv` | Registers the evidence contract as `held-known` with 2 blockers |
| `data/release-manifest.csv` | Registers the evidence contract as held public release metadata |
| `waves/2026-05-13-t2-stitched-member-evidence-contract/panels/evidence-contract/review.md` | Role review accepting source-needed proof rows as the conservative outcome |

## Contract Findings

| Route | Contract rows | Evidence status |
|---|---:|---|
| I295 | 9 | source-needed |
| I664 | 2 | source-needed |

All rows preserve `game;incident;publication;upgrade` with `blocker_delta = 0`.

## Gate Record

- `cargo fmt -p route`
- `cargo test -p route`
- `route t2-stitched-member-evidence-contract --gate`
- `route t2-stitched-member-selection-docket --gate`
- `route tier-optimize --all-tiers --gate`
- `route optimizer-manifest --gate`
- `route release-manifest --gate`
- `scripts/check-mileposts.ps1 -SkipTests`

## Next Wave Candidate

The next clean slice is a stitched-member evidence acquisition docket that
turns the 11 source-needed proof contracts into concrete source targets without
changing candidate membership.
