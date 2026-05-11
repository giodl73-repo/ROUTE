# Milepost 6 Blueprint Plan

Status: active. This stage turns Forum-reviewed pressure-test output into Interstate 2.0 feature packages without promoting heuristic claims to proof.

## Done Criteria

Milepost 6 is done when every Blueprint package has a stakeholder class, evidence label, delivery status, cost/source placeholder, Forum constraint, mitigation/exposure fields where required, and a named next evidence step. Expansion packages must stay conditional until mitigation, ROW complexity, lifecycle maintenance, and community exposure are recorded.

The first executable gate is:

```powershell
cargo run -q -p route -- blueprint --gate --details
```

## Intake Rules From Forum

1. `route standards-proof --gate-blueprint` stays locked until proof gaps are resolved or downgraded.
2. Every feature package carries `stakeholder_class`.
3. Conditional expansion packages carry mitigation, ROW complexity, maintenance burden, and community exposure fields.
4. Source-gated rural/access packages carry a rural-access exception field.
5. C.1 SLA/PTI and reliability-dollar claims stay heuristic until NPMRDS/FPM or validated queueing evidence exists.
6. Donner and Atlanta no-delta scenarios are fixture-readiness evidence only until loaded stressor and intervention sensitivity exist.

## Slice Tasklist

| Slice | Task | Status | Exit Gate / Artifact |
|---|---|---|---|
| B6-A | Create Blueprint package ledger and CLI gate | ✅ done | `data/blueprint-feature-packages.csv`; `route blueprint --gate --details` |
| B6-B | Write feature-package taxonomy and package briefs | ✅ done | `docs/blueprint/feature-packages.md` names operational, source-gated, conditional expansion, and mitigation packages |
| B6-C | Build evidence downgrade map against standards proof | ✅ done | `data/blueprint-evidence-map.csv`; `route blueprint-evidence --gate --details` |
| B6-D | Add cost and lifecycle range ledger | ✅ done | `data/blueprint-cost-ranges.csv`; `route blueprint-costs --gate --details` |
| B6-E | Draft phase sequence and dependency graph | ✅ done | `data/blueprint-phase-sequence.csv`; `docs/blueprint/phase-sequence.md` |
| B6-F | Amend Interstate 2.0 design spec | ✅ done | `specs/2026-05-06-interstate-2-design.md` now inherits package/evidence/cost labels |
| B6-G | Run Milepost 6 gate bundle and closeout | ✅ done | `docs/milepost-6-closeout.md`; CLI gates, tests, tracker, and closeout doc pass together |

## Current Package Spine

Phase 0 starts with operational and source-foundation work that can improve measurement without requiring new benefit claims: relay routing operations plus rest/WIM/bridge source joins.

Phase 1 contains proof-target packages: T1 diamond recovery zones, Donner calibration, managed-lane pilots, and EV/rest mitigation. These are held or heuristic until direct sensitivity and source evidence exists.

Phase 2 contains higher-blast-radius expansion and system relief packages: intermodal diversion, rural spurs, and T2 relief corridors. These remain conditional until the evidence map and cost ledger can show why each package is needed.
