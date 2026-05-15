---
wave: priority-a-pavement-funding-evidence-accepted-metadata-capture
date_open: 2026-05-15
status: done
---

# Priority A Pavement Funding Evidence Accepted Metadata Capture

## Mission

Record accepted-artifact metadata-capture placeholders for priority-A pavement
funding evidence after the accepted intake contract defined required fields.

## Opening Rule

Accepted intake rows are not evidence. A pavement repair blocker may not receive
relief until a real accepted funding artifact has captured metadata, attachment,
review, and acceptance.

## Inputs Inherited

- `data/tier-pavement-funding-evidence-accepted-intake.csv`
- `data/tier-pavement-funding-evidence-accepted-source-access.csv`
- `docs/optimizer-constraint-ledger-spec.md`

## Pulse Status

| Pulse | Status | Output |
|---|---|---|
| Accepted metadata-capture placeholders | done | `data/tier-pavement-funding-evidence-accepted-metadata-capture.csv` |

## Done Criteria

- Four priority-A rows receive explicit source-needed metadata-capture
  placeholders.
- No captured artifact, source title, URL, or commitment amount is present.
- All rows remain `not-reviewed`, `not-accepted`, and
  `not-eligible-for-relief`.
- `claim_blocker_delta` remains `0` for every row.
- Release manifest, spec index, optimizer ledger doctrine, and wave rail name the
  new artifact.

## Non-goals

- Do not fetch or cache funding artifacts.
- Do not attach, review, or accept evidence.
- Do not replay asset-condition blocker relief.
