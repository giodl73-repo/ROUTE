---
wave: columbus-south-terminal-contact-proof
pulse: 04
date: 2026-05-13
status: done
depends_on: [pulse-03]
governing_roles:
  - optimization-methodologist
  - numeracy-checker
  - scope-keeper
---

# Pulse 04 - Evidence And Scenario Propagation

## Mission

Propagate Columbus South proof decisions through the contact evidence queue,
scenario readiness, optimizer manifest, and release manifest while preserving
held claims.

## Scope Inventory

| Surface | Current artifact | Pulse target |
|---|---|---|
| Contact evidence | `data/t4-terminal-contact-evidence.csv` | Update only source-backed Columbus rows if proof exists. |
| Scenario readiness | `data/t4-terminal-scenario-readiness.csv` | Add candidates only for accepted proof rows. |
| Optimizer/release manifests | `data/tier-optimizer-runs.csv`; `data/release-manifest.csv` | Register any new Columbus artifacts with held status as needed. |

## Deliverables

- [x] Regenerate affected contact/proof/scenario artifacts.
- [x] Keep source-needed rows held and visible.
- [x] Promote only rows with traceable non-seed contact proof and selected
  attachment.
- [x] Update optimizer and release manifests for new artifacts/status changes.

## Expected Gates

- `route t4-terminal-contact-source-plan --gate`
- `route t4-terminal-contact-evidence --gate`
- `route t4-terminal-scenario-readiness --gate`
- `route tier-optimize --all-tiers --gate`
- `route optimizer-manifest --gate`
- `route release-manifest --gate`
- `cargo test -p route`

## Non-Goals

- Do not force a scenario candidate when proof remains source-needed.
- Do not change non-Columbus source rows.
