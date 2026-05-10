# Donner Weather Closure Playtest 001

Status: simulated blind-player pass  
Scenario version: G0 v0.1  
Rubric version: Interstate Tycoon scenario rubric v0.1  
Reference packet: `docs/game/donner-weather-closure-playtest.md`

This is a simulated blind pass, not a human playtest. It uses the playtest packet constraints and treats `S-L2-DONNER` as heuristic because the current engine fixture has bound closure edges but not proof-grade demand or alternate-capacity validation.

## Playtest Header

| Field | Value |
|---|---|
| Date | 2026-05-10 |
| Facilitator | Codex |
| Player / persona | logistics planner, medium strategy-game knowledge, low highway-engineering knowledge |
| Familiarity with highway engineering | low |
| Familiarity with strategy games | medium |
| Scenario version | G0 v0.1 |
| Rules used | `docs/game/donner-weather-closure-g0.md` |
| Notes path | this file |

## Pre-Play Prompt

| Question | Answer |
|---|---|
| What do you think the main problem is? | "The storm closes the pass, so I need a detour or snow-clearing project." |
| Which project looks most obviously useful? | "Managed freight tunnel, because it sounds like it bypasses weather entirely." |
| What do you expect freight to do when I-80 closes? | "Reroute south or north, then come back after the pass reopens." |

## Season Log

| Season | Storm card | Projects started | Projects completed | Budget | Crews | Weather | Patience | Ops | Evidence | Throughput | Recovery | SLA | Publication gate | Player note |
|---:|---|---|---|---:|---:|---:|---:|---:|---:|---|---|---|---|---|
| 1 | Whiteout closure | Managed freight tunnel; Dynamic closure routing | none | 6 | 1 | 3 | 5 | 4 | 1 | below target | weak | missed | locked: weather closure and alternate-capacity evidence missing | "The tunnel is not ready, and the detour cannot just absorb everything." |
| 2 | Detour capacity pinch | Winter operations package; Rail/intermodal surge slots | Dynamic closure routing | 1 | 1 | 5 | 5 | 4 | 1 | 0.70 heuristic | bounded heuristic | partial | locked: weather closure and alternate-capacity evidence missing | "Operations helped, but road detour capacity is still the bottleneck." |
| 3 | Chain-control slowdown | Source request | Winter operations package | 0 | 2 | 5 | 5 | 4 | 2 | 0.76 heuristic | 8h heuristic | bounded heuristic | locked: weather closure and alternate-capacity evidence missing | "I know what source is missing, but evidence did not create capacity." |
| 4 | High-value SLA wave | none | Rail/intermodal surge slots | 0 | 3 | 5 | 5 | 4 | 2 | 0.82 heuristic | 8h heuristic | bounded heuristic | locked: weather closure and alternate-capacity evidence missing | "Intermodal slots protect some freight, but only if I started them before the wave." |
| 5 | Evidence challenge | Validated weather evidence rejected: no observed artifact | Managed freight tunnel | 0 | 3 | 5 | 5 | 4 | 2 | 0.82 heuristic | 8h heuristic | bounded heuristic | locked: weather closure and alternate-capacity evidence missing | "The operational lesson landed; publication is still blocked by data." |

Stopped after season 5 because the intended aha had landed and the long tunnel build completed enough to resolve the tutorial arc.

## Final Score

| Dimension | Max | Score | Evidence |
|---|---:|---:|---|
| Throughput retention | 25 | 25 | Heuristic card effects reached 0.82 after rail/intermodal and tunnel completion |
| Recovery | 20 | 20 | 8h heuristic recovery proxy met the scenario threshold |
| SLA | 15 | 10 | Bounded heuristic; no direct PTI/SLA validation |
| Budget discipline | 10 | 10 | Budget reached 0 but did not go negative |
| Public support | 10 | 10 | Public patience and weather readiness remained non-negative |
| Evidence honesty | 20 | 20 | Publication gate stayed locked and source-needed fields were named |
| Total | 100 | 95 | Operational winter win, publication locked |

Result band: operational winter win.

Publication gate:

