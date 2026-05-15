---
wave: priority-a-pavement-funding-evidence-review-docket
pulse: 01
date: 2026-05-14
status: done
depends_on:
  - waves/2026-05-14-priority-a-pavement-funding-evidence-artifact-attachment/CLOSE.md
governing_roles:
  - scope-keeper
  - numeracy-checker
---

# Pulse 01 - Review Docket

## Mission

Create a gateable review docket for priority-A pavement funding evidence that
records unattached funding artifacts as held and not accepted.

## Scope Inventory

| Artifact | Role |
|---|---|
| `data/tier-pavement-funding-evidence-artifact-attachment.csv` | Input attachment placeholders |
| `data/tier-pavement-funding-evidence-review-docket.csv` | Output held review rows |

## Deliverables Checklist

- [x] Add `route tier-pavement-funding-evidence-review-docket --gate`.
- [x] Generate `data/tier-pavement-funding-evidence-review-docket.csv`.
- [x] Register the artifact in release/spec indexes and optimizer manifest.
- [x] Close the wave without relief replay.

## Expected Gates

- `cargo fmt -p route`
- `cargo test -p route`
- `cargo run -q -p route -- tier-pavement-funding-evidence-review-docket --gate`
- `cargo run -q -p route -- tier-optimize --all-tiers --gate`
- `cargo run -q -p route -- optimizer-manifest --gate`
- `cargo run -q -p route -- release-manifest --gate`
- `C:\src\target\debug\proof check docs\SPEC_INDEX.md docs\optimizer-constraint-ledger-spec.md waves\PHASES.md waves\2026-05-14-priority-a-pavement-funding-evidence-review-docket`
- `powershell -ExecutionPolicy Bypass -File scripts\check-mileposts.ps1 -SkipTests`

## Non-Goals

- No funding evidence acceptance.
- No blocker relief.
- No artifact attachment.
