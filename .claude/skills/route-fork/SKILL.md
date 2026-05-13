---
name: route-fork
description: "Materialize a ROUTE pulse plan plus governing role context into one fork file for agent execution."
tags: [route, fork, pulse, agents, context]
---

# route-fork

Use this skill when a pulse will be delegated or when the execution context
should be made auditable before implementation.

## Procedure

1. Resolve active wave from `waves/PHASES.md`.
2. Read the requested `waves/{active}/plans/pulse-NN.md`.
3. Resolve `governing_roles` by searching `.roles/`.
4. Write `waves/{active}/forks/pulse-NN.md` containing:
   - complete pulse plan
   - complete governing role text
   - execution contract and gates
5. The fork file is the only prompt a worker should need.

## Execution Contract

Workers must read the full fork file, edit only within the requested scope, run
the listed gates, and report files changed plus gate status.
