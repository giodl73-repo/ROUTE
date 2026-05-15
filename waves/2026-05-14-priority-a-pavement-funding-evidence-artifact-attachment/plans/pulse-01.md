---
wave: priority-a-pavement-funding-evidence-artifact-attachment
pulse: 01
date: 2026-05-14
status: done
depends_on:
  - waves/2026-05-14-priority-a-pavement-funding-evidence-source-capture/CLOSE.md
governing_roles:
  - scope-keeper
  - numeracy-checker
---

# Pulse 01 - Artifact Attachment

## Mission

Create a gateable artifact-attachment ledger for priority-A pavement funding
evidence that preserves blockers until an accepted funding artifact is attached
and reviewed.

## Scope Inventory

| Artifact | Role |
|---|---|
| `data/tier-pavement-funding-evidence-source-capture.csv` | Input source-capture placeholders |
| `data/tier-pavement-funding-evidence-artifact-attachment.csv` | Output artifact-attachment placeholders |

## Deliverables Checklist

- [x] Add `route tier-pavement-funding-evidence-artifact-attachment --gate`.
- [x] Generate `data/tier-pavement-funding-evidence-artifact-attachment.csv`.
- [x] Register the artifact in release/spec indexes and optimizer manifest.
- [x] Close the wave without relief replay.

## Expected Gates

- `cargo fmt -p route`
- `cargo test -p route`
- `cargo run -q -p route -- tier-pavement-funding-evidence-artifact-attachment --gate`
- `cargo run -q -p route -- tier-optimize --all-tiers --gate`
- `cargo run -q -p route -- optimizer-manifest --gate`
- `cargo run -q -p route -- release-manifest --gate`
- `C:\src\target\debug\proof check docs\SPEC_INDEX.md docs\optimizer-constraint-ledger-spec.md waves\PHASES.md waves\2026-05-14-priority-a-pavement-funding-evidence-artifact-attachment`
- `powershell -ExecutionPolicy Bypass -File scripts\check-mileposts.ps1 -SkipTests`

## Non-Goals

- No funding evidence acceptance.
- No blocker relief.
- No artifact review.
