---
wave: constraint-ledger-spine
pulse: 01
date: 2026-05-13
status: done
governing_roles:
  - optimization-methodologist
  - traffic-engineer
  - freight-economist
  - state-dot
---

# Pulse 01 - Pavement Debt Becomes Optimizer Debt

## Mission

Convert pavement standards from a side docket into priced optimizer debt that
can follow T1/T2 service bundles, selector rows, and game overlays.

## Delivered

- `data/tier-pavement-debt-budget.csv`
- payment/debt fields in T2 service columns
- pavement debt context in `data/game/t2-bundle-overlays.csv`
- compatibility path for downstream artifacts still carrying pavement-specific
  columns

## Evidence

Commits: `1594ace`, `be36060`, `18b656c`, `9acae00`.

## Gates

- [x] `route tier-pavement-debt-budget --gate`
- [x] `route t2-service-selection --gate`
- [x] `route t2-bundle-overlays --gate`
