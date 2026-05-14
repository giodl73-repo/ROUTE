---
wave: t3-lower-tier-feeder-gap-blocker-relief
date_closed: 2026-05-14
status: done
---

# Close - T3 Lower-Tier Feeder Gap Blocker Relief

## Decision

Accepted T3 lower-tier feeder-gap policy rows now have blocker relief rows,
pending optimizer constraint-ledger replay.

## Evidence

- `data/t3-lower-tier-feeder-gap-blocker-relief.csv` has six rows.
- The relief routes are `I-135`, `I-180`, `US22`, `US281`, `US74`, and
  `US90Z`.
- Artifact-local blockers reduce from six to zero.
- Every row has `ledger_replay_status =
  pending-optimizer-constraint-ledger-replay`.

## Gate Record

- `cargo fmt -p route`
- `cargo test -p route`
- `route t3-lower-tier-feeder-gap-blocker-relief --gate`
- `route tier-optimize --all-tiers --gate`
- `route optimizer-manifest --gate`
- `route release-manifest --gate`
- `proof check docs\SPEC_INDEX.md waves\PHASES.md waves\2026-05-14-t3-lower-tier-feeder-gap-blocker-relief`
- `scripts\check-mileposts.ps1 -SkipTests`

## Next Action

Wire accepted T3 feeder relief into the optimizer constraint ledger before
claim-blocker counts can drop.
