---
wave: priority-a-pavement-funding-commitment-review
pulse: 01
date: 2026-05-14
status: done
depends_on:
  - waves/2026-05-14-priority-a-pavement-repair-funding-package/CLOSE.md
governing_roles:
  - scope-keeper
  - numeracy-checker
---

# Pulse 01 - Commitment Review

## Mission

Create a gateable review showing that the priority-A pavement repair funding
package has no accepted commitment artifact attached and remains ineligible
for relief.

## Scope Inventory

| Artifact | Role |
|---|---|
| `data/tier-pavement-repair-funding-package.csv` | Input unfunded package |
| `data/tier-pavement-funding-commitment-review.csv` | Output commitment review |

## Deliverables Checklist

- [x] Add `route tier-pavement-funding-commitment-review --gate`.
- [x] Generate `data/tier-pavement-funding-commitment-review.csv`.
- [x] Register the artifact in release/spec indexes and optimizer manifest.
- [x] Close the wave without relief replay.

## Expected Gates

- `cargo fmt -p route`
- `cargo test -p route`
- `cargo run -q -p route -- tier-pavement-funding-commitment-review --gate`
- `cargo run -q -p route -- tier-optimize --all-tiers --gate`
- `cargo run -q -p route -- optimizer-manifest --gate`
- `cargo run -q -p route -- release-manifest --gate`
- `C:\src\target\debug\proof check docs\SPEC_INDEX.md docs\optimizer-constraint-ledger-spec.md waves\PHASES.md waves\2026-05-14-priority-a-pavement-funding-commitment-review`
- `powershell -ExecutionPolicy Bypass -File scripts\check-mileposts.ps1 -SkipTests`

## Non-Goals

- No commitment acceptance.
- No downgrade or exclusion implementation.
- No blocker relief.
