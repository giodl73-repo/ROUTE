---
wave: terminal-contact-source-acquisition-spine
pulse: 04
date: 2026-05-13
status: planned
depends_on: [pulse-03]
governing_roles:
  - optimization-methodologist
  - scope-keeper
  - numeracy-checker
---

# Pulse 04 - Evidence Propagation

## Mission

Propagate accepted, rejected, blocked, and source-needed proof decisions through
terminal contact evidence, scenario readiness, optimizer manifests, and release
surfaces.

## Scope Inventory

| Surface | Current artifact | Pulse target |
|---|---|---|
| Contact evidence | `data/t4-terminal-contact-evidence.csv` | Update only accepted non-seed proof rows. |
| Scenario readiness | `data/t4-terminal-scenario-readiness.csv` | Add candidates only for accepted proof rows. |
| Manifests | `data/tier-optimizer-runs.csv`; `data/release-manifest.csv` | Register new held/pass artifacts and row counts. |

## Deliverables

- [ ] Regenerate affected contact/proof/scenario artifacts.
- [ ] Promote only accepted proof rows with traceable source artifacts.
- [ ] Keep blocked/source-needed/rejected rows visible.
- [ ] Update optimizer and release manifests.

## Expected Gates

- `route t4-terminal-contact-evidence --gate`
- `route t4-terminal-scenario-readiness --gate`
- `route tier-optimize --all-tiers --gate`
- `route optimizer-manifest --gate`
- `route release-manifest --gate`
- `cargo test -p route`

## Non-Goals

- Do not make scenario candidates from source-needed or rejected proof rows.

