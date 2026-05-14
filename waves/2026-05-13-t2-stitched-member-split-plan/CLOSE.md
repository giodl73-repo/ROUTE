---
wave: t2-stitched-member-split-plan
date_closed: 2026-05-13
status: closed
type: closeout
---

# T2 Stitched Member Split Plan Closeout

## Decision

Close the wave with 11 stitched-member split-plan rows, all held for review.
The rows enumerate state-scoped candidate bundle choices for I295 and I664, but
do not select, merge, append, or remove registry or bundle members.

## Artifacts

| Artifact | Result |
|---|---|
| `data/t2-stitched-member-split-plan.csv` | 11 rows: 9 for I295 and 2 for I664, all `validation_status = review` |
| `data/tier-optimizer-runs.csv` | Registers the split plan as `held-known` with 2 blockers |
| `data/release-manifest.csv` | Registers the split plan as held public release metadata |
| `waves/2026-05-13-t2-stitched-member-split-plan/panels/split-plan/review.md` | Role review accepting state-scoped split rows as review targets |

## Split Findings

| Route | Split rows | State scopes | Decision |
|---|---:|---|---|
| I295 | 9 | FL; GA; MA; ME; NJ; NY; PA; SC; VA | review candidate bundles before membership mutation |
| I664 | 2 | NC; VA | review candidate bundles before membership mutation |

All rows preserve `game;incident;publication;upgrade` with `blocker_delta = 0`.

## Gate Record

- `cargo fmt -p route`
- `cargo test -p route`
- `route t2-stitched-member-split-plan --gate`
- `route t2-stitched-member-decision-docket --gate`
- `route tier-optimize --all-tiers --gate`
- `route optimizer-manifest --gate`
- `route release-manifest --gate`
- `scripts/check-mileposts.ps1 -SkipTests`

## Next Wave Candidate

The next clean slice is a stitched-member candidate selection docket that marks
which state-scoped candidate bundles remain in scope, are rejected, or require
manual evidence before registry membership can change.
