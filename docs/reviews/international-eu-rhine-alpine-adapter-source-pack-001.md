---
name: International EU Rhine-Alpine Adapter Source Pack 001
slug: international-eu-rhine-alpine-adapter-source-pack-001
type: review
status: draft
rubric_version: v1.0
author: codex
created: 2026-06-20
updated: 2026-06-20
sources:
  - tools/build_eu_rhine_alpine_adapter_source_pack.py
  - tools/check_eu_rhine_alpine_adapter_source_pack.py
  - data/international-eu-rhine-alpine-adapter-source-pack-001.csv
  - docs/templates/source-packs/international-adapter-source-pack-template.md
  - docs/reviews/international-eu-rhine-alpine-hierarchy-iteration-001.md
  - docs/reports/international-adapter-proof-kernel-report.md
---

# International EU Rhine-Alpine Adapter Source Pack 001

## Result

This starts applying the international adapter proof kernel to a second region:
the EU Rhine-Alpine example.

The source pack declares candidate source-custody rows for European Commission
TEN-T/TENtec surfaces, GISCO transport geodata, Rhine-Alpine context, and a held
service-target row. It is a preflight declaration only. It does not parse EU
source data, replace hierarchy fixtures, validate an EU corridor, create a
member-state approval claim, or promote official-corridor, SLA, construction,
ROI, eligibility, compliance, endorsement, public-readiness, external-readiness,
or external-validation claims.

## Source Family Decisions

| Source Family | Source Rows | Decision | Blocked Claims |
|---|---|---|---|
| Corridor context | EUR-SRC-001 / EUR-SRC-004 | source-candidate; not promoted | official corridor, approval, policy alignment |
| Network viewer | EUR-SRC-002 | source-candidate; not parsed | geometry, topology, official network |
| Transport geodata | EUR-SRC-003 | source-candidate; not parsed | geometry acceptance, node completeness |
| Rail freight context | EUR-SRC-005 | context-source; not promoted | terminal performance, road service inference |
| Service targets | EUR-SRC-SLA-001 | held | guaranteed SLA, travel-time proof, delivery commitment |

## Promotion Backlog

1. Inspect source metadata and select parseable fields before an EU parser
   contract exists.
2. Decide whether TENtec, GISCO, or another official source controls road graph,
   corridor, and node fields.
3. Create an EU parser output contract only after source fields and access notes
   are selected.
4. Keep the existing EU hierarchy map and rows as heuristic-held until
   source-row validation, geometry policy, role review, and fixture replacement
   close.

## Gate

Decision: **eu_rhine_alpine_source_pack_preflight_ready; promotion_held**

Run:

```powershell
npm run check:eu:source-pack
```

Rationale: EU Rhine-Alpine now has the first source-custody preflight needed to
apply the generic proof kernel beyond Canada. Parser, fixture replacement,
internal proof, external review, approval, SLA, ROI, construction,
public-readiness, and external-readiness claims remain held.
