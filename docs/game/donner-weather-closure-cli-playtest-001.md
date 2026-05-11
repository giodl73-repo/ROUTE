# Donner Weather Closure CLI Playtest 001

Scenario: `donner-weather-closure`  
CLI slice: G1-A seed for paper v0.2 rules  
Date: 2026-05-10  
Status: pass with publication hold

## Goal

Verify that the executable `route game` layer now represents the Donner Weather Closure paper rules closely enough to support a first CLI playtest: scenario listing, inspect output, trapped-queue behavior, source-observed copy, score fixture, campaign status, and the underlying ROUTE sim hook.

## Commands

```powershell
cargo run -q -p route -- game scenarios
cargo run -q -p route -- game inspect donner-weather-closure
cargo run -q -p route -- game run-season donner-weather-closure --season 1 --event whiteout-closure --project dynamic-closure-routing --project rail-intermodal-surge-slots
cargo run -q -p route -- game score donner-weather-closure --log data/game/donner-weather-closure-session-fixture.csv --details
cargo run -q -p route -- game campaign --gate
cargo run -q -p route -- sim scenario donner-closure
cargo test -p route game::tests
cargo test --workspace
```

## Results

| Check | Result | Notes |
|---|---|---|
| Scenario list includes Donner | pass | `donner-weather-closure` appears with T1 climate/resilience/intermodal standards and locked publication gate |
| Inspect output matches v0.2 paper rules | pass | Project cards, storm cards, trapped-queue marker, and "source requested is not source observed" copy are visible |
| Whiteout + early routing resolves deterministically | pass | Dynamic routing completes before the event; trapped queue stays false; rail/intermodal remains active for one more season |
| Score fixture separates game win from publication proof | pass | 85/100 operational win; SLA is 0/15; publication remains locked |
| Campaign gate | pass | Mountain Pass status is `G0-B paper prototype; G1-A seed` |
| ROUTE sim hook | pass with caveat | Current synthetic `donner-closure` fixture reports no throughput delta, so CLI score output explicitly labels the game result heuristic |
| Tests | pass | `cargo test -p route game::tests`: 20 passed; `cargo test --workspace`: passed |

## Key Output

`route game score donner-weather-closure` reports:

| Field | Value |
|---|---|
| operational_score | 85/100 |
| win_band | Operational win |
| publication_gate | locked: weather closure and alternate-capacity evidence missing |
| engine note | current sim shows no throughput delta; game scoring remains heuristic |
| throughput_retention | 25/25 |
| recovery | 20/20 |
| sla | 0/15 |
| evidence_honesty | 20/20 |

## Findings

The CLI now captures the main v0.2 amendment: a whiteout can create a trapped queue unless egress or routing is ready. It also preserves the publication lock language. This is enough for a first G1-A seed, but not enough for G0-C or publication proof.

The main unresolved gap is still evidence, not game mechanics: the current Donner sim fixture is bound and executable, but it does not yet model proof-grade pass demand, alternate truck capacity, or direct PTI/SLA impacts.

## Promotion Decision

| Decision | Check | Note |
|---|---|---|
| Hold at G1-A seed | yes | CLI path exists and is test-covered |
| Promote to G0-C | no | Needs human blind playtest or owner acceptance of simulated evidence |
| Publication proof | no | Needs observed closure history, truck-capable alternate capacity, and direct PTI/SLA validation |

## Next Action

Run an owner/human review of the Donner CLI slice, then decide whether to proceed to a browser G2-A prototype or first tighten the underlying `donner-closure` sim with calibrated alternate-capacity evidence.
