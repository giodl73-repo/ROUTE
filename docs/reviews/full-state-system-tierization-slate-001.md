---
name: Full State System Tierization Slate 001
slug: full-state-system-tierization-slate-001
type: review
status: draft
rubric_version: v1.0
author: codex
created: 2026-06-21
updated: 2026-06-21
sources:
  - data/full-state-system-tierization-slate-001.csv
  - data/full-state-system-tierization-slate-001-scorecard.csv
  - data/full-state-system-tier-role-taxonomy.csv
  - docs/reports/full-state-system-tierization-framework.md
  - docs/reports/state-system-failure-metrics-framework.md
---

# Full State System Tierization Slate 001

## Scope

This slate reworks Texas, California, Florida, and Iowa from selected corridor
examples into full-state tierization samples.

## What Changed

The package no longer asks only which corridors should appear on a map. It asks
what job each part of the state system should perform:

- `T1` for statewide trunk promises;
- `T2` for regional market connectors and trunk alternates;
- `T3` for rural and access connectors;
- `T4` for terminal and local access;
- `R` for resilience overlays;
- `M` for maintained segments with no promoted service role;
- `X` for features outside the current service scope.

## State Pressure Tests

| State | Full-system question |
|---|---|
| Texas | Where is the Triangle too interstate-dependent, and which state/US routes carry border, Gulf, Panhandle, rural, terminal, or evacuation roles? |
| California | Which non-interstate state routes deserve service roles: SR 99, SR 58, SR 299, port access, and coastal redundancy? |
| Florida | Which inland, Gulf, Panhandle, Keys, port, and evacuation roles need to be formalized beyond I-95/I-75/I-4? |
| Iowa | Which US/state highways make Iowa more than I-80/I-35: US 20, US 30, US 63, Iowa 9, terminals, and low-volume maintained routes? |

## Evidence Boundary

These are tierization samples, not full source-backed state inventories. They do
not claim official state designation, legal SLA, construction readiness, cost,
numeric ROI, funding eligibility, compliance, endorsement, external validation,
public readiness, or state approval.

## Gate

Decision: **state_tierization_slate_001_sampled; full_source_inventory_required**
