---
name: Terminal Contact Source Acquisition Spine Close
slug: terminal-contact-source-acquisition-spine-close
type: plan
status: validated
rubric_version: v1.0
author: route-pulse
created: 2026-05-13
updated: 2026-05-13
sources:
  - data/t4-terminal-contact-proof-artifact-contract.csv
  - data/t4-terminal-contact-proof-source-registry.csv
  - data/t4-terminal-contact-district-proof-import.csv
  - data/tier-optimizer-runs.csv
  - data/release-manifest.csv
---

# Terminal Contact Source Acquisition Spine Close

## Outcome

The wave built the manual/cached source-acquisition spine for terminal-contact
proof, but no route advanced to `source-backed` or `scenario-ready`. The new
contract and registry surfaces make source evidence importable and gateable; the
current registry still contains no manual citation or cached source artifact
satisfying the contract.

## Artifact Decisions

| Artifact | Rows | Status | Decision |
|---|---:|---|---|
| `data/t4-terminal-contact-proof-artifact-contract.csv` | 1 | pass | Defines required fields and prohibits seed/proximity proof. |
| `data/t4-terminal-contact-proof-source-registry.csv` | 33 | source-needed | Registry intake exists; no proof artifacts are attached. |
| `data/t4-terminal-contact-district-proof-import.csv` | 8 | source-needed | Largest unresolved district is Columbus South; all rows remain source-needed. |
| `data/t4-terminal-contact-evidence.csv` | 69 | held | No accepted proof row changed contact evidence. |
| `data/t4-terminal-scenario-readiness.csv` | 1 | held | No scenario-ready row exists. |

## District Import Result

| District | Routes imported | Accepted | Source-needed | Blocked | Rejected |
|---|---:|---:|---:|---:|---:|
| Columbus South | 8 | 0 | 8 | 0 | 0 |

Residual blocker for each imported row: manual citation or cached source
artifact not registered for route-to-terminal contact proof.

## Doctrine Result

Terminal-contact promotion now has a proof artifact contract before source
import. Future work should attach real manual citations or cached source
artifacts to `data/t4-terminal-contact-proof-source-registry.csv`; until then,
terminal district seeds, route proximity, and district membership remain
insufficient for source-backed contact evidence.

## Final Gates

- `cargo test -p route`
- `route t4-terminal-contact-proof-artifact-contract --gate`
- `route t4-terminal-contact-proof-source-registry --gate`
- `route t4-terminal-contact-district-proof-import --gate`
- `route t4-terminal-contact-source-plan --gate`
- `route t4-terminal-contact-evidence --gate`
- `route t4-terminal-scenario-readiness --gate`
- `route optimizer-constraint-ledger --gate`
- `route optimizer-constraint-budget --gate`
- `route tier-optimize --all-tiers --gate`
- `route optimizer-manifest --gate`
- `route release-manifest --gate`
- `scripts/check-mileposts.ps1 -SkipTests`

## Commits

| Pulse | Commit | Result |
|---|---|---|
| 01 | `5f4e787` | proof artifact contract |
| 02 | `59a6082` | proof source registry |
| 03 | `ec7e05d` | district proof import |
| 04 | `3e9f0fd` | held propagation |

