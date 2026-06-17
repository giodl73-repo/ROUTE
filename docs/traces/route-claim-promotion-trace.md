---
name: ROUTE Claim Promotion Trace
slug: route-claim-promotion-trace
type: trace
status: draft
rubric_version: v1.0
author: codex
created: 2026-06-17
updated: 2026-06-17
sources:
  - docs/vtrace/COMMUNICATIONS_STRATEGY.md
  - docs/vtrace/REQUIREMENTS.md
  - docs/vtrace/VERIFICATION.md
  - docs/reports/route-evidence-posture.md
  - docs/reviews/communications-pressure-test-run-001.md
  - docs/how-to/run-route-demo.md
---

# ROUTE Claim Promotion Trace

## Purpose

This trace shows how a communications claim moves from idea to evidence label,
review, blocker, and next step.

This file does not promote any claim to official, construction-ready,
guaranteed, ROI-positive, compliance-ready, eligibility-ready, or
endorsement-ready status.

## Claim Trace Table

| Claim ID | Claim Text | Requirement ID | Artifact Path | Command | Source Status | Evidence Label | Role Review Pointer | Blocker | Decision | Next Step |
|---|---|---|---|---|---|---|---|---|---|---|
| TRACE-CLAIM-001 | ROUTE can generate a current stop/SLA service surface. | REQ-001 / REQ-006 | `target/demo/beck-stop-sla-demo.csv` | `cargo run -q -p route -- stop-sla-surface --output target\demo\beck-stop-sla-demo.csv` | repo-local generated artifact | implemented / heuristic | `docs/reports/route-evidence-posture.md` | Captured demo evidence needed for Round 5 readiness. | use internally with label | Capture command output in `docs/evidence/round5-demo-capture.md`. |
| TRACE-CLAIM-002 | ROUTE can summarize and gate stop spacing against a planning threshold. | REQ-006 / REQ-007 | `target/demo/beck-stop-sla-demo.csv` | `cargo run -q -p route -- stop-sla-summary --input target\demo\beck-stop-sla-demo.csv --top 8 --gate-max-gap 250` | repo-local generated artifact | implemented / heuristic | `docs/reports/route-evidence-posture.md` | Planning threshold is not guaranteed operating SLA. | use internally with disclaimer | Preserve planning-target language and record pass/hold output. |
| TRACE-CLAIM-003 | ROUTE can produce a candidate docket for recurring stop/SLA gaps. | REQ-006 / REQ-007 | `target/demo/beck-stop-sla-candidates-demo.csv` | `cargo run -q -p route -- stop-sla-candidates --input target\demo\beck-stop-sla-demo.csv --output target\demo\beck-stop-sla-candidates-demo.csv --target-gap 250 --top 8 --gate --gate-no-algorithmic` | repo-local generated artifact | implemented / source-needed | `docs/reports/requirement-to-refinement-demonstration-report.md` | Candidate rows are review scaffolds, not construction recommendations. | use internally with hold labels | Capture output and source-needed posture. |
| TRACE-CLAIM-004 | ROUTE can generate T2 schematic maps and diagnostics. | REQ-006 / REQ-007 | `target/demo/beck-schematic-t2-only-demo.png`; `target/demo/beck-t2-diagnostics-demo.csv` | `cargo run -q -p route -- map BECKT2ONLY --output target\demo\beck-schematic-t2-only-demo.png`; `cargo run -q -p route -- beck-t2-diagnostics --output target\demo\beck-t2-diagnostics-demo.csv --gate` | repo-local generated artifact | implemented / schematic / heuristic | `docs/reports/maps-are-not-proof-report.md` | Map render validity is not SLA, upgrade, terminal, asset, or construction proof. | use only as structural map surface | Apply standard map caption pattern. |
| TRACE-CLAIM-005 | Structural maps make Interstate 2.0 visible but are not proof. | REQ-002 / REQ-010 | `docs/reports/maps-are-not-proof-report.md`; `docs/map-publication-scope.md` | inspection | repo-local source docs | story-ready / held-claim surface | `docs/reviews/communications-role-review.md` | Claim-specific evidence not closed for SLA, upgrade, terminal, asset, environmental, or construction readiness. | approved as guardrail | Use captions and non-claims in every map-facing meeting. |
| TRACE-CLAIM-006 | ROI starts as an evidence contract, not a number. | REQ-002 / REQ-008 / REQ-010 | `docs/reports/route-roi-cost-framework.md`; `docs/reports/roi-without-fake-numbers-report.md` | inspection | repo-local source docs | story-ready framework / numeric claims gated | `docs/reviews/communications-role-review.md` | Price year, cost basis, source pack, uncertainty, exclusions, and numeracy review not closed. | approved as non-promotion | Build ROI source-pack template before any number. |
| TRACE-CLAIM-007 | A tighter planning threshold changes the stop/SLA candidate artifact. | REQ-006 / REQ-007 / REQ-010 | `target/demo/beck-stop-sla-candidates-225-demo.csv` | `cargo run -q -p route -- stop-sla-candidates --input target\demo\beck-stop-sla-demo.csv --output target\demo\beck-stop-sla-candidates-225-demo.csv --target-gap 225 --top 5 --gate` | repo-local generated artifact | implemented / heuristic / source-needed | `docs/evidence/round5-demo-capture.md` | Stress-threshold candidates require source and role review before promotion. | use internally as before/after fixture | Keep named candidates heuristic and midpoint rows held/source-needed. |
| TRACE-CLAIM-008 | A stress-threshold promotion scaffold can expose source-needed review rows without promoting construction. | REQ-002 / REQ-007 / REQ-010 | `target/demo/beck-stop-sla-promotions-225-demo.csv` | `cargo run -q -p route -- stop-sla-promotions --input target\demo\beck-stop-sla-candidates-225-demo.csv --output target\demo\beck-stop-sla-promotions-225-demo.csv --gate` | repo-local generated artifact | source-needed / held | `docs/evidence/round5-demo-capture.md` | Rows are append-ready review scaffolds; not recommendations. | hold from public claims | Validate real interchange/service-city candidates and role review before promotion. |

## Promotion Rules

| Rule | Required Before Promotion |
|---|---|
| Numeric claim | Source path/title/date/access note, units, uncertainty, reviewer, and if financial, price year. |
| Map claim | Map level, claim label, excluded claims, artifact/gate pointer. |
| Demo claim | Command, output path, observed status, row count when applicable, pass/hold/fail, and non-claim label. |
| ROI/cost claim | Price year, time horizon, cost basis, benefit/cost inclusion/exclusion, uncertainty, negative-case handling, role review. |
| Public or political claim | Scope Keeper, Citation Auditor, Numeracy Checker, and affected stakeholder lanes. |
| Technical review claim | Trace row, demo/evidence capture where applicable, source-pack schema, verification gate. |

## Gate

Decision: pass_with_risk

Rationale: This trace creates the reviewer-facing walk from claim to artifact,
label, blocker, and next step. It remains draft until captured demo evidence and
source-pack templates close the Round 5 readiness blockers.
