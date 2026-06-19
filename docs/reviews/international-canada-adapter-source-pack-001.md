---
name: International Canada Adapter Source Pack 001
slug: international-canada-adapter-source-pack-001
type: review
status: draft
rubric_version: v1.0
author: codex
created: 2026-06-19
updated: 2026-06-19
sources:
  - docs/templates/source-packs/international-adapter-source-pack-template.md
  - data/international-canada-source-custody-preflight.csv
  - data/international-canada-adapter-field-map.csv
  - data/international-canada-source-adapter-readiness.csv
  - data/international-canada-adapter-source-pack-001.csv
  - docs/reviews/international-canada-source-adapter-preflight-001.md
  - docs/reviews/international-canada-source-adapter-readiness-001.md
  - docs/reviews/international-hierarchy-replication-closeout-001.md
---

# International Canada Adapter Source Pack 001

## Result

This fills the international adapter source-pack template for Canada using the
existing Canada source-custody preflight and readiness ledgers.

It is not a parsed source adapter. It does not replace fixture links, validate a
Canadian network, or create official network, route designation,
Transport Canada/provincial/port approval, guaranteed SLA, construction, ROI,
eligibility, compliance, endorsement, public-readiness, external-readiness, or
external validation claims.

## Source Family Decisions

| Source Family | Source Rows | Decision | Blocked Claims |
|---|---|---|---|
| Road graph | CAN-SRC-001 / CAN-SRC-003 | parse-ready-candidate; not promoted | official network, engineering precision, construction readiness |
| Road context / need vocabulary | CAN-SRC-002 / CAN-SRC-004 | source-candidate; not promoted | freight benefit proof, ROI, funding eligibility, project priority |
| Node and terminal catalog | CAN-SRC-005 | source-pack-required | port endorsement, terminal performance, node completeness |
| Service targets | CAN-SRC-SLA-001 | held | guaranteed SLA, travel-time proof, delivery commitment |

## Field Mapping Carry Forward

| Adapter Field | Current Readiness | Use Now | Held Before Promotion |
|---|---|---|---|
| jurisdiction_scope | preflight-ready / declared | carry as metadata | country approval and external validation |
| road_graph | source-candidate-found / parse-ready-candidate | inspect/cache before parser work | official network and engineering precision |
| node_catalog | source-needed | keep fixture centroids | port endorsement and node completeness |
| need_surfaces | source-candidate-found / parse-ready-candidate | convert only bounded vocabulary | freight benefit proof, ROI, and funding eligibility |
| service_target_set | held | keep planning assumptions | guaranteed SLA and delivery commitment |
| constraint_ledger | source-needed | create candidate/held rows only | resilience proof, bottleneck proof, construction priority |
| evidence_labels | carry-forward | attach to generated rows | validation, approval, public readiness |
| review_roles | carry-forward | rerun before stronger claims | agency signoff, endorsement, external review |

## Promotion Backlog

1. Cache or download source metadata for CAN-SRC-001 and CAN-SRC-003.
2. Inspect route/classification fields and record field transforms before any
   parser replaces fixture links.
3. Select official port/terminal source rows for Vancouver, Montreal, Halifax,
   and any added gateway nodes before replacing centroids.
4. Convert road-system and trade-corridor vocabulary into bounded need rows
   without funding, ROI, or priority claims.
5. Build a constraint ledger with source-needed and candidate rows before using
   resilience or bottleneck language.

## Validation Closeout

| Check | Command / Inspection | Result | Evidence |
|---|---|---|---|
| Template fit | compare source-pack rows against `international-adapter-source-pack-template.md` | pass | source families map to template categories and promotion gates remain held |
| Preflight consistency | compare source-pack rows against Canada custody, field-map, and readiness ledgers | pass | rows carry forward existing source IDs, readiness decisions, and blocked claims |
| Claim-boundary scan | scan source pack and edited index surfaces | pass | hits are guardrail, held, or do-not-infer contexts |
| L0 | `npm run check:l0` | pass | workspace lib/bin tests passed |

## Gate

Decision: **canada_adapter_source_pack_declared; parser_promotion_held**

Rationale: Canada now has a filled source-pack declaration using existing
candidate source custody and readiness rows. Parser work, fixture replacement,
external validation, official-network, SLA, construction, ROI, compliance,
endorsement, public-readiness, and external-readiness claims remain held.
