---
wave: i80-clean-clone-source-reproducibility
pulse: 05
date: 2026-07-11
status: done
depends_on:
  - pulse-04
governing_roles:
  - scope-keeper
  - citation-auditor
  - numeracy-checker
---

# Pulse 05 - Clean-Clone Report Reproduction

## Mission

Provide one non-destructive command that prepares sources, enforces the complete
contract, generates a separate I-80 report only when ready, and writes a
field-level comparison.

## Deliverables

- [x] Add `route report --output`.
- [x] Reuse canonical I-80 annotations for noncanonical output.
- [x] Add `reproduce:i80:report`.
- [x] Write complete blocker status when source gates fail.
- [x] Preserve the canonical reviewed report on every blocked run.
- [x] Add key-fact, dimension, and summary comparison logic.
- [x] Add fixture tests and CI coverage.
- [x] Run the complete command once.

## Gates

- `npm run test:i80:reproduction`
- `npm run check:i80:reproduction`
- `npm run reproduce:i80:report`
- Blocked reproduction leaves `corpus/existing/i80.md` byte-identical.
- Status identifies every blocking source.
- Generated comparison is written only after the full contract passes.
- `git diff --check`

## Non-Goals

- Request or store the user's Census key.
- Replace the canonical report automatically.
- Treat excluded sources as zero.
- Approve score changes without review.
- Resolve unrelated `data/t1-design-review.csv` worktree state.

## Result

The command reacquired all available no-credential inputs, then stopped on
exactly two blockers:

- `SRC-I80-ACS-POP`
- `SRC-I80-ACS-INCOME`

The canonical reviewed report remained byte-identical. Once
`CENSUS_API_KEY` is present, the same command can generate
`data/cache/i80-regenerated.md` and
`data/cache/i80-report-comparison.csv` without replacing the reviewed anchor.
