---
wave: t4-terminal-contact-evidence
date_open: 2026-05-13
status: active
source: constraint-ledger-blocker-burndown-closeout
---

# T4 Terminal Contact Evidence

## Mission

Turn the 69 zone-scoped `terminal_access_evidence_gap` blockers into a
source-backed route-to-terminal contact queue. The wave should prove, demote, or
explicitly carry T4 local-access claims without promoting any route to scenario
or publication readiness on generic terminal proximity alone.

## Opening Rule

Terminal districts are not contact proof. A T4 row becomes scenario-ready only
when an artifact names the route, terminal district, contact basis, source
status, and selected T3/T2/T1 attachment. Rows without that proof remain visible
claim blockers with a next artifact.

## Inputs Inherited

| Input | Source |
|---|---|
| Residual blocker closeout | `waves/2026-05-13-constraint-ledger-blocker-burndown/CLOSE.md` |
| Terminal/local access doctrine | `docs/t3-t4-access-optimization.md` |
| Normalized ledger doctrine | `docs/optimizer-constraint-ledger-spec.md` |
| Terminal district source seed | `data/intermodal_terminals.csv` |
| T4 local-access columns | `data/t4-terminal-access-columns.csv` |
| T3/T4 access gaps | `data/t3-t4-access-gaps.csv` |
| Constraint budget | `data/optimizer-constraint-budget.csv` |
| Release/publication policy | `data/release-manifest.csv`; `docs/release/release-checklist.md` |

## Current Backlog Shape

At wave open, `data/optimizer-constraint-budget.csv` exposes:

| Backlog slice | Current signal | First owning artifact |
|---|---:|---|
| T4 terminal evidence | 69 `terminal_access_evidence_gap` rows | `data/t4-terminal-access-columns.csv`; `data/t3-t4-access-gaps.csv` |
| Great Lakes terminal sample | 33 T4 rows | `data/intermodal_terminals.csv` |
| Mid-South terminal sample | 11 T4 rows | `data/intermodal_terminals.csv` |
| Southeast terminal sample | 12 T4 rows | `data/intermodal_terminals.csv` |
| Mountain West terminal sample | 9 T4 rows | `data/intermodal_terminals.csv` |
| Texas Border terminal sample | 4 T4 rows | `data/intermodal_terminals.csv` |

## Spec Decision

No new doctrine spec is required before Pulse 01. The existing T3/T4 access
optimization doctrine already says T4 rows need terminal, local freight
district, port, yard, warehouse, or last-mile access proof. A new data artifact
is expected, not a new spec: a terminal contact evidence queue that separates
source-seeded, source-needed, demotion, and scenario-ready rows.

## Scenario Decision

Do not run broad traffic or benefit/cost scenarios first. Run one bounded
Great Lakes contact sample before any scenario work. A scenario can only follow
after a pulse identifies at least one route-level terminal contact with source
status and a selected higher-tier attachment.

## Pulse Status

| Pulse | Status | Evidence |
|---|---|---|
| 01 - Terminal contact evidence schema | planned | create queue contract and gate |
| 02 - Great Lakes contact sample | planned | classify the 33-row largest zone sample |
| 03 - Remaining zone contact pass | planned | classify Southeast, Mid-South, Mountain West, and Texas Border rows |
| 04 - Scenario readiness docket | planned | separate scenario-ready rows from source holds |
| 05 - Ledger and manifest propagation | planned | regenerate normalized ledger, budget, optimizer manifest, and release surface |
| 06 - Wave close | planned | close after counts, residual holds, and gates agree |

## Done Criteria

- Every opening T4 terminal evidence blocker has a terminal-contact decision:
  source-backed, source-needed, demote/local-only, scenario-ready, or held-known.
- Any scenario-ready row names the route, zone, terminal district, evidence
  source, selected higher-tier attachment, and next scenario artifact.
- Source-needed rows remain claim blockers with explicit next artifacts.
- `data/optimizer-constraint-ledger.csv` and
  `data/optimizer-constraint-budget.csv` reflect the decisions without deleting
  unresolved claims.
- `cargo test -p route`, `route optimizer-constraint-ledger --gate`,
  `route optimizer-constraint-budget --gate`, `route tier-optimize --all-tiers
  --gate`, `route optimizer-manifest --gate`, `route release-manifest --gate`,
  and `scripts/check-mileposts.ps1 -SkipTests` pass.

## Non-Goals

- Do not scrape or fetch live terminal sources unless an existing safe source
  command and cache policy already cover the source.
- Do not promote terminal proximity to contact proof.
- Do not create a broad investment scenario before a route-level contact row is
  source-backed.
- Do not remove held rows from the ledger to improve blocker counts.
