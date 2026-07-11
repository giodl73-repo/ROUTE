---
wave: i80-clean-clone-source-reproducibility
pulse: 04
date: 2026-07-11
status: done
depends_on:
  - pulse-03
governing_roles:
  - citation-auditor
  - scope-keeper
  - numeracy-checker
---

# Pulse 04 - Credential And Adapter Decisions

## Mission

Support credentials safely where a bounded adapter exists and explicitly
exclude sources whose current values cannot be reproduced.

## Deliverables

- [x] Add environment-only Census API key support.
- [x] Keep ACS fixed to the reviewed 2022 vintage used by scoring.
- [x] Add direct USDA RUCC 2023 download and normalization.
- [x] Remove unconditional FAF5 and BEA report citations.
- [x] Exclude AFDC, NBI, FARS, and FEMA from reviewed regeneration pending
      reviewed adapters.
- [x] Remove excluded caches from the reviewed-report overwrite guard.
- [x] Preserve source rows and next actions in the readiness ledger.

## Gates

- Census commands fail clearly when `CENSUS_API_KEY` is absent.
- Errors never print the API key.
- RUCC normalization emits more than 3,000 county rows.
- Excluded sources do not satisfy claims and do not fail `--gate-all`.
- `cargo test -q --locked -p route-data`
- `cargo test -q --locked -p route --bin route`
- `npm run test:i80:sources`
- `git diff --check`

## Non-Goals

- Commit credentials.
- Implement AFDC, NBI, or FARS adapters without fixtures.
- Restore FAF5 or BEA citations without a wired join.
- Claim excluded dimensions are zero.
- Regenerate the reviewed I-80 report.
- Resolve unrelated `data/t1-design-review.csv` worktree state.

## Result

ACS population and income now accept `CENSUS_API_KEY` from the environment and
never include it in errors. RUCC 2023 is automated from the official USDA CSV.

AFDC, NBI, FARS, FEMA, FAF5, and BEA are excluded from reviewed regeneration,
with their affected claims held until reviewed adapters or joins exist.
