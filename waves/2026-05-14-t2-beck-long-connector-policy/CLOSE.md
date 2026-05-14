---
wave: t2-beck-long-connector-policy
date_closed: 2026-05-14
status: done
---

# Close - T2 Beck Long Connector Policy

## Decision

Long-connector policy has been authored for the three reviewed T2 Beck routes,
with no blocker relief.

## Evidence

- `data/t2-beck-long-connector-policy.csv` has three rows.
- Each row preserves one `map;promotion;publication` blocker.
- Each row has `claim_blocker_delta = 0`.
- The next artifact is `data/t2-beck-long-connector-policy-acceptance.csv`.

## Gate Record

- `cargo fmt -p route`
- `cargo test -p route`
- `route t2-beck-long-connector-policy --gate`
- `route tier-optimize --all-tiers --gate`
- `route optimizer-manifest --gate`
- `route release-manifest --gate`
- `proof check docs\SPEC_INDEX.md waves\PHASES.md waves\2026-05-14-t2-beck-long-connector-policy`
- `scripts\check-mileposts.ps1 -SkipTests`

## Next Action

Accept or reject the long-connector policy before any T2 Beck blocker relief or
final Beck replacement publication.
