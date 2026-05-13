---
wave: constraint-ledger-spine
pulse: 04
date: 2026-05-13
status: done
governing_roles:
  - optimization-methodologist
  - schematic-cartographer
  - traffic-engineer
---

# Pulse 04 - Selector Adoption Across T1/T2/T3/T4

## Mission

Carry the generalized constraint budget through selectors instead of leaving it
as a standalone ledger nobody consumes.

## Delivered

- T2 selector/regionalizer/service rows consume constraint budget rollups.
- T1 line selector consumes constraint budget rollups.
- T3/T4 selectors and access gaps consume and emit constraint pressure.
- All-tier optimizer manifest row counts updated.

## Evidence

Commits: `9f455e1`, `b37a8b8`, `943b25d`, `955b36a`.

## Gates

- [x] `route t1-line-selector --gate`
- [x] `route tier-candidate-columns --gate`
- [x] `route t2-regionalizer --gate`
- [x] `route t2-service-selection --gate`
- [x] `route t3-zone-route-columns --gate`
- [x] `route t4-terminal-access-columns --gate`
- [x] `route t3-t4-access-gaps --gate`
- [x] `route tier-optimize --all-tiers --gate`
