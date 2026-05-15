---
wave: priority-a-pavement-repair-debt-review
pulse: 01
date: 2026-05-14
status: done
depends_on:
  - waves/2026-05-14-priority-a-hpms-broad-fetch-execution/CLOSE.md
governing_roles:
  - scope-keeper
  - numeracy-checker
  - citation-auditor
---

# Pulse 01 - Repair Debt Review Artifact

## Mission

Create a gated artifact that confirms priority-A TX, LA, and NM pavement
repair debt after broadened HPMS source acquisition, while preserving all
claims for a later disposition or relief replay.

## Scope Inventory

| Artifact | Role |
|---|---|
| `data/tier-pavement-unmatched-join-review.csv` | Source of priority-A state repair counts |
| `data/tier-pavement-debt-budget.csv` | Source of bundle-level repair cost proxies |
| `data/tier-pavement-repair-debt-review.csv` | New review artifact |

## Deliverables Checklist

- [x] Add `route tier-pavement-repair-debt-review --gate`.
- [x] Generate `data/tier-pavement-repair-debt-review.csv`.
- [x] Register the artifact in release/spec indexes and optimizer manifest.
- [x] Attach a role review and close note.

## Expected Gates

- `cargo fmt -p route`
- `cargo test -p route`
- `cargo run -q -p route -- tier-pavement-repair-debt-review --gate`
- `cargo run -q -p route -- optimizer-manifest --gate`
- `cargo run -q -p route -- release-manifest --gate`
- `C:\src\target\debug\proof check docs\SPEC_INDEX.md docs\optimizer-constraint-ledger-spec.md waves\PHASES.md waves\2026-05-14-priority-a-pavement-repair-debt-review`
- `powershell -ExecutionPolicy Bypass -File scripts\check-mileposts.ps1 -SkipTests`

## Non-Goals

- No repair funding decision.
- No accepted-evidence relief replay.
- No raw cache changes.
