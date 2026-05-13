---
wave: terminal-contact-source-acquisition-spine
date_open: 2026-05-13
status: done
source: waves/2026-05-13-columbus-south-terminal-contact-proof/CLOSE.md
close: waves/2026-05-13-terminal-contact-source-acquisition-spine/CLOSE.md
---

# Terminal Contact Source Acquisition Spine

## Mission

Build the governed source-acquisition loop that can attach manual or cached
route-to-terminal contact proof artifacts to Great Lakes terminal-contact rows
without treating terminal district seed membership as proof.

## Opening Rule

No row can move beyond `source-needed` unless a proof artifact names route,
terminal district, route-to-terminal contact statement, source title, source URL
or cache artifact, capture date, selected higher-tier attachment, and validation
decision. Missing or inaccessible sources remain blockers, not gate failures.

## Inputs Inherited

| Input | Source |
|---|---|
| Columbus pilot close | `waves/2026-05-13-columbus-south-terminal-contact-proof/CLOSE.md` |
| Great Lakes source close | `waves/2026-05-13-great-lakes-terminal-contact-sources/CLOSE.md` |
| Source policy | `docs/source-fetch-cache-policy.md`; `data/source-fetch-policy.csv` |
| Terminal contact doctrine | `docs/t3-t4-access-optimization.md` |
| District source catalog | `data/t4-terminal-contact-source-catalog.csv` |
| Route proof docket | `data/t4-terminal-contact-proof-docket.csv` |
| Columbus proof attempts | `data/t4-terminal-columbus-proof-attempts.csv` |
| Optimizer/release manifests | `data/tier-optimizer-runs.csv`; `data/release-manifest.csv` |

## Pulse Status

| Pulse | Status | Evidence |
|---|---|---|
| 01 - Proof artifact contract | done | `data/t4-terminal-contact-proof-artifact-contract.csv`; gate requires non-seed manual/cache proof fields |
| 02 - Source registry intake | done | `data/t4-terminal-contact-proof-source-registry.csv`; 33 source-needed rows, seed proof rejected |
| 03 - District proof import | done | `data/t4-terminal-contact-district-proof-import.csv`; largest unresolved district remains source-needed |
| 04 - Evidence propagation | done | no accepted proof rows; contact evidence and scenario readiness remain held |
| 05 - Wave close | done | `CLOSE.md`; no accepted proof rows, residual source-needed blockers preserved |

## Done Criteria

- A gateable proof artifact contract exists for manual/cached route-to-terminal
  contact proof.
- Accepted proof rows require non-seed source artifacts and selected higher-tier
  attachment status.
- Rejected or inaccessible proof attempts stay visible as blockers.
- Any accepted rows propagate through contact evidence, scenario readiness,
  optimizer manifest, and release manifest.
- If no accepted proof exists, the wave closes with residual source-needed rows
  explicitly preserved.
- `cargo test -p route`, relevant `route ... --gate` commands,
  `route optimizer-manifest --gate`, `route release-manifest --gate`, and
  `scripts/check-mileposts.ps1 -SkipTests` pass before close.

## Non-Goals

- Do not scrape or fetch live terminal-contact pages without a policy-compliant
  safe fetch command.
- Do not infer route contact from route proximity, district membership, terminal
  names, or `data/intermodal_terminals.csv`.
- Do not process every Great Lakes district before the proof artifact contract
  has passed on one district slice.

