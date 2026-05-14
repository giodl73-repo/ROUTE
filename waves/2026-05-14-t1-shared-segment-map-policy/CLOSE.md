---
wave: t1-shared-segment-map-policy
date_closed: 2026-05-14
status: done
---

# Close - T1 Shared Segment Map Policy

## Decision

The shared T1 schematic geometry blockers now have pair-level map-policy rows.
I40/I95 and I80/I90 are both held for policy acceptance, preserving all eight
`map;publication` blockers.

## Evidence

- `data/t1-shared-segment-map-policy.csv` has two rows.
- Each row preserves `blocker_count_after = 4` and `claim_blocker_delta = 0`.
- Each row records the conservative treatment: represent the shared segment as
  interlined trunk service or split it at selected transfer stops.
- The next artifact is `data/t1-shared-segment-policy-acceptance.csv`.

## Gate Record

- `cargo fmt -p route`
- `cargo test -p route`
- `route t1-shared-segment-map-policy --gate`
- `route t1-schematic-geometry-claim-review --gate`
- `route tier-optimize --all-tiers --gate`
- `route optimizer-manifest --gate`
- `route release-manifest --gate`
- `proof check docs\SPEC_INDEX.md waves\PHASES.md waves\2026-05-14-t1-shared-segment-map-policy`
- `scripts\check-mileposts.ps1 -SkipTests`

## Next Action

Review and accept or reject the pair-level shared-segment policy before any T1
schematic geometry blocker relief.
