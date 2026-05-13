---
wave: t4-terminal-contact-evidence
pulse: 05
date: 2026-05-13
status: planned
depends_on: [pulse-04]
governing_roles:
  - optimization-methodologist
  - citation-auditor
  - scope-keeper
---

# Pulse 05 - Ledger And Manifest Propagation

## Mission

Propagate terminal contact decisions through the normalized ledger, constraint
budget, optimizer manifest, and release manifest.

## Scope Inventory

| Surface | Current artifact | Pulse target |
|---|---|---|
| Constraint ledger | `data/optimizer-constraint-ledger.csv` | Normalize terminal contact decisions without hiding held rows. |
| Constraint budget | `data/optimizer-constraint-budget.csv` | Roll decisions into selector-facing blocker counts. |
| Optimizer manifest | `data/tier-optimizer-runs.csv` | Reflect any new or changed artifacts. |
| Release manifest | `data/release-manifest.csv` | Preserve source-needed/publication holds. |

## Deliverables

- [ ] Regenerate all affected optimizer artifacts.
- [ ] Update manifest/release rows if a new queue artifact is introduced.
- [ ] Confirm blocker counts by class and tier.
- [ ] Document residual source-needed terminal backlog.

## Expected Gates

- `route optimizer-constraint-ledger --gate`
- `route optimizer-constraint-budget --gate`
- `route tier-optimize --all-tiers --gate`
- `route optimizer-manifest --gate`
- `route release-manifest --gate`
- `cargo test -p route`

## Non-Goals

- Do not require zero terminal blockers to pass.
- Do not change non-terminal blocker families in this pulse.
