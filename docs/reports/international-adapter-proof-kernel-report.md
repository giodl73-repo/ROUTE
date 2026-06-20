---
name: International Adapter Proof Kernel Report
slug: international-adapter-proof-kernel-report
type: report
status: draft
rubric_version: v1.0
author: codex
created: 2026-06-19
updated: 2026-06-19
sources:
  - tools/build_international_adapter_proof_kernel.py
  - tools/check_international_adapter_proof_kernel.py
  - data/international-adapter-proof-kernel-001.csv
  - docs/reports/international-network-inference-portability-report.md
  - docs/reviews/international-canada-internal-adapter-proof-001.md
  - docs/reviews/international-canada-external-review-pathway-001.md
---

# International Adapter Proof Kernel Report

## Purpose

This report separates the reusable ROUTE system from the Canada-specific
fixtures.

Canada is the first complete instantiation of the international adapter proof
kernel. It proves that the workflow can move a non-U.S. country pilot through
source custody, parser contracts, source-row validation, fixture replacement,
target holds, role review, internal proof, media-safe wording, and external
review preflight.

It does not prove an official Canadian network, route designation, agency or
port approval, geometry or topology acceptance, operating SLA, ROI,
construction readiness, eligibility, compliance, endorsement, public readiness,
external readiness, or external validation.

## Reusable Kernel

| Kernel Step | Generic Function | Canada Instantiation |
|---|---|---|
| Source custody | Declare owners, dates, URLs, access notes, and evidence labels before promotion. | Canada source pack, payload gates, field inventory, and node-source selection. |
| Parser contract | Define output columns and acceptance rules before parsing or fixture replacement. | Canada parser preflight, output contract, dry-run generator, and gate. |
| Fixture replacement | Replace internal rows only after validation, role review, geometry policy, and closeout. | Canada link and node fixture replacement closeouts. |
| Target posture | Keep service targets as held assumptions until local evidence and authority close. | Canada target posture and internal adapter proof. |
| Review packet | Convert internal proof into bounded review/media/external packet surfaces. | Canada media proof, external review pathway, and port-authority packet preflight. |

## What Is Generic

- The evidence labels.
- The source-custody contract.
- The parser-output contract.
- The replacement sequence.
- The role-review requirement.
- The blocked-claim vocabulary.
- The media and external-review packet pattern.
- The rule that external validation requires a named venue, selected packet,
  venue-specific role review, and validation closeout.

## What Is Canada-Specific

- The selected road-graph and port source rows.
- The Vancouver, Montreal, and Halifax node candidates.
- The Canada hierarchy maps and Canada fixture rows.
- The Canada media brief and Canada external review lane choices.

## Second-Region Application

EU Rhine-Alpine is the next region in the proof-kernel ladder. Its current
status is source-pack preflight only:

- source custody has started with candidate TEN-T/TENtec, GISCO, Rhine-Alpine,
  and service-target rows;
- parser contract, fixture replacement, internal proof, media proof, and
  external review remain held;
- the EU hierarchy map remains a heuristic-held review surface, not validation.

This is intentionally weaker than Canada. The point is to show that the generic
kernel can be applied to another region without promoting premature claims.

## Gate

Decision: **generic_kernel_instantiated_by_canada; external_validation_held**

Run:

```powershell
npm run check:international:proof-kernel
```

Rationale: Canada now demonstrates the reusable proof process, not a special
one-off claim. The generic kernel is ready to be applied to another country or
region after a new source pack and local review roles are selected.
