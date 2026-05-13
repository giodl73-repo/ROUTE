---
wave: columbus-south-terminal-contact-proof
pulse: 01
date: 2026-05-13
status: planned
depends_on: []
governing_roles:
  - citation-auditor
  - scope-keeper
  - optimization-methodologist
---

# Pulse 01 - Columbus Proof Intake

## Mission

Create a Columbus South proof-intake artifact and gate from the existing Great
Lakes proof docket.

## Scope Inventory

| Surface | Current artifact | Pulse target |
|---|---|---|
| Proof docket | `data/t4-terminal-contact-proof-docket.csv` | Filter eight Columbus South rows. |
| Source plan | `data/t4-terminal-contact-source-plan.csv` | Preserve seed/proof separation. |
| Contact queue | `data/t4-terminal-contact-evidence.csv` | Keep all rows source-needed. |

## Deliverables

- [ ] Add a Columbus South proof-intake artifact or command output.
- [ ] Gate exactly eight Columbus South source-needed proof tasks.
- [ ] Reject non-Columbus rows in the pilot intake.
- [ ] Preserve source-needed/review status until a proof source exists.

## Expected Gates

- `route t4-terminal-contact-source-plan --gate`
- Columbus intake gate if added
- `cargo test -p route`

## Non-Goals

- Do not attempt source acquisition in this pulse.
- Do not modify non-Columbus rows.
