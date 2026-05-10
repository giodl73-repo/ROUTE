# Des Moines Diamond G0 Paper Prototype

Des Moines Diamond is the first Interstate Tycoon paper scenario. It turns the Milepost 4 T1/T1 pressure-test work into a playable proof: the player should discover that a national freight interchange can fail because of topology, not only because of lane capacity.

Status: G0 paper prototype  
Evidence level: Heuristic  
Primary ROUTE scenario: `crates/route-sim/src/scenarios/des-moines-interchange.toml`  
Primary pressure row: `S-L2-DES-MOINES` in `data/pressure-test-scenarios.csv`
Playtest packet: `docs/game/des-moines-diamond-playtest.md`

## Player Promise

By the end of one playthrough, the player can explain why `T1-DIAMOND-K`, `T1-FLYOVER`, and `T1-RECOVERY` earn a place in Interstate 2.0 standards, and which parts are still only heuristic.

## One Aha

The interchange does not fail everywhere. The transfer fails at the fragile node.

Widening helps throughput, operations help recovery, and routing helps delay. None of those alone proves T1/T1 resilience. The proof requires redundant transfer paths and a recovery window that survives a major closure.

## Table Setup

Map zones:

| Zone | Function | Starting state |
|---|---|---|
| North T1 approach | I-35 freight approach | Open |
| South T1 approach | I-35 freight approach | Open |
| East T1 approach | I-80 freight approach | Open |
| West T1 approach | I-80 freight approach | Open |
| Interchange core | I-35/I-80 transfer | Fragile |
| I-235 local bypass | Passenger/local relief | Open, limited freight value |
| Relay/rest node | Driver swap and rest buffer | Available |
| Evidence drawer | Source and confidence cards | Partly locked |

Tracks:

| Track | Start | Failure condition |
|---|---:|---|
| Budget | 12 | Below 0 |
| Construction crews | 3 | No available crew for a required project |
| Political capital | 5 | Below 0 |
| Public patience | 6 | Below 0 |
| Operations capacity | 4 | Below 0 during an outage |
| Evidence confidence | 2 | Publication gate locked below 4 |

Time limit: 10 seasons. One season is one planning/build/stress cycle.

## Turn Sequence

1. Inspect map and evidence.
2. Draw or reveal one pressure card.
3. Choose up to two project cards.
4. Pay budget, crews, political capital, and patience.
5. Resolve construction and operations effects.
6. Run closure stress.
7. Score throughput retention, recovery, SLA, budget, support, and evidence.
8. Write a session-log row.

The first tutorial turn forces the `Full interchange-zone closure` event after the player inspects the normal map. Later turns draw from the event deck.

## Starting Engine Facts

Current CLI outputs on this repo:

| Command | Output used by prototype |
|---|---|
| `cargo run -q -p route -- sim scenario des-moines-interchange` | Baseline 86,671 vph; incident 83,423 vph; PTI 1.17; T90 0.9h |
| `cargo run -q -p route -- sim scenario des-moines-interchange --intervention` | Intervention restores throughput to 86,671 vph; incident PTI 1.37 to intervention PTI 1.36 |
| `cargo run -q -p route -- diamond all` | Current diamond ledger lists I-40/I-75 and I-80/I-90, not I-35/I-80 |
| `cargo run -q -p route -- diamond I35xI80` | Fails to find a matching T1/T1 intersection |

Prototype interpretation:

- Game win gate may use the scenario throughput restoration as heuristic evidence.
- Publication gate is locked until the diamond ledger and scenario naming agree on the I-35/I-80 node.
- The PTI intervention result is not strong enough to oversell; the teaching claim is topology and throughput restoration, not publication-grade PTI.

## Project Cards

| Card | Cost | Crew | Time | Effect | Evidence | Failure mode protected |
|---|---:|---:|---:|---|---|---|
| Diamond connector package | 5 | 2 | 3 | Unlocks redundant transfer paths; enables recovery gate attempt | Heuristic | Single interchange closure |
| Express freight flyovers | 4 | 2 | 2 | Protects freight transfer from local capture | Planned | Local congestion and connector closure |
| Work-zone sequencing | 1 | 1 | 1 | Reduces patience loss during closures | Heuristic | Construction backlash |
| Intelligent routing | 2 | 1 | 1 | Reduces incident delay penalty | Heuristic | Poor reroute timing |
| Relay hub reserve staffing | 2 | 1 | 1 | Protects driver swaps during outage | Heuristic | Missed relay windows |
| EV/rest hardening | 2 | 1 | 1 | Protects queue and dwell-time buffers | Heuristic | Rest/charging outage |
| General-purpose widening | 4 | 2 | 3 | Improves local capacity but does not add k-connectivity | Heuristic | Congestion-binding stress |
| Evidence acquisition | 1 | 0 | 1 | Raises evidence confidence by 1 | Implemented as artifact plan | Locked publication gate |

Project rule: the player may win the game with heuristic project effects, but the after-action report must keep publication gates locked when evidence remains weak.

## Event Cards

| Card | Trigger | Effect | Visible warning |
|---|---|---|---|
| Full interchange-zone closure | Forced tutorial, then rare | Apply closure stress; test transfer retention | "Transfer capacity is collapsing at the interchange zone." |
| Night work-zone closure | Common | Lose 1 patience unless sequencing is active | "The closure is short, but the warning signs are visible." |
| Relay hub surge | Medium | Lose 1 operations unless reserve staffing exists | "Pavement is open; operations are binding." |
| EV/rest queue | Medium | Lose SLA margin unless EV/rest hardening exists | "Rest and charging queues are now part of freight reliability." |
| Political lane-mile pressure | Common | Widening costs 1 less; connector package costs 1 political capital more | "The easy ribbon-cutting project is not the proof project." |
| Source challenge | Medium | Publication gate checks evidence confidence | "A reviewer asks what was observed versus modeled." |

