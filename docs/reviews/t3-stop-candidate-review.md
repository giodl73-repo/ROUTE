# T3 Stop Candidate Review

Date: 2026-05-10

## Summary

T3 stop selection is not ready for regional schematic maps yet. The gate now uses the T3 service standard rather than T1/T2 endpoint assumptions:

- A T3 chain needs at least two visible stops.
- At least one stop must be transfer-grade (`S1`, `S2`, or `S3`).
- Endpoints in regional schematics must connect cleanly to named higher-tier or T3 stops, or be recorded as a regional terminal exception.
- Bends and crossings must be stop-driven, matching the Beck-map rule established for T1/T2.

Current gate result:

- `125` T3 routes evaluated.
- `3` routes pass under the T3 regional-chain rule: `I71`, `I74`, `I83`.
- `122` routes are blockers, mostly because no stop candidates have been authored yet.

## Interpretation

This is expected. T3 should be built zone by zone, not forced onto the national Beck schematic. The useful next unit is a regional/zone map where T3 feeders resolve into T1/T2/T3 transfer stops and regional terminals.

Seed zone plan:

- `data/t3-regional-zone-plan.csv`
- Start with `T3Z-GREAT-LAKES` because it has the three currently passing routes (`I71`, `I74`, `I83`).
- Use the other zones as stop-authoring queues, not as map-ready overlays.

## Commands

```powershell
cargo run -q -p route -- stop-coverage --tier T3
cargo run -q -p route -- stop-coverage --tier T3 --blockers
cargo run -q -p route -- stop-candidates --gate
```
