# Plan: Freight Reliability on NY–LA and Houston–Chicago Corridors

**Track**: C — Freight & Throughput
**Venue**: Transportation Research Part B: Methodological
**Target**: 9,000–11,000 words

## The Questions

Four questions that every major shipper on these corridors needs answered:

1. **Throughput**: What is the current maximum freight throughput on NY→LA and HOU→CHI?
2. **Transit time**: What transit time can a shipper realistically commit to today?
3. **Resilience**: If the primary corridor has an incident, what is the best alternate and how much time does it add?
4. **Guarantee**: What would it take — in infrastructure terms — to offer a transit time SLA to shippers?

## The Method

**Corridor analysis**: Apply ROUTE scoring and max-flow analysis to the primary and alternate corridors:
- NY→LA: I-80 (primary), I-40 (southern alternate), I-70+I-15 (Denver alternate)
- HOU→CHI: I-45→I-35→I-55 (primary), I-69 (proposed direct, not yet built)

**Max-flow**: Edmonds-Karp on the national graph with source nodes near NYC/Houston, sink nodes near LA/Chicago. Reports binding bottleneck capacity and best alternate path.

**Transit time model**: Distance / speed_limit (from HPMS, state-weighted) × HOS factor (11h driving per day → days in transit). PTI-adjusted window.

**Incident simulation**: Remove binding bottleneck edge from graph, recompute max-flow and path. Report throughput drop and added hours.

**I2.0 scenario**: Model managed freight lanes (+65mph sustained, no stops) and compute new transit time distribution.

## The Finding (hypothesis)

- NY→LA primary capacity: ~180k vpd on full-build I-80 (4 lanes each direction urban, 2 rural — Donner is the binding constraint at ~91k vpd)
- NY→LA transit: 4.5 days current, 3.5 days with I2.0 managed lanes (1 full day improvement)
- Donner Pass closure (50 days/year): reroute to I-40 adds 4-6 hours; no interstate-standard alternate
- HOU→CHI: requires 3 corridor hops — no direct route. I-69 completion reduces distance by 120 miles and eliminates 2 interchange nodes
- Transit time SLA: PTI ≤ 1.15 on managed lanes → 48-hour commitment window for a 43-hour trip

## Key Claims

- C1: Donner Pass is the national single point of failure for the northern transcontinental route — no interstate-standard alternate
- C2: I-69 completion is the highest-value missing link for the Houston-Chicago O-D pair
- C3: Managed freight lanes reduce transcontinental transit time by ~20% and enable shipper SLAs
- C4: Current PTI values on I-80 (1.8–2.2) require 80–100 hour commitment windows, making JIT logistics impossible for cross-country truck freight

## Sections

1. Introduction — The four questions; why they matter for logistics
2. Background — O-D freight flows; HOS regulations; transit time economics
3. Data & Methods — ROUTE corpus; HPMS HPMS; max-flow algorithm; PTI model
4. NY–LA Corridor Analysis — Primary path; capacity; bottlenecks; alternates
5. Houston–Chicago Corridor Analysis — Primary path; I-69 gap; alternates
6. Incident Simulation — Donner closure; I-35 Oklahoma incident; recovery paths
7. Interstate 2.0 Scenario — Managed lanes; PTI targets; SLA feasibility
8. Conclusion
