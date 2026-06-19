---
name: International Canada Source Field Inventory 001
slug: international-canada-source-field-inventory-001
type: review
status: draft
rubric_version: v1.0
author: codex
created: 2026-06-19
updated: 2026-06-19
sources:
  - tools/build_canada_source_field_inventory.py
  - tools/check_canada_source_field_inventory.py
  - data/international-canada-source-field-inventory-001.csv
  - data/international-canada-source-payload-resolution-001.csv
  - data/international-canada-source-payload-probe-001.csv
---

# International Canada Source Field Inventory 001

## Result

This adds a Canada source field inventory after the bounded payload probe. The
inventory reads the resolved `CAN-SRC-001` ESRI REST layer metadata and emits 49
road-graph field candidates, including geometry, class, route-number, and
route-name candidate fields. It also emits held inventory rows for the HTML,
source-needed, and service-target sources.

The inventory does not query features, cache full payloads, parse source rows,
replace dry-run fixtures, promote a parsed adapter, or validate official
Canadian network, route designation, Transport Canada/provincial/port approval,
guaranteed SLA, construction, ROI, eligibility, compliance, endorsement,
public-readiness, external-readiness, or external validation claims.

## Command Closeout

Run:

```powershell
npm run check:canada:inventory
```

Expected gate result:

```text
Canada source field inventory gate: PASS
  checked source coverage, road-graph field candidates, and not-accepted posture
```

## Field Posture

| Source | Rows | Posture |
|---|---:|---|
| `CAN-SRC-001` | 49 | ESRI REST field candidates; evidence not accepted |
| `CAN-SRC-002` | 1 | field inventory held; manual extraction needed |
| `CAN-SRC-003` | 1 | field inventory held; manual/cache extraction needed |
| `CAN-SRC-004` | 1 | field inventory held; manual extraction needed |
| `CAN-SRC-005` | 1 | source selection needed |
| `CAN-SRC-SLA-001` | 1 | held target source |

## Validation Closeout

| Check | Command / Inspection | Result | Evidence |
|---|---|---|---|
| Build inventory | `python tools\build_canada_source_field_inventory.py` | pass | `data/international-canada-source-field-inventory-001.csv` written |
| Inventory gate | `python tools\check_canada_source_field_inventory.py` | pass | source coverage, road-graph field candidates, and not-accepted posture checked |
| Package command | `npm run check:canada:inventory` | pass | inventory build and gate run together |
| Python compile | `python -m py_compile tools\build_canada_source_field_inventory.py tools\check_canada_source_field_inventory.py` | pass | scripts compile |
| Claim-boundary scan | scan field-inventory artifacts and edited indexes | pass | hits are blocked, held, or do-not-infer contexts |
| Diff hygiene | `git diff --check` | pass | no whitespace errors |
| L0 | `npm run check:l0` | pass | workspace lib/bin tests passed |

## Gate

Decision: **canada_source_field_inventory_ready; feature_extraction_held**

Rationale: ROUTE now knows the road-graph field candidates it would map into a
Canada parser, but it still has not queried features, joined geometry, replaced
fixtures, or accepted source evidence.
