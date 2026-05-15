---
wave: iowa-511-snapshot-polling-acquisition
date_closed: 2026-05-15
status: done
---

# Iowa 511 Snapshot Polling Acquisition Close

## Decision

The Iowa 511 polling path produced real new observations, but the evidence
window remains snapshot-only and not promotion eligible.

## Evidence

| Source | Prior rows | Current rows | Net new | Evidence mode | Promotion eligible |
|---|---:|---:|---:|---|---|
| Iowa DOT 511 ArcGIS | 25 | 29 | 4 | snapshot_only | false |

## Residual Holds

- Source snapshot repeat-window or archive-history proof remains required before
  evidence claims.
- T4 terminal-access proof remains required for upgrade/evidence claims.
- T2 asset-condition debt remains payable before SLA, transit, or upgrade claims.

## Gate Bundle

- `powershell -ExecutionPolicy Bypass -File scripts\poll-t1-iowa511.ps1`
- `cargo run -q -p route -- t1-failure-events --gate-observations`
- `cargo run -q -p route -- t1-evidence-windows --gate-windows`
- `cargo run -q -p route -- optimizer-constraint-ledger --gate`
- `cargo run -q -p route -- optimizer-constraint-budget --gate`
- `cargo run -q -p route -- optimizer-residual-blocker-backlog --gate`
- `cargo run -q -p route -- map-publication-readiness --gate`
- `cargo run -q -p route -- map-publication-inventory --gate`
- `cargo run -q -p route -- release-manifest --gate`
- `C:\src\target\debug\proof check docs\evidence-campaigns\milepost-9-iowa-repeat-window.md waves\PHASES.md waves\2026-05-15-iowa-511-snapshot-polling-acquisition`
- `powershell -ExecutionPolicy Bypass -File scripts\check-mileposts.ps1 -SkipTests`

## Next Action

Continue scheduled Iowa 511 polling or obtain Iowa DOT archive history before
any evidence promotion.
