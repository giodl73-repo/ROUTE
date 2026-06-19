---
name: International Canada Parser Preflight 001
slug: international-canada-parser-preflight-001
type: review
status: draft
rubric_version: v1.0
author: codex
created: 2026-06-19
updated: 2026-06-19
sources:
  - data/international-canada-parser-preflight-001.csv
  - data/international-canada-adapter-source-pack-001.csv
  - data/international-canada-source-custody-preflight.csv
  - data/international-canada-adapter-field-map.csv
  - data/international-canada-source-adapter-readiness.csv
  - docs/reviews/international-canada-adapter-source-pack-001.md
  - docs/templates/source-packs/international-adapter-source-pack-template.md
---

# International Canada Parser Preflight 001

## Result

This creates the parser preflight for the Canada adapter. It defines what a
future parser may inspect, what table-shaped outputs it may create, and what
must remain blocked before any fixture replacement or source-bound adapter
promotion.

It is not a parser implementation, not a source download, not a parsed adapter,
and not a source-bound Canada network. It does not create official Canadian
network, route designation, Transport Canada/provincial/port approval,
guaranteed SLA, construction, ROI, eligibility, compliance, endorsement,
public-readiness, external-readiness, or external validation claims.

## Parser Task Ledger

The controlling ledger is:

`data/international-canada-parser-preflight-001.csv`

| Task | Allowed Output | Blocker |
|---|---|---|
| CAN-PARSE-001 | road graph parse-ready candidate rows | route id or source class missing |
| CAN-PARSE-002 | base road parse-ready candidate rows with suitability warning | geometry reference or version missing |
| CAN-PARSE-003 | road-context need candidate rows | source owner, date, or context field missing |
| CAN-PARSE-004 | trade need and constraint candidate rows | source owner, date, or vocabulary field missing |
| CAN-PARSE-005 | no parser output yet; source-needed | official port or terminal source row missing |
| CAN-PARSE-006 | no source-bound SLA output; held | adopted target source or basis missing |
| CAN-PARSE-007 | evidence labels carry forward | evidence label or blocked-claims field missing |
| CAN-PARSE-008 | Canada role-review backlog | role lanes missing |

## Allowed Dry-Run Outputs

| Output Table | Allowed Label | What It May Contain | What It Must Not Claim |
|---|---|---|---|
| `canada_source_link_candidates` | parse-ready-candidate | source metadata, route/class fields, geometry reference, owner/date/access note | official network, route designation, engineering precision, construction readiness |
| `canada_source_need_candidates` | source-candidate | bounded road-system and trade-corridor vocabulary with citation notes | freight benefit proof, ROI, funding eligibility, project priority |
| `canada_source_node_candidates` | source-needed | only gap rows until official port/terminal custody exists | port endorsement, terminal performance, node completeness |
| `canada_service_target_candidates` | held | assumption labels only | guaranteed SLA, travel-time proof, delivery commitment |
| `canada_adapter_evidence_labels` | carry-forward | artifact path, row id, evidence label, blocked claims | validation, approval, public readiness |
| `canada_adapter_review_backlog` | carry-forward | role lanes, questions, holds, rerun requirements | agency signoff, endorsement, external review |

## Promotion Rules

Parser dry-run outputs may become source-bound only after:

1. Source metadata is cached or otherwise reproducibly accessible.
2. Field inventory records exact source fields, transforms, units, and warnings.
3. Missing port/terminal, constraint, and service-target rows remain
   source-needed or held.
4. Evidence labels and blocked claims attach to every output row.
5. Canada-specific internal role lanes rerun.
6. Prohibited-claim scan and L0 close.

## Validation Closeout

| Check | Command / Inspection | Result | Evidence |
|---|---|---|---|
| Source-pack consistency | compare parser tasks against Canada adapter source pack | pass | parser tasks carry forward source IDs, required fields, and blocked claims |
| Readiness consistency | compare parser tasks against Canada readiness ledger | pass | parse-ready, source-needed, held, and carry-forward decisions match readiness posture |
| Claim-boundary scan | scan parser preflight and edited index surfaces | pass | hits are guardrail, held, or do-not-infer contexts |
| L0 | `npm run check:l0` | pass | workspace lib/bin tests passed |

## Gate

Decision: **canada_parser_preflight_ready; parser_implementation_held**

Rationale: The parser job is now auditable before implementation. Road graph
and need rows may be inspected as candidates, while node, terminal, constraint,
and service-target gaps remain source-needed or held. Parsed-adapter,
fixture-replacement, official-network, SLA, construction, ROI, compliance,
endorsement, public-readiness, external-readiness, and validation claims remain
held.
