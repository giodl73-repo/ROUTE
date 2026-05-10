# Interstate Tycoon Plan

Interstate Tycoon is the public, playable proof surface for ROUTE. It should let a player experience why Interstate 2.0 standards exist, while keeping the engineering evidence honest enough that the same scenario can feed Milepost 4 pressure tests and later Blueprint claims.

The plan is to build the game in layers:

1. Paper game that proves the rules and lesson.
2. CLI game state that proves reproducibility.
3. Browser prototype that proves the experience.
4. Campaign that proves standard-by-standard learning.
5. Public demo that sells Interstate 2.0 without hiding uncertainty.

## North Star

A player inherits today's highway network, watches it fail under real pressure classes, chooses infrastructure investments, and learns which standards protect throughput, SLA, resilience, access, and public accountability.

The game succeeds when a non-specialist can say:

- What failed.
- Which standard would have helped.
- What the tradeoff cost.
- Whether the proof was observed, modeled, heuristic, planned, or source-needed.

## Current State

| Artifact | Status | Notes |
|---|---|---|
| Game concept | Drafted | `docs/INTERSTATE_TYCOON.md` |
| First paper scenario | Drafted | `docs/game/des-moines-diamond-g0.md` |
| First playtest packet | Drafted | `docs/game/des-moines-diamond-playtest.md` |
| First narrated playthrough | Drafted | `docs/game/des-moines-diamond-playthrough.md` |
| First simulated blind playtest | Drafted | `docs/game/des-moines-diamond-blind-playtest-001.md` |
| Simulated playtest set | Drafted | `docs/game/des-moines-diamond-blind-playtest-001.md`, `002.md`, `003.md` |
| Playtest synthesis | Drafted | `docs/game/des-moines-diamond-playtest-synthesis.md` |
| G0 panel review | Complete | `docs/game/des-moines-diamond-panel-g0.md`; G0-B pass, G0-C held for human blind playtest or owner acceptance of simulated evidence |
| First CLI playtest | Complete | `docs/game/des-moines-diamond-cli-playtest-001.md`; list/inspect passes against live scenario and diamond hooks |
| First run-season playtest | Complete | `docs/game/des-moines-diamond-cli-playtest-002.md`; deterministic event/project resolution, state write, log append, and resume pass |
| Active countdown playtest | Complete | `docs/game/des-moines-diamond-cli-playtest-003.md`; multi-season connector completes before closure resolution |
| First score playtest | Complete | `docs/game/des-moines-diamond-cli-playtest-004.md`; operational win and publication hold are scored separately |
| First amendment log | Drafted | `docs/game/des-moines-diamond-amendments.md` |
| CLI design note | Drafted | `docs/game/route-game-cli-design.md` |
| Scenario engine hook | Heuristic executable | `route sim scenario des-moines-interchange` |
| Intervention hook | Heuristic executable | `route sim scenario des-moines-interchange --intervention` |
| Pressure-test catalog row | Present | `S-L2-DES-MOINES` |
| Diamond analyzer link | Fixed | `route diamond I35xI80` recognizes the curated Des Moines anchor |
| Campaign spine | Drafted | 8-scenario proof arc |
| CLI game command | G1-B starting | `route game score` now derives Des Moines engine facts from route-sim and route-network when run from the CLI |
| Browser plan | Drafted | `docs/game/des-moines-diamond-g2-plan.md`; map-first G2-A slice and Playwright gates defined |
| First browser prototype | Drafted | `docs/game/browser/des-moines-diamond.html`; static map-first fixture playback |
| Browser prototype | Not started | Map-first UI |

## Learning Inputs Folded In

Interstate Tycoon is not inventing a new process. It folds the portfolio's working loops into one game-building machine.

| Source | Learning absorbed | Game consequence |
|---|---|---|
| ROUTE | Standards must earn their place under pressure | Every scenario maps fun choices to proof gates |
| TIGRIS | Board games grow through axes, gaps, Parliament, and playthroughs | Each scenario gets a TIGER BEAT target, design review, and promotion gate |
| HUNT | Complex systems are learned through fair reveal and one aha | Each tutorial teaches one infrastructure insight through play |
| ASPECT | Screens need aim, school, precision, effect, clarity, and truth | Every screen has explicit encodings and visible evidence labels |
| PROSE | Purpose, reader, organization, style, and economy govern text | UI copy and after-action reports are reviewed as writing |
| SCORE | Structure, craft, originality, resonance, and economy govern audio/tempo | Cues follow real state changes and silence remains a tool |
| QUEST | Campaigns learn from session logs, surprises, and persistent consequences | Seasons write logs; consequences carry forward |
| SIGNALS | Skills are earned from repeated decision evidence | Repeated playtest findings can become new game skills or gates |
| PANEL | Claims improve through adversarial review before publication | Scenario promotion requires panel findings resolved or logged |
| PROOF/MDPATH | Artifacts should compile, link, and survive refactors | Game docs and future score sheets should be addressable, validated artifacts |

## Growth Loop

The game grows by running the same forward-only evidence loop as the rest of the portfolio.

