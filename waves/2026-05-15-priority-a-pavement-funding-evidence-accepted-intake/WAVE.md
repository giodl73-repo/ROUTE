---
wave: priority-a-pavement-funding-evidence-accepted-intake
date_open: 2026-05-15
status: done
---

# Priority A Pavement Funding Evidence Accepted Intake

## Mission

Define the manual/cache intake contract for accepted priority-A pavement funding
evidence artifacts after source access classified them as source-needed.

## Opening Rule

No priority-A pavement repair row may accept funding evidence or become eligible
for asset-condition relief until a real accepted full-cost programming or DOT
commitment artifact is captured, attached, reviewed, and accepted.

## Inputs Inherited

- `data/tier-pavement-funding-evidence-accepted-source-access.csv`
- `data/tier-pavement-funding-evidence-accepted-artifact-acquisition.csv`
- `docs/optimizer-constraint-ledger-spec.md`

## Pulse Status

| Pulse | Status | Output |
|---|---|---|
| Accepted intake contract | done | `data/tier-pavement-funding-evidence-accepted-intake.csv` |

## Done Criteria

- Four priority-A rows receive explicit accepted-artifact intake requirements.
- All rows remain `source-needed`, `not-cached`, `not-reviewed`,
  `not-accepted`, and `not-eligible-for-relief`.
- `claim_blocker_delta` remains `0` for every row.
- Release manifest, spec index, optimizer ledger doctrine, and wave rail name the
  new artifact.

## Non-goals

- Do not fetch or cache funding artifacts.
- Do not attach, review, or accept evidence.
- Do not replay asset-condition blocker relief.
