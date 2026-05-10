# Des Moines Diamond Blind Playtest 003

Status: simulated blind-player pass  
Scenario version: G0 v0.2  
Rubric version: Interstate Tycoon scenario rubric v0.1  
Reference packet: `docs/game/des-moines-diamond-playtest.md`

Player persona: policy/evidence-minded reviewer, medium highway knowledge, low strategy-game optimization.

## Pre-Play Prompt

| Question | Answer |
|---|---|
| What do you think the main problem is? | "The claim may be under-evidenced. I want to know what is observed." |
| Which project looks most obviously useful? | "Source request, because publication seems blocked." |
| What do you expect to happen during a closure? | "The system will show whether the model is credible." |

## Season Log

| Season | Event | Projects started | Projects completed | Budget | Crews | Political | Patience | Ops | Evidence | Throughput | Recovery | SLA | Publication gate | Player note |
|---:|---|---|---|---:|---:|---:|---:|---:|---:|---|---|---|---|---|
| 1 | Full interchange-zone closure | Source request; Intelligent routing | none | 9 | 2 | 5 | 6 | 4 | 3 | below target | bounded heuristic | weak | locked: empirical closure evidence missing | "I know what data is missing, but the transfer still failed." |
| 2 | Source challenge | Validated evidence rejected: no matching observed artifact | Intelligent routing; Source request | 9 | 3 | 5 | 6 | 4 | 3 | below target | bounded heuristic | weak | locked: empirical closure evidence missing | "The game is honest, but I have not improved the network." |
| 3 | Political lane-mile pressure | Diamond connector package | none | 4 | 1 | 4 | 6 | 4 | 3 | below target | bounded heuristic | weak | locked: empirical closure evidence missing | "Now I see evidence is not a substitute for redundancy." |
| 4 | EV/rest queue | Work-zone sequencing rejected: insufficient crews | none | 4 | 1 | 4 | 6 | 4 | 3 | below target | bounded heuristic | weak | locked: empirical closure evidence missing | "I have to wait for the connector." |
| 5 | Night work-zone closure | none | Diamond connector package | 4 | 3 | 4 | 5 | 4 | 3 | 0.962 heuristic | 0.9 heuristic | bounded heuristic | locked: empirical closure evidence missing | "Operational solution landed after I stopped chasing publication first." |

Stopped after season 5 because the tutorial end condition was met.

## Final Score

| Dimension | Max | Score | Evidence |
|---|---:|---:|---|
| Throughput retention | 25 | 25 | Heuristic scenario retention |
| Recovery | 20 | 20 | 0.9h heuristic recovery proxy |
| SLA | 15 | 10 | Bounded heuristic |
| Budget discipline | 10 | 10 | Budget non-negative |
| Public support | 10 | 10 | Political capital and patience non-negative |
| Evidence honesty | 20 | 20 | Publication gate stayed locked |
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
| What actually failed in the first closure? | "The transfer had no independent backup path." |
| Why did widening help or fail? | "I did not test widening; the game showed evidence alone also does not fix topology." |
| What did the diamond connector package change? | "It made the backup path real instead of just documented." |
| What evidence would make this publication-grade? | "Observed closure rates, durations, and PTI validation attached to this node." |

Aha status: Landed.

## Surprise Log

| Surprise | Type | Severity | Evidence | Amendment candidate? |
|---|---|---|---|---|
| Evidence-first player learned a complementary lesson: proof work cannot substitute for physical redundancy | learning | medium | Seasons 1-3 | Candidate: preserve evidence-first path as valid partial route |
| Insufficient crew rejection was clear and useful | rules | low | Season 4 | No |
| Source request before connector made publication lock less frustrating | evidence | low | Seasons 1-2 | No |

## G0 Promotion Checklist

| Gate | Pass? | Evidence |
|---|---|---|
| Player completed a run from paper rules | yes | Tutorial ended by v0.2 condition |
| Forced tutorial closure produced the intended aha | yes | Aha check |
| Player explained independent transfer paths before `k-connectivity` was named | yes | Aha check |
| Player made at least one meaningful tradeoff | yes | Evidence-first route delayed construction |
| Evidence labels stayed visible in scoring | yes | Publication table |
| Operational win/loss separated from publication gate | yes | Final score |
| Surprise log was filled | yes | Three surprises |
| At least one amendment candidate was accepted or explicitly declined | pending | Needs panel/amendment review |

Promotion decision: Hold at G0-B until panel review; simulated pass supports G0-C readiness.

