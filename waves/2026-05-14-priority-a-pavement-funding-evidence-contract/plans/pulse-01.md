---
wave: priority-a-pavement-funding-evidence-contract
pulse: 01
date: 2026-05-14
status: done
depends_on:
  - waves/2026-05-14-priority-a-pavement-downgrade-exclusion-decision/CLOSE.md
governing_roles:
  - scope-keeper
  - numeracy-checker
---

# Pulse 01 - Evidence Contract

## Mission

Create a gateable contract for accepted funding evidence required before
priority-A pavement repair rows can be considered for relief replay.

## Scope Inventory

| Artifact | Role |
|---|---|
| `data/tier-pavement-downgrade-exclusion-decision.csv` | Input held current-tier decisions |
| `data/tier-pavement-funding-evidence-contract.csv` | Output funding evidence contract |

## Deliverables Checklist

- [x] Add `route tier-pavement-funding-evidence-contract --gate`.
- [x] Generate `data/tier-pavement-funding-evidence-contract.csv`.
- [x] Register the artifact in release/spec indexes and optimizer manifest.
- [x] Close the wave without relief replay.

## Expected Gates

- `cargo fmt -p route`
- `cargo test -p route`
- `cargo run -q -p route -- tier-pavement-funding-evidence-contract --gate`
- `cargo run -q -p route -- tier-optimize --all-tiers --gate`
- `cargo run -q -p route -- optimizer-manifest --gate`
- `cargo run -q -p route -- release-manifest --gate`
- `C:\src\target\debug\proof check docs\SPEC_INDEX.md docs\optimizer-constraint-ledger-spec.md waves\PHASES.md waves\2026-05-14-priority-a-pavement-funding-evidence-contract`
- `powershell -ExecutionPolicy Bypass -File scripts\check-mileposts.ps1 -SkipTests`

## Non-Goals

- No funding evidence acceptance.
- No blocker relief.
- No raw source capture.
