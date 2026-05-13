---
wave: constraint-ledger-blocker-burndown
pulse: 03
date: 2026-05-13
status: done
depends_on: [pulse-02]
governing_roles:
  - traffic-engineer
  - freight-economist
  - state-dot
  - citation-auditor
---

# Pulse 03 - T4 Terminal Evidence Holds

## Mission

Resolve or explicitly carry the `terminal_access_evidence_gap` blockers by
naming terminal obligations, source actions, or local-access holds.

## Scope Inventory

| Surface | Current artifact | Pulse target |
|---|---|---|
| Terminal access blockers | `data/optimizer-constraint-budget.csv` | Identify terminal/source evidence needed for each held row. |
| Terminal access columns | `data/t4-terminal-access-columns.csv` | Attach terminal obligation and source status. |
| Access diagnostics | `data/t3-zone-map-diagnostics.csv` | Carry terminal evidence decision into map readiness. |
| Release surface | `data/release-manifest.csv` | Preserve publication holds where terminal evidence remains source-needed. |

## Deliverables

- [x] Add source/terminal obligation detail for each terminal evidence blocker.
- [x] Regenerate T4 access, T3/T4 diagnostics, ledger, budget, and manifests.
- [x] If a terminal row becomes scenario-ready, name the scenario artifact; do not
  run broad scenarios without that claim.
- [x] Update review notes for any continued source holds.

## Results

- After Pulse 02, the terminal-evidence queue contains 69 zone-scoped T4 rows,
  not the six-row opening queue.
- `data/t4-terminal-access-columns.csv` now names a source-backed terminal
  district obligation for every terminal-review row using the existing
  `data/intermodal_terminals.csv` source surface.
- No row became scenario-ready: all 69 remain explicit
  `terminal_access_evidence_gap` claim blockers until route-to-terminal contact
  proof is authored.
- Continued source holds are summarized in
  `waves/2026-05-13-constraint-ledger-blocker-burndown/panels/pulse-03-terminal-source-holds.md`.

## Gate Results

- `cargo test -p route`: pass
- `route t4-terminal-access-columns --gate`: pass
- `route t3-t4-access-gaps --gate`: pass
- `route t3-zone-map-diagnostics --gate`: pass
- `route t3-zone-render-board --gate`: pass
- `route optimizer-constraint-ledger --gate`: pass
- `route optimizer-constraint-budget --gate`: pass
- `route tier-optimize --all-tiers --gate`: pass
- `route optimizer-manifest --gate`: pass
- `route release-manifest --gate`: pass
- `scripts/check-mileposts.ps1 -SkipTests`: pass

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

