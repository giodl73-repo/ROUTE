# T2 Stop Candidate Review

Date: 2026-05-10

## Decision

The T2 stop process is ready for thin-line schematic work with a held exception set.

Current result:

- `23` T2 routes have passing stop plans under the T2 stop-chain rule.
- `17` T2 routes remain blockers.
- The blocker set is recorded in `data/tier-node-exceptions.csv` as demotion, local-access, or graph-disambiguation work.

## T2 Gate Rule

T2 stop coverage is intentionally lighter than T1:

- at least two stops for a visible thin-line chain;
- at least one `S1` or `S2` terminal/major-hub anchor;
- all stop rows must pass the stop-candidate contract gate.

This lets a T2 connector be legible without pretending it has the same national station density as a T1 trunk.

## Passing Thin-Line Families

The first T2 pass supports connector and relief lines such as:

- `I-15`
- `I-22`
- `I-24`
- `I-25`
- `I-29`
- `I-37`
- `I-44`
- `I-49`
- `I-59`
- `I-65`
- `I-76`
- `I-77`
- `I-81`
- `I-85`
- `I-495`
- `US-6`
- `US-30`
- `US-70`
- `US-80`
- `US-83`
- `US-90`

These are candidates for the thin colored T2 map layer.

## Held/Demotion Set

The remaining blockers should not be forced onto the national thin-line map without more evidence:

- local or urban access loops: `I-110`, `I-220`, `I-225`, `I-240`, `I-264`, `I-275`, `I-610`, `I-635`, `I-664`, `I-680`, `US-2`
- metro beltways needing source-backed relief proof: `I-285`, `I-405`
- graph or route-family disambiguation: `I-195`, `I-205`, `I-295`
- one-ended/ambiguous relief family: `I-270`

## Review Notes

All rows remain `heuristic`. The T2 layer is suitable for planning and schematic iteration, not publication-grade investment claims.

The main risk is over-promoting S2 anchors for US-route connectors. Those should stay visually thinner than T1 hubs unless source review confirms the service package.

## Verified Commands

```text
cargo run -q -p route -- stop-coverage --tier T2
cargo run -q -p route -- stop-coverage --tier T2 --blockers
cargo run -q -p route -- endpoint-exceptions --tier T2 --details
cargo run -q -p route -- stop-candidates --gate
cargo test -q -p route stop_coverage
```
