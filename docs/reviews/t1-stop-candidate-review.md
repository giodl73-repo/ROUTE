# T1 Stop Candidate Review

Date: 2026-05-10

## Decision

The T1 stop-chain process is usable for schematic work, with one explicit hold:

- `I-395` should not receive a forced national stop chain. It is recorded in `data/tier-node-exceptions.csv` as a demotion candidate.

All other T1 routes have passing stop plans under the current gate.

## Findings

1. All `data/tier-stop-candidates.csv` rows remain `heuristic`.
   The ledger is good enough for schematic planning and review, not for publication-grade investment claims.

2. Several stops are intentionally merged schematic names.
   Examples: `Baltimore/Philadelphia`, `Louisville/Lexington Bluegrass`, `Madison/Milwaukee`, `New Orleans/Baton Rouge`, and `Columbia/Charleston Gateway`. These should be split or confirmed before final map labeling.

3. Some S2 promotions are based mainly on tier topology rather than external source evidence.
   Examples include `Barstow/Inland Empire Gateway`, `Oklahoma City`, `Research Triangle/Raleigh`, `Baltimore/Washington Gateway`, and `Cincinnati`. These are valid review candidates but need stronger evidence before they become public investment claims.

4. Border and port terminals are allowed to have a single route reference.
   This is correct for line endpoints such as `Laredo`, `San Diego/Tijuana`, and `Blaine/Vancouver Gateway`.

## Verified Commands

```text
cargo run -q -p route -- stop-coverage --tier T1
cargo run -q -p route -- stop-candidates --gate
cargo run -q -p route -- endpoint-exceptions --tier T1 --details
cargo test -q -p route stop_candidate
cargo test -q -p route stop_plan
cargo test -q -p route stop_coverage
```

## Next Review Target

Build Tier 2 stop coverage using T1 stop candidates as anchors. T2 should define thin-line service patterns rather than promote every local spur into a named national stop.
