---
wave: i80-clean-clone-source-reproducibility
pulse: 02
date: 2026-07-11
status: done
depends_on:
  - pulse-01
governing_roles:
  - citation-auditor
  - scope-keeper
  - numeracy-checker
---

# Pulse 02 - No-Credential Acquisition Orchestration

## Mission

Execute the existing no-credential source paths as one bundle and emit a
readiness ledger that continues through failures.

## Deliverables

- [x] Add `prepare:i80:sources`.
- [x] Add deterministic Gazetteer extraction.
- [x] Add source-contract parsing and readiness output.
- [x] Add I-80 HPMS state-coverage and FEMA tile-coverage gates.
- [x] Preserve blockers for credentialed, adapter-missing, and unwired sources.
- [x] Add focused unit tests and CI checks.
- [x] Run the live no-credential bundle.

## Gates

- `npm run test:i80:sources`
- `npm run check:i80:sources`
- `npm run gate:i80:sources:no-credential` reports
  unresolved sources instead of hiding them.
- Readiness output is written under gitignored `data/cache/`.
- `git diff --check`

## Non-Goals

- Make incomplete HPMS or FEMA coverage pass.
- Add credentials.
- Commit raw source data.
- Regenerate the reviewed I-80 report.
- Resolve unrelated `data/t1-design-review.csv` worktree state.

## Result

The live bundle produced 2 ready and 10 blocked source rows.

- TIGER: ready.
- County Gazetteer: ready after deterministic extraction.
- HPMS: fetched 145,949 records; I-80 had 40,915 rows across 10 of 11 corridor
  states, with Indiana missing.
- FEMA: command succeeded for its legacy tile set, but produced zero
  I-80-labeled coverage tiles and remains blocked.

The next pulse repairs the Indiana HPMS and I-80 FEMA coverage blockers before
credential and adapter work begins.
