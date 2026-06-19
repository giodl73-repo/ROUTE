---
name: International Canada Source Adapter Readiness 001
slug: international-canada-source-adapter-readiness-001
type: review
status: draft
rubric_version: v1.0
author: codex
created: 2026-06-18
updated: 2026-06-18
sources:
  - docs/reviews/international-canada-source-adapter-preflight-001.md
  - data/international-canada-source-custody-preflight.csv
  - data/international-canada-adapter-coverage-preflight.csv
  - data/international-canada-adapter-field-map.csv
  - data/international-canada-source-adapter-readiness.csv
  - data/international-canada-source-adapter-gap-backlog.csv
  - tools/build_canada_adapter_readiness.py
  - docs/reports/international-network-inference-portability-report.md
  - docs/vtrace/COMMUNICATIONS_STRATEGY.md
  - docs/vtrace/VERIFICATION.md
---

# International Canada Source Adapter Readiness 001

## Purpose

This run turns the Canada source-adapter preflight into a repeatable readiness
ledger. It decides which adapter fields can move to a future parser, which
fields can carry forward as internal metadata, which require source packs, and
which must remain held assumptions.

It does not fetch, parse, download, validate, endorse, approve, or publish a
Canadian service network. It is not a Transport Canada review, provincial
review, port review, official-plan claim, guaranteed-SLA claim, construction
claim, numeric ROI claim, eligibility finding, compliance finding,
public-readiness gate, or external-readiness gate.

## Run Command

```powershell
python tools\build_canada_adapter_readiness.py
```

The command reads:

- `data/international-canada-source-custody-preflight.csv`
- `data/international-canada-adapter-coverage-preflight.csv`
- `data/international-canada-adapter-field-map.csv`

The command writes:

- `data/international-canada-source-adapter-readiness.csv`
- `data/international-canada-source-adapter-gap-backlog.csv`

## Readiness Decisions

| Decision | Meaning | Claim Boundary |
|---|---|---|
| `ready_for_parse_not_promoted` | Candidate source and field map exist, but no source-bound adapter row has been produced yet. | Do not claim validation, official network, SLA, construction, ROI, compliance, or endorsement. |
| `carry_forward_internal` | The field can move as metadata, label discipline, or review control. | This is not source validation or external review. |
| `source_pack_required` | Specific source custody is missing before fixture rows can be replaced. | Do not use the field as source-backed evidence. |
| `assumption_held` | Planning assumption exists and must stay labeled as an assumption. | Do not treat targets as guaranteed service. |

## Output Summary

| Adapter Field | Decision | Reason |
|---|---|---|
| jurisdiction_scope | carry_forward_internal | scope metadata is declared but does not imply approval |
| road_graph | ready_for_parse_not_promoted | NHS/base-road source candidates and target columns exist; parsing remains future work |
| node_catalog | source_pack_required | fixture centroids need node-specific source custody |
| need_surfaces | ready_for_parse_not_promoted | road-system and trade-corridor sources can seed bounded vocabulary |
| service_target_set | assumption_held | target windows are planning assumptions only |
| constraint_ledger | source_pack_required | constraints need dedicated rows before use |
| evidence_labels | carry_forward_internal | labels can travel through future adapter outputs |
| review_roles | carry_forward_internal | internal role controls can be rerun after source-bound rows |

## What This Proves

| Claim | Status |
|---|---|
| ROUTE can convert a country preflight into machine-readable adapter readiness rows. | supported by run |
| ROUTE can separate parse-ready source candidates from source-needed and held fields. | supported by run |
| ROUTE can emit a gap backlog before replacing fixture rows. | supported by run |

## What This Does Not Prove

| Non-Claim | Reason |
|---|---|
| Canada has a validated ROUTE service network. | No source-bound graph rows, local review, or agency review exist. |
| The candidate sources are sufficient for service-role inference. | Field inspection and source extraction remain future work. |
| The service targets are Canadian SLAs. | They remain fixture planning assumptions. |
| Nodes, ports, constraints, or needs are complete. | Source packs and field-level custody remain open. |

## Validation Closeout

| Check | Command / Inspection | Result | Evidence |
|---|---|---|---|
| Readiness builder run | `python tools\build_canada_adapter_readiness.py` | pass | readiness ledger and gap backlog generated |
| Output inspection | compare source custody, coverage, field map, readiness, and gap rows | pass | parse-ready, carry-forward, source-needed, and held decisions align with inputs |
| Prohibited-claim scan | scan readiness package for promoted prohibited claims | pass | hits are guardrail, held, source-needed, or do-not-infer contexts |
| L0 | `npm run check:l0` | pass | workspace lib/bin tests passed |

## Gate

Decision: **canada_adapter_readiness_generated; validation_held**

Rationale: Canada now has an executable adapter-readiness step between source
preflight and source-bound parsing. The package improves replicability by
making promotion decisions machine-readable while preserving all official-plan,
guaranteed-SLA, construction, ROI, eligibility, compliance, endorsement,
public-readiness, and external-readiness holds.
