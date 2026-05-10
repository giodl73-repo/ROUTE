# Interstate Tycoon

Interstate Tycoon is the game-facing layer for ROUTE: a highway network strategy game built on top of the Interstate 2.0 scoring, pressure-test, and simulation engine.

The sell is simple: players do not read a standards paper first. They inherit today's interstate network, watch it fail under freight growth and adversity, then discover why managed freight lanes, T1/T1 diamonds, relay hubs, EV/rest-area standards, flood hardening, and T2 relief corridors matter.

## TIGRIS Transfer

TIGRIS contributes the design discipline, not the subject matter.

| TIGRIS element | ROUTE game translation |
|---|---|
| TIGER BEAT profile | Target player experience for the game layer |
| Parliament review | Scenario review gates and stakeholder reactions |
| Tier-A matrix | Fast scenario scoring before implementation |
| Tier-B narrated playthrough | Playable scenario transcripts and tutorial arcs |
| Tier-C tournament | AI/campaign balance testing across strategies |
| Artifact contracts | Each scenario has a concept, rules, commands, scoring, and handoff |
| Forbidden vague claims | Every upgrade must name the metric it improves |
| Design-gap search | Find scenarios where existing infrastructure games do not explain real network resilience |

## HUNT Transfer

HUNT contributes reveal structure: how a player learns a complex system through discovery, confirmation, and fair surprise.

| HUNT element | ROUTE game translation |
|---|---|
| Riven Standard | The puzzle is the infrastructure system itself, not a quiz pasted onto a map |
| Solving = Proving Understanding | A successful project choice proves the player understood the bottleneck |
| Blame the Player | Failures must be fair in retrospect: the warning signs were visible |
| One Aha | Each tutorial scenario teaches one core infrastructure insight |
| No Over-Scaffolding | UI gives evidence and tools, not step-by-step optimization instructions |
| 80% Rule | Scenarios should remain solvable when one evidence source or project path is unavailable |
| Layered hints | Advisor nudges can reveal diagnosis, mechanism, then recommended intervention |
| Blind solver testing | New scenario tutorials should be tested by naive player personas before shipping |
| World-as-puzzle | Maps, ledgers, event cards, and dashboards are all clue surfaces |

The design principle to steal hardest is: solving should leave the player knowing more. If a player beats Des Moines Diamond, they should be able to explain why a single T1/T1 interchange is fragile and why redundancy is not the same thing as raw capacity.

## ASPECT Transfer

ASPECT contributes the screen grammar. Interstate Tycoon should not merely show maps; every screen should make a specific infrastructure claim visible, legible, and honest.

| ASPECT element | ROUTE game translation |
|---|---|
| Aim | Each screen has one job: diagnose, choose, stress, replay, or justify |
| School | Each screen names its visual grammar: Beck topology, Minard flow, Tufte small multiples, Rosling playback, Neurath icons, Nightingale accountability |
| Precision | Encodings are explicit and stable across the game |
| Effect | The player feels the right pressure: scarcity, fragility, recovery, or public accountability |
| Clarity | The first read is map-first and decision-ready, not a decorative dashboard |
| Truth | Every claim exposes evidence level, source status, and uncertainty |

Screen families:

| Screen | Visual school | Job |
|---|---|---|
| National map | Beck topology plus Minard flow | Show tier structure, freight flows, bottlenecks, and fragile transfers |
| Scenario board | Board-game map | Make projects, events, resources, and turns playable |
| Evidence drawer | Tufte table | Separate observed, modeled, heuristic, planned, and source-needed claims |
| Pressure playback | Rosling/Minard animation | Show before/after flow retention under adversity |
| Public scorecard | Nightingale/Du Bois accountability | Explain who benefits, who waits, and which promises were met |

Visual encoding contract:

