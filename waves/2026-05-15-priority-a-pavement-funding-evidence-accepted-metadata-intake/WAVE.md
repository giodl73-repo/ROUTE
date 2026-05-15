---
wave: priority-a-pavement-funding-evidence-accepted-metadata-intake
date_open: 2026-05-15
status: done
---

# Priority A Pavement Funding Evidence Accepted Metadata Intake

## Mission

Define manual/cache intake requirements for accepted priority-A pavement funding
evidence metadata sources.

## Opening Rule

Intake requirements are not evidence. Pavement repair blockers remain until a
real accepted funding artifact is manually collected or cached, source metadata
is captured, the artifact is attached, reviewed, accepted, and replayed.

## Inputs Inherited

- `data/tier-pavement-funding-evidence-accepted-metadata-source-access.csv`
- `data/tier-pavement-funding-evidence-accepted-metadata-artifact-acquisition.csv`
- `docs/optimizer-constraint-ledger-spec.md`

## Pulse Status

| Pulse | Status | Output |
|---|---|---|
| Accepted metadata intake | done | `data/tier-pavement-funding-evidence-accepted-metadata-intake.csv` |

## Done Criteria

- Four priority-A rows receive accepted-artifact metadata intake requirements.
- Intake remains `artifact-required`, cache status remains `not-cached`, and no
  evidence artifact is attached.
- All rows remain `source-needed`, `not-reviewed`, `not-accepted`, and
  `not-eligible-for-relief`.
- `claim_blocker_delta` remains `0` for every row.
- Release manifest, spec index, optimizer ledger doctrine, and wave rail name the
  new artifact.

## Non-goals

- Do not fetch or cache funding artifacts.
- Do not capture source metadata or attach artifacts.
- Do not review, accept, or replay asset-condition blocker relief.
