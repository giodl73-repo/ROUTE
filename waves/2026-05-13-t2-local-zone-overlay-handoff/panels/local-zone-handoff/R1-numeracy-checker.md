---
wave: t2-local-zone-overlay-handoff
review: local-zone-handoff
round: 1
role: numeracy-checker
status: complete
---

# R1 - Numeracy Checker

| Severity | Finding | Artifact | Consequence | Fix |
|---|---|---|---|---|
| NOTE | The handoff surface has seven rows, matching the seven `local-zone` docket rows. | `data/t2-local-zone-overlay-handoff.csv` | No row is dropped or duplicated. | Keep gate comparison against the docket. |
| NOTE | Optimizer manifest blocker count for the new stage is seven. | `data/tier-optimizer-runs.csv` | Optimizer accounting matches the handoff hold count. | Preserve `held-known` until local-zone promotion rules exist. |

Decision: numeracy-pass.
