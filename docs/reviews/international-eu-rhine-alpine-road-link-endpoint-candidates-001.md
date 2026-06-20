# International EU Rhine-Alpine Road-Link Endpoint Candidates 001

Status: draft; endpoint not acquired.

## Result

ROUTE tested ten direct official-path candidates for a GISCO Transport version 3
road-link package or document route. None are accepted as a road-link endpoint.

The probe records HEAD metadata only. It does not download payloads, parse
fields, accept geometry, validate road rows, replace fixtures, or promote an
internal adapter proof.

## Sources

- GISCO transport networks page: `https://ec.europa.eu/eurostat/web/gisco/geodata/transport-networks`
- JRC EIGL data documentation: `https://joint-research-centre.ec.europa.eu/document/download/d99c0bcf-21db-46bf-ba66-fd46fdf5e3de_en?filename=Data+documentation+EIGL+PUBLIC+V1_7.pdf`

## Boundary

This probe does not prove an official EU road network, source-row validation,
fixture replacement, parsed adapter, geometry, topology, map overlay, SLA, ROI,
validation, public readiness, external readiness, or internal adapter proof.

## Next Step

Find an official GISCO road-link endpoint before source-row extraction.

## Verification

Run:

```powershell
npm run check:eu:road-link-endpoint-candidates
```
