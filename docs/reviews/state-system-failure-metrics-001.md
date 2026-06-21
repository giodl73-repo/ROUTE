---
name: State System Failure Metrics 001
slug: state-system-failure-metrics-001
type: review
status: draft
rubric_version: v1.0
author: codex
created: 2026-06-21
updated: 2026-06-21
sources:
  - data/state-system-failure-metric-menu.csv
  - data/state-system-failure-scorecard-template.csv
  - docs/reports/state-system-failure-metrics-framework.md
  - docs/reports/state-market-system-value-add-report.md
---

# State System Failure Metrics 001

## Scope

This review adds the state-system failure metrics layer that should sit between
the candidate state maps and any client-facing scorecard.

## Product Reason

The state maps do not create enough value if they only restate familiar
corridors. ROUTE needs to show where a state's current service system may be
fragile:

- major interchange single-point exposure;
- alternate-route penalties after closures;
- overreliance on interstate links;
- underuse of state highways for redundancy and access;
- terminal and institutional last-mile friction;
- rural isolation during disruptions;
- missing recovery-time evidence.

## Evidence Boundary

The added artifacts define metrics and scorecard structure. They do not assert
that a state has failed a metric or that a specific interchange, corridor,
terminal, or state highway has been validated as deficient.

## Gate

Decision: **state_failure_metric_layer_defined; source_needed_for_scoring**
