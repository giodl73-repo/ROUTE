---
wave: priority-a-pavement-source-access
date_closed: 2026-05-14
status: done
---

# Close - Priority A Pavement Source Access

## Decision

Priority-A pavement acquisition is now governed by explicit source-access rows
before any HPMS/state fetch can mutate pavement inputs.

## Evidence

- `data/tier-pavement-source-access.csv` has three rows: TX, LA, and NM.
- Each row uses `hpms-scoped-fetch` and `scoped-cache-merge`.
- Each row names both the national HPMS cache and the per-state cache target.
- Each row preserves `publication;sla;transit;upgrade` with
  `claim_blocker_delta = 0`.
- No asset-condition debt was reduced in this wave.

## Gate Record

- `cargo fmt --all`
- `cargo test -p route tier_pavement_source_access_preserves_blockers_for_priority_fetches`
- `cargo test -p route`
- `route tier-pavement-acquisition-docket --gate`
- `route tier-pavement-source-access --gate`
- `route tier-optimize --all-tiers --gate`
- `route optimizer-manifest --gate`
- `route release-manifest --gate`
- `proof check docs\SPEC_INDEX.md waves\PHASES.md waves\2026-05-14-priority-a-pavement-source-access`
- `scripts\check-mileposts.ps1 -SkipTests`

## Next Action

Run the priority-A scoped pavement fetches only when prepared to rebuild and
review pavement evidence: TX, LA, and NM are the first states.

