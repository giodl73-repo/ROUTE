---
wave: priority-a-pavement-funding-evidence-accepted-artifact-acquisition
pulse: 01
date: 2026-05-15
status: done
depends_on:
  - waves/2026-05-15-priority-a-pavement-funding-evidence-accepted-attachment-review/CLOSE.md
governing_roles:
  - scope-keeper
  - numeracy-checker
---

# Pulse 01 - Accepted Artifact Acquisition Targets

## Mission

Create a gateable acquisition/cache target ledger for accepted priority-A
pavement funding artifacts while preserving all blockers until a real accepted
artifact is cached, attached, and reviewed.

## Scope Inventory

| Artifact | Role |
|---|---|
| `data/tier-pavement-funding-evidence-accepted-attachment-review.csv` | Input held accepted-attachment review rows |
| `data/tier-pavement-funding-evidence-accepted-artifact-acquisition.csv` | Output source-needed acquisition/cache targets |

## Deliverables Checklist

- [x] Add `route tier-pavement-funding-evidence-accepted-artifact-acquisition --gate`.
- [x] Generate `data/tier-pavement-funding-evidence-accepted-artifact-acquisition.csv`.
- [x] Register the artifact in release/spec indexes and optimizer manifest.
- [x] Close the wave without relief replay.

## Expected Gates

- `cargo fmt -p route`
- `cargo test -p route`
- `cargo run -q -p route -- tier-pavement-funding-evidence-accepted-artifact-acquisition --gate`
- `cargo run -q -p route -- tier-optimize --all-tiers --gate`
- `cargo run -q -p route -- optimizer-manifest --gate`
- `cargo run -q -p route -- release-manifest --gate`
- `C:\src\target\debug\proof check docs\SPEC_INDEX.md docs\optimizer-constraint-ledger-spec.md waves\PHASES.md waves\2026-05-15-priority-a-pavement-funding-evidence-accepted-artifact-acquisition`
- `powershell -ExecutionPolicy Bypass -File scripts\check-mileposts.ps1 -SkipTests`

## Non-Goals

- No funding evidence acceptance.
- No blocker relief.
- No artifact attachment.
