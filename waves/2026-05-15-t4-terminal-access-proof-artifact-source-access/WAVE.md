---
wave: t4-terminal-access-proof-artifact-source-access
date_open: 2026-05-15
status: done
---

# T4 Terminal Access Proof Artifact Source Access

## Mission

Classify source/cache access for the 69 held T4 terminal-access proof artifact
acquisition targets.

## Opening Rule

Source-access classification is not evidence. It may declare manual or cached
source requirements and live-fetch limits, but it must not accept proof or reduce
map, publication, or upgrade blockers.

## Inputs Inherited

- `data/t4-terminal-access-proof-artifact-acquisition-targets.csv`
- `docs/source-fetch-cache-policy.md`
- `data/source-fetch-policy.csv`

## Pulse Status

| Pulse | Status | Output |
|---|---|---|
| T4 terminal-access proof artifact source access | done | `data/t4-terminal-access-proof-artifact-source-access.csv` |

## Done Criteria

- All 69 source-needed acquisition targets receive source-access rows.
- Every row remains `manual-or-cached-source-needed` and `not-cached`.
- Live fetch remains `unsupported-no-safe-terminal-access-fetcher`.
- Every row keeps evidence as `source-needed` and proof as `not-accepted`.
- `map;publication;upgrade` blockers are preserved for every row.
- `claim_blocker_delta` remains `0` for every row.
- Release manifest, spec index, optimizer manifest, ledger doctrine, and wave
  rail name the new source-access artifact.

## Non-goals

- Do not fetch, attach, or accept terminal-access proof artifacts.
- Do not replay terminal-access blocker relief.
- Do not claim map publication validity from source-access policy rows.