| Data element | Encoding |
|---|---|
| Corridor tier | Route hue and label |
| Throughput | Stroke width |
| Failure or stress | Warning overlay and event icon |
| Confidence | Opacity plus evidence badge |
| `source_needed` | Dashed outline and lock icon |
| Time | Season scrubber and event log |
| Project cost | Card cost, crew slots, and budget track |
| SLA | Commitment-window meter |

The screen rule is simple: every visual claim gets one encoding and one evidence label.

## PROSE Transfer

PROSE contributes the writing contract. The game should explain complex infrastructure without flattening it into slogans.

| PROSE element | ROUTE game translation |
|---|---|
| Purpose | Each sentence must help the player decide, understand, or trust the claim |
| Reader | Text assumes an intelligent player who may not know highway engineering |
| Organization | Scenario copy moves from hook, to evidence, to choice, to consequence |
| Style | Plain, concrete, and operational; no bureaucratic fog |
| Economy | Short labels in UI; longer explanations only in evidence and after-action views |

Writing rules:

- A project name should imply its mechanism: "Diamond connector package" beats "Mobility improvement."
- A warning should name the failing system: "Transfer capacity is collapsing at the interchange zone."
- A win/loss summary should teach the lesson the scenario was built to reveal.
- A public claim should carry its evidence level in the same sentence or table row.
- Flavor can dramatize pressure, but cannot hide uncertainty.

PROSE also gives the review lenses for scenario text:

| Lens | Interstate Tycoon use |
|---|---|
| Copy Editor | UI labels, card text, and warnings are correct and consistent |
| Developmental Editor | The scenario opens, escalates, and resolves in the right order |
| First Reader | A new player can tell what changed and why it matters |
| Subject Expert | Highway, freight, SLA, and evidence claims are technically defensible |
| Rhetorician | The demo persuades without hiding uncertainty or overselling heuristics |

## SCORE Transfer

SCORE contributes the audio and tempo contract. Interstate Tycoon does not need a large soundtrack at first, but it does need sound and pacing that help the player feel system pressure without manipulating the evidence.

| SCORE element | ROUTE game translation |
|---|---|
| Structure | Seasons, events, closures, recovery, and hearings have a clear rhythmic arc |
| Craft | Sound cues and pacing reinforce real state changes, not decoration |
| Originality | The game should sound like logistics, infrastructure, and public consequence, not generic city-builder music |
| Resonance | Audio should make fragility, relief, and accountability legible |
| Economy | Every cue earns its place; silence is allowed when the player needs to think |

Audio contract:

- Closure, recovery, SLA miss, budget exhaustion, evidence unlock, and publication gate each get distinct cues.
- Cue intensity follows modeled severity, not dramatic convenience.
- Repeated turns use variation so the campaign does not punish long strategic play.
- Audio can guide attention, but it cannot be the only place a state change is communicated.
- Demo mode can use stronger musical identity; analysis mode should stay quieter.

## QUEST Transfer

QUEST contributes campaign structure, continuity, and the engine/narrative split.

| QUEST element | ROUTE game translation |
|---|---|
| Treasures are the story | Projects are not generic upgrades; each one changes the network's future options |
| Deterministic engine | ROUTE owns state, metrics, event resolution, and reproducibility |
| Narrative layer | Advisors, stakeholders, and briefings explain consequences without changing math |
| Session log | Each season writes decisions, events, evidence labels, and score changes |
| Checkpoint/resume | Campaign state is re-entrant and auditable |
| Surprise log | Playtests record where players learned, stalled, or found an unintended strategy |
| Forward-only rubric | Scenario scoring can improve over time without rewriting old campaign records |

Campaign consequence rule: if a player underbuilds resilience, that debt should follow them. If they overbuild a corridor, the opportunity cost should appear somewhere else. The campaign should feel like a civic system remembering what happened.

## Target Experience Profile

The game should not be a spreadsheet wearing a map. It should have a board-game-clear loop with a serious engine underneath.

