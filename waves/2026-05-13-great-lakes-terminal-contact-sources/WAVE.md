---
wave: great-lakes-terminal-contact-sources
date_open: 2026-05-13
status: active
source: t4-terminal-contact-evidence-closeout
---

# Great Lakes Terminal Contact Sources

## Mission

Turn the 33 Great Lakes / Ohio Valley `source-needed` terminal-contact rows into
a governed source-acquisition and proof docket. The wave should identify what
source evidence is needed for each route-to-terminal contact without promoting
terminal district seeds, proximity, or candidate districts into source-backed
claims.

## Opening Rule

A candidate terminal district is not route-to-terminal contact proof. A row can
move beyond `source-needed` only when a separate source names the route, terminal
district, contact basis, selected higher-tier attachment, and proof status.

## Inputs Inherited

| Input | Source |
|---|---|
| Terminal contact closeout | `waves/2026-05-13-t4-terminal-contact-evidence/CLOSE.md` |
| T4 contact queue | `data/t4-terminal-contact-evidence.csv` |
| Scenario readiness docket | `data/t4-terminal-scenario-readiness.csv` |
| Terminal district seed list | `data/intermodal_terminals.csv` |
| T3/T4 access doctrine | `docs/t3-t4-access-optimization.md` |
| Source fetch policy | `docs/source-fetch-cache-policy.md`; `data/source-fetch-policy.csv` |
| Optimizer manifests | `data/tier-optimizer-runs.csv`; `data/release-manifest.csv` |

## Current Backlog Shape

At wave open, the Great Lakes slice contains 33 `source-needed` rows:

| Candidate district | Rows |
|---|---:|
| Chicago Intermodal Complex | 4 |
| Columbus South | 8 |
| Detroit Livernois | 5 |
| Indianapolis Avon | 3 |
| Minneapolis Twin Cities | 1 |
| New York Fresh Pond | 6 |
| Philadelphia Frankford | 3 |
| St. Louis Gateway | 3 |

## Source Decision

Do not fetch live terminal sources unless a safe cache policy and command exist.
The first outcome is a source plan and proof docket: what evidence would count,
which source family owns it, and which rows remain blocked. A later wave can add
fetchers only after the policy and cache contract are explicit.

## Pulse Status

| Pulse | Status | Evidence |
|---|---|---|
| 01 - Source plan contract | planned | define source-plan artifact and gate |
| 02 - District source catalog | planned | map eight Great Lakes terminal districts to source families |
| 03 - Route contact proof docket | planned | create route-level source tasks for 33 rows |
| 04 - Queue and manifest propagation | planned | keep unresolved rows visible in optimizer/release surfaces |
| 05 - Wave close | planned | reconcile counts, residual holds, and gates |

## Done Criteria

- Every Great Lakes `source-needed` row has a source-acquisition task with route,
  terminal district, required proof field, source family, and next artifact.
- Terminal district seed sources remain separate from contact proof sources.
- No row is promoted to `source-backed` or `scenario-ready` without a traceable
  contact proof artifact.
- Optimizer and release manifests carry any new source-plan/proof artifacts with
  held status where claims remain unresolved.
- `cargo test -p route`, relevant `route ... --gate` commands, `route
  optimizer-manifest --gate`, `route release-manifest --gate`, and
  `scripts/check-mileposts.ps1 -SkipTests` pass before close.

## Non-Goals

- Do not scrape or fetch live sources in this wave unless a safe source command
  and cache policy are introduced first.
- Do not process non-Great-Lakes terminal rows.
- Do not create a scenario from a source-needed row.
- Do not reduce blocker counts by deleting unresolved terminal-contact claims.
