# Des Moines Diamond Browser Playtest

Status: ready for human blind run  
Scenario version: G2-A static browser prototype  
Rubric version: Interstate Tycoon scenario rubric v0.1  
Prototype: `docs/game/browser/des-moines-diamond.html`  
Reference packet: `docs/game/des-moines-diamond-playtest.md`

Use this packet for the first human browser pass. The facilitator should not explain the intended lesson before the aha check. The player may think aloud, but the facilitator should record exact phrases when possible.

## Setup

| Field | Value |
|---|---|
| Date |  |
| Facilitator |  |
| Player / persona |  |
| Familiarity with highway engineering | none / low / medium / high |
| Familiarity with strategy games | none / low / medium / high |
| Device / viewport | desktop / tablet / phone |
| Browser |  |
| Prototype commit |  |
| Notes path |  |

## Blind Prompt

Ask the player to open the prototype and look at the first screen for up to two minutes without clicking.

| Question | Answer |
|---|---|
| What do you think this scenario is asking you to fix? |  |
| Which part of the screen did your eyes go to first? |  |
| Which project looks most useful? |  |
| What do you expect the closure playback to show? |  |
| What do you think the publication lock means? |  |

## Task Script

Record whether the player completes each task without help. If help is needed, write the exact cue given.

| Step | Player task | Expected evidence | Completed without help? | Notes |
|---:|---|---|---|---|
| 1 | Identify the fragile I-35/I-80 transfer core | Player points to or names the fragile core / connector zone |  |  |
| 2 | Explain the difference between the core and the bypass | Player distinguishes local bypass from independent transfer paths |  |  |
| 3 | Use Before / After playback | Incident value changes from 86,671 to 83,423 and back |  |  |
| 4 | Select Source request | Project selection visibly changes |  |  |
| 5 | Advance the source challenge season | Season becomes 4; budget becomes 6; evidence becomes 3 |  |  |
| 6 | Read the event log | Player sees publication remains locked |  |  |
| 7 | Read the after-action panel | Player separates 100/100 operational score from locked publication status |  |  |
| 8 | Download the session CSV | Download filename is `des-moines-diamond-session.csv` |  |  |
| 9 | Explain what would unlock publication | Player asks for observed closure evidence / direct PTI or NPMRDS validation |  |  |

## Observation Log

| Moment | What happened | UI / rules / evidence / copy / scoring | Severity | Amendment candidate? |
|---|---|---|---|---|
|  |  |  | low / medium / high |  |
|  |  |  | low / medium / high |  |
|  |  |  | low / medium / high |  |

## Export Check

Paste the final session row or attach the downloaded CSV path.

| Check | Value |
|---|---|
| CSV downloaded? |  |
| Final season |  |
| Accepted projects |  |
| Budget remaining |  |
| Evidence confidence |  |
| Throughput retention |  |
| Recovery hours |  |
| SLA status |  |
| Publication gate |  |

## Aha Check

Ask after the tasks, before explaining the intended lesson.

| Question | Player answer |
|---|---|
| What actually failed during the closure? |  |
| Why is adding a transfer path different from widening? |  |
| What did the connector package change? |  |
| Why can the operational score be high while publication stays locked? |  |
| What evidence would make the claim publication-grade? |  |

Aha status:

| Status | Definition | Check |
|---|---|---|
| Landed | Player says topology / independent transfer paths and separates operational score from evidence gate |  |
| Partial | Player understands either topology or evidence gate, but not both |  |
| Missed | Player reads the scenario as generic congestion or treats score as publication proof |  |

## Browser Promotion Checklist

| Gate | Pass? | Evidence |
|---|---|---|
| Player understood the map without facilitator explanation |  |  |
| Player used playback controls correctly |  |  |
| Player advanced a season and saw state change |  |  |
| Player found event log and session log |  |  |
| Player downloaded CSV or understood the export |  |  |
| Player separated operational score from publication status |  |  |
| Player stated the intended T1/T1 lesson |  |  |
| At least one surprise was logged or explicitly none found |  |  |

Promotion decision:

| Decision | Check | Note |
|---|---|---|
| Browser G2-A pass |  | Prototype supports playable browser demonstration |
| Hold browser G2-A |  | Interaction, comprehension, or export trust failed |
| Promote G0-C evidence |  | Human blind run proves the aha or owner accepts simulated evidence |
