---
name: Map Publication Scope
slug: map-publication-scope
type: spec
status: reviewed
rubric_version: v1.0
author: copilot
created: 2026-05-15
updated: 2026-05-15
sources:
  - data/map-publication-scope-decision.csv
  - data/optimizer-residual-blocker-backlog.csv
  - data/optimizer-constraint-budget.csv
---

# Map Publication Scope

ROUTE now separates render validity from publication validity.

`route map-atlas --gate`, Beck diagnostics, and T3 zone diagnostics prove that
map images and schematic surfaces satisfy their file and geometry contracts.
They do not prove that all publication, upgrade, terminal-access, evidence, or
asset-condition claims are valid.

The controlling decision artifact is:

```text
data/map-publication-scope-decision.csv
```

## Current Decision

Full T1-T4 map publication is blocked until the optimizer blockers are actually
relieved or explicitly excluded. The first accepted exclusion is:

```text
data/t4-terminal-access-map-exclusion.csv
```

That exclusion removes unresolved T4 terminal-access overlay rows from current
`map` and `publication` claims only. It does not accept terminal-access proof and
does not clear `upgrade` or evidence work.

After that exclusion, the residual publication blockers are:

- 1 source snapshot guard blocking `evidence` and `publication` claims.
- 9 T2 asset-condition debt rows that remain publication-relevant debt even
  though they are budget debt rather than claim blockers.

Structural map renders may be used as work-in-progress maps only when labeled as
held-claim surfaces. They must not be described as fully publication-valid T1-T4
maps until the blocker ledger says so.

## Anti-Churn Rule

Do not create more `source-needed` placeholder ledgers for map publication.

The next map-validity artifact must do one of these:

1. Attach, review, accept, and replay real non-seed evidence.
2. Explicitly downgrade or exclude unresolved rows from the publication claim.
3. Keep the full T1-T4 map claim blocked.

`data/intermodal_terminals.csv` remains a seed only and cannot be used as
terminal-access proof.
