---
wave: fresh-pond-terminal-source-review
date_open: 2026-05-15
status: done
---

# Fresh Pond Terminal Source Review

## Mission

Review the remaining New York Fresh Pond terminal-access blockers against the
current FHWA intermodal connector source before any accepted or rejected proof is
added to the terminal-contact overlays.

## Opening Rule

Do not clear Fresh Pond rows from search snippets, terminal seed proximity, or a
generic NYC truck-route rule. A rejected proof row needs an official source that
lists Fresh Pond terminal access routes and excludes the held route; an accepted
proof row needs a route-specific contact statement for the held route.

## Inputs Inherited

- `data/optimizer-residual-blocker-backlog.csv`
- `data/t4-terminal-contact-proof-docket.csv`
- `data/t4-terminal-contact-proof-source-registry.csv`
- `data/t4-terminal-contact-accepted-proof-sources.csv`
- `data/t4-terminal-contact-rejected-proof-sources.csv`
- [FHWA NHS Intermodal Connectors](https://www.fhwa.dot.gov/planning/national_highway_system/intermodal_connectors/)
- [FHWA all connectors workbook](https://www.fhwa.dot.gov/planning/national_highway_system/intermodal_connectors/all_connectors.xlsx)

## Pulse Status

| Pulse | Status | Output |
|---|---|---|
| Fresh Pond source review | done | Current FHWA connector workbook does not name Fresh Pond; six rows remain held |

## Done Criteria

- Current FHWA connector source is checked before any Fresh Pond proof overlay
  mutation.
- No Fresh Pond accepted or rejected proof row is added unless the source meets
  the non-seed route-to-terminal proof rule.
- Remaining Fresh Pond blockers are preserved if the current source cannot name
  terminal access routes for the district.

## Non-goals

- Do not infer terminal access from New York City truck-route eligibility alone.
- Do not use the seed terminal row in `data/intermodal_terminals.csv` as proof.
- Do not clear T2 repair debt or T1 snapshot evidence guards.
