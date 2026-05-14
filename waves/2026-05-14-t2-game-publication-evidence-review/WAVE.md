---
wave: t2-game-publication-evidence-review
date_open: 2026-05-14
status: done
source: data/optimizer-claim-review.csv
---

# T2 Game Publication Evidence Review

## Mission

Expand the residual T2 `game_ops_publication_readiness` claim-blocker family
into scenario-level evidence review rows before any policy, evidence
acceptance, blocker relief, or ledger replay.

## Opening Rule

This wave may classify scenario evidence holds and name the next evidence
policy artifact, but it must not reduce blockers or promote game, publication,
or upgrade claims.

## Inputs Inherited

| Input | Source |
|---|---|
| Optimizer claim-review docket | `data/optimizer-claim-review.csv` |
| T2 scenario hooks | `data/game/t2-scenario-hooks.csv` |

## Pulse Status

| Pulse | Status | Evidence |
|---|---|---|
| 01 - Wave card and scenario scope | done | this wave card and pulse plan |
| 02 - Game publication review artifact | done | `data/t2-game-publication-evidence-review.csv`; CLI regression test |
| 03 - Doctrine close | done | `CLOSE.md`; `panels/review/review.md`; manifest and index updates |

## Done Criteria

- The three backlog representative scenarios are represented exactly once.
- Review rows preserve all three game publication readiness claim blockers.
- Every row points to the game publication evidence policy artifact needed
  before relief.
- Final gates pass before close.

## Non-Goals

- Do not author the evidence policy in this wave.
- Do not change scenario hooks, game overlays, or optimizer constraint ledger.
- Do not reduce residual claim blockers.

