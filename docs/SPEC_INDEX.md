# ROUTE Spec Index

Start here when deciding which document owns a claim.

| Document | Owns | Use when |
|---|---|---|
| `docs/SYSTEM_PLAN.md` | Living roadmap, Milepost phases, roles, truth labels, forward plan | You need the current operating plan |
| `docs/DIMENSIONS.md` | Current 16-dimension registry, evidence path, and truth label | You need the canonical rubric dimension list |
| `docs/STANDARDS_EVALUATION.md` | Standards proof obligations, pressure-test gates, and T1/T1 evaluation framing | You need to know whether a tier standard has earned its place |
| `docs/INTERSTATE_TYCOON.md` | Game-facing product concept that translates ROUTE simulations into a highway tycoon experience | You need to explain or prototype ROUTE as a playable game |
| `docs/game/interstate-tycoon-plan.md` | Execution plan for the Interstate Tycoon paper, CLI, browser, campaign, and public-demo phases | You need the current game build sequence and gates |
| `docs/game/des-moines-diamond-g0.md` | First playable paper prototype for the Interstate Tycoon T1/T1 tutorial scenario | You need the G0 rules, cards, screen/copy/audio contracts, or session-log shape |
| `docs/game/des-moines-diamond-amendments.md` | Forward-only amendment log for Des Moines Diamond rules and copy | You need to see which playtest findings changed the scenario |
| `docs/game/des-moines-diamond-playtest.md` | Score sheet, season log, surprise log, and promotion checklist for Des Moines Diamond playtests | You need to run or review a G0 playtest |
| `docs/game/des-moines-diamond-playthrough.md` | Narrated reference playthrough showing the intended topology aha and publication lock | You need the Tier-B transcript for review or blind-playtest comparison |
| `docs/game/des-moines-diamond-blind-playtest-001.md` | First simulated blind-player playtest log and amendment candidates | You need the first G0 playtest evidence record |
| `docs/game/des-moines-diamond-blind-playtest-002.md` and `003.md` | Follow-on v0.2 simulated playtests for optimizer and evidence-first personas | You need post-amendment playtest evidence |
| `docs/game/des-moines-diamond-playtest-synthesis.md` | Cross-playtest synthesis and promotion readiness decision | You need the current G0-B/G0-C evidence summary |
| `docs/game/des-moines-diamond-panel-g0.md` | TIGRIS/HUNT/ASPECT/PROSE/SCORE/QUEST panel decision for the first Des Moines paper scenario | You need the G0-B pass, G0-C hold, or G1-A implementation requirements |
| `docs/game/des-moines-diamond-cli-playtest-001.md` | First G1-A terminal playtest for `route game scenarios`, `inspect`, and live engine hooks | You need to verify the first game CLI slice against the paper scenario |
| `docs/game/des-moines-diamond-cli-playtest-002.md` | First deterministic `run-season` CLI playtest with state write, log append, and resume | You need to verify season resolution before scoring |
| `docs/game/des-moines-diamond-cli-playtest-003.md` | Active-project countdown playtest for multi-season connector completion | You need to verify construction timing before score bands |
| `docs/game/des-moines-diamond-cli-playtest-004.md` | First score-command playtest for operational score versus publication hold | You need to verify the full G1-A CLI loop |
| `docs/game/des-moines-diamond-g2-plan.md` | Browser-prototype plan for the map-first Des Moines scenario board | You need to start or review G2-A UI work |
| `docs/game/des-moines-diamond-browser-playtest.md` | Human browser blind-playtest packet for the G2-A prototype | You need to run or review the browser comprehension/export/aha gate |
| `docs/game/des-moines-diamond-browser-playtest-001.md` | Simulated browser playtest baseline for the G2-A prototype | You need the reproducible browser baseline before a human run |
| `docs/game/browser/des-moines-diamond.html` | Static first G2-A browser prototype for fixture playback | You need to open or inspect the map-first Des Moines game board |
| `docs/game/browser/check-des-moines-browser.ps1` | Static fixture-contract check for the Des Moines browser prototype | You need to verify browser fixture copy and required regions without a browser harness |
| `docs/game/browser/des-moines-browser.spec.js` | Playwright desktop/mobile smoke check for the Des Moines browser prototype | You need screenshot-era browser regression coverage |
| `docs/game/route-game-cli-design.md` | G1 `route game` command, state, scoring, gate, and test design | You need to implement or review the CLI game layer |
| `data/game/des-moines-diamond-session-fixture.csv` | Canonical G1-A score fixture for operational win with publication hold | You need stable score-output regression coverage |
| `data/game/des-moines-diamond-state-fixture.json` | Canonical G2 seed state showing completed connector with publication hold | You need browser/campaign state fixture data |
| `data/pressure-test-scenarios.csv` | L2 scenario catalog and readiness/blocker labels | You need to know which adversity scenarios are real pressure tests versus named shells |
| `data/t1-intersection-failures.csv` | T1/T1 failure-rate, duration, throughput-retention, and reroute evidence ledger | You need to know whether T1/T1 resilience claims have empirical incident anchors |
| `data/t1-diamond-validation.csv` | T1/T1 diamond anchor manual-validation ledger | You need to know which curated T1/T1 anchors are recognized, manually validated, or still heuristic |
| `data/t1-failure-source-plan.csv` | Source acquisition plan for T1/T1 failure-rate and reroute fields | You need to know which DOT/FHWA data systems can fill failure evidence gaps |
| `data/t1-source-health.csv` | Source health/status ledger for T1/T1 evidence ingestion | You need to know whether a source is live, blocked, key-gated, or archive-only |
| `data/t1-failure-events.csv` | Normalized raw T1/T1 incident, closure, and work-zone observations | You need to compute empirical annual failure rates and duration percentiles |
| `specs/2026-05-06-route-design.md` | Core ROUTE method: corpus, dimensions, gap map, parliament, design proposals | You need the conceptual process |
| `specs/2026-05-06-route-rust-architecture.md` | Rust workspace architecture, CLI contracts, data model, output formats | You need implementation boundaries |
| `specs/2026-05-06-interstate-2-design.md` | Interstate 2.0 feature set, investment thesis, simulation toolkit, transit integration | You need the national design framework |
| `specs/2026-05-06-tier-standards.md` | T1/T2/T3/T4 tier standards and service expectations | You need tier definitions |
| `research/MODULE.md` | Tracks A-F, paper chain, quantification contracts, review history | You need the research program |
| `.roles/ROLE.md` | Parliament, stakeholder, editorial, and panel-review role index | You need review gates or role selection |
| `TRACKER.md` | Current status board | You need the live project state |

---

## Ownership Rules

1. If a claim describes what ROUTE is trying to do, update `specs/2026-05-06-route-design.md`.
2. If a claim describes how the Rust system works, update `specs/2026-05-06-route-rust-architecture.md`.
3. If a claim describes Interstate 2.0 as a build program, update `specs/2026-05-06-interstate-2-design.md`.
4. If a claim changes the roadmap, phase theme, or done criteria, update `docs/SYSTEM_PLAN.md` and `TRACKER.md`.
5. If a claim appears in a paper, make sure `research/MODULE.md` still names the dependency and quantification contract.
6. If a claim requires judgment, attach the relevant `.roles` pass before calling it validated.

---

## Claim Status Vocabulary

Use the same status words across specs, README, CLI help, and papers:

| Status | Meaning |
|---|---|
| Implemented | Code runs end-to-end and is testable |
| Heuristic | Code runs but uses a proxy, partial data, or simplified model |
| Stub | Interface exists but the analysis is not real yet |
| Planned | Specified but not implemented |
| Deprecated | Historical and no longer a current claim |

When in doubt, mark a claim lower. ROUTE is stronger when it is honest about what is measured versus what is proposed.
