---
wave: t4-terminal-contact-evidence
pulse: 05
date: 2026-05-13
status: done
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

- [x] Regenerate all affected optimizer artifacts.
- [x] Update manifest/release rows if a new queue artifact is introduced.
- [x] Preserve the contact queue as a visible source artifact in optimizer and
  release manifests before any scenario candidate is treated as release-facing.
- [x] Confirm blocker counts by class and tier.
- [x] Document residual source-needed terminal backlog.

## Expected Gates

- `route optimizer-constraint-ledger --gate`
- `route optimizer-constraint-budget --gate`
- `route tier-optimize --all-tiers --gate`
- `route optimizer-manifest --gate`
- `route release-manifest --gate`
- `cargo test -p route`

## Evidence

- Added optimizer-manifest rows for `data/t4-terminal-contact-evidence.csv` and
  `data/t4-terminal-scenario-readiness.csv`.
- Added held-public release-manifest rows for both artifacts.
- Blocker counts remain stable: 142 optimizer constraint rows, 137 constraint
  budget rows, 0 hard blockers, and 117 claim blockers, including 69
  `terminal_access_evidence_gap` rows.
- Gates passed: `cargo test -p route`, `route optimizer-constraint-ledger
  --gate`, `route optimizer-constraint-budget --gate`, `route tier-optimize
  --all-tiers --gate`, `route optimizer-manifest --gate`, and `route
  release-manifest --gate`.

## Non-Goals

- Do not require zero terminal blockers to pass.
- Do not change non-terminal blocker families in this pulse.
