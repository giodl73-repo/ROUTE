---
wave: t4-terminal-access-source-access
date_closed: 2026-05-14
status: done
---

# Close - T4 Terminal Access Source Access

## Decision

All 69 held T4 terminal-access proof review rows now have source-access policy
rows. Each row requires manual or cached non-seed proof; live fetch remains
unsupported until a policy-compliant terminal-access fetcher exists.

## Evidence

- `data/t4-terminal-access-source-access.csv` has 69 rows.
- Every row preserves `map;publication;upgrade`.
- Every row has `claim_blocker_delta = 0`, `evidence_artifact =
  source-needed`, and `proof_acceptance_status = not-accepted`.
- Every row has `access_mode = manual-or-cached-source-needed` and
  `live_fetch_status = unsupported-no-safe-terminal-access-fetcher`.

## Gate Record

- `cargo fmt -p route`
- `cargo test -p route`
- `route t4-terminal-access-source-access --gate`
- `route t4-terminal-access-proof-review --gate`
- `route tier-optimize --all-tiers --gate`
- `route optimizer-manifest --gate`
- `route release-manifest --gate`
- `proof check docs\SPEC_INDEX.md waves\PHASES.md waves\2026-05-14-t4-terminal-access-source-access`
- `scripts\check-mileposts.ps1 -SkipTests`

## Next Action

Attach manual or cached non-seed terminal-access proof artifacts, or add a
source-fetch-cache-policy-compliant terminal-access fetcher before any proof
acceptance or blocker relief.
