# Standards Package Parliament Review

Date: 2026-05-10  
Review id: F5-07  
Review type: Parliament  
Artifact reviewed: `docs/forum/standards-stakeholder-pass.md`  
Roles: Anthony Foxx, Freight Economist, State-build lens, Rural Advocate, Climate Resilience Engineer

## Question

Does the stakeholder classification make the Interstate 2.0 standards package safer for Blueprint, or does it hide the hardest tradeoffs inside neutral-sounding classes?

## Finding

The stakeholder classification is necessary but not sufficient.

It prevents the standards package from entering Blueprint as an undifferentiated wish list, but the four classes can still be abused. "Operational must-have" can become a magic word for unfunded mandates. "Mitigation companion" can become decorative if it is not budgeted and tested. "Conditional expansion" can become default expansion if the condition is weak.

Blueprint must therefore treat each class as a contract, not a label.

## Voice Notes

### Anthony Foxx

The classification helps because it forces expansion standards to carry mitigation companions. That is the correct instinct. But the environmental/community burden cannot be an afterthought attached after lanes and flyovers are already selected.

If managed freight lanes, C-D roads, flyovers, or spurs add diesel exposure, noise, runoff, or neighborhood division, the mitigation package must be part of the project scope and score. It cannot be a later promise. Blueprint should also require an exposure check for communities within the freight corridor impact zone before any expansion feature gets a "program" label.

### Freight Economist

The classification usefully separates high-return operations from capital-heavy expansion. `T1-REST`, `T1-BRIDGE`, `T1-OPS-WIM`, and `T1-RECOVERY` are likely to produce reliability value without the same footprint risk as new lane-miles.

But `T1-OPS-FREIGHT-LANES`, `T1-FLYOVER`, `T1-CD`, and `T2-RELIEF` need corridor-level benefit tests. The Freight Economist will not accept a national program that assumes every conditional expansion has positive NPV. The condition must include demand, take-up, downstream bottleneck, and lifecycle cost.

### State-Build Lens

The DOT implementation problem is correctly surfaced. A feature can be nationally desirable and still fail delivery because it creates unfunded maintenance obligations or impossible right-of-way requirements.

Blueprint should add delivery fields to every feature package: existing alignment reuse, ROW complexity, likely federal funding program, state match burden, lifecycle maintenance burden, and environmental review complexity.

### Rural Advocate

The classification protects rural access only if `source-gated must-have` does not become a parking lot where rural standards die. `T1-SPURS`, `T2-RESILIENCE`, `T3-OPERATIONS`, and `T3-COVERAGE` are not luxuries for rural users. They are access and life-safety standards.

Blueprint should require a rural-access exception rule: low volume cannot by itself demote a resilience or access feature when the detour is extreme, the community has no alternate, or agricultural export access is at stake.

### Climate Resilience Engineer

The mitigation companion class is strongest where it changes actual system performance: EV freight charging that reduces emissions, intermodal diversion that reduces closure demand, climate hardening that reduces expected outage days, and transit hubs that reduce access burden.

But mitigation only counts if it is measured. A feature should not be credited as climate mitigation just because it sounds greener. It needs an emissions, exposure, outage, or diversion metric.

## Earned Claims

| Claim | Status |
|---|---|
| Four stakeholder classes are a valid Blueprint intake structure | Earned |
| Rest, bridge, WIM, recovery, and maintenance should be early operations/safety foundations | Earned |
| Expansion features need corridor-specific demand, ROW, lifecycle, and mitigation evidence | Earned |
| Rural resilience/access features should not be demoted by low volume alone | Earned |

## Held Or Amended Claims

| Claim | Status | Required amendment |
|---|---|---|
| "Operational must-have" means early build | Held | Requires source label, delivery path, and no hidden maintenance impossibility |
| "Mitigation companion" is sufficient by label | Refuted | Must be budgeted, scoped, and measured |
| "Conditional expansion" can enter a national sequence by default | Refuted | Requires corridor-specific benefit/cost and burden case |
| Rural source-gated standards can wait indefinitely | Refuted | Add rural-access exception rule |

## Blueprint Intake Rules

1. Every feature package must carry `stakeholder_class`.
2. Every expansion package must carry `mitigation_companion`, `ROW_complexity`, `maintenance_burden`, and `community_exposure_check`.
3. Every source-gated rural/access feature must carry a `rural_access_exception` check.
4. Every mitigation companion must carry a metric: emissions, exposure, outage days, diverted demand, or access burden.
5. No feature moves from Forum to Blueprint only because its class sounds important.

## Decision

Advance the standards classification to Blueprint intake, with the five intake rules above as hard constraints.
