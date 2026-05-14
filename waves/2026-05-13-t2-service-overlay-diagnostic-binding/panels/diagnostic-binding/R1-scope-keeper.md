---
wave: t2-service-overlay-diagnostic-binding
review: diagnostic-binding
round: 1
role: scope-keeper
status: complete
---

# R1 - Scope Keeper

| Severity | Finding | Artifact | Consequence | Fix |
|---|---|---|---|---|
| NOTE | The diagnostic surface binds service-overlay repair rows to Beck diagnostic requirements without authoring new service classes. | `data/t2-service-overlay-diagnostic-decisions.csv` | The wave stays inside diagnostic binding scope. | Keep service-class assignment in a future Beck diagnostic wave. |
| WARN | Local-zone rows are intentionally not resolved by this wave. | `data/t2-service-class-repair-docket.csv` | Seven local-zone rows remain below national game overlay claims. | Carry them into a local/zone repair wave. |

Decision: scope-pass with residual holds visible.