| Dimension | Target | Why |
|---|---:|---|
| Tension | High | Closures, budget scarcity, SLA misses, and political deadlines create pressure |
| Interaction | Medium | In single-player, stakeholders and events oppose the player; multiplayer can compete for funding and regional outcomes |
| Game-spectrum | Mid-high | Deep enough for infrastructure strategy, readable enough for a first scenario |
| Experiential | Medium | The player should feel national logistics fragility, not only optimize points |
| Range | High | Different corridors, event decks, budgets, and policy constraints create replayability |
| Breadth | Medium | Start solo/campaign first; leave multiplayer and sandbox for later |
| Emotional Arc | Medium-high | The arc is inherited fragility -> hard choices -> visible resilience |
| Accessibility | Medium | First turn should be: inspect bottleneck, choose project, run season |
| Texture | High | Scarcity must bite: budget, right-of-way, construction time, public tolerance, outage risk |

## Core Loop

1. Inspect the network.
2. Choose projects for the build season.
3. Spend budget, crews, right-of-way, political capital, and time.
4. Run freight/passenger demand.
5. Draw adversity events.
6. Score SLA, throughput, access, resilience, cost, and public support.
7. Upgrade, defer, or downgrade claims for the next season.

The player wins by making the network resilient enough to meet service commitments under adversity, not by paving the most miles.

## First Playable Slice

### Scenario: Des Moines Diamond

The player inherits the I-35 x I-80 T1/T1 node. The baseline network concentrates too much national freight transfer through a fragile interchange zone. The player has a fixed budget and 10 simulated years to keep freight moving.

Available projects:

| Project | Game effect | ROUTE engine hook |
|---|---|---|
| Diamond connectors | Adds redundant transfer paths | `route diamond --at I35xI80` |
| Express freight flyovers | Protects T1 transfer capacity from local traffic | `route throughput-proof --gate` plus scenario fixture |
| Relay hub expansion | Absorbs driver-swap disruption | `route hub-outage` |
| EV/rest-area hardening | Reduces charging/rest outage penalties | `route ev-rest-outage` |
| Intelligent routing | Reduces incident delay and missed swaps | `route od ...`, incident delay modifiers |
| T2 relief investment | Adds alternate routing capacity | graph/intervention scenario fixture |
| Work-zone sequencing | Trades short-term closure pain for long-term capacity | `data/t1-failure-events.csv` and scenario events |

Win gates:

| Gate | Target |
|---|---|
| T1/T1 transfer retention | At least 80% under closure stress |
| Recovery | Restore 80% transfer capacity within 4 hours or label unproven |
| SLA | Keep key OD commitments within modeled p95 windows |
| Budget | Stay within authorized program budget |
| Evidence honesty | No publication-grade claim may use `source_needed` evidence |

### Tutorial Aha

The first scenario should not start by explaining T1/T1 resilience. It should make the player discover it.

1. The player sees a normal-looking national freight map.
2. The game asks them to meet a routine SLA target.
3. A closure hits I-35 x I-80.
4. The player tries obvious fixes: add lane capacity, accelerate cleanup, reroute trucks.
5. Those fixes help but do not solve the transfer collapse.
6. The aha: the problem is not only congestion; it is network topology.
7. Diamond connectors and flyovers become legible because the failure has already happened.

That is HUNT's "solving = proving understanding" applied to infrastructure.

### Scenario Card

```text
Name: Des Moines Diamond
Hook: The national freight grid looks healthy until one interchange closure breaks the transfer.
Hidden lesson: Redundancy and recovery depend on topology, not only spare lane capacity.
Player promise: By the end, the player can explain why k-connectivity earns a T1 standard.

Starting evidence:
- T1/T1 node: I-35 x I-80
- Current modeled throughput retention
- Iowa 511 work-zone observation sample
- Source confidence warnings

Event deck:
- Night work-zone closure
- Full interchange-zone closure
- Relay hub surge
- EV/rest-area queue
- Political pressure to buy lane miles instead of connectors

Project deck:
- General-purpose widening
- Diamond connector package
- Express freight flyovers
- Intelligent routing
- Relay hub reserve staffing
- EV/rest hardening

Hints:
- Nudge: "The backup route is not failing everywhere. The transfer is failing."
- Push: "Count edge-disjoint paths through the 50-mile zone."
- Shove: "Build redundancy first; then buy capacity."

Win gates:
- Post-build k >= 3
- Closure stress retains target transfer throughput
- Recovery gate is met or honestly labeled unproven
- Evidence labels remain visible
```

