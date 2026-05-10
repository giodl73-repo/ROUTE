# Des Moines Diamond Browser Playtest 001

Status: simulated browser-player pass  
Scenario version: G2-A static browser prototype  
Rubric version: Interstate Tycoon scenario rubric v0.1  
Prototype: `docs/game/browser/des-moines-diamond.html`  
Playtest packet: `docs/game/des-moines-diamond-browser-playtest.md`

This is not a human blind playtest. It records a reproducible browser baseline after the static prototype gained local season mutation, session export, and after-action scoring.

## Setup

| Field | Value |
|---|---|
| Date | 2026-05-10 |
| Facilitator | Codex |
| Player / persona | first-time browser player, low highway-engineering knowledge |
| Familiarity with highway engineering | low |
| Familiarity with strategy games | medium |
| Device / viewport | desktop 1280x820 and mobile 390x840 checks |
| Browser | Playwright Chromium |
| Prototype commit | `a5fe213` plus working verification |
| Notes path | this file |

## Blind Prompt

| Question | Answer |
|---|---|
| What do you think this scenario is asking you to fix? | "The interchange core is fragile, and the map wants me to add a transfer path rather than only widen roads." |
| Which part of the screen did your eyes go to first? | "The central warning around the I-35/I-80 crossing." |
| Which project looks most useful? | "The connector package, but source request looks needed for the publication lock." |
| What do you expect the closure playback to show? | "Before/after throughput under a closure." |
| What do you think the publication lock means? | "The operating result can win, but the public claim still lacks observed evidence." |

## Task Results

| Step | Player task | Expected evidence | Completed without help? | Notes |
|---:|---|---|---|---|
| 1 | Identify the fragile I-35/I-80 transfer core | Fragile core is visible in the SVG map | yes | Central marker reads as the failure point |
| 2 | Explain the difference between the core and the bypass | Player distinguishes local bypass from independent transfer paths | partial | The bypass label helps, but the connector lesson still depends on evidence text |
| 3 | Use Before / After playback | Incident value changes from 86,671 to 83,423 and back | yes | Playwright confirms the value changes |
| 4 | Select Source request | Project selection visibly changes | yes | Button state is clickable and deterministic |
| 5 | Advance the source challenge season | Season 4, budget 6, evidence 3 | yes | Browser mutation matches the packet |
| 6 | Read the event log | Player sees publication remains locked | yes | Log row is visible after advance |
| 7 | Read the after-action panel | 100/100 operational, publication locked | yes | Score and lock are separate DOM targets |
| 8 | Download the session CSV | Filename is `des-moines-diamond-session.csv` | yes | Playwright validates filename and row content |
| 9 | Explain what would unlock publication | Observed closure evidence / direct PTI or NPMRDS validation | yes | Footer and after-action copy both preserve this distinction |

## Export Check

| Check | Value |
|---|---|
| CSV downloaded? | yes |
| Final season | 4 |
| Accepted projects | source-request |
| Budget remaining | 6 |
| Evidence confidence | 3 |
| Throughput retention | 1.000 |
| Recovery hours | 0.9 |
| SLA status | bounded heuristic |
| Publication gate | locked: empirical closure evidence and direct PTI/NPMRDS validation missing |

## Aha Check

| Question | Player answer |
|---|---|
| What actually failed during the closure? | "The transfer core had too little independent path redundancy." |
| Why is adding a transfer path different from widening? | "Widening raises capacity on a path; a connector changes the topology under closure." |
| What did the connector package change? | "It provided independent transfer paths around the fragile core." |
| Why can the operational score be high while publication stays locked? | "The game can prove a bounded modeled result, but publication needs observed closure and validation evidence." |
| What evidence would make the claim publication-grade? | "Observed closure frequency/duration and direct PTI/NPMRDS validation attached to this interchange." |

Aha status: Landed in simulation.

## Observation Log

| Moment | What happened | UI / rules / evidence / copy / scoring | Severity | Amendment candidate? |
|---|---|---|---|---|
| Desktop opening | Board, evidence, and playback were all discoverable | UI | low | No |
| Playback | Before/After changed only the incident value; intervention stayed fixed | scoring | low | Candidate: add a small baseline/current label if human players miss it |
| After-action | Operational score and publication lock were distinct | evidence | low | No |
| Mobile opening | Scenario board stayed visible before panels | UI | low | No |

## Browser Promotion Checklist

| Gate | Pass? | Evidence |
|---|---|---|
| Player understood the map without facilitator explanation | simulated yes | Central fragile core and route labels visible |
| Player used playback controls correctly | yes | Playwright playback check |
| Player advanced a season and saw state change | yes | Playwright mutation check |
| Player found event log and session log | yes | DOM and export checks |
| Player downloaded CSV or understood the export | yes | Playwright download check |
| Player separated operational score from publication status | yes | After-action score and lock check |
| Player stated the intended T1/T1 lesson | simulated yes | Aha answers above |
| At least one surprise was logged or explicitly none found | yes | Observation log |

Promotion decision:

| Decision | Check | Note |
|---|---|---|
| Browser G2-A pass | yes | Prototype supports playable browser demonstration in simulation |
| Hold browser G2-A | no | No blocking browser issue found in automated/simulated pass |
| Promote G0-C evidence | no | Needs human blind playtest or explicit owner acceptance of simulated evidence |