| Gate | Pass / lock | Note |
|---|---|---|
| Observed weather closure frequency/duration exists | lock | Source request named the need, but no observed history is attached |
| Truck-capable alternate capacity is validated | lock | Detour capacity pinch remained heuristic |
| Direct PTI/SLA validation exists | lock | No NPMRDS/PTI extract or direct SLA validation |
| Heuristic scenario limitation is cited | pass | Playtest cites `S-L2-DONNER` as heuristic and not proof-grade |
| Final publication result | locked | Operational win only |

## Aha Check

| Question | Player answer |
|---|---|
| What actually failed during the whiteout? | "The pass closed, but the bigger failure was that freight hit the closure before enough alternate capacity existed." |
| Why was the visible detour not automatically enough? | "It had limited truck capacity and still had weather/recovery limits. A line on the map did not mean T1 throughput." |
| What changed when you added egress, operations, bypass, tunnel, or intermodal capacity? | "Operations gave warning time, rail/tunnel protected priority freight, and the detour needed actual capacity before it could count." |
| What evidence would make this publication-grade? | "Observed closure frequency and duration, truck-capable alternate capacity, and PTI/SLA data for the pass and detours." |

Aha status: Landed.

## Surprise Log

| Surprise | Type | Severity | Evidence | Amendment candidate? |
|---|---|---|---|---|
| Managed freight tunnel looked like the obvious first pick, but its long build time made the first forced closure feel harsh | balance | medium | Season 1 | Candidate: add a quick "forecast window" setup before first storm or mark tunnel as long-term |
| Early egress spurs were important in the rules but easy to skip because no card explicitly punished trapped queues unless the facilitator emphasized it | learning | medium | Seasons 1-2 | Candidate: add a trapped-queue marker to the whiteout card |
| Source request worked as evidence honesty, but the publication lock needs a stronger one-line explanation after validated evidence is rejected | copy | low | Season 5 | Candidate: add "source requested is not source observed" copy |
| The 8-hour recovery threshold was understandable, but players may ask why Donner uses 8h while Des Moines uses 4h | rules | low | Final score | Candidate: add scenario-specific recovery-window note |

## Facilitator Notes

| Prompt | Notes |
|---|---|
| Where did the player slow down? | Choosing between tunnel, bypass, and operations after seeing that the first storm arrives before long builds complete |
| Which rule required clarification? | Whether rail/intermodal slots count as throughput retention or only SLA protection |
| Which card was ignored or overvalued? | Early egress spurs were ignored; managed freight tunnel was overvalued as an immediate fix |
| Did the forced whiteout feel fair in retrospect? | Mostly; warning copy was clear, but early egress needs a stronger visual/rule consequence |
| Did evidence labels change player behavior? | Yes; source request was chosen once publication lock was visible |
| Did the publication lock make sense? | Yes after the player saw that source request did not equal observed evidence |

## G0 Promotion Checklist

| Gate | Pass? | Evidence |
|---|---|---|
| Player completed a run from paper rules | partial | Stopped at season 5 after aha landed |
| Forced whiteout produced the intended aha | yes | Aha check |
| Player explained truck-capable alternate capacity before PTI/SLA was named | yes | Aha check |
| Player made at least one meaningful tradeoff | yes | Tunnel versus operations/intermodal/source request |
| Evidence labels stayed visible in scoring | yes | Publication gate table |
| Operational win/loss separated from publication gate | yes | Final score |
| Surprise log was filled | yes | Four surprises |
| At least one amendment candidate was accepted or explicitly declined | pending | Needs amendment log or v0.2 update |

Promotion decision:

| Decision | Check | Note |
|---|---|---|
| Hold at G0-B | yes | Paper-playable in simulation, but needs amendments and human/owner acceptance before G0-C |
| Promote to G0-C | no | Simulated aha landed; human/owner acceptance still missing |
| Rewrite | no | Core lesson works |

## Amendment Candidates

| Candidate | Proposed change |
|---|---|
| Trapped queue marker | Add a visible marker when the first whiteout hits before early egress or routing is ready |
| Long-term tunnel framing | Mark managed freight tunnel as powerful but not an immediate first-storm answer |
| Source observed copy | Add copy explaining that source request names a data path but does not validate the claim |
| Recovery-window note | Explain why the Donner tutorial uses an 8-hour reopening/queue window rather than the Des Moines 4-hour transfer window |
