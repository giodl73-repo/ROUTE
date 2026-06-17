---
name: Round 5 Demo Capture
slug: round5-demo-capture
type: evidence
status: draft
rubric_version: v1.0
author: codex
created: 2026-06-17
updated: 2026-06-17
sources:
  - docs/how-to/run-route-demo.md
  - docs/traces/route-claim-promotion-trace.md
  - docs/reports/requirement-to-refinement-demonstration-report.md
---

# Round 5 Demo Capture

## Scope

This record captures one current ROUTE demo command bundle for internal Round 5
technical review rehearsal.

It does not prove a final optimizer, construction recommendation, official plan,
operating SLA, public-release readiness, positive ROI, eligibility, compliance,
or endorsement.

## Environment

| Field | Value |
|---|---|
| Working directory | `C:\src\TRACKER\repos\applied-systems\route` |
| Output directory | `target\demo` |
| Capture date | 2026-06-17 |
| Claim posture | implemented / heuristic / source-needed / held by row |

## Command Evidence

| Command ID | Command | Output Path | Observed Status | Row Count / Size | Pass / Hold / Fail | Non-Claim Label |
|---|---|---|---|---|---|---|
| DEMO-CMD-001 | `cargo run -q -p route -- stop-sla-surface --output target\demo\beck-stop-sla-demo.csv` | `target\demo\beck-stop-sla-demo.csv` | wrote stop/SLA surface; source evidence status reported as `heuristic-planning` | 12,404 lines; 4,211,517 bytes | pass | Generated service surface; not SLA guarantee. |
| DEMO-CMD-002 | `cargo run -q -p route -- stop-sla-summary --input target\demo\beck-stop-sla-demo.csv --top 8 --gate-max-gap 250` | console summary | max stop gap reported as 248 mi; stop SLA max-gap gate PASS | n/a | pass | Planning threshold gate; not operating proof. |
| DEMO-CMD-003 | `cargo run -q -p route -- stop-sla-candidates --input target\demo\beck-stop-sla-demo.csv --output target\demo\beck-stop-sla-candidates-demo.csv --target-gap 250 --top 8 --gate --gate-no-algorithmic` | `target\demo\beck-stop-sla-candidates-demo.csv` | inspected gaps: 0; candidate gate PASS; named-candidate gate PASS | 1 line; 321 bytes | pass / hold | Header-only docket means no open candidate rows at this threshold; not a construction recommendation. |
| DEMO-CMD-004 | `cargo run -q -p route -- stop-sla-promotions --input target\demo\beck-stop-sla-candidates-demo.csv --output target\demo\beck-stop-sla-promotions-demo.csv --gate` | `target\demo\beck-stop-sla-promotions-demo.csv` | promotion rows: 0; promotion gate PASS | 1 line; 219 bytes | pass / hold | Header-only promotion scaffold; no promotion claim. |
| DEMO-CMD-005 | `cargo run -q -p route -- map BECKT2ONLY --output target\demo\beck-schematic-t2-only-demo.png` | `target\demo\beck-schematic-t2-only-demo.png` | rendered Beck T2-only schematic; 2400 x 1350 | 732,205 bytes | pass | Structural map surface; not proof of SLA, upgrade, terminal, asset, or construction readiness. |
| DEMO-CMD-006 | `cargo run -q -p route -- beck-t2-diagnostics --output target\demo\beck-t2-diagnostics-demo.csv --gate` | `target\demo\beck-t2-diagnostics-demo.csv` | T2 lines: 26; diagnostics gate PASS | 27 lines; 4,569 bytes | pass_with_risk | Diagnostics classify review categories; not a route-promotion claim. |
| DEMO-CMD-007 | `cargo run -q -p route -- beck-t2-service-standards --output target\demo\beck-t2-service-standards-demo.csv --gate` | `target\demo\beck-t2-service-standards-demo.csv` | service classes: 4; standards gate PASS | 5 lines; 1,197 bytes | pass | Service-class contract; not proof of delivery readiness. |
| DEMO-CMD-008 | `cargo run -q -p route -- beck-t2-qualification-actions --output target\demo\beck-t2-qualification-actions-demo.csv --gate` | `target\demo\beck-t2-qualification-actions-demo.csv` | qualification actions: 4; actions gate PASS | 5 lines; 1,447 bytes | pass | Qualification-action contract; review categories do not imply construction. |

## Before / After Fixture Status

| Field | Entry |
|---|---|
| Fixture ID | ROUND5-FIXTURE-001 |
| Requirement | Show how a local/state/industry requirement changes an artifact, hold, or evidence label. |
| Before artifact | Current stop/SLA surface and T2 diagnostics captured above. |
| Changed input / threshold / evidence field | Not yet captured in this run. |
| After artifact or held row | Pending. |
| Evidence label change | Pending. |
| Role-review implication | Pending Optimization Methodologist, State DOT, rural/freight/community lane review. |
| Claim boundary | Current capture proves the command path, not a full before/after optimizer fixture. |

## Gate

Decision: pass_with_risk

Rationale: The current command bundle is captured and passes its documented
gates. Round 5 technical readiness still requires a before/after fixture and
claim-specific source packs before stronger demo claims can be promoted.
