---
wave: constraint-ledger-blocker-burndown
pulse: 03
date: 2026-05-13
status: planned
depends_on: [pulse-02]
governing_roles:
  - traffic-engineer
  - freight-economist
  - state-dot
  - citation-auditor
---

# Pulse 03 - T4 Terminal Evidence Holds

## Mission

Resolve or explicitly carry the six `terminal_access_evidence_gap` blockers by
naming terminal obligations, source actions, or local-access holds.

## Scope Inventory

| Surface | Current artifact | Pulse target |
|---|---|---|
| Terminal access blockers | `data/optimizer-constraint-budget.csv` | Identify terminal/source evidence needed for each held row. |
| Terminal access columns | `data/t4-terminal-access-columns.csv` | Attach terminal obligation and source status. |
| Access diagnostics | `data/t3-zone-map-diagnostics.csv` | Carry terminal evidence decision into map readiness. |
| Release surface | `data/release-manifest.csv` | Preserve publication holds where terminal evidence remains source-needed. |

## Deliverables

- [ ] Add source/terminal obligation detail for each terminal evidence blocker.
- [ ] Regenerate T4 access, T3/T4 diagnostics, ledger, budget, and manifests.
- [ ] If a terminal row becomes scenario-ready, name the scenario artifact; do not
  run broad scenarios without that claim.
- [ ] Update review notes for any continued source holds.

## Expected Gates

- `route optimizer-constraint-ledger --gate`
- `route optimizer-constraint-budget --gate`
- `route tier-optimize --all-tiers --gate`
- `route release-manifest --gate`
- `scripts/check-mileposts.ps1 -SkipTests`

## Non-Goals

- Do not scrape or fetch live terminal sources unless the source policy already
  names a safe command.
- Do not promote source-needed terminal access to publication-ready status.

