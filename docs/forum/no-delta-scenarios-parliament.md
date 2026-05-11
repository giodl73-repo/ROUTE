# No-Delta Scenario Parliament Review

Date: 2026-05-10  
Review id: F5-08  
Review type: Parliament  
Artifact reviewed: `data/pressure-test-scenarios.csv`; `data/throughput-proof-matrix.csv`  
Roles: Traffic Engineer, Freight Economist, Climate Resilience Engineer

## Question

How should ROUTE treat pressure scenarios where the graph is bound and executable, but the synthetic demand run shows no throughput delta?

## Finding

A no-delta executable scenario is useful as a fixture, but it is not positive evidence for a standard.

The current Donner and Atlanta rows are honest because they state the limitation directly. The Forum confirms that this label must survive Blueprint: no-delta scenarios may prove that the scenario is wired, but they do not prove that the intervention works, that the standard earns capital, or that the system is resilient.

## Scenarios Reviewed

| Scenario | Current status | Forum interpretation |
|---|---|---|
| `S-L2-DONNER` | Bound I-80 edge IDs, but synthetic demand run shows no throughput delta | Executable fixture; not proof of weather-resilience benefit |
| `S-L2-ATLANTA` | Bound I-75/I-285 edge IDs, but synthetic demand run shows no throughput delta | Executable fixture; not proof of managed-lane or C-D-road benefit |
| `TP-RES-DONNER` | Alternate capacity and rail diversion not modeled | Resilience hypothesis, not validated recovery proof |
| `TP-CONG-ATLANTA` | Demand surge/take-up/downstream merge calibration missing | Congestion hypothesis, not managed-lane proof |

## Voice Notes

### Traffic Engineer

No throughput delta means the stressor is not binding under the current demand assignment. It does not mean the real corridor is safe or the intervention is unnecessary. It means the fixture has not loaded the failure mode correctly.

For Donner, the model needs demand focused on the I-80 mountain crossing, closure timing, queue formation, alternate road capacity, and recovery behavior. For Atlanta, it needs peak demand, weaving/merge geometry, toll/take-up assumptions, and downstream capacity.

### Freight Economist

No-delta scenarios cannot support benefit-cost claims. If the model shows no avoided delay, no recovered throughput, and no reliability gain, then the economic benefit is zero inside that model. The correct response is not to infer real-world benefit anyway; it is to calibrate the scenario or withhold NPV claims.

Blueprint should block any investment ranking that cites Donner or Atlanta no-delta scenarios as evidence of monetized benefit.

### Climate Resilience Engineer

The Donner no-delta result is especially dangerous if misread. A mountain-pass closure with no modeled throughput delta usually means the model does not represent trapped queues, truck-capable alternate capacity, winter reliability, or rail diversion, not that the system is resilient.

A resilience scenario must test independence of alternates. If the detour shares the same hazard class or lacks winter truck capacity, it is not redundancy.

## Required Scenario Promotion Ladder

| Level | Meaning | Allowed claim |
|---|---|---|
| Bound fixture | Edges and scenario file run | "Executable heuristic fixture" |
| Loaded stressor | Demand/failure mode creates degradation consistent with scenario purpose | "Scenario produces bounded stress response" |
| Intervention sensitivity | Intervention changes throughput, PTI, recovery, or exposure under sensitivity ranges | "Modeled benefit under stated assumptions" |
| Source-validated | Direct source data or calibrated model validates demand, capacity, hazard, or PTI | "Publication/Blueprint evidence candidate" |

Donner and Atlanta are currently between bound fixture and loaded stressor. They should not be promoted beyond that until their next evidence steps are complete.

## Blueprint Rules

1. A scenario with no throughput/PTI/recovery delta cannot be cited as evidence of a standard's benefit.
2. No-delta rows must keep an explicit `no_delta` or equivalent caveat in Blueprint notes.
3. Feature packages may cite no-delta scenarios only as evidence of implementation readiness, not outcome effectiveness.
4. Donner next evidence must include I-80 mountain-crossing demand, truck-capable alternates, and rail diversion.
5. Atlanta next evidence must include peak demand, managed-lane take-up, downstream merge/capacity, and spillback sensitivity.

## Decision

No-delta executable scenarios remain in the pressure-test library, but they are barred from supporting Blueprint benefit claims until promoted to loaded-stressor and intervention-sensitivity status.
