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

## Build Plan

### Phase G0 - Paper Prototype

Write the Des Moines Diamond scenario as board-game rules:

- Map zones
- Project cards
- Event cards
- Resource tracks
- Turn sequence
- Win/loss conditions

### Phase G1 - CLI Prototype

Add a `route game` command family that can print a scenario state and resolve one season using existing ROUTE outputs.

Likely commands:

- `route game scenarios`
- `route game inspect des-moines-diamond`
- `route game run-season des-moines-diamond`
- `route game score des-moines-diamond`

### Phase G2 - Browser Prototype

Build a small map-first UI around one scenario. The first screen is the playable map, not a landing page.

### Phase G3 - Campaign

Add a national campaign:

- Des Moines Diamond
- Donner weather closure
- Atlanta managed-lane stress
- Houston port surge/flood
- NY-LA 48-hour SLA
- Relay network outage
- EV/rest-area outage

## Design Rule

Every fun choice must correspond to a real ROUTE claim. Every real ROUTE claim used by the game must expose its evidence label. The game can dramatize the system, but it must not hide uncertainty.
