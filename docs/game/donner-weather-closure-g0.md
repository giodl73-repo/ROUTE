# Donner Weather Closure G0 Paper Prototype

Donner Weather Closure is the second Interstate Tycoon campaign stop. It turns the I-80 mountain-pass resilience standard into a playable lesson: a national corridor can fail even when the map shows alternate lines, because weather, truck suitability, capacity, and recovery windows decide whether freight can actually route around the closure in time.

Status: G0-A seed to G0-B paper prototype  
Scenario version: G0 v0.1  
Evidence level: Heuristic  
Primary ROUTE scenario: `crates/route-sim/src/scenarios/donner-closure.toml`  
Primary pressure row: `S-L2-DONNER` in `data/pressure-test-scenarios.csv`  
Campaign stop: `Mountain Pass` in `data/game/campaign-spine.csv`  
Playtest packet: `docs/game/donner-weather-closure-playtest.md`

## Player Promise

By the end of one playthrough, the player can explain why `T1-SPURS`, `T1-CLIMATE`, `T1-RECOVERY`, and `T1-INTERMODAL` earn a place in Interstate 2.0 standards, and why a visible detour is not the same thing as a dependable freight alternate.

## One Aha

The route around the storm is not free capacity.

Closing I-80 at Donner does not ask only, "Can freight go somewhere else?" It asks whether trucks receive early egress, whether the alternate has winter operating capacity, whether recovery happens inside the SLA window, and whether rail/intermodal relief can absorb eligible freight. A player can win operationally with heuristic investments, but publication remains locked until observed weather closure history and alternate-capacity evidence exist.

## Table Setup

Map zones:

| Zone | Function | Starting state |
|---|---|---|
| I-80 west approach | Sacramento/Auburn side | Open, storm watch |
| I-80 pass segment | Donner closure zone | Fragile in winter |
| I-80 east approach | Truckee/Reno side | Open, queue-prone |
| Early egress nodes | Truckee/SR-89 and Auburn/SR-49 concepts | Unsigned / low-readiness |
| Southern road alternate | Lower-elevation highway routing concept | Capacity-limited |
| Northern road alternate | Longer I-90/I-84 style diversion concept | Time-expensive |
| Rail/intermodal relief | Eligible freight diversion buffer | Planned, capacity-limited |
| Evidence drawer | Source and confidence cards | Mostly locked |

Tracks:

| Track | Start | Failure condition |
|---|---:|---|
| Budget | 13 | Below 0 |
| Construction crews | 3 | No available crew for a required project |
| Weather readiness | 3 | Below 0 during a storm |
| Public patience | 5 | Below 0 |
| Operations capacity | 4 | Below 0 during reroute or reopening |
| Evidence confidence | 1 | Publication gate locked below 4 |

Time limit: 8 seasons. One season is one winter-readiness, build, storm, or recovery cycle.

## Turn Sequence

1. Inspect the I-80 regional atlas map and evidence drawer.
2. Reveal one storm card.
3. Choose up to two project cards.
4. Pay budget, crews, readiness, and operations costs.
5. Resolve build timing and preparedness effects.
6. Run closure, detour, or reopening stress.
7. Score throughput retention, recovery, SLA, budget, support, and evidence.
8. Write a session-log row.

The first tutorial turn forces the `Whiteout closure` event after the player sees the normal I-80 corridor. Later turns draw from the storm deck.

## Starting Engine Facts

Current repo facts:

| Artifact | Output used by prototype |
|---|---|
| `crates/route-sim/src/scenarios/donner-closure.toml` | Bound I-80 Donner Pass weather closure with 48-hour duration and SnowIce incident type |
| `data/pressure-test-scenarios.csv` | `S-L2-DONNER` is heuristic; scenario edges are bound but demand and alternate-route capacities are not proof-grade |
| `docs/STANDARDS_EVALUATION.md` | Donner and Atlanta currently show no throughput delta under the synthetic demand proxy |
| `data/map-atlas.csv` | `i80-region` map is gated for campaign reuse |

Prototype interpretation:

