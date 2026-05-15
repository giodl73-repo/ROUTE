---
wave: priority-a-pavement-fetch-attempt
date_closed: 2026-05-14
status: done
---

# Close - Priority A Pavement Fetch Attempt

## Decision

Priority-A pavement fetches were attempted under scoped source-access policy.
TX and LA produced populated per-state HPMS caches; NM failed/produced an empty
per-state cache. No pavement evidence was accepted and no asset-condition debt
was reduced.

## Evidence

- `route fetch-hpms --states TX,LA,NM` populated ignored cache files for TX and
  LA and reported an NM parse/fetch failure.
- `data/tier-pavement-source-fetch-attempt.csv` records:
  - TX: 43,381 cache records, `cache-populated-unreviewed`.
  - LA: 10,892 cache records, `cache-populated-unreviewed`.
  - NM: 0 cache records, `fetch-failed-or-empty-cache`.
- The rebuilt pavement chain still reports 13 pavement debt budget rows and
  $95.95M in planning debt.
- All fetch-attempt rows keep `publication;sla;transit;upgrade`,
  `evidence_acceptance_status = not-accepted`, and `claim_blocker_delta = 0`.

## Gate Record

- `cargo fmt --all`
- `cargo test -p route tier_pavement_source_fetch_attempt_records_empty_cache_without_relief`
- `cargo test -p route`
- `route source-fetch-policy --gate`
- `route tier-pavement-docket --gate`
- `route tier-pavement-source-gaps --gate`
- `route tier-pavement-debt-budget --gate`
- `route tier-pavement-acquisition-plan --gate`
- `route tier-pavement-acquisition-docket --gate`
- `route tier-pavement-source-fetch-attempt --gate`
- `route optimizer-constraint-ledger --gate`
- `route optimizer-constraint-budget --gate`
- `route optimizer-residual-blocker-backlog --gate`
- `route tier-optimize --all-tiers --gate`
- `route optimizer-manifest --gate`
- `route release-manifest --gate`
- `proof check docs\SPEC_INDEX.md waves\PHASES.md waves\2026-05-14-priority-a-pavement-fetch-attempt`
- `scripts\check-mileposts.ps1 -SkipTests`

## Next Action

Review TX and LA pavement cache joins before any debt relief. NM needs a
follow-up source-access/fetch repair because its per-state cache is empty.

