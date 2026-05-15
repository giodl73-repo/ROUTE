---
wave: t2-game-ops-bundle-evidence-ledger-replay
date_closed: 2026-05-14
status: done
---

# Close - T2 Game/Ops Bundle Evidence Ledger Replay

## Decision

Accepted T2 game/ops bundle evidence relief is now replayed into the optimizer
constraint ledger and downstream selector artifacts.

## Evidence

- `data/optimizer-constraint-ledger.csv` now has 136 rows, including sixteen
  `game_ops_bundle_binding_relief` pass rows.
- `game_ops_bundle_binding` claim blockers are suppressed for relieved bundle
  ids; total residual claim blockers are now 70.
- `data/optimizer-constraint-budget.csv` now has 135 rows after grouping budget
  rows by the same subject key used by budget ids, avoiding duplicate bundle
  ids when a relieved game/ops bundle also carries asset-condition debt.
- `data/optimizer-residual-blocker-backlog.csv` now has four rows. The largest
  remaining claim blocker family is T4 `terminal_access_evidence_gap` with 69
  claim blockers.
- `data/tier-optimizer-runs.csv` now has 124 rows with 69 held-known rows and
  55 pass rows.

## Gate Record

- `cargo fmt --all`
- `cargo test -p route optimizer_constraint_ledger_replays_t2_game_ops_bundle_relief`
- `route optimizer-constraint-ledger --gate`
- `route optimizer-constraint-budget --gate`
- `route optimizer-residual-blocker-backlog --gate`
- `route tier-optimize --all-tiers --gate`
- `route optimizer-manifest --gate`
- `route release-manifest --gate`
- `cargo test --workspace`
- `proof check docs\SPEC_INDEX.md waves\PHASES.md waves\2026-05-14-t2-game-ops-bundle-evidence-ledger-replay`
- `scripts\check-mileposts.ps1 -SkipTests`

## Next Action

Return to residual optimizer burn-down. The next blocker-reduction target is T4
terminal-access evidence, unless map publication work first needs a full
pass-ready selector set.

