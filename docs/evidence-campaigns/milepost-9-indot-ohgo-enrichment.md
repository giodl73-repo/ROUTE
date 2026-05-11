# Milepost 9 INDOT/OHGO Enrichment Path

Status: documented blocker and enrichment path.

Target: `T1X-I80-I90` Indiana/Ohio shared-corridor evidence window.

## Decision

Milepost 9 keeps the INDOT TrafficWise path active as a source-access path, but does not treat it as an observation-grade evidence window yet.

Milepost 8 proved that the TrafficWise GraphQL map feed can be fetched. The importer produced zero timed observation rows, so Milepost 9 must either enrich TrafficWise event detail timing or join another historical source before this site can contribute to annual rates or duration distributions.

## Current Blocker

The current TrafficWise map rows are useful for source discovery but insufficient for promotion because they do not provide the observation contract required by `data/t1-failure-events.csv`:

- stable source event id,
- start time,
- end time or duration,
- event type,
- freight relevance,
- confidence label.

## Enrichment Options

Preferred order:

1. Probe TrafficWise detail endpoints linked from map feature `uri` values and look for start/end timing fields.
2. Register for OHGO API access and test incident/construction endpoints for the Ohio side of the shared corridor.
3. Request Ohio Turnpike or Indiana Toll Road historical closure records for the I-80/I-90 split zone.
4. If none of those paths provide timing, keep `T1X-I80-I90` as `enrichment_blocker` and do not annualize it.

## Gate

The `B9-WIN-IN-TW-M8` row in `data/t1-evidence-windows.csv` remains:

```text
evidence_mode=enrichment_blocker
promotion_eligible=false
event_count=0
```

It may only move to `repeated_window` or `historical_archive` after the normalized event table contains observation-grade timed rows for `T1X-I80-I90` and a review approves the interpretation.
