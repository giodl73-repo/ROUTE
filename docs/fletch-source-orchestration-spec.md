---
name: FLETCH Source Orchestration Spec
slug: fletch-source-orchestration
type: artifact-contract
status: accepted
owner: route-data
review: docs/reviews/fletch-source-orchestration-role-review.md
---

# FLETCH Source Orchestration Spec

ROUTE should use FLETCH as the shared fetch/cache/url orchestration substrate while
keeping ROUTE's transportation evidence, scoring, optimizer, and map claims inside
ROUTE.

This is an integration contract, not a migration of ROUTE domain logic into
FLETCH. FLETCH owns neutral acquisition plans, cache manifests, registries,
quivers, adapter handoffs, local URL maps, and publisher bundles. ROUTE owns
which highway sources are acceptable, how cached artifacts become evidence, and
which claims can be promoted.

## Problem

ROUTE has several source families that already behave like cachelines:

| Family | Current entry point | Current cache/evidence surface | FLETCH fit |
|---|---|---|---|
| Manifest downloads | `route fetch` | `data/cache/<manifest filename>` | FLETCH cache manifest and registry entry |
| HPMS national/state fetches | `route fetch-hpms` | `data/cache/hpms_2018.csv`, `data/cache/hpms_<state>.csv` | partitioned cacheline with scoped merge |
| ACS county fetches | `route fetch-acs`, `route fetch-acs-income` | `data/cache/acs_county_*_2022.csv` | cacheline with parse gate before activation |
| FEMA corridor fetches | `route fetch-fema`, `route fetch-fema-d1` | `data/cache/fema_sfha*_counts.csv` | query bundle with validation floor |
| T1 live-event snapshots | `route t1-fetch-*` | `data/cache/*events.json`, `data/cache/*incidents.json` | snapshot cacheline with preserve-last-good semantics |
| Manual/cached proof artifacts | proof intake and artifact attachment ledgers | proof CSV rows plus local cached artifacts | adapter handoff and PROOF document manifest |

The current ROUTE rule remains binding: a failed HTTP request, parse error,
empty response, or partial write must preserve the previous usable cache.

## Integration Goals

1. Express each ROUTE source family as a FLETCH cacheline or cacheline bundle.
2. Preserve ROUTE's temp-then-replace and scoped-merge safety rules.
3. Let ROUTE ask for on-demand, bootstrap, group, or bundle acquisition without
   hardcoding every fetch sequence in ROUTE commands.
4. Emit local URL maps and proof-doc manifests so source evidence can be browsed
   and cited without relying on live remote URLs.
5. Keep publication, optimizer, Beck, game, and evidence-promotion claims gated
   by ROUTE artifacts, not by successful download alone.

## Boundary

FLETCH may provide:

- `fletch.cache-manifest.v1` rows for source URL, cache path, policy, freshness,
  expected format, and dependencies.
- `fletch.registry.v1` edges for source-to-derived relationships.
- `fletch.adapter-sources.v1` reports so ROUTE can see which sources an adapter
  declares before it runs.
- `fletch.registry-validation.v1` and
  `fletch.archive-expansion-preview.v1` reports before archives or bundles are
  activated.
- `fletch.adapter-handoff.v1` reports to hand fetched/cacheable sources back to
  ROUTE commands.
- `fletch.local-url-map.v1`, `fletch.proof-docs.v1`, and
  `fletch.publisher-bundle.v1` for local citation/proof surfaces.

FLETCH must not provide:

- ROUTE corridor scores, tier promotions, stop selection, or optimizer decisions.
- Claims that a cached source proves throughput, contact, condition, publication,
  or game readiness.
- Silent substitution of a different source when an authoritative ROUTE source is
  unavailable.
- Domain-specific highway assumptions in FLETCH core schemas.

## Cacheline Model

Each ROUTE source family should have a stable cacheline id:

| Cacheline id pattern | Partition key | Activation rule |
|---|---|---|
| `route.manifest.<name>` | source name | Activate after HTTP success and complete write |
| `route.hpms.national` | year | Activate after non-empty parsed HPMS rows |
| `route.hpms.state.<state>` | state plus functional-system scope | Merge fetched scope, preserve non-requested state rows |
| `route.acs.county.population` | year | Activate after Census JSON parse and CSV write |
| `route.acs.county.income` | year | Activate after Census JSON parse and CSV write |
| `route.fema.sfha.<scope>` | corridor/tile scope | Activate after query loop and CSV flush |
| `route.t1.events.<provider>` | provider plus capture window | Activate latest snapshot only after source envelope validation |
| `route.proof.<campaign>.<artifact>` | campaign plus artifact id | Activate after ROUTE proof-intake row accepts the artifact |

