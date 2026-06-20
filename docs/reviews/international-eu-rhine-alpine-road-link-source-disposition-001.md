# International EU Rhine-Alpine Road-Link Source Disposition 001

Status: draft; official road-link endpoint not acquired.

## Result

EU has a documented GISCO Transport version 3 road-link lead, but not an
acquired official road-link endpoint.

Current evidence chain:

| Surface | Result |
| --- | --- |
| JRC documentation lead | GISCO Transport version 3 road links are referenced, but not exposed as a download endpoint. |
| Official GISCO transport page | Airport and port links are exposed; road-link package is not exposed in the scraped surface. |
| Direct endpoint candidates | Ten GISCO-style road/transport URL candidates were probed and not accepted. |
| Link fixture | Replacement remains blocked. |

## Decision

Do not replace the EU link fixture from metadata rows, context rows, or guessed
URLs.

The required next step is to request or locate an official road-link endpoint,
then run source-row extraction and validation before fixture replacement.

## Boundary

This disposition does not prove an official EU network, source-row validation,
fixture replacement, parsed adapter, geometry, topology, SLA, ROI, validation,
public readiness, external readiness, or internal adapter proof.

## Verification

Run:

```powershell
npm run check:eu:road-link-source-disposition
```
