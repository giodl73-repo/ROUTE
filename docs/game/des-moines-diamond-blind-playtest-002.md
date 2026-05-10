# Des Moines Diamond Blind Playtest 002

Status: simulated blind-player pass  
Scenario version: G0 v0.2  
Rubric version: Interstate Tycoon scenario rubric v0.1  
Reference packet: `docs/game/des-moines-diamond-playtest.md`

Player persona: optimization-minded board gamer, low highway-engineering knowledge, high comfort with resource tradeoffs.

## Pre-Play Prompt

| Question | Answer |
|---|---|
| What do you think the main problem is? | "The closure probably punishes bad project sequencing. I should buy the highest-leverage permanent fix." |
| Which project looks most obviously useful? | "Diamond connector package, because it sounds like the unique scenario mechanic." |
| What do you expect to happen during a closure? | "The game will test whether I built enough redundancy." |

## Season Log

| Season | Event | Projects started | Projects completed | Budget | Crews | Political | Patience | Ops | Evidence | Throughput | Recovery | SLA | Publication gate | Player note |
|---:|---|---|---|---:|---:|---:|---:|---:|---:|---|---|---|---|---|
| 1 | Full interchange-zone closure | Diamond connector package; Work-zone sequencing | none | 6 | 0 | 5 | 6 | 4 | 2 | below target | bounded heuristic | weak | locked: empirical closure evidence missing | "I can see the transfer core is the problem immediately." |
| 2 | Political lane-mile pressure | Source request | none | 5 | 0 | 4 | 6 | 4 | 3 | below target | bounded heuristic | weak | locked: empirical closure evidence missing | "Source request tells me what is missing, but it does not solve publication." |
| 3 | Night work-zone closure | none | Work-zone sequencing | 5 | 1 | 4 | 6 | 4 | 3 | below target | bounded heuristic | weak | locked: empirical closure evidence missing | "Sequencing paid off by avoiding patience loss." |
| 4 | Source challenge | Validated evidence rejected: no matching observed artifact | Diamond connector package | 5 | 3 | 4 | 6 | 4 | 3 | 0.962 heuristic | 0.9 heuristic | bounded heuristic | locked: empirical closure evidence missing | "The game will not let me buy my way out of missing data." |

Stopped after season 4 because the tutorial end condition was met.

## Final Score

| Dimension | Max | Score | Evidence |
|---|---:|---:|---|
| Throughput retention | 25 | 25 | Heuristic scenario retention |
| Recovery | 20 | 20 | 0.9h heuristic recovery proxy |
| SLA | 15 | 10 | Bounded heuristic |
| Budget discipline | 10 | 10 | Budget non-negative |
| Public support | 10 | 10 | Political capital and patience non-negative |
| Evidence honesty | 20 | 20 | Validated evidence correctly blocked |
| Total | 100 | 95 | Operational win, publication locked |

Publication gate:

| Gate | Pass / lock | Note |
|---|---|---|
| Diamond analyzer recognizes Des Moines node | pass | `route diamond I35xI80` |
| No headline claim uses `source_needed` | partial | Empirical history missing |
| Observed versus modeled evidence is cited | partial | Modeled labels visible |
| Final publication result | locked | Operational win only |

## Aha Check

| Question | Player answer |
|---|---|
| What actually failed in the first closure? | "The core transfer had too few independent paths." |
| Why did widening help or fail? | "I ignored it because capacity was not the scarce thing." |
| What did the diamond connector package change? | "It created independent transfer paths." |
| What evidence would make this publication-grade? | "Observed closure probability/duration and validated PTI." |

Aha status: Landed.

## Surprise Log

| Surprise | Type | Severity | Evidence | Amendment candidate? |
|---|---|---|---|---|
| Optimization-minded player identified the unique mechanic before the forced failure | learning | medium | Pre-play prompt | Hold: may reduce aha for experienced players |
| Validated evidence rejection worked better than the old evidence acquisition card | evidence | low | Season 4 | No |
| Tutorial end at season 4 felt clean | rules | low | End condition | No |

## G0 Promotion Checklist

| Gate | Pass? | Evidence |
|---|---|---|
| Player completed a run from paper rules | yes | Tutorial ended by v0.2 condition |
| Forced tutorial closure produced the intended aha | yes | Aha check |
| Player explained independent transfer paths before `k-connectivity` was named | yes | Aha check |
| Player made at least one meaningful tradeoff | yes | Connector plus source request under budget |
| Evidence labels stayed visible in scoring | yes | Publication table |
| Operational win/loss separated from publication gate | yes | Final score |
| Surprise log was filled | yes | Three surprises |
| At least one amendment candidate was accepted or explicitly declined | pending | Needs panel/amendment review |

Promotion decision: Hold at G0-B until panel review; simulated pass supports G0-C readiness.