Activation means "available for ROUTE to consume." It does not mean "claim
validated."

## Registry Edges

ROUTE should publish source-to-derived edges so downstream gates can explain
missing or stale evidence:

| Source cacheline | Derived ROUTE surface |
|---|---|
| `route.hpms.*` | pavement, throughput, and traffic reliability acquisition rows |
| `route.acs.*` | population reach and equity/rural-access evidence rows |
| `route.fema.*` | climate and flood exposure rows |
| `route.t1.events.*` | T1 failure evidence windows and incident scenario fixtures |
| `route.proof.*` | terminal contact, stitched-member, and publication proof dockets |

The edge graph is advisory for orchestration and audit. ROUTE gates remain the
source of truth for promotion decisions.

## Acquisition Modes

ROUTE should support four FLETCH-backed acquisition modes:

| Mode | ROUTE use | Required behavior |
|---|---|---|
| On-demand | A command needs one missing source | Fetch only the requested cacheline and its declared prerequisites |
| Bootstrap | A new checkout needs baseline sources | Fetch a named baseline bundle without mutating derived artifacts |
| Group | A wave needs one source family | Fetch all cachelines in the group, preserving existing unrelated cache |
| Bundle | Publication/release needs a frozen source set | Verify exact cacheline versions and emit a publisher bundle report |

## Implementation Phases

### Phase 1 - Spec and Registry Bridge

- **Implemented.** Add a ROUTE-owned FLETCH registry file that maps existing source families to
  cacheline ids, local paths, and dependencies.
- **Implemented.** Add a non-mutating command that reports FLETCH adapter handoff readiness for
  ROUTE source families.
- **Implemented.** Keep existing fetch command semantics unchanged while this bridge proves coverage.

### Phase 2 - Manifest Fetch Delegation

- **Implemented.** `route fetch` delegates manifest-backed HTTP downloads through
  FLETCH and then writes the legacy ROUTE cache target atomically.
- **Implemented.** Preserve `--force` semantics and the existing temp-then-replace writer.
- **Implemented.** Record FLETCH validation output beside ROUTE's existing source-fetch policy
  rows.

### Phase 3 - Partitioned Source Families

- **Implemented as adapter handoff.** Move HPMS state-scope, ACS, FEMA, and live-event snapshot source descriptions
  into FLETCH cacheline groups.
- **Implemented.** Keep ROUTE parser and gate logic in `route-data`.
- **Implemented.** Require scoped-merge and preserve-last-good behavior for every partitioned
  cacheline.

### Phase 4 - Proof and Publisher Surfaces

- Generate local URL maps and proof document manifests for ROUTE proof artifacts.
- Attach FLETCH publisher bundle reports to release and map-publication evidence
  packages.
- Treat publisher bundles as derived audit surfaces, not source truth.

## Acceptance Gates

The first implementation slice is accepted only when:

1. **Implemented.** Every row in `data/source-fetch-policy.csv` has a corresponding FLETCH
   cacheline id or a documented non-FLETCH reason.
2. **Implemented.** A FLETCH registry validation report can be generated without mutating cache
   files.
3. **Implemented.** A ROUTE adapter handoff report names source families, cache targets, and
   missing prerequisites.
4. **Implemented.** Existing ROUTE fetch tests still pass.
5. **Implemented.** No ROUTE optimizer, map, game, or publication gate treats "downloaded" as
   "claim validated."

## Implemented Artifacts

| Artifact | Role |
|---|---|
| `data/fletch-registry.json` | ROUTE-owned FLETCH registry for all source-fetch policy families |
| `route fletch-sources --gate` | Non-mutating registry/source-policy handoff gate |
| `data/fletch-source-handoff.csv` | Generated adapter handoff/readiness ledger |
| `route fetch` | Manifest HTTP acquisition now runs through FLETCH before atomically updating legacy ROUTE cache paths |

## Non-Goals

- Replacing ROUTE's evidence ledgers with FLETCH.
- Moving highway-specific source policy into FLETCH core.
- Auto-fetching paid, credentialed, or manually reviewed sources.
- Publishing cached raw data that ROUTE policy excludes from publication.
- Changing current cache paths before a compatibility bridge exists.