## ROUTE Engine as Game State

| Game state | Existing ROUTE source |
|---|---|
| Corridor scores | `data/scores-all.csv`, `docs/DIMENSIONS.md` |
| Tier standards | `specs/2026-05-06-tier-standards.md` |
| Project packages | `specs/2026-05-06-interstate-2-design.md` |
| Scenario catalog | `data/pressure-test-scenarios.csv` |
| Throughput proof | `data/throughput-proof-matrix.csv` |
| T1/T1 failure observations | `data/t1-failure-events.csv` |
| Source confidence | `data/t1-intersection-failures.csv`, `data/t1-source-health.csv` |
| L0/L1/L2 regression tests | `cargo test --workspace` |

## Game System Sketch

### Resources

- Budget
- Construction crews
- Right-of-way tolerance
- Political capital
- Public patience
- Evidence confidence
- Operations capacity

### Project Cards

Each project card must include:

- Cost
- Build time
- Affected corridors
- Standards tested
- Expected metric impact
- Evidence level
- Failure mode it protects against
- Command or artifact that backs the effect

### Event Cards

Event cards are generated from pressure-test classes:

- T1/T1 closure
- Corridor segment weather closure
- Port surge
- Flood disruption
- Relay hub outage
- EV/rest-area outage
- Managed-lane demand sensitivity
- Bridge restriction
- Work-zone sequencing conflict

### Evidence Cards

Evidence cards are the bridge between serious ROUTE claims and playable discovery.

Each card should expose:

- What is known
- What is modeled
- What is missing
- Which source would improve confidence
- Which project or event it affects

Example:

| Card | Known | Missing | Game effect |
|---|---|---|---|
| Iowa 511 work-zone sample | Normalized observations exist for I-35/I-80 | Annual history depth | Unlocks low-confidence closure probability |
| NPMRDS access gated | Historical PTI source identified | Direct extract | Blocks publication-grade SLA proof |
| Des Moines diamond model | Scenario runs and restores throughput proxy | Geometry validation | Unlocks heuristic diamond project |

### Scoring

The scoreboard should expose both game points and engineering truth:

- Freight SLA kept
- Passenger access improved
- Throughput retained
- Rural coverage improved
- Redundancy gained
- Emissions and delay avoided
- Budget efficiency
- Evidence confidence

The player can win a game scenario with heuristic evidence, but cannot unlock Blueprint/publication mode until the evidence labels clear.

## Hint And Advisor System

The advisor system should be HUNT-style layered hints, not a tutorial rail.

| Tier | In-game form | Example |
|---|---|---|
| Nudge | Advisor points at a clue surface | "Throughput did not collapse everywhere. It collapsed at the transfer." |
| Push | Advisor names the mechanism | "This is a k-connectivity problem, not only a lane-count problem." |
| Shove | Advisor recommends an intervention | "Build the diamond connector package before adding more general-purpose lanes." |

Using hints can cost public confidence, score, or nothing depending on audience. For a demo, hints should be free and visible; for a campaign, hints can reduce final mastery score.

## Scenario Authoring Contract

Every playable scenario should have a HUNT/TIGRIS/ASPECT/PROSE/SCORE/QUEST artifact bundle:

