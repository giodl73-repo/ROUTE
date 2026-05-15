---
wave: priority-a-pavement-funding-evidence-accepted-metadata-artifact-attachment
date_open: 2026-05-15
status: done
---

# Priority A Pavement Funding Evidence Accepted Metadata Artifact Attachment

## Mission

Record artifact-attachment placeholders after accepted priority-A pavement
funding evidence metadata capture.

## Opening Rule

Accepted metadata placeholders are not attached evidence. Pavement repair
blockers remain until a real accepted funding artifact is attached, reviewed,
accepted, and replayed.

## Inputs Inherited

- `data/tier-pavement-funding-evidence-accepted-metadata-capture.csv`
- `data/tier-pavement-funding-evidence-accepted-intake.csv`
- `docs/optimizer-constraint-ledger-spec.md`

## Pulse Status

| Pulse | Status | Output |
|---|---|---|
| Accepted metadata artifact attachment | done | `data/tier-pavement-funding-evidence-accepted-metadata-artifact-attachment.csv` |

## Done Criteria

- Four priority-A rows receive explicit source-needed attachment placeholders.
- No accepted artifact is attached.
- All rows remain `not-reviewed`, `not-accepted`, and
  `not-eligible-for-relief`.
- `claim_blocker_delta` remains `0` for every row.
- Release manifest, spec index, optimizer ledger doctrine, and wave rail name the
  new artifact.

## Non-goals

- Do not fetch or cache funding artifacts.
- Do not review or accept evidence.
- Do not replay asset-condition blocker relief.
