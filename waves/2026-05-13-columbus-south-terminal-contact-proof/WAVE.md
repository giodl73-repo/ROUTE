---
wave: columbus-south-terminal-contact-proof
date_open: 2026-05-13
status: active
source: waves/2026-05-13-great-lakes-terminal-contact-sources/CLOSE.md
---

# Columbus South Terminal Contact Proof

## Mission

Run the first terminal-contact proof pilot against the largest Great Lakes
district slice: the eight Columbus South route proof tasks. The wave should
attempt to turn route-level source-needed proof tasks into traceable evidence
decisions without laundering terminal district seeds into contact proof.

## Opening Rule

No Columbus South row can move beyond `source-needed` unless a separate source
names the route, Columbus South terminal district, route-to-terminal contact
statement, source title, source URL or cached artifact, capture date, and
selected higher-tier attachment. Source access failures are recorded as blockers,
not hidden by demotion or deletion.

## Inputs Inherited

| Input | Source |
|---|---|
| Great Lakes source closeout | `waves/2026-05-13-great-lakes-terminal-contact-sources/CLOSE.md` |
| Source-acquisition plan | `data/t4-terminal-contact-source-plan.csv` |
| District source catalog | `data/t4-terminal-contact-source-catalog.csv` |
| Route proof docket | `data/t4-terminal-contact-proof-docket.csv` |
| Contact evidence queue | `data/t4-terminal-contact-evidence.csv` |
| Scenario readiness docket | `data/t4-terminal-scenario-readiness.csv` |
| Source fetch policy | `docs/source-fetch-cache-policy.md`; `data/source-fetch-policy.csv` |
| T3/T4 doctrine | `docs/t3-t4-access-optimization.md` |
| Optimizer/release manifests | `data/tier-optimizer-runs.csv`; `data/release-manifest.csv` |

## Current Backlog Shape

At wave open, the Columbus South slice contains eight `source-needed` proof
tasks:

| Route | Queue id | Status |
|---|---|---|
| I-271 | `T4CONTACT-T3GREATLAKES-I271` | `source-needed` |
| I-279 | `T4CONTACT-T3GREATLAKES-I279` | `source-needed` |
| I-471 | `T4CONTACT-T3GREATLAKES-I471` | `source-needed` |
| US22 | `T4CONTACT-T3GREATLAKES-US22` | `source-needed` |
| US224 | `T4CONTACT-T3GREATLAKES-US224` | `source-needed` |
| US250 | `T4CONTACT-T3GREATLAKES-US250` | `source-needed` |
| US35 | `T4CONTACT-T3GREATLAKES-US35` | `source-needed` |
| US74 | `T4CONTACT-T3GREATLAKES-US74` | `source-needed` |

## Source Decision

Prefer cached/manual source artifacts first. Do not add a live fetcher unless the
command uses temp-then-replace writes, preserves existing cache scope on failure,
and has a gate matching `docs/source-fetch-cache-policy.md`.

## Pulse Status

| Pulse | Status | Evidence |
|---|---|---|
| 01 - Columbus proof intake | done | `data/t4-terminal-columbus-proof-intake.csv`; `route t4-terminal-columbus-proof-intake --gate`; eight rows held source-needed |
| 02 - Source access contract | done | `data/t4-terminal-columbus-source-access.csv`; live fetch unsupported, manual/cached source-needed blockers recorded |
| 03 - Route proof attempt | done | `data/t4-terminal-columbus-proof-attempts.csv`; eight blocked attempts, no source-backed promotion |
| 04 - Evidence and scenario propagation | planned | promote only source-backed rows; keep scenario docket held otherwise |
| 05 - Wave close | planned | reconcile proof decisions, blockers, manifests, and gates |

## Done Criteria

- The eight Columbus South route proof tasks have explicit proof-attempt rows.
- Every proof attempt names route, terminal district, source family, source
  artifact or blocker, selected higher-tier attachment status, and next artifact.
- Terminal district seed sources remain separate from contact proof sources.
- Any promotion to `source-backed` or `scenario-ready` is backed by a traceable
  non-seed proof artifact and propagated through contact evidence, scenario
  readiness, optimizer manifest, and release manifest.
- If no source-backed proof exists, the wave closes with all eight rows still
  visible as source-needed/blocker rows.
- `cargo test -p route`, relevant `route ... --gate` commands, `route
  optimizer-manifest --gate`, `route release-manifest --gate`, and
  `scripts/check-mileposts.ps1 -SkipTests` pass before close.

## Non-Goals

- Do not process non-Columbus South Great Lakes rows.
- Do not create a scenario from source-needed rows.
- Do not fetch live terminal sources unless the cache policy and command are
  implemented first.
- Do not reduce blocker counts by deleting unresolved terminal-contact claims.
