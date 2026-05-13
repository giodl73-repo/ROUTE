---
wave: terminal-contact-source-acquisition-spine
pulse: 03
date: 2026-05-13
status: done
depends_on: [pulse-02]
governing_roles:
  - traffic-engineer
  - freight-economist
  - citation-auditor
---

# Pulse 03 - District Proof Import

## Mission

Run one terminal district slice through the registry/import path and classify
route proof attempts as accepted, source-needed, blocked, or rejected.

## Scope Inventory

| Surface | Current artifact | Pulse target |
|---|---|---|
| Columbus proof attempts | `data/t4-terminal-columbus-proof-attempts.csv` | Reconcile with registry rows if proof artifacts exist. |
| Great Lakes proof docket | `data/t4-terminal-contact-proof-docket.csv` | Preserve source-needed rows without matched proof. |
| Source registry | Pulse 02 artifact | One district imported through proof attempt gate. |

## Deliverables

- [x] Select one district slice by explicit backlog rule.
- [x] Emit proof-decision rows with accepted/rejected/blocked/source-needed status.
- [x] Preserve source-needed blockers for unmatched rows.
- [x] Add tests/gates for non-seed proof evidence.

## Expected Gates

- `route t4-terminal-contact-proof-source-registry --gate`
- `route t4-terminal-contact-district-proof-import --gate`
- `route t4-terminal-contact-proof-artifact-contract --gate`
- `cargo test -p route`

## Non-Goals

- Do not require any row to become accepted.
- Do not expand beyond one district slice.

