# Des Moines Diamond Narrated Playthrough

This is the first Tier-B narrated playthrough for the G0 paper prototype. It is not a balanced final session. It is a reference transcript showing the intended learning arc: a player starts by treating the closure as a capacity problem, then discovers it is a topology problem.

Scenario: `docs/game/des-moines-diamond-g0.md`  
Playtest packet: `docs/game/des-moines-diamond-playtest.md`  
Rubric version: Interstate Tycoon scenario rubric v0.1  
Promotion target: G0-C Aha Proven

## Starting State

Tracks:

| Track | Value |
|---|---:|
| Budget | 12 |
| Construction crews | 3 |
| Political capital | 5 |
| Public patience | 6 |
| Operations capacity | 4 |
| Evidence confidence | 2 |

Player expectation:

| Question | Answer |
|---|---|
| What do you think the main problem is? | "The interchange probably needs more capacity." |
| Which project looks most obviously useful? | "General-purpose widening." |
| What do you expect to happen during a closure? | "Traffic diverts and slows down, but more lanes should help." |

## Season 1 - Forced Closure

Event: Full interchange-zone closure.

Player chooses:

- General-purpose widening.
- Intelligent routing.

Resolution:

| Metric | Result |
|---|---|
| Budget | 6 |
| Crews | 0 |
| Political capital | 5 |
| Public patience | 6 |
| Operations capacity | 4 |
| Evidence confidence | 2 |
| Throughput | Incident stress still exposes transfer fragility |
| Recovery | Heuristic recovery remains bounded, but the transfer lesson has not landed |
| Publication gate | Locked: diamond ledger mismatch |

Narration:

The player sees the first closure and reaches for the familiar answer: widen the road and route around the mess. The map improves at the edges, but the warning stays pinned to the interchange core. The backup route is not failing everywhere; the transfer is failing.

Hint used: Nudge.

Player note:

"I thought widening would solve the bottleneck. It helped traffic, but the transfer still has one obvious failure point."

## Season 2 - Reframing

Event: Political lane-mile pressure.

Player chooses:

- Diamond connector package.
- Work-zone sequencing.

Resolution:

| Metric | Result |
|---|---|
| Budget | 0 |
| Crews | 0 |
| Political capital | 4 |
| Public patience | 6 |
| Operations capacity | 4 |
| Evidence confidence | 2 |
| Throughput | Redundant transfer paths are queued but not complete |
| Recovery | Recovery gate cannot be claimed yet |
| Publication gate | Locked |

Narration:

The political card makes widening cheaper and connectors harder to justify. The player still chooses connectors because the first closure made the hidden problem visible. This is the intended moment: the player is no longer optimizing the most obvious stat. They are protecting the transfer.

Hint used: Push.

Player note:

"The connector is expensive, but now I see why it is different from widening."

## Season 3 - Evidence Pressure

Event: Source challenge.

Player chooses:

- Evidence acquisition.

Resolution:

| Metric | Result |
|---|---|
| Budget | -1 |
| Crews | 0 |
| Political capital | 4 |
| Public patience | 6 |
| Operations capacity | 4 |
| Evidence confidence | 3 |
| Throughput | No new construction effect |
| Recovery | No new construction effect |
| Publication gate | Still locked: evidence improved, diamond mismatch remains |

Narration:

The player tries to strengthen the claim, but the publication gate stays locked. This is supposed to feel slightly frustrating. The game is saying: you may have made a good operational choice, but the evidence pipeline still matters.

Player note:

"I can win the scenario and still not publish the claim. That is annoying, but fair."

## Season 4 - Connector Completion

Event: Night work-zone closure.

Player completes:

- Diamond connector package.
- Work-zone sequencing.

Resolution:

| Metric | Result |
|---|---|
| Budget | -1 |
| Crews | 3 |
| Political capital | 4 |
| Public patience | 6 |
| Operations capacity | 4 |
| Evidence confidence | 3 |
| Throughput | Heuristic transfer retention clears the operational threshold |
| Recovery | Heuristic T90 clears the 4-hour threshold |
| Publication gate | Locked: analyzer/data mismatch |

Narration:

The replay now shows a different kind of improvement. The player did not merely add volume; they changed the shape of the transfer. The pressure playback shows the interchange core no longer carrying the entire burden.

Audio cue: Recovery pulse.

Player note:

"This is the first time I can explain the standard: k-connectivity gives the system another way to transfer freight."

## Season 5 - After-Action

Event: Source challenge.

No new project.

Final operational score:

| Dimension | Max | Score | Evidence |
|---|---:|---:|---|
| Throughput retention | 25 | 25 | Heuristic scenario output |
| Recovery | 20 | 20 | Heuristic T90 proxy |
| SLA | 15 | 10 | PTI bounded but not publication-grade |
| Budget discipline | 10 | 0 | Budget went negative in this teaching run |
| Public support | 10 | 10 | Patience and political capital survived |
| Evidence honesty | 20 | 20 | Publication lock preserved |
| Total | 100 | 85 | Operational win, publication locked |

Publication gate:

| Gate | Result | Note |
|---|---|---|
| Diamond analyzer recognizes Des Moines node | Locked | Current `route diamond all` mismatch |
| No headline claim uses `source_needed` | Partial | Failure rates still need empirical depth |
| Observed versus modeled evidence is cited | Partial | Scenario output is modeled/heuristic |
| Final publication result | Locked | Operational win only |

## Aha Check

| Question | Player answer |
|---|---|
| What actually failed in the first closure? | "The transfer point, not the whole corridor." |
| Why did widening help or fail? | "It helped capacity, but it did not create another transfer path." |
| What did the diamond connector package change? | "It gave the system redundant paths through the 50-mile zone." |
| What evidence would make this publication-grade? | "The analyzer has to validate the Des Moines node, and the closure evidence needs real observed data." |

Aha status: Landed.

## Surprise Log

| Surprise | Type | Severity | Evidence | Amendment candidate? |
|---|---|---|---|---|
| Budget can go negative in the teaching run while the player still gets an operational win | rules | medium | Season 3 evidence acquisition | Yes: clarify whether negative budget is immediate failure or just score loss |
| Player accepted the publication lock once it was framed as evidence honesty | evidence | low | After-action note | No |
| Political lane-mile pressure made the connector choice feel more meaningful | balance | low | Season 2 choice | No |

## Review Notes

What worked:

- The forced closure created the intended reframing.
- The connector decision became legible after the first failure.
- The publication lock reinforced evidence honesty instead of feeling like a hidden loss.

What needs tightening:

- Budget rules need a firm failure/penalty definition.
- Evidence acquisition should not be buyable when budget is already exhausted unless debt is a deliberate rule.
- The Des Moines diamond mismatch needs either code/data repair or a formal blocker artifact.

Promotion decision:

| Decision | Check | Note |
|---|---|---|
| Hold at G0-B |  | Paper playable, but this is narrated rather than blind |
| Promote to G0-C |  | Needs at least one blind playtest with aha landed |
| Rewrite |  | Not indicated |

Next action: run or simulate one blind playtest using `docs/game/des-moines-diamond-playtest.md`, then compare surprises against this narrated reference.

