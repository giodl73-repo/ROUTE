# Des Moines Diamond Blind Playtest 001

Status: simulated blind-player pass  
Scenario version: G0 v0.1  
Rubric version: Interstate Tycoon scenario rubric v0.1  
Reference packet: `docs/game/des-moines-diamond-playtest.md`

This is a simulated blind pass, not a human playtest. It uses the playtest packet constraints and intentionally avoids reading the narrated playthrough until after the aha check.

## Playtest Header

| Field | Value |
|---|---|
| Date | 2026-05-10 |
| Facilitator | Codex |
| Player / persona | first-time strategy player, low highway-engineering knowledge |
| Familiarity with highway engineering | low |
| Familiarity with strategy games | medium |
| Scenario version | G0 v0.1 |
| Rules used | `docs/game/des-moines-diamond-g0.md` |
| Notes path | this file |

## Pre-Play Prompt

| Question | Answer |
|---|---|
| What do you think the main problem is? | "The interchange is overloaded; widening or routing should keep traffic moving." |
| Which project looks most obviously useful? | "General-purpose widening, because it sounds like more capacity." |
| What do you expect to happen during a closure? | "Traffic slows, but smart routing can spread it out." |

## Season Log

| Season | Event | Projects started | Projects completed | Budget | Crews | Political | Patience | Ops | Evidence | Throughput | Recovery | SLA | Publication gate | Player note |
|---:|---|---|---|---:|---:|---:|---:|---:|---:|---|---|---|---|---|
| 1 | Full interchange-zone closure | General-purpose widening; Intelligent routing | none | 6 | 0 | 5 | 6 | 4 | 2 | below target | bounded heuristic | weak | locked: empirical closure evidence missing | "I still do not understand why widening did not fix the warning." |
| 2 | Political lane-mile pressure | Diamond connector package | Intelligent routing | 1 | 1 | 4 | 6 | 4 | 2 | below target | bounded heuristic | weak | locked: empirical closure evidence missing | "The connector seems to solve a different problem than widening." |
| 3 | Night work-zone closure | Work-zone sequencing | none | 0 | 0 | 4 | 5 | 4 | 2 | below target | bounded heuristic | weak | locked: empirical closure evidence missing | "I am spending everything to make the connector work." |
| 4 | Source challenge | Evidence acquisition | Diamond connector package; Work-zone sequencing | -1 | 3 | 4 | 5 | 4 | 3 | 0.962 heuristic | 0.9 heuristic | bounded heuristic | locked: empirical closure evidence missing | "Operationally this works, but the source challenge says I still cannot publish." |
| 5 | Relay hub surge | none | none | -1 | 3 | 4 | 5 | 3 | 3 | 0.962 heuristic | 0.9 heuristic | bounded heuristic | locked: empirical closure evidence missing | "Operations can become the constraint even when the roads work." |

Stopped after season 5 because the intended aha had landed and the remaining turns would mostly repeat resource pressure.

## Final Score

| Dimension | Max | Score | Evidence |
|---|---:|---:|---|
| Throughput retention | 25 | 25 | Connector completion uses heuristic scenario retention |
| Recovery | 20 | 20 | 0.9h heuristic recovery proxy |
| SLA | 15 | 10 | Bounded heuristic, not direct PTI validation |
| Budget discipline | 10 | 0 | Budget dropped below zero |
| Public support | 10 | 10 | Political capital and patience remained non-negative |
| Evidence honesty | 20 | 20 | Publication gate stayed locked |
| Total | 100 | 85 | Operational win, publication locked |

Result band: operational win.

Publication gate:

| Gate | Pass / lock | Note |
|---|---|---|
| Diamond analyzer recognizes Des Moines node | pass | `route diamond I35xI80` recognizes curated anchor |
| No headline claim uses `source_needed` | partial | Empirical closure history still missing |
| Observed versus modeled evidence is cited | partial | Modeled/heuristic labels visible |
| Final publication result | locked | Operational win only |

## Aha Check

| Question | Player answer |
|---|---|
| What actually failed in the first closure? | "The interchange transfer failed, not the whole route." |
| Why did widening help or fail? | "It added capacity but not another independent way through the transfer." |
| What did the diamond connector package change? | "It changed the topology by adding another transfer path." |
| What evidence would make this publication-grade? | "Observed closure frequency/duration and direct PTI validation." |

Aha status: Landed.

## Surprise Log

| Surprise | Type | Severity | Evidence | Amendment candidate? |
|---|---|---|---|---|
| Player overspent because the rules did not clearly say whether negative budget is illegal or just scored down | rules | high | Seasons 3-4 | Yes |
| First closure was fair, but the phrase "k-connectivity" remained opaque until after connector completion | copy | medium | Aha check | Yes |
| Evidence acquisition felt like a required tax rather than a strategic project | balance | medium | Season 4 | Yes |
| Stopping after season 5 felt natural once the aha landed | rules | low | Facilitator decision | Yes: consider short tutorial win condition |

## Facilitator Notes

| Prompt | Notes |
|---|---|
| Where did the player slow down? | First project choice and publication gate explanation |
| Which rule required clarification? | Whether negative budget is immediate failure |
| Which card was ignored or overvalued? | General-purpose widening was overvalued early, as intended |
| Did the forced closure feel fair in retrospect? | Yes; warning signs were visible after evidence drawer was read |
| Did evidence labels change player behavior? | Yes, but evidence acquisition needs a clearer payoff |
| Did the publication lock make sense? | Yes after the after-action explanation |

## G0 Promotion Checklist

| Gate | Pass? | Evidence |
|---|---|---|
| Player completed a run from paper rules | partial | Stopped at season 5 after aha landed |
| Forced tutorial closure produced the intended aha | yes | Aha check |
| Player made at least one meaningful tradeoff | yes | Widening vs connector; evidence vs budget |
| Evidence labels stayed visible in scoring | yes | Publication gate table |
| Operational win/loss separated from publication gate | yes | Final score |
| Surprise log was filled | yes | Four surprises |
| At least one amendment candidate was accepted or explicitly declined | pending | Needs amendment log |

Promotion decision:

| Decision | Check | Note |
|---|---|---|
| Hold at G0-B | yes | Paper playable and aha landed in simulation, but not a full human/blind run |
| Promote to G0-C | no | Needs full blind playtest or explicit acceptance of simulated pass |
| Rewrite | no | Core lesson works |

## Amendment Candidates

| Candidate | Proposed change |
|---|---|
| Budget floor | Budget below zero should trigger immediate fiscal crisis unless a debt rule is introduced |
| Term language | Replace first-use "k-connectivity" with "independent transfer paths"; introduce k after the aha |
| Evidence acquisition | Split into "source request" and "validated evidence" so players see why money does not instantly unlock publication |
| Tutorial length | Allow G0 tutorial to end after the aha plus after-action gate instead of always requiring 10 seasons |