- Game win gate may use bounded heuristic outputs and card effects.
- Publication gate is locked until observed closure frequency/duration, alternate truck capacity, and PTI/SLA impacts are validated.
- The game should not claim that a modeled no-delta run proves Donner resilience.

## Project Cards

| Card | Cost | Crew | Time | Effect | Evidence | Failure mode protected |
|---|---:|---:|---:|---|---|---|
| Early egress spurs | 3 | 1 | 2 | Lets trucks leave before the closure zone instead of queueing at the pass | Planned | Trapped freight and late reroute |
| Winter operations package | 2 | 1 | 1 | Raises weather readiness by 2 and reduces reopening delay | Heuristic | Slow clearance and chain-control shock |
| Lower-elevation freight bypass | 6 | 2 | 4 | Adds independent road capacity below the storm zone | Planned | Pass closure with no road alternate |
| Managed freight tunnel | 5 | 2 | 3 | Protects eligible priority freight from snow closure | Planned | High-value SLA misses |
| Rail/intermodal surge slots | 3 | 1 | 2 | Moves eligible freight before the road queue grows | Heuristic | Overloaded road detour |
| Dynamic closure routing | 2 | 0 | 1 | Reduces operations loss when storms are forecast | Heuristic | Late route decisions |
| General snow storage / shoulders | 2 | 1 | 1 | Helps reopening but does not create alternate capacity | Heuristic | Recovery delay |
| Source request | 1 | 0 | 1 | Raises evidence confidence by 1 and names missing weather/closure sources | Implemented as artifact plan | Unknown evidence blocker |
| Validated weather evidence | 2 | 0 | 1 | Requires source request; raises evidence confidence only when observed closure history exists | Planned | Publication-grade proof |

Project rule: road capacity, operations, and evidence are separate. A player cannot buy publication proof with a construction project.

## Storm Cards

| Card | Trigger | Effect | Visible warning |
|---|---|---|
| Whiteout closure | Forced tutorial, then rare | Close the pass for 48 hours; test throughput and recovery | "The pass is closed; the queue is forming before the alternate decision." |
| Chain-control slowdown | Common | Lose 1 weather readiness unless winter operations are active | "The route is open, but speed and compliance are binding." |
| Detour capacity pinch | Common | Lose 1 operations unless a bypass or rail slots are active | "The alternate exists on the map, but it is not absorbing T1 freight." |
| Reopening surge | Medium | Lose 1 public patience unless snow storage/shoulders or routing is active | "The closure ended; the queue did not." |
| High-value SLA wave | Medium | Lose SLA margin unless tunnel or intermodal slots are active | "Not all freight can wait for the road to reopen." |
| Evidence challenge | Medium | Publication gate checks evidence confidence and observed-source status | "A reviewer asks how many closures, how long, and what the alternate carried." |

## Evidence Cards

| Card | Known | Missing | Game effect |
|---|---|---|
| Donner scenario fixture | Bound weather closure scenario exists | Focused I-80 mountain demand and intervention acceptance gate | Unlocks heuristic storm scoring |
| Weather closure history | Source need is named | Observed annual frequency and duration distribution | Locks publication-grade closure probability |
| Alternate road capacity | Standards need is named | Truck-capable capacity, winter reliability, and detour time | Locks reroute throughput claim |
| NPMRDS/PTI | Source target is known | Direct pass and alternate travel-time validation | Locks SLA proof |
| Rail/intermodal relief | Design concept exists | Eligible freight share and surge slot capacity | Keeps intermodal project heuristic |
| Map atlas | `i80-region` is gated | Real closure overlays and alternate volume layers | Supports campaign context |

## Scoring

Game score is separate from publication status.

| Dimension | Points | How earned |
|---|---:|---|
| Throughput retention | 25 | Storm stress retains at least 70% modeled freight movement through road, bypass, or intermodal buffers |
| Recovery | 20 | Reopening/queue recovery stays within 8 hours after the closure lifts |
| SLA | 15 | Priority freight or relay plan keeps the scenario SLA inside the chosen service window |
| Budget discipline | 10 | Budget remains non-negative |
| Public support | 10 | Public patience and weather readiness remain non-negative |
| Evidence honesty | 20 | The after-action report labels heuristic, planned, and source-needed claims correctly |

