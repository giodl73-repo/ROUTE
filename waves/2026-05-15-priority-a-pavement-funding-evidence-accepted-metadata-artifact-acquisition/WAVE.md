---
wave: priority-a-pavement-funding-evidence-accepted-metadata-artifact-acquisition
date_open: 2026-05-15
status: done
---

# Priority A Pavement Funding Evidence Accepted Metadata Artifact Acquisition

## Mission

Turn held accepted metadata attachment-review rows into source-needed
acquisition/cache targets for priority-A pavement funding evidence artifacts.

## Opening Rule

Acquisition targets are not evidence. Pavement repair blockers remain until a
real accepted funding artifact is acquired or cached, attached, reviewed,
accepted, and replayed.

## Inputs Inherited

- `data/tier-pavement-funding-evidence-accepted-metadata-attachment-review.csv`
- `data/tier-pavement-funding-evidence-accepted-metadata-artifact-attachment.csv`
- `docs/optimizer-constraint-ledger-spec.md`

## Pulse Status

| Pulse | Status | Output |
|---|---|---|
| Accepted metadata artifact acquisition | done | `data/tier-pavement-funding-evidence-accepted-metadata-artifact-acquisition.csv` |

## Done Criteria

- Four priority-A rows receive source-needed acquisition/cache targets.
- All rows remain `not-cached`, `not-accepted`, and `not-eligible-for-relief`.
- `claim_blocker_delta` remains `0` for every row.
- Release manifest, spec index, optimizer ledger doctrine, and wave rail name the
  new artifact.

## Non-goals

- Do not fetch or cache funding artifacts.
- Do not attach, review, or accept evidence.
- Do not replay asset-condition blocker relief.
