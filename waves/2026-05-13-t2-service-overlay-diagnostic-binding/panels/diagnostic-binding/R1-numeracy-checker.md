---
wave: t2-service-overlay-diagnostic-binding
review: diagnostic-binding
round: 1
role: numeracy-checker
status: complete
---

# R1 - Numeracy Checker

| Severity | Finding | Artifact | Consequence | Fix |
|---|---|---|---|---|
| NOTE | The decision surface has seven rows, matching the seven `service-overlay` docket rows. | `data/t2-service-overlay-diagnostic-decisions.csv` | No row is dropped or duplicated. | Keep gate comparison against the docket. |
| NOTE | Optimizer manifest blocker count for the new stage is seven. | `data/tier-optimizer-runs.csv` | Optimizer accounting matches the diagnostic hold count. | Preserve `held-known` until diagnostics are authored. |

Decision: numeracy-pass.
