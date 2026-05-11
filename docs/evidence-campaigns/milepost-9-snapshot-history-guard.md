# Milepost 9 Snapshot-History Guard

Status: implemented in CLI gate.

The Milepost 9 guard prevents live snapshot rows from being promoted as annual or recovery-grade evidence.

## Command

```powershell
cargo run -q -p route -- t1-evidence-windows --gate-windows
```

## Guard Rules

The gate fails when:

- the evidence-window ledger is empty,
- a row lacks source-window metadata,
- `freight_relevant_count` exceeds `event_count`,
- `evidence_mode=snapshot_only` is marked `promotion_eligible=true`,
- any promotion-eligible row is not `repeated_window` or `historical_archive`,
- any promotion-eligible row lacks observation start/end dates or nonzero event rows.

## Effect

This does not make the Des Moines Iowa rows stronger by itself. It makes the limitation executable: snapshot-only evidence can remain visible, but it cannot quietly become annual history.
