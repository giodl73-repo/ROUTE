---
wave: priority-a-hpms-scope-broadening-policy
date_open: 2026-05-14
status: done
source: data/tier-pavement-unmatched-join-review.csv
---

# Priority A HPMS Scope Broadening Policy

## Mission

Author governed HPMS functional-system broadening rows for the TX/LA/NM
priority-A source-needed US-route pavement members before any broader cache
mutation or evidence acceptance.

## Opening Rule

This wave may extend the HPMS fetch command contract to support explicit
functional-system scope and emit a source-broadening plan. It must not run the
broadened fetch, commit raw cache data, accept pavement evidence, or reduce
asset-condition blockers.

## Inputs Inherited

| Input | Source |
|---|---|
| Unmatched join review | `data/tier-pavement-unmatched-join-review.csv` |
| Source fetch policy | `data/source-fetch-policy.csv`; `docs/source-fetch-cache-policy.md` |
| HPMS fetcher | `crates/route-data/src/hpms_fetch.rs` |

## Pulse Status

| Pulse | Status | Evidence |
|---|---|---|
| 01 - Functional-system fetch contract | done | `route fetch-hpms --states ... --functional-systems ...`; tests |
| 02 - Scope-broadening plan | done | `data/tier-pavement-hpms-scope-broadening.csv`; CLI gate |
| 03 - Review and close | done | `CLOSE.md`; `panels/scope-broadening/review.md`; final gates |

## Done Criteria

- Default `route fetch-hpms` behavior remains system `1` unless explicitly
  broadened.
- Non-default functional-system scope requires `--states` scoped mutation.
- TX, LA, and NM have governed broadened fetch commands for systems `1,2,3`.
- Rows preserve `publication;sla;transit;upgrade` blockers with
  `claim_blocker_delta = 0`.
- Optimizer and release manifests register the broadening artifact.
- Final gates pass before close.

## Non-Goals

- Do not run the broadened HPMS fetch in this wave.
- Do not commit `data/cache/` HPMS payloads.
- Do not accept HPMS cache rows as pavement evidence.
- Do not reduce pavement evidence or repair debt.
