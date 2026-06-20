---
name: International EU Rhine-Alpine Kernel Application 001
slug: international-eu-rhine-alpine-kernel-application-001
type: review
status: draft
rubric_version: v1.0
author: codex
created: 2026-06-20
updated: 2026-06-20
sources:
  - tools/build_eu_rhine_alpine_kernel_application.py
  - tools/check_eu_rhine_alpine_kernel_application.py
  - data/international-eu-rhine-alpine-kernel-application-001.csv
  - data/international-adapter-proof-kernel-001.csv
  - docs/reports/international-adapter-proof-kernel-report.md
  - docs/reviews/international-eu-rhine-alpine-adapter-source-pack-001.md
---

# International EU Rhine-Alpine Kernel Application 001

## Result

This review compares the EU Rhine-Alpine state against the reusable
international adapter proof kernel.

EU now has the first kernel step started: source-custody preflight. The other
kernel steps remain held. This is the correct result for a second-region switch:
it proves the generic process can be started outside Canada without pretending
the EU example has reached Canada-level internal proof.

## Kernel Status

| Kernel Step | EU Status | Promotion Decision |
|---|---|---|
| Source custody | source pack preflight declared | preflight ready; not promoted |
| Parser contract | not started | held |
| Fixture replacement | not started | held |
| Target posture | target assumptions held | held |
| Review packet | not started | held |

## Allowed Language

Use:

- "EU Rhine-Alpine is the second-region application of the generic proof
  kernel."
- "EU source custody has started; parser, fixture replacement, internal proof,
  and external review remain held."
- "The EU hierarchy map remains a heuristic-held review surface until
  source-row validation and role review close."

Do not say:

- "ROUTE has validated an EU corridor."
- "EU institutions, member states, or corridor bodies approved or endorsed
  ROUTE."
- "The EU example proves geometry, topology, SLA, ROI, construction readiness,
  eligibility, compliance, public readiness, or external readiness."

## Gate

Decision: **eu_kernel_application_started; internal_proof_not_started**

Run:

```powershell
npm run check:eu:kernel-application
```

Rationale: EU Rhine-Alpine is now the next region in the proof-kernel ladder,
but only at source-pack preflight. Canada remains the completed internal proof
example; EU is the second-region source-custody application.
