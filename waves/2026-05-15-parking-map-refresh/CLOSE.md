---
wave: 2026-05-15-parking-map-refresh
date_closed: 2026-05-15
status: done
---

# Close: Parking Map Refresh

## Result

All 17 atlas maps were regenerated from `data/map-atlas.csv` before parking the
project. The national, Beck, T2 Beck, T2-only Beck, and T1 regional PNGs changed.
The five T3 zone maps regenerated cleanly and remained byte-identical.

## Parked State

- Map publication readiness: 17 maps, zero publication blockers, held
  `evidence;sla;transit;upgrade` labels preserved.
- Optimizer residual backlog: T2 asset-condition repair debt at 2 rows / $30.0M
  plus the T1 live-event snapshot evidence guard.
- No active wave.

## Gates

- `cargo run -q -p route -- map-atlas --gate`
- `cargo run -q -p route -- map-publication-readiness --gate`
- `cargo run -q -p route -- map-publication-inventory --gate`
- `cargo run -q -p route -- optimizer-residual-blocker-backlog --gate`
- `cargo run -q -p route -- optimizer-manifest --gate`
- `cargo run -q -p route -- release-manifest --gate`
- `C:\src\target\debug\proof check waves\PHASES.md waves\2026-05-15-la-i220-ca-i110-funding-insufficiency-review`
- `powershell -ExecutionPolicy Bypass -File scripts\check-mileposts.ps1 -SkipTests`
