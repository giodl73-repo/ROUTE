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
relieved or explicitly excluded. The accepted exclusions are:

```text
data/t4-terminal-access-map-exclusion.csv
data/source-snapshot-publication-exclusion.csv
data/t2-asset-condition-map-publication-exclusion.csv
```

The terminal-access exclusion removes unresolved T4 terminal-access overlay rows
from current `map` and `publication` claims only. It does not accept
terminal-access proof and does not clear `upgrade` or evidence work.

The source-snapshot exclusion removes the live snapshot guard from the current
map `publication` claim only. It does not accept live-event evidence and still
blocks `evidence` until repeat-window or archive-history proof exists.

The T2 asset-condition exclusion removes pavement/source repair debt from the
current map `publication` claim only. It does not fund repairs, accept missing
pavement evidence, or clear `SLA`, `transit`, or `upgrade` obligations.

The residual non-publication holds are:

- 1 source snapshot guard blocking `evidence`.
- 69 T4 terminal-access evidence gaps blocking `upgrade`.
- 9 T2 asset-condition debt rows blocking `SLA`, `transit`, and `upgrade`.

There are now no residual `publication` blockers in
`data/optimizer-residual-blocker-backlog.csv`. Structural T1-T4 maps may be
published as maps only when labeled as held-claim surfaces. They must not be
described as evidence-valid, SLA-valid, transit-ready, upgrade-ready, or
asset-condition repaired until the blocker ledger says so.

## Anti-Churn Rule

Do not create more `source-needed` placeholder ledgers for map publication.

The next map-validity artifact must do one of these:

1. Attach, review, accept, and replay real non-seed evidence.
2. Explicitly downgrade or exclude unresolved non-publication claims from the
   claim being made.
3. Keep the affected non-publication claim blocked.

`data/intermodal_terminals.csv` remains a seed only and cannot be used as
terminal-access proof.
