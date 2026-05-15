---
wave: 2026-05-15-la-i220-ca-i110-funding-insufficiency-review
date_closed: 2026-05-15
status: done
---

# Close: LA I-220 CA I-110 Funding Insufficiency Review

## Result

No additional full-cost pavement repair funding acceptance was supported.

## Source Findings

| Row | Official source checked | Finding | Decision |
|---|---|---|---|
| LA I-220 / `US.HWYBUNDLE.5FA9BA2B1304E6EB` | Louisiana DOTD current STIP, plus SFY 2025-2026 Highway Program by parish | Program hits include I-220 preservation/operations items such as Bossier I-220 Red River Bridge to I-20 preservation at about $3.0M and Shed Road interchange lighting at about $1.6M; these do not cover the $25.0M planning repair debt. | Preserve repair hold. |
| CA I-110 / `US.HWYBUNDLE.D6B01122CB05A1BA` | Caltrans 2024 SHOPP document and project list | Route 110 hits are bridge, viaduct, sign, lighting, or drainage work; no full-cost pavement repair funding row was found for the $5.0M planning repair debt. | Preserve repair hold. |

## Residual State

- T2 asset-condition repair debt remains 2 rows / $30.0M.
- LA I-220 remains in the priority-A repair funding package chain.
- CA I-110 remains priced in the optimizer budget but outside the priority-A
  repair package chain.

## Gates

- `C:\src\target\debug\proof check waves\PHASES.md waves\2026-05-15-la-i220-ca-i110-funding-insufficiency-review`
- `cargo run -q -p route -- optimizer-residual-blocker-backlog --gate`
- `powershell -ExecutionPolicy Bypass -File scripts\check-mileposts.ps1 -SkipTests`
