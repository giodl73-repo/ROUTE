# Des Moines Diamond Playtest Synthesis

Scope: simulated G0 playtests 001-003  
Current scenario version: G0 v0.2  
Decision: hold at G0-B, panel-ready for G0-C review

## Runs

| Run | Version | Persona | Result | Aha | Publication |
|---|---|---|---|---|---|
| 001 | v0.1 | first-time strategy player | Operational win, budget issue | Landed | Locked |
| 002 | v0.2 | optimization-minded board gamer | Operational win | Landed | Locked |
| 003 | v0.2 | policy/evidence-minded reviewer | Operational win | Landed | Locked |

## Repeated Findings

| Finding | Runs | Status |
|---|---|---|
| Players initially frame the problem as capacity or evidence, then learn topology | 001, 002, 003 | Core loop working |
| Publication lock is accepted when evidence labels are explicit | 001, 002, 003 | Keep |
| Source work must not instantly unlock publication | 001, 002, 003 | v0.2 fixed |
| Tutorial can end before season 10 once the aha lands | 001, 002, 003 | v0.2 fixed |

## Amendment Effects

| Amendment | Evidence after v0.2 |
|---|---|
| Budget floor | No v0.2 run overspent; rejected actions were clear |
| Language ladder | Both v0.2 runs explained independent transfer paths without needing `k-connectivity` first |
| Evidence split | Both v0.2 runs understood source request versus validated evidence |
| Early tutorial end | Both v0.2 runs ended cleanly at season 4 or 5 |

## New Candidate Findings

| Candidate | Source | Recommendation |
|---|---|---|
| Optimization-minded players may identify the connector before the forced closure | Run 002 | Hold; not a flaw unless the aha disappears |
| Evidence-first path is a valid learning route | Run 003 | Accept as scenario branch, not a rewrite |
| Crew rejection clarity matters | Run 003 | Keep explicit rejection messages in CLI |

## Promotion Readiness

| Gate | Status | Evidence |
|---|---|---|
| Paper playable | Pass | Runs 001-003 |
| Aha proven | Simulated pass | Aha checks in all runs |
| Evidence honesty | Pass | Publication stayed locked |
| Meaningful tradeoff | Pass | Capacity-first, connector-first, and evidence-first routes all diverged |
| Human blind test | Missing | Required before confident G0-C promotion |
| Panel pass | Missing | Next step |

## Decision

Hold at G0-B, but mark as panel-ready. The next review should focus on:

- Whether simulated playtests are enough to begin G1 CLI implementation.
- Whether budget/crew rejection rules are clear enough for code.
- Whether evidence-first play should become an explicit tutorial branch.
- Whether G0-C requires a human blind playtest or can accept simulated evidence for now.

