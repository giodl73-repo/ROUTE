---
wave: t4-terminal-access-proof-acquisition
date_closed: 2026-05-14
status: done
---

# Close - T4 Terminal Access Proof Acquisition

## Decision

All 69 held T4 terminal-access evidence review rows now have explicit proof
acquisition tasks. The acquisition docket keeps each row `source-needed`, records
the terminal district seed as prohibited proof, and routes the family to
`data/t4-terminal-access-proof-artifacts.csv`.

## Evidence

- `data/t4-terminal-access-proof-acquisition.csv` has 69 rows.
- Every row preserves `map;publication;upgrade`.
- Every row has `claim_blocker_delta = 0`, `proof_artifact_status =
  not-attached`, and `validation_status = review`.
- Required proof is non-seed route-to-terminal contact evidence with route,
  terminal, connector, and date.

## Gate Record

- `cargo fmt -p route`
- `cargo test -p route`
- `route t4-terminal-access-proof-acquisition --gate`
- `route t4-terminal-access-evidence-review --gate`
- `route tier-optimize --all-tiers --gate`
- `route optimizer-manifest --gate`
- `route release-manifest --gate`
- `proof check docs\SPEC_INDEX.md waves\PHASES.md waves\2026-05-14-t4-terminal-access-proof-acquisition`
- `scripts\check-mileposts.ps1 -SkipTests`

## Next Action

Open a proof artifact attachment or acquisition execution wave for
`data/t4-terminal-access-proof-artifacts.csv`. No terminal-access blocker should
be reduced until a non-seed proof artifact is attached and accepted.