- Concept: one-sentence player-facing hook
- Map: corridors, nodes, and stress points
- Hidden lesson: the intended aha
- Starting evidence: what the player can inspect
- Event deck: adversity cards and probabilities
- Project deck: available interventions
- Scoring gates: success, partial success, failure
- ROUTE commands: exact engine hooks behind the scenario
- Hints: nudge, push, shove
- Screen contract: aim, visual school, encodings, and evidence badges
- Copy contract: player-facing labels, warning text, and after-action explanation
- Audio contract: cues, intensity rules, silence points, and accessibility fallback
- Campaign contract: persistent consequences, save/resume fields, and session log shape
- Verification: tests or CLI gates proving the scenario still runs

This keeps game design from drifting away from the engine.

## Campaign Spine

The first campaign should turn the pressure-test library into a playable proof arc.

| Scenario | Lesson | ROUTE proof path |
|---|---|---|
| Des Moines Diamond | T1/T1 redundancy is topology, not lane count | k-connectivity, throughput retention, failure observations |
| Donner Weather Closure | Resilience depends on alternate winter capacity and recovery windows | incident degradation, weather closure, SLA buffers |
| Atlanta Managed-Lane Stress | Managed lanes protect reliability only if demand and merge behavior are honest | managed-lane sensitivity and PTI proof |
| Houston Port Surge | Port access fails differently under flood, surge, and connector stress | intermodal gaps and port-corridor scenarios |
| NY-LA 48-Hour SLA | Relay buffers and p95 planning windows decide whether promises survive incidents | SLA proof table and relay timing |
| Relay Network Outage | Operations capacity can be the bottleneck even when pavement exists | `route hub-outage` |
| EV/Rest Hardening | Energy and rest standards protect freight only when outages are modeled | `route ev-rest-outage` |
| Blueprint Hearing | Public proof requires evidence labels, not just a winning score | standards proof ledger and panel review |

Each scenario should have a playable win condition and a publication gate. Winning teaches the system; publication proves the claim.

## Build Plan

### Phase G0 - Paper Prototype

Write the Des Moines Diamond scenario as board-game rules. Current artifact: [`docs/game/des-moines-diamond-g0.md`](game/des-moines-diamond-g0.md).

- Map zones
- Project cards
- Event cards
- Evidence cards
- Layered hints
- Screen sketches with ASPECT encodings
- Player-facing copy with PROSE labels
- Audio cue list with SCORE economy rules
- Resource tracks
- Turn sequence
- Win/loss conditions
- Session log and checkpoint fields

### Phase G1 - CLI Prototype

Add a `route game` command family that can print a scenario state and resolve one season using existing ROUTE outputs.

Likely commands:

- `route game scenarios`
- `route game inspect des-moines-diamond`
- `route game run-season des-moines-diamond`
- `route game score des-moines-diamond`

### Phase G2 - Browser Prototype

Build a small map-first UI around one scenario. The first screen is the playable map, not a landing page.

Required checks:

- The map has stable encodings for tier, throughput, stress, confidence, and SLA.
- Every project card exposes cost, effect, evidence, and failure mode.
- The evidence drawer separates observed from modeled claims.
- The pressure playback shows what changed after the player's intervention.
- No screen relies on explanatory wall text to make the game playable.
- Audio cues are mirrored visually and can be disabled without losing game state.

### Phase G3 - Campaign

Add a national campaign:

- Des Moines Diamond
- Donner weather closure
- Atlanta managed-lane stress
- Houston port surge/flood
- NY-LA 48-hour SLA
- Relay network outage
- EV/rest-area outage
- Blueprint hearing

### Phase G4 - Public Demo

Package the campaign as the friendly proof surface for Interstate 2.0:

- A player can learn why each standard exists.
- A reviewer can inspect the evidence behind each effect.
- A policymaker can see the cost of ignoring resilience.
- A developer can reproduce the scenario from ROUTE commands.

## Design Rule

Every fun choice must correspond to a real ROUTE claim. Every real ROUTE claim used by the game must expose its evidence label. The game can dramatize the system, but it must not hide uncertainty.