```
SCENARIO SEED
  -> PAPER PLAYTEST
  -> SESSION LOG
  -> SURPRISE LOG
  -> PANEL REVIEW
  -> RUBRIC / RULE AMENDMENT
  -> CLI ENCODING
  -> BROWSER PLAYTEST
  -> CAMPAIGN PROMOTION
```

Rules:

- Old playtests are judged by the rubric version they used.
- New scenarios must meet the current rubric.
- Findings do not become rules after one anecdote.
- Two repeated findings create an amendment candidate.
- Three repeated findings across scenarios create a candidate skill, gate, card type, or player-style label.
- A scenario can be fun and still fail publication if evidence is weak.
- A scenario can be technically correct and still fail if players do not learn the intended aha.

## Promotion Ladder

| Level | Name | Entry condition | Exit condition |
|---|---|---|---|
| G0-A | Seed | Standard, stressor, and intended aha are named | Paper rules exist |
| G0-B | Paper playable | Cards, tracks, scoring, and session log exist | Blind player can complete one run |
| G0-C | Aha proven | Playtest shows the intended lesson landed | Panel issues are resolved or logged |
| G1-A | CLI encoded | State, events, projects, and scoring are deterministic | L0/L1 tests pass |
| G1-B | Engine-backed | CLI calls or summarizes ROUTE proof artifacts | Evidence labels survive scoring |
| G2-A | Browser playable | Map-first UI supports a full run | Playwright layout checks pass |
| G2-B | Public preview | Evidence drawer, after-action, and accessibility pass | Non-specialist player can explain the lesson |
| G3 | Campaign node | Scenario has persistence consequences | It fits the campaign spine |
| G4 | Demo-ready | Scenario is polished and honest | Public claims link to artifacts |

## Game Rubric

Each scenario gets scored on six dimensions before promotion.

| Dimension | Question | Gate |
|---|---|---|
| Proof | Does the scenario test a real ROUTE standard under a named stressor? | Required |
| Aha | Does play make the intended infrastructure lesson visible? | Required |
| Choice | Are there meaningful tradeoffs, not a disguised correct answer? | Required |
| Evidence | Are observed, modeled, heuristic, planned, and source-needed labels visible? | Required |
| Experience | Does the scenario have tension, clarity, and replayable texture? | Required for browser |
| Reproducibility | Can the same inputs regenerate the same result? | Required for CLI |

Promotion requires no required dimension below "pass." A scenario with weak evidence can promote as a game lesson, but not as a publication proof.

## Artifact Trail

Each scenario should eventually have the following artifacts.

| Artifact | Purpose |
|---|---|
| Scenario brief | Hook, standard, stressor, aha, audience |
| Paper rules | Cards, tracks, turn order, scoring |
| Session log | Per-season decisions, events, scores, evidence labels |
| Surprise log | Where players learned, stalled, exploited, or misunderstood |
| Panel notes | TIGRIS/HUNT/ASPECT/PROSE/SCORE/QUEST review findings |
| Amendment log | Rule, rubric, or card changes caused by evidence |
| CLI fixture | Deterministic state and expected score |
| Browser fixture | Screenshot and interaction checks |
| Publication gate | Observed/model/heuristic/source-needed status |

## Phase G0 - Paper Prototype

Goal: prove that Des Moines Diamond teaches the T1/T1 topology lesson before we build software around it.

Primary artifact: `docs/game/des-moines-diamond-g0.md`

Scope:

- 10-season paper scenario.
- Map zones, resource tracks, project cards, event cards, evidence cards.
- Forced first-turn closure.
- Scoring split between operational win and publication gate.
- Screen, copy, audio, and session-log contracts.

Done criteria:

- A player can run the game from the document without reading Rust code.
- The first forced closure produces the topology aha.
- The after-action report clearly separates heuristic win from publication proof.
- The Des Moines diamond anchor remains recognized by `route diamond I35xI80`.
- `route sim scenario des-moines-interchange --intervention` still runs.

Immediate tasks:

| Task | Status | Output |
|---|---|---|
| Write G0 scenario artifact | Done | `docs/game/des-moines-diamond-g0.md` |
| Add playtest score sheet | Done | `docs/game/des-moines-diamond-playtest.md` |
| Add first narrated playthrough | Done | `docs/game/des-moines-diamond-playthrough.md` |
| Decide mismatch handling | Done | Curated Des Moines anchor added to diamond analyzer |
| Run first panel pass | Done | `docs/game/des-moines-diamond-panel-g0.md`; G1-A implementation may begin |

## Phase G1 - CLI Prototype

Goal: turn the paper scenario into reproducible state transitions backed by ROUTE outputs.

Commands:

| Command | Purpose |
|---|---|
| `route game scenarios` | List playable game scenarios and evidence status |
| `route game inspect des-moines-diamond` | Print setup, cards, gates, and current blockers |
| `route game run-season des-moines-diamond --event ... --project ...` | Resolve one season deterministically |
| `route game score des-moines-diamond --log ...` | Score a session log and publication gate |

Done criteria:

