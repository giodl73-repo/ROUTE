---
wave: t2-game-publication-evidence-blocker-relief
date_closed: 2026-05-14
status: done
---

# Close - T2 Game Publication Evidence Blocker Relief

## Decision

Accepted T2 game publication evidence policy is now replayed into explicit
blocker-relief rows. Three scenario rows reduce from one blocker each to zero in
`data/t2-game-publication-evidence-blocker-relief.csv`, for a total
`claim_blocker_delta = -3`.

## Evidence

- `data/t2-game-publication-evidence-blocker-relief.csv` has three rows.
- Each row has `blocker_count_before = 1`, `blocker_count_after = 0`, and
  `claim_blocker_delta = -1`.
- Each row remains `pending-optimizer-constraint-ledger-replay`.
- The next artifact is `data/optimizer-constraint-ledger.csv`.

## Gate Record

- `cargo fmt -p route`
- `cargo test -p route`
- `route t2-game-publication-evidence-blocker-relief --gate`
- `route tier-optimize --all-tiers --gate`
- `route optimizer-manifest --gate`
- `route release-manifest --gate`
- `proof check docs\SPEC_INDEX.md waves\PHASES.md waves\2026-05-14-t2-game-publication-evidence-blocker-relief`
- `scripts\check-mileposts.ps1 -SkipTests`

## Next Action

Wire the relief artifact into the optimizer constraint ledger so budget and
backlog counts drop before scenario publication or map refresh.

