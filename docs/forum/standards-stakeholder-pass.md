# Standards Package Stakeholder Pass

Date: 2026-05-10  
Review id: F5-04  
Review type: Stakeholder  
Artifact reviewed: `data/standards-proof-ledger.csv`  
Roles: Long-Haul Trucker, Freight Industry, State DOT Planner, Environmental Community

## Question

Which Interstate 2.0 standards create operational value for users, and which are unfunded wish lists?

## Decision

The standards package should not move into Blueprint as one bundle. It should split into four feature classes:

| Class | Meaning | Blueprint treatment |
|---|---|---|
| Operational must-have | Directly affects driver safety, legal operation, freight reliability, or network recovery | Eligible for early packages if evidence label is Heuristic or better |
| Source-gated must-have | Plausibly essential, but source inventory is missing | Keep in Blueprint backlog with L1/L2 evidence task attached |
| Conditional expansion | Adds capacity or footprint and needs benefit, ROW, environmental, and maintenance proof | No default inclusion; require corridor-specific case |
| Mitigation companion | Must accompany expansion/hardening to control health, runoff, noise, habitat, or community burden | Attach to any expansion package before review |

## Stakeholder Findings

### Long-Haul Trucker

The strongest operational standards are `T1-REST`, `T1-RECOVERY`, `T1-OPS-PTI`, `T1-OPS-SPEED`, `T1-BRIDGE`, and `T1-CLIMATE`. These directly affect whether a driver can make a legal, safe trip without losing the HOS window to avoidable congestion, closures, parking scarcity, grades, or detours.

The rest-area standard should not be treated as an amenity. If the system promises 48-hour freight windows and relay operations while ignoring legal parking and rest spacing, the SLA is being pushed onto drivers as fatigue risk.

### Freight Industry

The freight standards with the clearest industry value are `T1-OPS-FREIGHT-LANES`, `T1-OPS-WIM`, `T1-BRIDGE`, `T1-INTERMODAL`, and `T2-RELIEF`. They remove productive-time losses, clearance/weight restrictions, and closure bottlenecks.

However, `T1-OPS-FREIGHT-LANES` and `T1-FLYOVER` cannot be default national prescriptions yet. They need demand, take-up, downstream merge, cost, and geometry proof by corridor. The industry wants managed reliability, not lane miles that move the bottleneck downstream.

### State DOT Planner

The standards most ready for practical staging are those that improve existing assets or operations without exploding permanent lane-mile obligations: `T1-OPS-WIM`, `T1-REST`, `T1-BRIDGE`, `T1-RECOVERY`, `T2-RESILIENCE`, and targeted `T1-CD`.

The highest delivery risks are `T1-FLYOVER`, broad `T1-OPS-FREIGHT-LANES`, `T1-SPURS`, and greenfield-adjacent resilience work. These need ROW complexity, lifecycle maintenance, federal match strategy, and environmental process labels before Blueprint can schedule them.

### Environmental Community

The package is acceptable only if expansion standards carry mitigation companions. `T1-CLIMATE`, `T1-EV-PASSENGER`, `T1-EV-TRUCK`, `T1-TRANSIT-HUB`, and `T1-INTERMODAL` can reduce exposure or emissions if designed correctly. But managed freight lanes, flyovers, C-D roads, and spurs add impervious surface, noise, runoff, and health burdens unless paired with mitigation.

Environmental review asks for four missing ledgers before public Blueprint claims: population within freight-exposure distance, stormwater treatment/receiving waters, wildlife/habitat crossings, and noise mitigation need.

## Standards Classification

| Standard | Stakeholder class | Forum action before Blueprint |
|---|---|---|
| T1-OPS-PTI | Operational must-have | Keep, but label Heuristic until NPMRDS/FPM validation exists |
| T1-OPS-FREIGHT-LANES | Conditional expansion | Require demand/take-up/downstream merge and environmental mitigation case |
| T1-OPS-WIM | Source-gated must-have | Build weigh-station/WIM inventory before packaging |
| T1-OPS-SPEED | Operational must-have | Keep as service target; source speed/grade assumptions before claims |
| T1-EV-PASSENGER | Mitigation companion | Pair with station inventory/uptime and equity siting |
| T1-EV-TRUCK | Mitigation companion | Pair with freight terminal/rest-area charging and grid outage evidence |
| T1-REST | Operational must-have | Treat as safety/SLA infrastructure, not optional amenity |
| T1-TRANSIT-HUB | Mitigation companion | Keep as access/emissions feature, but require operational access proof |
| T1-DIAMOND-K | Operational must-have | Keep for resilience; manual geometry and empirical failure evidence remain blockers |
| T1-FLYOVER | Conditional expansion | Require site geometry, freight turning demand, cost, and mitigation |
| T1-SPURS | Source-gated must-have | Require spur candidate ledger and truck-capable alternate capacity |
| T1-CLIMATE | Operational must-have | Keep, but require hazard frequency and independent detour exposure checks |
| T1-RECOVERY | Operational must-have | Keep as recovery target; validate operational restoration model |
| T1-INTERMODAL | Mitigation companion | Require commodity/diversion sensitivity before claiming highway relief |
| T1-CD | Conditional expansion | Require metro span inventory, weave benefit, ROW, and mitigation |
| T1-BRIDGE | Operational must-have | Upgrade to high priority once clearance/posting joins exist |
| T2-RELIEF | Conditional expansion | Require per-dollar comparison against T1 widening |
| T2-RESILIENCE | Source-gated must-have | Require alternate-route bridge/capacity inventory |
| T3-COVERAGE | Operational must-have | Keep implemented gap label discipline; do not overbuild artifact counties |
| T3-OPERATIONS | Source-gated must-have | Add regional feeder SLA/access fixture before feature package |
| T4-MAINTENANCE | Operational must-have | Keep as no-expansion state-of-good-repair floor |

## Required Blueprint Changes

1. Blueprint feature packages must include a `stakeholder_class` field using the four classes above.
2. Any capacity-expansion feature must include a mitigation companion or an explicit reason why no new footprint/health burden is created.
3. Rest, WIM, bridge, recovery, and T4 maintenance should be framed as operations/safety foundations, not optional add-ons.
4. Managed freight lanes, flyovers, C-D roads, and spurs must stay corridor-specific until demand, ROW, lifecycle cost, and environmental exposure evidence exists.
5. Transit, EV, and intermodal features should be allowed to count as mitigation only when they reduce emissions, access burden, or closure demand in a measured scenario.

## Docket Outcome

F5-04 is complete. The standards package can enter Blueprint only as a classified package set, not as an undifferentiated list of desirable standards.
