---
wave: t4-terminal-access-proof-intake
date_closed: 2026-05-14
status: done
---

# Close - T4 Terminal Access Proof Intake

## Decision

All 69 T4 terminal-access source-access rows now have proof-intake contracts.
Each row remains source-needed and specifies the required manual or cached
non-seed artifact fields before any proof capture, attachment, review, or
blocker relief.

## Evidence

- `data/t4-terminal-access-proof-intake.csv` has 69 rows.
- Every row preserves `map;publication;upgrade`.
- Every row has `claim_blocker_delta = 0`, `proof_artifact = source-needed`,
  and `proof_status = source-needed`.
- Required fields include source title, URL or cached artifact, capture date,
  route, terminal, connector, and a non-seed route-to-terminal contact
  statement.

## Gate Record

- `cargo fmt -p route`
- `cargo test -p route`
- `route t4-terminal-access-proof-intake --gate`
- `route t4-terminal-access-source-access --gate`
- `route tier-optimize --all-tiers --gate`
- `route optimizer-manifest --gate`
- `route release-manifest --gate`
- `proof check docs\SPEC_INDEX.md waves\PHASES.md waves\2026-05-14-t4-terminal-access-proof-intake`
- `scripts\check-mileposts.ps1 -SkipTests`

## Next Action

Capture or attach manual/cached non-seed terminal-access proof artifacts. No
terminal-access blocker should be reduced until proof is attached, reviewed, and
accepted.
