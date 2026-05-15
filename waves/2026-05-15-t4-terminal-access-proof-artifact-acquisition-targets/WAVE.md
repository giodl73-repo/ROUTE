---
wave: t4-terminal-access-proof-artifact-acquisition-targets
date_open: 2026-05-15
status: done
---

# T4 Terminal Access Proof Artifact Acquisition Targets

## Mission

Convert held terminal-access proof attachment-review rows into an explicit
non-seed artifact acquisition/cache target list.

## Opening Rule

An acquisition target is not evidence. It may name the owner, required fields,
and prohibited seed source, but it must not accept proof or reduce map,
publication, or upgrade blockers.

## Inputs Inherited

- `data/t4-terminal-access-proof-attachment-review.csv`
- `data/optimizer-residual-blocker-backlog.csv`

## Pulse Status

| Pulse | Status | Output |
|---|---|---|
| T4 terminal-access proof artifact acquisition targets | done | `data/t4-terminal-access-proof-artifact-acquisition-targets.csv` |

## Done Criteria

- All 69 held terminal-access attachment-review rows receive acquisition/cache
  targets.
- Every row names `data/intermodal_terminals.csv` as a prohibited seed source.
- Every row remains `source-needed`, `not-cached`, and `not-accepted`.
- `map;publication;upgrade` blockers are preserved for every row.
- `claim_blocker_delta` remains `0` for every row.
- Release manifest, spec index, optimizer manifest, ledger doctrine, and wave
  rail name the new acquisition-target artifact.

## Non-goals

- Do not fetch, attach, or accept terminal-access proof artifacts.
- Do not replay terminal-access blocker relief.
- Do not claim map publication validity from acquisition targets.
