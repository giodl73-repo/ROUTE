---
name: International Canada Port Authority Packet Preflight 001
slug: international-canada-port-authority-packet-preflight-001
type: review
status: draft
rubric_version: v1.0
author: codex
created: 2026-06-19
updated: 2026-06-19
sources:
  - tools/build_canada_port_authority_packet_preflight.py
  - tools/check_canada_port_authority_packet_preflight.py
  - data/international-canada-port-authority-packet-preflight-001.csv
  - data/canada_source_node_candidates.csv
  - docs/reviews/international-canada-external-review-pathway-001.md
  - docs/reviews/international-canada-node-fixture-replacement-closeout-001.md
---

# International Canada Port Authority Packet Preflight 001

## Purpose

This preflight selects the Canada port-authority lane as the first narrow
external-review candidate. It is not a filled external packet, not a meeting
record, not a port review, and not evidence of port authority endorsement,
approval, validation, or acceptance.

The packet focus is source custody for the Vancouver, Montreal, and Halifax
port-node candidates. It deliberately excludes terminal performance, throughput,
road-access adequacy, geometry, topology, official network, SLA, ROI,
construction, eligibility, compliance, public-readiness, and external-readiness
claims.

## Selected Packet Shape

| Packet Part | Current Entry | Status |
|---|---|---|
| Lane | Port authority source-custody review | preflight only |
| Venue | Not named | hold |
| Source anchors | Vancouver, Montreal, and Halifax node candidate rows | internal source candidates |
| Material set | Canada internal proof, node fixture closeout, media brief, external pathway | selected for preflight |
| Required roles | Scope Keeper, Citation Auditor, Freight Industry, Schematic Cartographer, V&V | venue-specific rerun required |
| Safe ask | Review source custody and terminology for one named port venue | not sent |

## Allowed Language

Use:

- "A Canada port-authority packet preflight exists for future source-custody
  review."
- "The Vancouver, Montreal, and Halifax port nodes have selected public
  source-custody candidates."
- "A named venue and venue-specific role review are still required before
  external use."

Do not say:

- "A port authority has reviewed, approved, endorsed, validated, or accepted
  ROUTE."
- "The port nodes prove terminal performance, throughput, road access, or
  service quality."
- "The packet is public-ready, externally ready, or a completed external
  review."

## Gate

Decision: **canada_port_authority_packet_preflight_ready_named_venue_held**

Run:

```powershell
npm run check:canada:port-authority-packet
```

Rationale: the port-authority lane is now concrete enough to become the first
named Canada packet once a venue exists. It remains internal preflight until a
named venue, packet-specific role review, prohibited-claim scan, and validation
closeout exist.
