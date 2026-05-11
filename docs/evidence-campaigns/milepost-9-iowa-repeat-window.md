# Milepost 9 Iowa Repeat-Window Path

Status: executable polling path added.

Target: `T1X-I35-I80` Des Moines, Iowa T1/T1 evidence window.

## Decision

Milepost 9 will use repeated Iowa 511 polling as the first operational path for moving from a single live snapshot toward a documented observation window.

The polling path does not promote the T1/T1 diamond recovery claim. It only creates repeatable raw and normalized artifacts that can later support a review if the capture window becomes long enough and the event rows are joined to duration, reroute, and throughput evidence.

## Command

```powershell
powershell -ExecutionPolicy Bypass -File scripts/poll-t1-iowa511.ps1
```

The script writes date-stamped cache files under:

```text
data/cache/t1-evidence-windows/iowa511/<UTC timestamp>/
```

Then it accumulates normalized rows into `data/t1-failure-events.csv` and runs:

```powershell
cargo run -q -p route -- t1-failure-events --gate-observations
cargo run -q -p route -- t1-evidence-windows --gate-windows
```

## Evidence-Window Rule

Each polling run remains `snapshot_only` until a review records a real repeated window. The source-window ledger must not mark the row `promotion_eligible=true` until the evidence has:

- repeated capture dates or historical archive depth,
- observation start and end dates,
- nonzero observation-grade event rows,
- review approval that bounds annualization.

## Next Step

After several runs exist, add or update a `repeated_window` row in `data/t1-evidence-windows.csv` with the capture span, observation span, event counts, blocker, and review artifact.
