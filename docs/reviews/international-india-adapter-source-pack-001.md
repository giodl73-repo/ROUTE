# International India Adapter Source Pack 001

Status: draft; source-pack preflight only.

## Purpose

This starts India as the third source-bound portability branch after Canada and
EU. The package identifies candidate source families for road authority context,
port-system context, port statistics, the existing held hierarchy fixture, and
service-target holds.

## Source Families

| Family | Role |
| --- | --- |
| Highway ministry context | Defines official source ownership and document inventory candidates. |
| Highway authority context | Candidate road-network/asset authority source lane. |
| Port system context | Candidate node-catalog source lane for major ports. |
| Port statistics context | Candidate need-surface and port context lane. |
| Hierarchy fixture context | Keeps the current v2 map as heuristic-held input, not source proof. |
| Service targets | Holds service targets until local evidence and numeracy review exist. |

## Boundary

This source pack does not claim an official Indian corridor, national or state
approval, route designation, parsed road graph, source-row validation, fixture
replacement, geometry, topology, map overlay, terminal performance, construction
readiness, guaranteed SLA, travel-time proof, delivery commitment, ROI,
compliance, endorsement, validation, public readiness, or external readiness.

## Verification

Run:

```powershell
npm run check:india:source-pack
```
