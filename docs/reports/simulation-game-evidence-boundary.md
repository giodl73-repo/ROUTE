---
name: Simulation And Game Evidence Boundary
slug: simulation-game-evidence-boundary
type: report
status: draft
rubric_version: v1.0
author: copilot
created: 2026-06-16
updated: 2026-06-16
sources:
  - README.md
  - docs/game/interstate-tycoon-plan.md
  - docs/game/route-game-cli-design.md
  - docs/reports/optimizer-evidence-appendix.md
  - docs/reports/t3-t4-access-evidence-appendix.md
  - docs/reports/route-evidence-posture.md
  - data/pressure-test-scenarios.csv
  - data/throughput-proof-matrix.csv
  - data/game/t2-service-overlays.csv
  - data/game/t2-bundle-overlays.csv
  - data/game/t2-scenario-hooks.csv
  - data/t2-game-publication-evidence-review.csv
  - data/t2-game-publication-evidence-policy.csv
  - data/t2-game-publication-evidence-blocker-relief.csv
  - data/t2-game-ops-binding-intake.csv
  - data/t2-game-ops-binding-decisions.csv
  - data/t2-game-ops-bundle-evidence-review.csv
  - data/t2-game-ops-bundle-evidence-blocker-relief.csv
  - data/t4-terminal-scenario-readiness.csv
  - crates/route-sim/src/
  - crates/route-cli/src/game.rs
---

# Simulation And Game Evidence Boundary

## Purpose

This report explains how ROUTE can use simulation and Interstate Tycoon in the
communications package without confusing a playable or heuristic scenario with
proof-grade infrastructure evidence.

The current simulation/game stack is useful because it makes pressure visible:
closures, weather, port surges, relay outages, EV/rest-area outages, T2 service
overlays, local access gaps, and player tradeoffs can be inspected as bounded
scenarios. It is not a substitute for source-backed transportation validation,
agency review, release readiness, or construction authority.

## Core Rule

Simulation and game artifacts may teach, stress, compare, and preserve evidence
labels. They must not promote a claim beyond the evidence label carried by the
underlying artifact.

```text
scenario hook
  -> evidence label
  -> player or reviewer action
  -> operational/game outcome
  -> publication gate
```

The operational/game outcome and the publication gate are separate decisions.
A scenario can be playable and useful while its publication proof remains held.

## Current Artifact Inventory

These counts are local artifact counts from the current repo snapshot, not
claims that the simulation/game program is public-ready.

| Artifact | Rows / Scope | What It Shows | Boundary |
|---|---:|---|---|
| `data/pressure-test-scenarios.csv` | 8 | L2 pressure scenarios, adversity classes, standards tested, current status, blocking gaps, and next evidence steps. | All current rows are heuristic; they do not close L2 proof. |
| `data/throughput-proof-matrix.csv` | 4 | Throughput/resilience proof bindings, primary metrics, existing artifacts, blocking gaps, and next evidence steps. | Matrix rows are proof posture, not proof closure. |
| `data/game/t2-service-overlays.csv` | 4 | T2 service classes mapped to scenario hooks, levers, and release gates. | Service-class overlay does not bind every route. |
| `data/game/t2-bundle-overlays.csv` | 40 | Bundle-bound or held T2 game overlay rows with service class, scenario hooks, debt, binding status, and next artifacts. | Held rows remain blocked from game/ops use. |
| `data/game/t2-scenario-hooks.csv` | 3 | Scenario hooks for transfer-spine, long-connector, and connector choices. | Hooks carry evidence holds and are not public proof. |
| `data/t2-game-publication-evidence-review.csv` | 3 | Publication evidence review rows for game scenarios. | Review preserves blockers until evidence policy and replay. |
| `data/t2-game-publication-evidence-policy.csv` | 3 | Policy rows defining required evidence and publication treatment. | Policy is not publication approval. |
| `data/t2-game-publication-evidence-blocker-relief.csv` | 3 | Accepted relief replay rows for selected publication evidence blockers. | Relief is bounded to the accepted blocker class. |
| `data/t2-game-ops-binding-intake.csv` | 16 | Intake rows for T2 game/ops bundle-binding blockers. | Intake keeps blockers visible; it does not repair them. |
| `data/t2-game-ops-binding-decisions.csv` | 16 | Bundle-binding decisions and repair routing. | Residual held/repair rows cannot be used as ready overlays. |
| `data/t2-game-ops-bundle-evidence-review.csv` | 16 | Evidence review for residual T2 game/ops bundle-binding rows. | Review does not reduce blockers by itself. |
| `data/t2-game-ops-bundle-evidence-blocker-relief.csv` | 16 | Accepted bounded relief rows for game/ops bundle evidence. | Relief does not imply browser or public release readiness. |
| `data/t4-terminal-scenario-readiness.csv` | 1 held docket | Records that T4 terminal scenarios remain empty until source-backed contact rows exist. | No T4 terminal row is scenario-ready or release-ready. |
| `crates/route-sim/src/` | Rust crate | Incidents, assignment, OD, chaos, hub, relay, EV/rest, and scenario code. | Simulation output remains labeled by calibration and source posture. |
| `crates/route-cli/src/game.rs` | Rust module | Deterministic game state, scenario cards, season resolution, scoring, and evidence labels. | Game scoring is not publication proof. |

