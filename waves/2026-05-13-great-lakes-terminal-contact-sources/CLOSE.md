---
wave: great-lakes-terminal-contact-sources
date_closed: 2026-05-13
status: done
source: waves/2026-05-13-t4-terminal-contact-evidence/CLOSE.md
---

# Great Lakes Terminal Contact Sources Closeout

## Decision

Close the wave as done.

The wave created governed source-acquisition surfaces for the Great Lakes /
Ohio Valley terminal-contact backlog without promoting any terminal district
seed, candidate district, proximity claim, or route row into contact proof.

## Artifacts Landed

| Artifact | Rows | Status | Meaning |
|---|---:|---|---|
| `data/t4-terminal-contact-source-plan.csv` | 33 | held-known | One source-acquisition row for each Great Lakes `source-needed` terminal-contact row. |
| `data/t4-terminal-contact-source-catalog.csv` | 8 | held-known | District-level public terminal-contact proof family rows. |
| `data/t4-terminal-contact-proof-docket.csv` | 33 | held-known | One route-level proof task per Great Lakes contact row. |
| `data/t4-terminal-contact-evidence.csv` | 69 | held-known | Full T4 terminal-contact queue; all rows remain `source-needed`. |
| `data/t4-terminal-scenario-readiness.csv` | 1 | held-known | Held clear row; no source-backed terminal-contact row exists. |

## Great Lakes Residual Backlog

| Candidate district | Route proof tasks | Source family | Status |
|---|---:|---|---|
| Chicago Intermodal Complex | 4 | `public-terminal-contact-proof` | `source-needed` |
| Columbus South | 8 | `public-terminal-contact-proof` | `source-needed` |
| Detroit Livernois | 5 | `public-terminal-contact-proof` | `source-needed` |
| Indianapolis Avon | 3 | `public-terminal-contact-proof` | `source-needed` |
| Minneapolis Twin Cities | 1 | `public-terminal-contact-proof` | `source-needed` |
| New York Fresh Pond | 6 | `public-terminal-contact-proof` | `source-needed` |
| Philadelphia Frankford | 3 | `public-terminal-contact-proof` | `source-needed` |
| St. Louis Gateway | 3 | `public-terminal-contact-proof` | `source-needed` |

Every task requires a separate route-to-terminal contact statement, source title,
source URL or cached artifact, capture date, and selected higher-tier attachment
before any row can move beyond `source-needed`.

## Blocker Reconciliation

The wave intentionally did not reduce terminal-contact blockers.

| Surface | Count |
|---|---:|
| Optimizer constraint ledger rows | 142 |
| Optimizer constraint budget rows | 137 |
| Claim blockers | 117 |
| `terminal_access_evidence_gap` rows | 69 |
| Source-backed terminal-contact rows | 0 |
| Scenario-ready terminal-contact rows | 0 |

The new source-plan, source-catalog, and proof-docket artifacts make the
blockers executable; they do not satisfy the blockers.

## Manifest And Release State

`data/tier-optimizer-runs.csv` now records 52 optimizer stages. The source-plan,
source-catalog, proof-docket, and scenario-readiness stages are `held-known`; the
remaining stages are `pass`.

`data/release-manifest.csv` now records the three Great Lakes source artifacts
as `held` / `held_public`. They are publishable as held planning ledgers, not as
validated terminal-contact evidence.

## Final Gates

Final close gates:

- `cargo test -p route`
- `route t4-terminal-contact-source-plan --gate`
- `route optimizer-constraint-ledger --gate`
- `route optimizer-constraint-budget --gate`
- `route tier-optimize --all-tiers --gate`
- `route optimizer-manifest --gate`
- `route release-manifest --gate`
- `scripts/check-mileposts.ps1 -SkipTests`

## Next Backlog

The next wave should either acquire/cache traceable route-to-terminal contact
proof for one district slice or extend the source-fetch/cache policy with a safe
terminal-contact source command. Until then, no Great Lakes terminal-contact row
is source-backed or scenario-ready.
