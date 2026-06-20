# International EU Rhine-Alpine Port Node Fixture Closeout 001

Status: draft; internal no-geometry node fixture replaced.

## What Changed

The EU Rhine-Alpine dry-run node table now uses five GISCO Ports 2013 attribute
candidate rows already sampled and source-row validated:

| Node ID | Label | Posture |
| --- | --- | --- |
| NLRTM | Rotterdam | source-candidate; internal node fixture only |
| BEANR | Antwerpen | source-candidate; internal node fixture only |
| ITGOA | Genova | source-candidate; internal node fixture only |
| CHBSL | Basel | source-candidate; internal node fixture only |
| DEDUI | Duisburg | source-candidate; internal node fixture only |

## Evidence Chain

| Step | Artifact |
| --- | --- |
| Source record sample | `data/international-eu-rhine-alpine-port-node-record-sample-001.csv` |
| Role review | `data/international-eu-rhine-alpine-port-node-role-review-001.csv` |
| Source-row validation | `data/international-eu-rhine-alpine-port-node-source-row-validation-001.csv` |
| Fixture contract | `data/international-eu-rhine-alpine-port-node-fixture-contract-001.csv` |
| Replaced fixture table | `data/eu_rhine_alpine_source_node_candidates.csv` |
| Closeout ledger | `data/international-eu-rhine-alpine-port-node-fixture-closeout-001.csv` |

## Boundaries

This closeout does not accept geometry and does not prove topology, road access,
terminal performance, node completeness, throughput, SLA, ROI, construction
readiness, member-state approval, endorsement, validation, public readiness, or
external readiness.

The allowed use is internal adapter node-candidate fixture rows only.

## Verification

Run:

```powershell
npm run check:eu:port-node-fixture-closeout
```
