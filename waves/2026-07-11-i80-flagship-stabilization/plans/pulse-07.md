---
wave: i80-flagship-stabilization
pulse: 07
date: 2026-07-11
status: done
depends_on:
  - pulse-06
governing_roles:
  - scope-keeper
  - citation-auditor
  - numeracy-checker
---

# Pulse 07 - Flagship Software Hardening

## Mission

Make the I-80 build, packet, and reviewed-corpus path deterministic enough for a
clean CI checkout while preventing incomplete source caches from silently
degrading the reviewed anchor.

## Deliverables

- [x] Commit `Cargo.lock`.
- [x] Pin FLETCH and METIS-CORE to exact revisions.
- [x] Add Linux and Windows flagship quality jobs.
- [x] Add locked tests, targeted clippy, formatting, and packet freshness gates.
- [x] Refuse reviewed-corpus overwrite when required source caches are absent.
- [x] Add explicit `--allow-partial` escape hatch for intentional degraded output.
- [x] Add focused guard tests.
- [x] Record the remaining source-cache reproducibility hold.

## Gates

- `cargo fmt --all -- --check`
- `cargo clippy --locked -p route-score -p route-report --all-targets --no-deps -- -D warnings`
- `cargo test -q --locked -p route-score -p route-report`
- `cargo test -q --locked -p route --bin route`
- `npm run check:i80:packet`
- `route report I80` fails safely when reviewed sources are incomplete.
- The reviewed corpus file remains byte-identical after the failed command.
- `git diff --check`

## Non-Goals

- Commit raw or cached transportation data.
- Claim the complete I-80 measurement set can be regenerated from public URLs
  without manual source work.
- Refactor the entire CLI monolith.
- Fix unrelated pre-existing clippy findings outside the flagship crates.
- Resolve unrelated `data/t1-design-review.csv` worktree state.

## Result

The dependency graph and flagship CI path are pinned and portable. The packet
is deterministic. Reviewed corpus regeneration now fails explicitly rather
than silently replacing the anchor with partial measurements.

The remaining hold is source-data reproducibility: HPMS, ACS income/population,
RUCC, DCFC, FEMA, NBI, and FARS cache artifacts are not all derivable from the
current manifest in one clean-clone command.