Win bands:

| Score | Result |
|---:|---|
| 80-100 | Operational winter win; publication may still be locked |
| 60-79 | Partial win; freight moves but recovery, budget, or evidence is weak |
| 0-59 | Failure; the storm breaks the corridor or the evidence story |

Publication gate:

- Pass requires observed weather closure frequency and duration evidence.
- Pass requires truck-capable alternate-route capacity and recovery validation.
- Pass requires direct PTI/SLA validation for the pass and chosen alternate.
- Until then, final publication status is locked.

## Hint Ladder

| Tier | Hint |
|---|---|
| Nudge | "The storm is not only on the pass. It is also in the queue before the pass." |
| Push | "Ask what the alternate can carry, not only whether it exists." |
| Shove | "Build egress and operating capacity before betting on a long detour." |

Language rule: before the first storm, say "alternate route." After the first closure, say "truck-capable alternate capacity." Only introduce SLA/PTI after the player understands the closure and recovery window.

## Screen Sketches

| Screen | ASPECT aim | Encoding contract |
|---|---|---|
| I-80 regional board | Show closure zone and alternate decisions | Pass = warning band; egress = gates; detours = thinner lines |
| Storm deck | Show forecast uncertainty | Storm severity = card header and event-log row |
| Capacity panel | Separate road, operations, and intermodal buffers | Each buffer has text and numeric state |
| Evidence drawer | Keep proof honest | Observed/model/heuristic/planned/source-needed labels visible |
| After-action report | Teach the lesson and lock/unlock publication | Operational score and publication status are separate |

No screen may rely on audio or prose alone to communicate a state change.

## Copy Deck

| Location | Text |
|---|---|
| Scenario hook | "I-80 looks like a line across the mountains until winter turns it into a timed gate." |
| First warning | "The pass is closed; the queue is forming before the alternate decision." |
| Detour warning | "A route on the map is not yet a freight alternate." |
| Egress unlock | "Trucks can leave before the queue hardens." |
| Intermodal unlock | "Eligible freight moves before the road queue consumes the SLA." |
| Publication lock | "Operational winter win recorded. Publication claim locked: observed closure and alternate-capacity evidence are still missing." |

## Session Log Schema

Each paper playtest should write one row per season.

| Field | Example |
|---|---|
| `season` | `1` |
| `storm_card` | `Whiteout closure` |
| `projects_started` | `Winter operations package; Dynamic closure routing` |
| `projects_completed` | `none` |
| `budget_remaining` | `9` |
| `weather_readiness` | `4` |
| `public_patience` | `5` |
| `operations_capacity` | `4` |
| `throughput_retention` | `0.70 heuristic` |
| `recovery_hours` | `8 heuristic` |
| `sla_status` | `bounded heuristic` |
| `evidence_confidence` | `1` |
| `publication_gate` | `locked: weather closure and alternate-capacity evidence missing` |
| `player_note` | `The alternate needed capacity before the storm` |

## Tutorial End Condition

The G0 tutorial can end before season 8 when:

- The player has resolved one whiteout closure.
- The player has chosen at least one operations/egress/intermodal response.
- The after-action report has been scored.
- The player can explain route existence versus truck-capable alternate capacity.
- Publication status is explicitly locked or unlocked.

## Verification Checklist

G0-B is complete when:

- A player can run the paper game from this document without ROUTE internals.
- The playtest packet records a season log, score, aha check, surprise log, and promotion decision.
- The forced whiteout closure produces the intended alternate-capacity aha.
- The after-action report separates operational winter win from publication proof.
- `cargo run -q -p route -- sim scenario donner-closure` still runs or its limitation is explicitly recorded.
- `data/pressure-test-scenarios.csv` still lists `S-L2-DONNER` as heuristic evidence, not proof-grade evidence.
