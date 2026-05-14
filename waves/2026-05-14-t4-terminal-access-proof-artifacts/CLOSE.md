---
wave: t4-terminal-access-proof-artifacts
date_closed: 2026-05-14
status: done
---

# Close - T4 Terminal Access Proof Artifacts

## Decision

All 69 source-needed T4 terminal-access acquisition tasks now have explicit
proof artifact placeholders. The artifact docket keeps each row source-needed,
not-reviewed, and not-accepted, and routes the family to proof review only after
non-seed source artifacts are attached.

## Evidence

- `data/t4-terminal-access-proof-artifacts.csv` has 69 rows.
- Every row preserves `map;publication;upgrade`.
- Every row has `claim_blocker_delta = 0`, `attachment_status =
  source-needed`, `evidence_review_status = not-reviewed`, and
  `proof_acceptance_status = not-accepted`.
- Required proof remains non-seed route-to-terminal contact evidence with route,
  terminal, connector, and date.

## Gate Record

- `cargo fmt -p route`
- `cargo test -p route`
- `route t4-terminal-access-proof-artifacts --gate`
- `route t4-terminal-access-proof-acquisition --gate`
- `route tier-optimize --all-tiers --gate`
- `route optimizer-manifest --gate`
- `route release-manifest --gate`
- `proof check docs\SPEC_INDEX.md waves\PHASES.md waves\2026-05-14-t4-terminal-access-proof-artifacts`
- `scripts\check-mileposts.ps1 -SkipTests`

## Next Action

Open a proof review or source acquisition wave for
`data/t4-terminal-access-proof-review.csv`. No terminal-access blocker should be
reduced until a non-seed proof artifact is attached, reviewed, and accepted.
