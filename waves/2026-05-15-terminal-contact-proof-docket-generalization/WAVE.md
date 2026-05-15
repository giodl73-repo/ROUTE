---
wave: terminal-contact-proof-docket-generalization
date_open: 2026-05-15
status: done
---

# Terminal Contact Proof Docket Generalization

## Mission

Remove the Great-Lakes-only limit from the positive T4 terminal-contact proof
docket so residual rows in New Orleans, Texas Border, Mountain West, and other
districts can use the same non-seed proof registry/import path as Indianapolis
Avon.

## Opening Rule

This is proof infrastructure, not blocker relief. Generalize the governed
source-backed path without accepting weak sources, seed assignments, or metro
proximity as contact proof.

## Inputs Inherited

- `data/t4-terminal-contact-evidence.csv`
- `data/t4-terminal-contact-source-plan.csv`
- `data/t4-terminal-contact-source-catalog.csv`
- `data/t4-terminal-contact-proof-docket.csv`
- `data/t4-terminal-contact-proof-source-registry.csv`
- `data/t4-terminal-contact-district-proof-import.csv`

## Pulse Status

| Pulse | Status | Output |
|---|---|---|
| Generalize positive proof docket | done | Source-plan/proof docket expanded to all source-needed terminal-contact rows |

## Done Criteria

- Source plan covers every `source-needed` terminal-contact row, not only `t3-great-lakes`.
- Source catalog covers all terminal districts represented by the current source-needed rows.
- Proof docket contains all 69 terminal-contact proof tasks.
- Proof source registry preserves the existing I-465 accepted proof and keeps unresolved rows source-needed.
- District proof import remains gate-clean and does not accept any new weak proof.

## Non-goals

- Do not clear T4 residual blockers without accepted or rejected route-specific proof.
- Do not promote T1 source evidence or T2 repair-debt claims.
- Do not use `data/intermodal_terminals.csv` as route-to-terminal proof.
