---
wave: priority-a-pavement-repair-disposition
pulse: 01
date: 2026-05-14
status: done
depends_on:
  - waves/2026-05-14-priority-a-pavement-repair-debt-review/CLOSE.md
governing_roles:
  - scope-keeper
  - numeracy-checker
---

# Pulse 01 - Repair Disposition

## Mission

Classify the four priority-A repair-debt rows as funding-required and
relief-ineligible before any optimizer relief replay.

## Scope Inventory

| Artifact | Role |
|---|---|
| `data/tier-pavement-repair-debt-review.csv` | Input repair-debt review |
| `data/tier-pavement-repair-disposition.csv` | Output disposition artifact |

## Deliverables Checklist

- [x] Add `route tier-pavement-repair-disposition --gate`.
- [x] Generate `data/tier-pavement-repair-disposition.csv`.
- [x] Register the artifact in release/spec indexes and optimizer manifest.
- [x] Close the wave without relief replay.

## Expected Gates

- `cargo fmt -p route`
- `cargo test -p route`
- `cargo run -q -p route -- tier-pavement-repair-disposition --gate`
- `cargo run -q -p route -- tier-optimize --all-tiers --gate`
- `cargo run -q -p route -- optimizer-manifest --gate`
- `cargo run -q -p route -- release-manifest --gate`
- `C:\src\target\debug\proof check docs\SPEC_INDEX.md docs\optimizer-constraint-ledger-spec.md waves\PHASES.md waves\2026-05-14-priority-a-pavement-repair-disposition`
- `powershell -ExecutionPolicy Bypass -File scripts\check-mileposts.ps1 -SkipTests`

## Non-Goals

- No repair funding acceptance.
- No downgrade or exclusion implementation.
- No blocker relief.
