---
name: International Canada Node Source Probe 001
slug: international-canada-node-source-probe-001
type: review
status: draft
rubric_version: v1.0
author: codex
created: 2026-06-19
updated: 2026-06-19
sources:
  - tools/probe_canada_node_sources.py
  - tools/check_canada_node_source_probe.py
  - data/international-canada-node-source-selection-001.csv
  - data/international-canada-node-source-probe-001.csv
---

# International Canada Node Source Probe 001

## Result

This adds bounded HTTP reachability metadata for the selected Canada port source
candidates. The probe records HTTP status, final URL, content type, bytes
sampled, not-accepted evidence posture, blocked claims, and next action.

Allowed language:

- ROUTE has probed selected public port source-custody candidates for Canada
  node-catalog work.
- The probe is intake metadata for future terminal field inspection.

Do not say:

- The Canada node catalog has been replaced, validated, or promoted.
- The selected source rows prove terminal performance, node completeness, road
  access adequacy, throughput, construction readiness, guaranteed SLA, ROI,
  compliance, endorsement, public readiness, external readiness, or external
  validation.

## Command Closeout

Run:

```powershell
npm run check:canada:node-source-probe
```

Expected gate result:

```text
Canada node source-probe gate: PASS
  checked probe coverage, HTTP metadata posture, not-accepted status, and claim blocks
```

## Gate

Decision: **node_sources_probed; node_fixture_replacement_held**

Rationale: selected public port source candidates now have bounded reachability
metadata. Facility field extraction, node-catalog fixture replacement, and role
review remain open.
