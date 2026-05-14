---
wave: t2-bundle-readiness-repair-evidence
review: readiness-evidence
round: 1
role: citation-auditor
status: complete
---

# R1 - Citation Auditor

| Severity | Finding | Artifact | Consequence | Fix |
|---|---|---|---|---|
| NOTE | Each probe row names the downstream artifact used for evidence lookup. | `data/t2-bundle-readiness-repair-evidence.csv` | Candidate evidence is traceable. | Keep artifact and row-count fields visible in future replay. |
| WARN | Probe counts are candidate counts, not validated repair counts. | `data/tier-segment-candidates.csv`; `data/national-segment-registry.csv`; `data/t2-service-selection.csv` | Counts cannot support promotion by themselves. | Require replay gate before claim status changes. |

Decision: citation-pass for held evidence probe.
