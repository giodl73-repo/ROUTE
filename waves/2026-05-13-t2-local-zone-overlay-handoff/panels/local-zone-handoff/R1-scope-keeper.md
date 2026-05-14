---
wave: t2-local-zone-overlay-handoff
review: local-zone-handoff
round: 1
role: scope-keeper
status: complete
---

# R1 - Scope Keeper

| Severity | Finding | Artifact | Consequence | Fix |
|---|---|---|---|---|
| NOTE | The handoff surface binds local-zone repair rows to T3 zone context without changing zone thresholds or map selection. | `data/t2-local-zone-overlay-handoff.csv` | The wave stays inside handoff scope. | Keep map authoring in T3 zone artifacts. |
| WARN | Service-overlay and bundle-readiness blockers remain outside this wave. | `data/t2-service-overlay-diagnostic-decisions.csv`; `data/t2-bundle-readiness-disposition.csv` | Other blocker classes remain held. | Carry them as separate backlog. |

Decision: scope-pass with residual holds visible.