- CLI prints the same setup as the paper artifact.
- Season resolution is deterministic from event/project inputs.
- Outputs include evidence labels.
- Session logs can be scored without browser/UI code.
- L0/L1 tests cover parsing, scoring, gate labels, and deterministic season resolution.

Implementation notes:

- Add a small `route-game` module or a bounded module inside `route-cli` first.
- Store scenario definitions as structured data only after the paper rules stabilize.
- Do not duplicate simulation math; call or summarize existing ROUTE outputs.
- Keep the game state deterministic so QUEST-style checkpoint/resume is natural.

## Phase G2 - Browser Prototype

Goal: make one scenario playable as a map-first experience.

First screen:

- Scenario board, not a landing page.
- Des Moines map zones visible immediately.
- Project cards and evidence drawer available without tutorial text blocking play.

Required screens:

| Screen | Job | Gate |
|---|---|---|
| Scenario board | Choose projects under scarcity | Stable ASPECT encodings |
| Evidence drawer | Separate observed/model/missing | Evidence labels visible |
| Pressure playback | Show before/after closure | Throughput/recovery change visible |
| After-action report | Teach result and lock/unlock publication | PROSE copy passes first-reader test |

Done criteria:

- User can complete one 10-season run.
- Visual encodings match `docs/INTERSTATE_TYCOON.md`.
- Audio cues are optional and mirrored visually.
- No game-critical state is audio-only or prose-only.
- Playwright screenshots verify desktop and mobile layouts are non-overlapping.

## Phase G3 - Campaign

Goal: make the pressure-test library into a standard-by-standard learning arc.

Scenario order:

| Order | Scenario | Standard lesson |
|---:|---|---|
| 1 | Des Moines Diamond | T1/T1 topology and recovery |
| 2 | Donner Weather Closure | climate resilience and alternate capacity |
| 3 | Atlanta Managed-Lane Stress | managed freight lanes and downstream merge honesty |
| 4 | Houston Port Surge | port access, flood stress, and relief corridors |
| 5 | NY-LA 48-Hour SLA | relay buffers, PTI, and shipper planning windows |
| 6 | Relay Network Outage | operations capacity and workforce reliability |
| 7 | EV/Rest Hardening | energy/rest standards under outages |
| 8 | Blueprint Hearing | evidence labels and public proof |

Done criteria:

- Every scenario has one aha.
- Every scenario names the standards it teaches.
- Every scenario has a playable win gate and a publication gate.
- Campaign consequences persist across scenarios.
- The final hearing can pass only if evidence labels are honest.

## Phase G4 - Public Demo

Goal: package Interstate Tycoon as the friendly front door for ROUTE.

Audience modes:

| Mode | Audience | Emphasis |
|---|---|---|
| Player | general public | playable decisions and visible consequences |
| Reviewer | technical reader | evidence labels, commands, and gates |
| Policymaker | decision maker | costs, tradeoffs, public outcomes |
| Developer | contributor | reproducible CLI and test path |

Done criteria:

- The demo explains Interstate 2.0 better than a standards document alone.
- Every public claim links back to a ROUTE artifact.
- Heuristic claims cannot be mistaken for observed proof.
- README points to the demo and research path.

## Cross-Cutting Gates

| Gate | Applies to | Pass condition |
|---|---|---|
| Evidence honesty | all phases | Observed/model/heuristic/planned/source-needed labels remain visible |
| One aha | every scenario | Player can state the intended lesson after play |
| Reproducibility | CLI and beyond | Same inputs produce same state and score |
| Accessibility | browser and demo | Audio is optional; visual state does not depend on color alone |
| Panel review | scenario promotion | TIGRIS/HUNT/ASPECT/PROSE/SCORE/QUEST notes resolved or logged |
| Publication lock | public claims | Source-needed headline claims stay locked |

## Immediate Next Actions

1. Add browser screenshot/layout checks for `docs/game/browser/des-moines-diamond.html`.
2. Keep browser state compatible with `data/game/des-moines-diamond-state-fixture.json`.
3. Decide whether simulated G0-C readiness is enough after the first CLI inspect/playtest pass.
4. Run at least one human blind playtest or record explicit owner acceptance of simulated G0 evidence.
5. Expand curated/validated T1/T1 anchors beyond Des Moines.
6. Run `cargo test --workspace` after any Rust changes; doc-only changes use `git diff --check`.

## Open Decisions

| Decision | Options | Current leaning |
|---|---|---|
| Scenario source of truth | Markdown first, TOML later, or TOML now | Markdown first until G0 playtest stabilizes |
| First implementation | CLI-only or browser-first | CLI-first for reproducibility |
| Diamond mismatch | Fix analyzer/data or choose a recognized T1/T1 node | Fixed with a curated Des Moines analyzer anchor; expand to all 15 later |
| Audio | Generated cues, simple web audio, or silent first demo | Silent-capable first demo with cue contract |
| Campaign persistence | JSON session log or custom save format | JSON session log shaped by G0 table |

## Rule

The game can make ROUTE easier to feel. It cannot make ROUTE less true.
