---
name: International Canada Parser Output Contract 001
slug: international-canada-parser-output-contract-001
type: review
status: draft
rubric_version: v1.0
author: codex
created: 2026-06-19
updated: 2026-06-19
sources:
  - data/international-canada-parser-output-contract-001.csv
  - data/international-canada-parser-preflight-001.csv
  - docs/reviews/international-canada-parser-preflight-001.md
  - data/international-canada-adapter-source-pack-001.csv
  - data/international-canada-source-adapter-readiness.csv
---

# International Canada Parser Output Contract 001

## Result

This defines the output contract for a future Canada parser dry run. It names
the exact table surfaces, required columns, required evidence labels, acceptance
rules, and blocked claim values that must be enforced before parser output can
be inspected.

It is not a parser implementation, not a source download, not a parsed adapter,
and not a source-bound Canada network. It does not replace fixture rows or
create official Canadian network, route designation, Transport
Canada/provincial/port approval, guaranteed SLA, construction, ROI,
eligibility, compliance, endorsement, public-readiness, external-readiness, or
external validation claims.

## Contract Tables

The controlling contract is:

`data/international-canada-parser-output-contract-001.csv`

| Output Table | Required Label | Required Posture |
|---|---|---|
| `canada_source_link_candidates` | parse-ready-candidate | road graph/base road metadata only; no official-network or engineering-precision claim |
| `canada_source_need_candidates` | source-candidate | bounded context/need vocabulary only; no ROI, funding, priority, bottleneck, or resilience proof |
| `canada_source_node_candidates` | source-needed | gap rows only until official port/terminal custody exists |
| `canada_service_target_candidates` | held | assumption rows only; no source-bound SLA output |
| `canada_adapter_evidence_labels` | carry-forward | every emitted candidate row must retain evidence label and blocked claims |
| `canada_adapter_review_backlog` | carry-forward | role-review backlog exists before any output is used beyond internal inspection |

## Acceptance Rules

1. Candidate link rows may only come from CAN-SRC-001 or CAN-SRC-003.
2. Candidate need rows may only come from CAN-SRC-002 or CAN-SRC-004.
3. Node and service-target tables must remain gap/held rows unless a future
   source pack adds specific custody.
4. Every candidate output row must have owner, date/year, access note, evidence
   label, and blocked claims.
5. Any row containing official-network, approval, guaranteed-SLA, construction,
   ROI, eligibility, compliance, endorsement, validation, public-readiness, or
   external-readiness language fails the contract unless it is in a blocked
   claims field.

## Implementation Hold

A future parser implementation can satisfy this contract without promoting any
source-bound adapter claim. The first valid implementation target is a dry run
that emits candidate/gap/held rows and a review backlog, then stops.

## Validation Closeout

| Check | Command / Inspection | Result | Evidence |
|---|---|---|---|
| Preflight alignment | compare output tables and labels against Canada parser preflight | pass | contract tables match preflight target tables and allowed labels |
| Readiness alignment | compare allowed labels against Canada readiness ledger | pass | parse-ready, source-candidate, source-needed, held, and carry-forward labels match readiness posture |
| Claim-boundary scan | scan output contract and edited index surfaces | pass | hits are guardrail, held, or do-not-infer contexts |
| L0 | `npm run check:l0` | pass | workspace lib/bin tests passed |

## Gate

Decision: **canada_parser_output_contract_ready; implementation_held**

Rationale: The parser output surfaces are now mechanically specified, but the
parser, source download/cache, parsed adapter, fixture replacement,
official-network, SLA, construction, ROI, compliance, endorsement,
public-readiness, external-readiness, and validation claims remain held.