## Evidence Labels

Use these labels consistently in decks, reports, demos, and game screens.

| Label | Meaning | Allowed Use |
|---|---|---|
| Observed | Backed by accepted source evidence for the specific claim. | May support stronger evidence claims if role review and gates agree. |
| Modeled | Generated by a defined model with accepted inputs and stated assumptions. | May support bounded analytical claims. |
| Heuristic | Useful scenario logic or proxy values with known blockers. | May teach or stress-test; must not be used as proof. |
| Planned | Designed but not implemented or not wired to current artifacts. | May describe roadmap only. |
| Source-needed | Evidence task exists but accepted source proof is missing. | Must remain held. |
| Held-known | Artifact exists and the blocker is intentionally preserved. | May be shown as a blocker, not as readiness. |

## Simulation Boundary

`route-sim` can make pressure legible:

- T1/T1 interchange closure and diamond intervention;
- mountain pass weather closure;
- urban peak and managed-lane stress;
- hurricane, flood, and port surge;
- long-haul SLA, relay, EV/rest, and outage scenarios;
- incident, chaos, OD, hub, assignment, and demand experiments.

The current pressure-test catalog is useful precisely because it names blocking
gaps. For example, current scenarios still need geometry validation,
calibration, alternate-route sensitivity, observed PTI/source-speed evidence,
operator plans, station inventories, or source-backed assumptions before they
can support stronger proof.

Safe claim: "ROUTE can run bounded simulation scenarios and preserve their
blocking gaps."

Held claim: "The scenario proves a real-world SLA, resilience, or throughput
result."

## Game Boundary

Interstate Tycoon is the playable learning layer. Its job is to help a player
experience why standards, evidence labels, and tradeoffs matter.

The current design already separates:

- paper scenarios and playtests;
- deterministic CLI state transitions;
- browser prototype checks;
- campaign spine;
- operational game win;
- publication gate.

The publication gate is the key. A player may win a scenario operationally while
the evidence drawer still says the claim is heuristic, source-needed, or held.

Safe claim: "The game teaches the evidence posture of a scenario."

Held claim: "A game win proves the infrastructure choice should be adopted."

## Overlay Boundary

Game overlays must bind to service objects, not hand-picked route labels.

The current T2 overlay chain does three useful things:

1. Defines service-class hooks and player levers.
2. Attempts to bind overlays to `segment_bundle_id` rows.
3. Preserves game/ops blockers when service class, bundle readiness, stop
   chain, local-zone handoff, or evidence policy is not ready.

This keeps Interstate Tycoon from becoming a cosmetic map over unreviewed
network rows. If a row is `service-class-held-known`, `repair-needed`, or
otherwise held, the game may show the issue as a blocker or lesson but cannot
use it as a ready overlay.

## Public-Use Guardrails

| Use This | Avoid This |
|---|---|
| "This scenario is heuristic and shows what would need validation." | "This scenario proves the project works." |
| "The player outcome is separate from the publication gate." | "Winning the game proves the policy." |
| "The game preserves evidence labels and blockers." | "The game is a public proof surface." |
| "A bundle-bound overlay can be inspected." | "A route label is enough for game or ops binding." |
| "Held rows can appear as blockers or lessons." | "Held rows are ready gameplay or release content." |
| "Browser/game release claims need L2 and publication gates." | "The browser prototype is public-ready." |

## Reviewer Pressure Questions

- Which scenario id owns the claim?
- Is the scenario observed, modeled, heuristic, planned, source-needed, or
  held-known?
- What are the blocking gaps and next evidence steps?
- Does the game output show the publication gate separately from the player win?
- Does the overlay bind to `segment_bundle_id` and preserve bundle status?
- Are held rows displayed only as blockers, lessons, or review work?
- Has the browser/game surface passed the required L2 checks, or is it still a
  prototype?
- Does any slide, report, or demo imply approval, construction, SLA, ROI, or
  public readiness from a scenario or game result?

## Communications Use

Simulation and game material can be used in internal reviews when captions or
speaker notes preserve the label:

- "heuristic pressure scenario";
- "deterministic game fixture";
- "source-needed terminal scenario docket";
- "bundle-bound overlay";
- "held game/ops binding row";
- "publication gate locked";
- "browser prototype, not public release."

External rehearsal use remains held until the external readiness packet selects
the exact scenario, verifies its source posture, runs affected role review,
passes the prohibited-claim scan, and closes the applicable L0/L1/L2 gates.

## Non-Goals

- This report does not make any game scenario public-ready.
- This report does not close browser, release, or L2 readiness.
- This report does not convert heuristic simulations into observed evidence.
- This report does not claim construction readiness, operating SLA, positive
  ROI, agency endorsement, legal eligibility, compliance, or public approval.
- This report does not authorize T4 terminal scenario readiness; the current
  terminal scenario docket remains held until source-backed contact proof
  exists.

## Gate

Decision: pass_with_risk for internal communications review.

Rationale: ROUTE has a strong simulation/game evidence story when labels,
blockers, bundle identity, and publication gates are preserved. The package can
use simulation and game artifacts for internal pressure testing and teaching,
but external/public game, browser, SLA, resilience, throughput, ROI, or release
claims remain gated by source evidence, scenario validation, affected role
review, publication gates, and L2 readiness.
