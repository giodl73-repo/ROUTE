# Beck And SLA Spec Role Review

Date: 2026-05-12

## Scope

Reviewed:

- `docs/beck-renderer-contract.md`
- `docs/sla-promise-portfolio.md`

Roles used:

- `.roles/parliament/optimization-methodologist.md`
- `.roles/parliament/schematic-cartographer.md`
- `.roles/panel-reviewer/R-T4.md`
- `.roles/panel-reviewer/R-T10.md`
- `.roles/parliament/freight-economist.md`
- `.roles/parliament/traffic-engineer.md`
- `.roles/stakeholders/state-dot.md`

## Role Coverage

ROUTE already had partial coverage:

- R-T4 covers graph algorithms, reproducibility, and network formulation.
- R-T10 covers transit-network legibility, coverage/ridership tradeoffs, and
  multimodal network geometry.
- Freight Economist and Traffic Engineer cover value and operational reality.

The missing role gap was explicit advocacy for:

- optimizer formulation as a first-class artifact;
- Beck/schematic cartography as truthful abstraction rather than decoration.

This review adds those two local parliament voices.

## Findings

1. Optimization Methodologist: Pass; requested artifact now exists.
   The SLA spec correctly says score alone is insufficient and that route
   promotion needs a promise, topology, or exception witness.
   `data/t1-sla-candidate-pairs.csv` now provides the ranked candidate table
   for pairs 26+ with drop reasons, so the T1 promise cut line can be audited
   rather than narrated.

2. Schematic Cartographer: Pass with implementation hold.
   The Beck renderer contract has the right topology rules: bends at stops,
   real transfers only, no near-miss contacts, no same-color self-loops. The
   spec is stronger than current rendering, because maps are still alignment
   checked rather than directly optimizer-rendered. That hold is acceptable only
   if future map slices target the new contract.

3. R-T4 Network Scientist: Graph-contract follow-through now started.
   The specs distinguish topology from layout and promises from routes, which is
   correct. `docs/route-stop-column-schema.md` now defines the common promise,
   route, stop, service, repair, graph, and manifest column vocabulary. The next
   methods gap is implementation migration: older artifacts should gradually add
   the shared lineage fields where they are missing.

4. R-T10 Transit Planner: Pass on map doctrine, asks for user meaning.
   The Beck spec is appropriately transit-like: legibility is a function of
   stops, transfers, hierarchy, and service patterns. The SLA spec should
   eventually connect promise horizons to user-facing service concepts so game
   players and reviewers understand what a 48h/36h promise buys.

5. Freight Economist: Conditional pass.
   The SLA promise portfolio has a credible industry-conversion frame, but the
   candidate-pair doctrine still needs commodity flow, air-substitution, port,
   border, and logistics market evidence columns before it can support
   investment-grade claims.

6. Traffic Engineer: Conditional pass.
   The specs do not overclaim physical feasibility, which is good. The next
   bridge is operational: selected routes should eventually join to capacity,
   grade, bridge, rest, and incident evidence before SLA promises are treated as
   operationally earned.

7. State DOT Planner: Delivery hold.
   The specs correctly separate design-grade claims from proof-grade claims.
   They still need a later delivery lens for funding match, ROW complexity,
   maintenance burden, and designation/permitting status.

## Decisions

- Accept both specs as doctrine.
- Add Optimization Methodologist and Schematic Cartographer to the local role
  roster.
- Keep Beck rendering in implementation hold until maps consume optimizer
  artifacts directly.
- Keep SLA promises design-grade until evidence joins exist. The candidate-pair
  cut line is now represented by `data/t1-sla-candidate-pairs.csv`.

## Next Spec Gaps

1. `docs/t3-t4-access-optimization.md`
2. `docs/optimizer-artifact-manifest.md`
