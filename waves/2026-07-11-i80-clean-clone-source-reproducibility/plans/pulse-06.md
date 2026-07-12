---
wave: i80-clean-clone-source-reproducibility
pulse: 06
date: 2026-07-11
status: done
depends_on:
  - pulse-05
governing_roles:
  - scope-keeper
  - citation-auditor
  - numeracy-checker
---

# Pulse 06 - Source Reproducibility Closeout

## Mission

Close the wave with a complete source posture, reproducible command contract,
credential hold, and next trigger.

## Deliverables

- [x] Record ready, excluded, and blocked source counts.
- [x] Confirm canonical I-80 safety on blocked reproduction.
- [x] Record wave commits and remaining claims.
- [x] Update GOAL, TRACKER, PHASES, and WAVE.
- [x] Define the next trigger without opening another wave.

## Gates

- `cargo test -q --locked`
- `npm run test:i80:sources`
- `npm run gate:i80:sources:no-credential`
- `npm run test:i80:reproduction`
- `npm run check:i80:packet`
- `npm run check:i80:reproduction` fails only on the two ACS blockers.
- `git diff --check`

## Non-Goals

- Request the Census key.
- Mark credentialed sources ready without artifacts.
- Reopen excluded claims.
- Replace the canonical report.
- Resolve unrelated `data/t1-design-review.csv` worktree state.

## Result

The wave closes with:

- 4 ready sources;
- 6 excluded sources;
- 2 blocked ACS sources;
- one safe reproduction command;
- one complete machine-readable blocker path.