## Evidence Cards

| Card | Known | Missing | Game effect |
|---|---|---|---|
| Scenario run | The closure scenario runs and gives bounded throughput outputs | Geometry validation for the intervention | Unlocks heuristic win scoring |
| Diamond ledger mismatch | `route diamond all` does not include I-35/I-80 | Node binding between scenario and diamond analyzer | Locks publication gate |
| Iowa 511 sample | Normalized observations exist for I-35/I-80 work-zone rows | Annual depth and closure-rate confidence | Unlocks low-confidence failure probability |
| NPMRDS/PTI | Source target is identified | Direct extract and validation | Locks publication-grade SLA proof |
| Standards proof ledger | T1/T1 standards have acceptance gates | Empirical top-site validation | Shows why the project matters |

## Scoring

Game score is separate from publication status.

| Dimension | Points | How earned |
|---|---:|---|
| Throughput retention | 25 | Closure stress retains at least 80% modeled throughput |
| Recovery | 20 | T90 or recovery proxy stays within 4 hours |
| SLA | 15 | PTI and relay/rest penalties stay within scenario bounds |
| Budget discipline | 10 | Budget remains non-negative |
| Public support | 10 | Public patience and political capital remain non-negative |
| Evidence honesty | 20 | The after-action report labels heuristic, planned, and source-needed claims correctly |

Win bands:

| Score | Result |
|---:|---|
| 80-100 | Operational win; publication may still be locked |
| 60-79 | Partial win; resilience works but support, budget, or evidence is weak |
| 0-59 | Failure; the player did not protect the network under adversity |

Publication gate:

- Pass requires the diamond analyzer to recognize the Des Moines node.
- Pass requires no `source_needed` field for the headline claim.
- Pass requires the after-action report to cite observed versus modeled failure data.

## Hint Ladder

| Tier | Hint |
|---|---|
| Nudge | "The backup route is not failing everywhere. The transfer is failing." |
| Push | "Count edge-disjoint paths through the 50-mile zone." |
| Shove | "Build redundancy first; then buy capacity." |

Hints are free in G0. Later campaign mode can subtract mastery score for push/shove use.

## Screen Sketches

| Screen | ASPECT aim | Encoding contract |
|---|---|---|
| Scenario board | Choose projects under scarcity | Tier = hue; stress = warning overlay; project cost = card track |
| Evidence drawer | Separate known/model/missing | Evidence level = badge; missing source = dashed outline |
| Pressure playback | Show before/after closure result | Throughput = stroke width; recovery = time marker |
| After-action report | Teach the lesson and lock/unlock publication | Gate status = badge; weak claim = explicit label |

No screen may rely on audio or prose alone to communicate a state change.

## Copy Deck

| Location | Text |
|---|---|
| Scenario hook | "The national freight grid looks healthy until one interchange closure breaks the transfer." |
| First warning | "Transfer capacity is collapsing at the interchange zone." |
| Widening warning | "More lanes help congestion, but they do not create another path through the failure." |
| Connector unlock | "Redundant transfer paths are now available. Test them under closure." |
| Publication lock | "Operational win recorded. Publication claim locked: the Des Moines diamond node is not validated in the diamond ledger." |
| Full win | "You protected the transfer and told the truth about the evidence." |

## Audio Cue List

| Cue | State change | SCORE rule |
|---|---|---|
| Closure hit | Full interchange-zone closure starts | Short, severe, mirrored by visual warning |
| Recovery pulse | Throughput retention clears threshold | Quiet relief, not victory music |
| SLA miss | PTI or relay/rest gate fails | Distinct from closure cue |
| Evidence unlock | Evidence confidence rises | Brief cue; no repeated fanfare |
| Publication lock | Game win but evidence gate fails | Low, unresolved cadence |
| Season silence | Player is choosing projects | Silence supports thinking |

Accessibility fallback: every cue has a visible event-log row and badge.

## Session Log Schema

Each paper playtest should write one row per season.

| Field | Example |
|---|---|
| `season` | `1` |
| `event_card` | `Full interchange-zone closure` |
| `projects_started` | `Diamond connector package; Work-zone sequencing` |
| `projects_completed` | `none` |
| `budget_remaining` | `6` |
| `political_capital` | `4` |
| `public_patience` | `5` |
| `operations_capacity` | `4` |
| `throughput_retention` | `0.962 heuristic` |
| `recovery_hours` | `0.9 heuristic` |
| `sla_status` | `bounded heuristic` |
| `evidence_confidence` | `2` |
| `publication_gate` | `locked: diamond ledger mismatch` |
| `player_note` | `Widening did not solve topology` |

## Verification Checklist

G0 is complete when:

- A player can run a 10-season paper game from this document without ROUTE internals.
- The playtest packet records a season log, score, aha check, surprise log, and promotion decision.
- The forced tutorial turn produces the intended topology aha.
- The after-action report separates operational win from publication proof.
- The diamond analyzer mismatch is either fixed in code/data or remains visibly labeled as a blocker.
- `route sim scenario des-moines-interchange --intervention` still runs.
- `data/pressure-test-scenarios.csv` still lists `S-L2-DES-MOINES` as executable heuristic evidence.
