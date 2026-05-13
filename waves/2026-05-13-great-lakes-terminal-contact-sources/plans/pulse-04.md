---
wave: great-lakes-terminal-contact-sources
pulse: 04
date: 2026-05-13
status: done
depends_on: [pulse-03]
governing_roles:
  - optimization-methodologist
  - citation-auditor
  - scope-keeper
---

# Pulse 04 - Queue And Manifest Propagation

## Mission

Propagate the Great Lakes source plan and proof docket through optimizer and
release surfaces while preserving held claims.

## Scope Inventory

| Surface | Current artifact | Pulse target |
|---|---|---|
| Source plan/proof docket | Pulse 01/03 artifacts | Register as governed held artifacts. |
| Optimizer manifest | `data/tier-optimizer-runs.csv` | Add or update source-plan stage if needed. |
| Release manifest | `data/release-manifest.csv` | Preserve held-public status. |
| Constraint budget | `data/optimizer-constraint-budget.csv` | Confirm blockers remain visible. |

## Deliverables

- [x] Regenerate affected optimizer artifacts.
- [x] Register new source-plan/proof artifacts in optimizer and release manifests.
- [x] Confirm terminal blocker counts remain visible.
- [x] Document residual source-needed backlog.

## Expected Gates

- `route t4-terminal-contact-source-plan --gate`
- `route optimizer-constraint-ledger --gate`
- `route optimizer-constraint-budget --gate`
- `route tier-optimize --all-tiers --gate`
- `route optimizer-manifest --gate`
- `route release-manifest --gate`
- `cargo test -p route`

## Non-Goals

- Do not require zero terminal blockers.
- Do not change non-terminal blocker families.
