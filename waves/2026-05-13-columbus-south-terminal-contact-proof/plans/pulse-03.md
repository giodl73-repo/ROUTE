---
wave: columbus-south-terminal-contact-proof
pulse: 03
date: 2026-05-13
status: done
depends_on: [pulse-02]
governing_roles:
  - traffic-engineer
  - freight-economist
  - citation-auditor
---

# Pulse 03 - Route Proof Attempt

## Mission

Attach source attempts or explicit blockers to all eight Columbus South route
proof tasks.

## Scope Inventory

| Surface | Current artifact | Pulse target |
|---|---|---|
| Columbus intake | Pulse 01/02 artifact | One attempt row per route. |
| Proof docket | `data/t4-terminal-contact-proof-docket.csv` | Preserve one source-needed task per route unless proof is accepted. |
| Source/cache artifacts | planned or blocked | Trace source attempts without live-fetch side effects. |

## Deliverables

- [x] Add proof-attempt rows for I-271, I-279, I-471, US22, US224, US250, US35,
  and US74.
- [x] Each attempt names source artifact or blocker, capture status, contact
  statement status, and selected higher-tier attachment status.
- [x] Classify each attempt as accepted, source-needed, blocked, or rejected.
- [x] Add tests or gates that prevent source-backed status without a non-seed
  proof artifact.

## Expected Gates

- `route t4-terminal-columbus-proof-attempts --gate`
- `route t4-terminal-contact-source-plan --gate`
- `route t4-terminal-contact-evidence --gate`
- `cargo test -p route`

## Non-Goals

- Do not create a scenario.
- Do not infer contact from route proximity or district membership.
