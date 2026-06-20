# International EU Rhine-Alpine Road-Link Endpoint Request 001

Status: draft; request packet preflight only.

## Purpose

The road-link source disposition shows that ROUTE has a documentation lead but
no official road-link endpoint. This packet converts that result into a bounded
source-acquisition request plan.

## Request Lanes

| Lane | Ask |
| --- | --- |
| Eurostat GISCO support | Identify the official GISCO Transport version 3 road-link endpoint and metadata path, if public. |
| TENtec / Mobility and Transport | Clarify whether public TENtec or European Transport Corridors road-network attributes can support no-geometry adapter testing. |
| JRC EIGL documentation | Confirm whether documented GISCO Transport v3 road links have a public distribution endpoint. |
| Alternative public source | If no official endpoint is available, select a separate fallback source branch with its own claim boundaries. |

## Boundary

This packet does not claim a named contact, agency review, source-row
validation, fixture replacement, parsed adapter, geometry, topology, official
network, SLA, ROI, validation, public readiness, external readiness, or internal
adapter proof.

## Verification

Run:

```powershell
npm run check:eu:road-link-endpoint-request
```
