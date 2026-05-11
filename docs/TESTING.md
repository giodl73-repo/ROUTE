# Testing Layers

ROUTE uses three practical regression layers.

| Layer | Scope | Command |
|---|---|---|
| L0 | Rust unit and crate-local contract tests | `npm run check:l0` |
| L1 | Rust workspace regression suite, including CLI contract tests | `npm run check:l1` |
| L2 | System smoke/e2e: canonical CLI flows plus browser prototype | `npm run check:l2` |

## L2 Coverage

The CLI e2e harness lives in `crates/route-cli/tests/e2e_cli.rs`.

It verifies:

- Beck T2 map generation writes a real PNG artifact.
- Stop-to-stop SLA surface generation writes a CSV with heuristic evidence labels.
- Stop SLA summary gate keeps max stop gaps under 360 miles.
- Stop SLA candidate scoring produces a reviewable candidate docket for oversized
  recurring gaps and gates that inspected gaps have candidates.
- Stop SLA promotion scaffolding converts the candidate docket into
  stop-candidate-shaped source-needed rows and gates the result.
- Stop SLA promotion defaults to the best non-ledger candidate per gap, keeping
  alternates opt-in for review.
- Map atlas gate passes.
- L2 pressure-scenario coverage gates pass.
- T1 stop coverage remains visible and reports the known `I395` blocker.

The browser e2e suite lives in `docs/game/browser/des-moines-browser.spec.js`.

It verifies:

- Desktop game board, evidence, playback, and publication-lock affordances.
- Mobile scenario-board visibility.
- Local season mutation and CSV download behavior.
