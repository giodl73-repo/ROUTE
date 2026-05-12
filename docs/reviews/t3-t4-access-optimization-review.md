# T3/T4 Access Optimization Review

Date: 2026-05-12

## Scope

Reviewed:

- `docs/t3-t4-access-optimization.md`

Roles used:

- `.roles/parliament/optimization-methodologist.md`
- `.roles/parliament/schematic-cartographer.md`
- `.roles/panel-reviewer/R-T4.md`
- `.roles/panel-reviewer/R-T10.md`
- `.roles/parliament/rural-advocate.md`
- `.roles/stakeholders/regional-shipper.md`
- `.roles/stakeholders/state-dot.md`

## Verdict

Pass as doctrine, implementation hold.

The spec correctly frames T3/T4 as zone and access obligations rather than
leftover route tiers. It also preserves the key optimizer constraint: lower-tier
pressure may bubble upward only through contact, access, stop, topology, or SLA
witnesses, not score alone.

It is not yet implementation-complete because the artifacts it calls for do not
exist: zone access obligations, T3 route/stop columns, T4 terminal/local access
columns, and full zone-map diagnostics.

## Findings

1. Optimization Methodologist: Pass with artifact hold.
   The spec separates hard constraints from score pressure and defines clear gap
   classes. The missing next artifact is a machine-readable zone obligation
   table: zone id, obligation class, access target, service horizon, candidate
   routes/stops, selected repair, and validation status.

2. Schematic Cartographer: Pass with map hold.
   The map rule is right: T3 maps must include local T1/T2 context and cannot be
   standalone horizontal placeholders. The implementation hold is that current
   T3 maps are atlas-tracked, but not yet generated from selected zone
   route/stop columns.

3. R-T4 Network Scientist: Needs graph definition next.
   The spec names zone-first optimization, but a future implementation must
   declare whether each zone uses a primal stop graph, dual route graph, path
   graph, or layered access graph, and what weights are balanced.

4. R-T10 Transit Planner: Pass on service meaning.
   The spec avoids national-map clutter and treats lower tiers as access
   networks. The next user-facing improvement is to describe what a 6h feeder
   or 1h terminal promise lets a shipper, traveler, or game player do.

5. Rural Advocate: Conditional pass.
   The spec finally gives rural production zones and smaller communities a
   formal access surface. It needs county/zone obligation rows so low-volume
   rural access does not disappear behind route score thresholds.

6. Regional Shipper: Conditional pass.
   The T4 terminal/local access framing is strong. It needs drayage, port-gate,
   intermodal, warehouse district, truck parking, rest, and charging fields
   before shippers can treat the output as operationally meaningful.

7. State DOT Planner: Delivery hold.
   The spec should eventually attach local funding, jurisdiction, ROW,
   maintenance, and permitting complexity to T4 access repairs. Local access is
   often where delivery gets hardest.

## Required Next Artifacts

1. `data/t3-zone-access-obligations.csv`
2. `data/t3-zone-route-columns.csv`
3. `data/t4-terminal-access-columns.csv`
4. `data/t3-t4-access-gaps.csv`
5. `data/t3-zone-map-diagnostics.csv`

## Decision

Accept `docs/t3-t4-access-optimization.md` as the doctrine for lower-tier access
optimization.

Do not claim T3/T4 optimization is implemented until a gate can prove selected
zone and terminal access columns satisfy the obligations above.
