# FLETCH Source Orchestration Role Review

Date: 2026-05-15

## Scope

Reviewed:

- `docs/fletch-source-orchestration-spec.md`
- `docs/source-fetch-cache-policy.md`
- `data/source-fetch-policy.csv`

Roles used:

- `.roles/parliament/optimization-methodologist.md`
- `.roles/parliament/traffic-engineer.md`
- `.roles/parliament/schematic-cartographer.md`
- `.roles/stakeholders/state-dot.md`
- `.roles/stakeholders/freight-industry.md`
- `.roles/editorial/scope-keeper.md`
- `.roles/editorial/citation-auditor.md`
- `.roles/editorial/numeracy-checker.md`

## Verdict

Pass as an integration doctrine and implementation-start spec.

The spec correctly makes FLETCH the neutral acquisition/cache/url management
substrate while preserving ROUTE as the owner of transportation evidence,
promotion gates, optimizer decisions, and publication claims. It is ready to
drive the first ROUTE implementation slice: a non-mutating FLETCH registry and
adapter-handoff bridge over existing source families.

## Findings

1. Optimization Methodologist: Pass.
   The spec separates orchestration from claim promotion. Cacheline success does
   not become optimizer feasibility, publication readiness, route selection, or
   stop selection. The implementation phases start with non-mutating registry
   coverage, which gives counterfactual audit value without baking in a selected
   acquisition path.

2. Traffic Engineer: Pass.
   HPMS, NBI-adjacent pavement/condition, FEMA, ACS, and event snapshot inputs
   remain subject to ROUTE parser and gate logic. The spec does not let FLETCH
   assert operational condition, reliability, or incident frequency from the
   presence of a cached file.

3. Schematic Cartographer: Pass.
   The spec keeps maps and publisher bundles downstream of source truth. Local
   URL maps and proof docs improve traceability, but they do not invent stops,
   transfers, bends, service boundaries, or map publication readiness.

4. State DOT Planner: Pass.
   The spec preserves manual/cached proof and non-FLETCH exceptions for sources
   that require credentialed, local, or reviewed access. That is necessary for
   state DOT material, long-range plan artifacts, and source families that cannot
   be treated as public anonymous downloads.

5. Freight Industry: Pass.
   The spec supports repeatable acquisition for freight-relevant sources without
   softening evidence standards. Weight limits, clearances, bottlenecks, and
   operational constraints still need ROUTE evidence rows before they affect
   freight claims or investment logic.

6. Scope Keeper: Pass.
   The artifact stays within an `artifact-contract` role. It defines boundaries,
   cacheline identities, phases, and acceptance gates; it does not claim the
   bridge is implemented.

7. Citation Auditor: Pass.
   The spec's source-family claims are traceable to local ROUTE artifacts:
   `docs/source-fetch-cache-policy.md` and `data/source-fetch-policy.csv`.
   External factual claims are not introduced.

8. Numeracy Checker: Pass.
   The only ordinal counts are process counts: four acquisition modes, four
   implementation phases, and five acceptance gates. They match the listed
   sections and do not introduce transportation measurements.

## Required Next Artifacts

- ROUTE-owned FLETCH registry over existing source families.
- Non-mutating ROUTE command/report for FLETCH adapter handoff readiness.
- Registry validation smoke that proves coverage without touching existing cache
  files.

## Decision

Accept `docs/fletch-source-orchestration-spec.md` as the ROUTE/FLETCH bridge
doctrine. Start implementation with a registry and adapter-handoff report before
delegating live fetch commands.

## Implementation Addendum

Reviewed implementation artifacts:

- `data/fletch-registry.json`
- `data/fletch-source-handoff.csv`
- `route fletch-sources --gate`
- `route fetch`
- `route_data::fetch_all_manifest_sources_with_fletch`

Verdict: pass.

The implementation satisfies the accepted boundary. `route fetch` now delegates
manifest-backed HTTP acquisition through FLETCH before atomically writing the
legacy ROUTE cache path. HPMS, ACS, FEMA, and live-event source families are
represented as adapter-owned FLETCH cachelines, which is the correct treatment
because ROUTE still owns their parsers, scoped merges, envelope checks, and
evidence promotion rules.

Role findings:

1. Optimization Methodologist: Pass.
   `route fletch-sources --gate` proves every source-fetch policy family has a
   FLETCH cacheline without letting cache acquisition become optimizer
   feasibility.

2. Traffic Engineer: Pass.
   HPMS and condition-adjacent data stay parser-gated in ROUTE. The handoff row
   can say `adapter-required` or `generic-fetch-ready`, but it cannot promote a
   throughput, pavement, reliability, or safety claim.

3. Schematic Cartographer: Pass.
   The new registry graph exposes downstream source relationships without
   changing Beck, map atlas, stop, transfer, or publication truth.

4. State DOT Planner: Pass.
   Manual and credential-sensitive sources remain adapter/manual cachelines
   rather than anonymous public downloads.

5. Freight Industry: Pass.
   Freight-relevant source acquisition is more repeatable, but constraints such
   as bottlenecks, bridge limits, and reliability still need ROUTE evidence rows.

6. Editorial roles: Pass.
   Scope, citation, and numeracy remain clean. The addendum records
   implementation artifacts and does not introduce external transportation
   measurements.

