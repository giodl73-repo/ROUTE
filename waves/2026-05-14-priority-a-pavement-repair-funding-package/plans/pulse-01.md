---
wave: priority-a-pavement-repair-funding-package
pulse: 01
date: 2026-05-14
status: done
depends_on:
  - waves/2026-05-14-priority-a-pavement-repair-disposition/CLOSE.md
governing_roles:
  - scope-keeper
  - numeracy-checker
---

# Pulse 01 - Funding Package

## Mission

Create a gateable priority-A pavement repair funding package while preserving
unfunded, relief-ineligible status.

## Scope Inventory

| Artifact | Role |
|---|---|
| `data/tier-pavement-repair-disposition.csv` | Input disposition rows |
| `data/tier-pavement-repair-funding-package.csv` | Output funding package |

## Deliverables Checklist

- [x] Add `route tier-pavement-repair-funding-package --gate`.
- [x] Generate `data/tier-pavement-repair-funding-package.csv`.
- [x] Register the artifact in release/spec indexes and optimizer manifest.
- [x] Close the wave without relief replay.

## Expected Gates

- `cargo fmt -p route`
- `cargo test -p route`
- `cargo run -q -p route -- tier-pavement-repair-funding-package --gate`
- `cargo run -q -p route -- tier-optimize --all-tiers --gate`
- `cargo run -q -p route -- optimizer-manifest --gate`
- `cargo run -q -p route -- release-manifest --gate`
- `C:\src\target\debug\proof check docs\SPEC_INDEX.md docs\optimizer-constraint-ledger-spec.md waves\PHASES.md waves\2026-05-14-priority-a-pavement-repair-funding-package`
- `powershell -ExecutionPolicy Bypass -File scripts\check-mileposts.ps1 -SkipTests`

## Non-Goals

- No funding commitment acceptance.
- No downgrade or exclusion implementation.
- No blocker relief.
