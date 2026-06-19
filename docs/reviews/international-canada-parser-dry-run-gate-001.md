---
name: International Canada Parser Dry Run Gate 001
slug: international-canada-parser-dry-run-gate-001
type: review
status: draft
rubric_version: v1.0
author: codex
created: 2026-06-19
updated: 2026-06-19
sources:
  - tools/build_canada_parser_dry_run.py
  - tools/check_canada_parser_dry_run.py
  - data/international-canada-parser-output-contract-001.csv
  - data/canada_source_link_candidates.csv
  - data/canada_source_need_candidates.csv
  - data/canada_source_node_candidates.csv
  - data/canada_service_target_candidates.csv
  - data/canada_adapter_evidence_labels.csv
  - data/canada_adapter_review_backlog.csv
---

# International Canada Parser Dry Run Gate 001

## Result

This adds `tools/check_canada_parser_dry_run.py`, a mechanical gate for the
generated Canada parser dry-run fixture. The gate checks:

- output columns against `data/international-canada-parser-output-contract-001.csv`;
- required evidence labels for candidate, gap, and held tables;
- minimum row posture for node and service-target gap tables;
- source-ID limits for link, need, and node candidate tables;
- evidence-label coverage for every emitted candidate/gap/held row;
- pending `.roles` review backlog lanes.

This makes the Canada dry-run adapter surface runnable and checkable. It does
not validate Canadian source payloads, prove an official Canadian network,
promote a parsed adapter, replace the Canada hierarchy fixture, or support
Transport Canada/provincial/port approval, route designation, guaranteed SLA,
construction, ROI, eligibility, compliance, endorsement, public-readiness,
external-readiness, or external validation claims.

## Command Closeout

Run:

```powershell
npm run check:canada
```

Equivalent commands:

```powershell
python tools\build_canada_parser_dry_run.py
python tools\check_canada_parser_dry_run.py
```

Expected gate result:

```text
Canada parser dry-run gate: PASS
  checked columns, labels, source limits, evidence coverage, and review backlog
```

## Validation Closeout

| Check | Command / Inspection | Result | Evidence |
|---|---|---|---|
| Regenerate fixture | `python tools\build_canada_parser_dry_run.py` | pass | six dry-run CSVs written |
| Dry-run gate | `python tools\check_canada_parser_dry_run.py` | pass | columns, labels, source limits, evidence coverage, and review backlog checked |
| Package command | `npm run check:canada` | pass | generator and gate run together |
| Python compile | `python -m py_compile tools\build_canada_parser_dry_run.py tools\check_canada_parser_dry_run.py` | pass | scripts compile |
| Claim-boundary scan | scan gate, review, and edited index surfaces | pass | hits are blocked, held, or do-not-infer contexts |
| Diff hygiene | `git diff --check` | pass | no whitespace errors |
| L0 | `npm run check:l0` | pass | workspace lib/bin tests passed |

## Gate

Decision: **canada_parser_dry_run_gate_passed; source_payload_validation_held**

Rationale: Canada now has a reproducible dry-run adapter fixture and a
mechanical contract gate. The remaining promotion boundary is source payload
fetch/cache, field inventory, parser implementation, fixture replacement
review, Canada-specific role review, and validation closeout.
