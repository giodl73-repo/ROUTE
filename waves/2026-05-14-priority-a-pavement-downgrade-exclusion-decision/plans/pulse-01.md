---
wave: priority-a-pavement-downgrade-exclusion-decision
pulse: 01
date: 2026-05-14
status: done
depends_on:
  - waves/2026-05-14-priority-a-pavement-funding-commitment-review/CLOSE.md
governing_roles:
  - scope-keeper
  - numeracy-checker
---

# Pulse 01 - Downgrade/Exclusion Decision

## Mission

Create a gateable decision artifact showing that no priority-A pavement repair
row is downgraded or excluded without separate authorization, and all remain
held at current tier.

## Scope Inventory

| Artifact | Role |
|---|---|
| `data/tier-pavement-funding-commitment-review.csv` | Input funding commitment review |
| `data/tier-pavement-downgrade-exclusion-decision.csv` | Output downgrade/exclusion decision |

## Deliverables Checklist

- [x] Add `route tier-pavement-downgrade-exclusion-decision --gate`.
- [x] Generate `data/tier-pavement-downgrade-exclusion-decision.csv`.
- [x] Register the artifact in release/spec indexes and optimizer manifest.
- [x] Close the wave without relief replay.

## Expected Gates

- `cargo fmt -p route`
- `cargo test -p route`
- `cargo run -q -p route -- tier-pavement-downgrade-exclusion-decision --gate`
- `cargo run -q -p route -- tier-optimize --all-tiers --gate`
- `cargo run -q -p route -- optimizer-manifest --gate`
- `cargo run -q -p route -- release-manifest --gate`
- `C:\src\target\debug\proof check docs\SPEC_INDEX.md docs\optimizer-constraint-ledger-spec.md waves\PHASES.md waves\2026-05-14-priority-a-pavement-downgrade-exclusion-decision`
- `powershell -ExecutionPolicy Bypass -File scripts\check-mileposts.ps1 -SkipTests`

## Non-Goals

- No downgrade implementation.
- No exclusion implementation.
- No blocker relief.
