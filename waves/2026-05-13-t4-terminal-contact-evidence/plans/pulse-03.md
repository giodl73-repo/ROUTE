---
wave: t4-terminal-contact-evidence
pulse: 03
date: 2026-05-13
status: done
depends_on: [pulse-02]
governing_roles:
  - freight-economist
  - state-dot
  - traffic-engineer
  - citation-auditor
---

# Pulse 03 - Remaining Zone Contact Pass

## Mission

Apply the terminal contact evidence queue to Southeast, Mid-South, Mountain
West, and Texas Border T4 rows.

## Scope Inventory

| Surface | Current artifact | Pulse target |
|---|---|---|
| Non-Great-Lakes T4 holds | `data/t4-terminal-access-columns.csv` | Classify remaining zone rows. |
| Contact queue | Pulse 01 artifact | Carry all non-Great-Lakes decisions. |
| T3/T4 access gaps | `data/t3-t4-access-gaps.csv` | Keep unresolved rows visible with next artifacts. |
| Constraint budget | `data/optimizer-constraint-budget.csv` | Confirm blocker counts reflect decisions. |

## Deliverables

- [x] Classify all remaining zone terminal rows.
- [x] Preserve source-needed and held-known rows as claim blockers.
- [x] Promote only source-backed contact rows to scenario-readiness candidates.
- [x] Regenerate T4/access-gap/ledger/budget artifacts.

## Expected Gates

- `route t4-terminal-access-columns --gate`
- `route t4-terminal-contact-evidence --gate`
- `route t3-t4-access-gaps --gate`
- `route optimizer-constraint-ledger --gate`
- `route optimizer-constraint-budget --gate`
- `cargo test -p route`

## Evidence

- Classified the remaining 36 terminal-contact rows in
  `data/t4-terminal-contact-evidence.csv`: Southeast (12), Mid-South (11),
  Mountain West (9), and Texas Border (4).
- Candidate districts are assigned as seed districts only. No remaining-zone row
  earned source-backed or scenario-ready status because no separate
  route-to-terminal contact proof source exists yet.
- All 69 terminal-contact rows across the wave remain `source-needed` claim
  blockers.
- Gates passed: `cargo test -p route`, `route t4-terminal-access-columns
  --gate`, `route t4-terminal-contact-evidence --gate`, `route
  t3-t4-access-gaps --gate`, `route optimizer-constraint-ledger --gate`, and
  `route optimizer-constraint-budget --gate`.

## Non-Goals

- Do not change T3/T4 zone taxonomy.
- Do not fetch live sources outside existing source policy.
