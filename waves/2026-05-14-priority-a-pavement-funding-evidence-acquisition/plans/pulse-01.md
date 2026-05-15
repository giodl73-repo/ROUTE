---
wave: priority-a-pavement-funding-evidence-acquisition
pulse: 01
date: 2026-05-14
status: done
depends_on:
  - waves/2026-05-14-priority-a-pavement-funding-evidence-review-docket/CLOSE.md
governing_roles:
  - scope-keeper
  - numeracy-checker
---

# Pulse 01 - Acquisition Targets

## Mission

Create a gateable acquisition ledger for accepted priority-A pavement funding
artifacts while preserving all blockers until evidence is attached and reviewed.

## Scope Inventory

| Artifact | Role |
|---|---|
| `data/tier-pavement-funding-evidence-review-docket.csv` | Input held review rows |
| `data/tier-pavement-funding-evidence-acquisition.csv` | Output accepted-artifact acquisition targets |

## Deliverables Checklist

- [x] Add `route tier-pavement-funding-evidence-acquisition --gate`.
- [x] Generate `data/tier-pavement-funding-evidence-acquisition.csv`.
- [x] Register the artifact in release/spec indexes and optimizer manifest.
- [x] Close the wave without relief replay.

## Expected Gates

- `cargo fmt -p route`
- `cargo test -p route`
- `cargo run -q -p route -- tier-pavement-funding-evidence-acquisition --gate`
- `cargo run -q -p route -- tier-optimize --all-tiers --gate`
- `cargo run -q -p route -- optimizer-manifest --gate`
- `cargo run -q -p route -- release-manifest --gate`
- `C:\src\target\debug\proof check docs\SPEC_INDEX.md docs\optimizer-constraint-ledger-spec.md waves\PHASES.md waves\2026-05-14-priority-a-pavement-funding-evidence-acquisition`
- `powershell -ExecutionPolicy Bypass -File scripts\check-mileposts.ps1 -SkipTests`

## Non-Goals

- No funding evidence acceptance.
- No blocker relief.
- No artifact attachment.
