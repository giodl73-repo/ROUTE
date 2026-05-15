---
wave: priority-a-pavement-funding-evidence-accepted-metadata-source-capture
date_open: 2026-05-15
status: done
---

# Priority A Pavement Funding Evidence Accepted Metadata Source Capture

## Mission

Record source-needed capture placeholders for accepted priority-A pavement
funding evidence metadata intake rows.

## Opening Rule

Source-capture placeholders are not evidence. Pavement repair blockers remain
until a real accepted funding artifact is manually collected or cached, attached,
reviewed, accepted, and replayed.

## Inputs Inherited

- `data/tier-pavement-funding-evidence-accepted-metadata-intake.csv`
- `data/tier-pavement-funding-evidence-accepted-metadata-source-access.csv`
- `docs/optimizer-constraint-ledger-spec.md`

## Pulse Status

| Pulse | Status | Output |
|---|---|---|
| Accepted metadata source capture | done | `data/tier-pavement-funding-evidence-accepted-metadata-source-capture.csv` |

## Done Criteria

- Four priority-A rows receive source-needed capture placeholders.
- No artifact title, URL, amount, or artifact path is captured.
- All rows remain `source-needed`, `none`, `not-reviewed`, `not-accepted`, and
  `not-eligible-for-relief`.
- `claim_blocker_delta` remains `0` for every row.
- Release manifest, spec index, optimizer ledger doctrine, and wave rail name the
  new artifact.

## Non-goals

- Do not fetch or cache funding artifacts.
- Do not attach, review, or accept evidence.
- Do not replay asset-condition blocker relief.
