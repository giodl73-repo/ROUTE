# Optimizer Artifact Manifest

## Purpose

The optimizer manifest is the run certificate for ROUTE tier optimization.

It does not replace individual gates. It records which commands produced which
artifacts, whether each artifact passed, and which blockers are intentionally
carried forward.

The current manifest artifact is `data/tier-optimizer-runs.csv`, emitted by:

```text
route tier-optimize --all-tiers --gate
```

An existing manifest can be verified without regenerating it with:

```text
route optimizer-manifest --gate
```

## Manifest Row Contract

Each manifest row must include:

| Field | Meaning |
|---|---|
| `step` | Stable stage order for this bundle |
| `optimizer_stage` | Short stable stage id |
| `command` | Reproducible `route ... --gate` command that generates or verifies the artifact |
| `artifact` | Primary output artifact for the stage |
| `row_count` | Parsed CSV record count or equivalent count, verified against the artifact by `route optimizer-manifest --gate` |
| `gate_status` | `pass`, `held-known`, `review`, or `fail` |
| `blocker_count` | Number of unresolved blockers intentionally carried |
| `blocker_summary` | Human-readable blocker class or next artifact |
| `validation_status` | Manifest-level interpretation: `pass`, `held`, `review`, or `missing-or-empty` |

Every row should be understandable without opening the command source.

## Status Semantics

| Status | Meaning | Manifest allowed? |
|---|---|---|
| `pass` | Artifact exists, has rows, and its gate accepts the current stage | yes |
| `held-known` | Artifact exists and the blocker is intentionally recorded for a later stage | yes |
| `review` | Artifact exists but policy or source review remains unresolved | yes, only with blocker summary |
| `fail` | Gate failed unexpectedly | no for committed optimizer bundle |
| `missing-or-empty` | Artifact is absent or has zero rows | no unless the stage is explicitly out of scope |

A passing bundle may contain `held-known` rows. That is how ROUTE avoids hiding
real blockers while still allowing downstream repair artifacts to run.

## Required Bundle Rules

A tier optimizer bundle passes only when:

1. every `pass` row has a non-empty artifact;
2. every `held-known` row has `blocker_count > 0`;
3. every `held-known` row has a non-empty `blocker_summary`;
4. every held blocker points, directly or indirectly, to a repair or review
   artifact;
5. no row has `validation_status = missing-or-empty`;
6. no committed bundle contains `gate_status = fail`;
7. stage order is deterministic;
8. each recorded `row_count` matches the current artifact on disk;
9. every stage records a gateable `route ... --gate` command.

The manifest should fail loudly if a new stage is added without an artifact
contract.

## Current Stages

The current all-tier manifest includes:

- T1 SLA candidate-pair cut line;
- T1 stop selection;
- T1 topology repairs;
- T1 Beck alignment;
- T2 region workloads;
- T2 contact witnesses;
- T2 contact/blocker closure surfaces;
- T2 candidate columns;
- T2 bundle repair queue;
- T2 regionalizer;
- T2 service selection;
- T2 service diagnostic queue;
- T2 parallel service queue;
- T1/T2 segment candidates;
- T1/T2 pavement docket;
- T1/T2 pavement source-gap rollup;
- T1/T2 pavement debt budget;
- T1/T2 pavement acquisition plan;
- T1/T2 pavement acquisition docket;
- normalized optimizer constraint ledger and budget rollup once implemented;
- T2 bundle overlays;
- lower-tier pressure witnesses;
- T3/T4 pressure intake;
- T3 zone render board;
- T3 zone stop placement;
- national segment registry;
- national segment bundles;
- bundle architecture adoption;
- T2 bubble-up review;
- T1 feedback docket.

The current held-known rows are:

1. T2 region workloads: bridged T2 component pending contact repair.
2. T2 contact witnesses: unresolved contact, parent, relief, and terminal
   validation rows.

Those holds are allowed because downstream closure, demotion, pressure, and
feedback artifacts consume them.

## Relationship To Other Artifacts

The manifest proves the bundle shape. It does not prove domain truth by itself.

| Artifact class | Proof owner |
|---|---|
| Promise-pair selection | `data/t1-sla-candidate-pairs.csv` |
| Route/stop/service schema | `docs/route-stop-column-schema.md` |
| Segment identity and bundle joins | `docs/route-architecture.md`, `docs/national-segment-identity-spec.md`, `docs/tier-segment-stitching-spec.md`, `data/tier-segment-candidates.csv`, `data/national-segment-registry.csv`, `data/national-segment-bundles.csv`, and `data/bundle-architecture.csv` |
| Optimizer constraints, debt, penalties, and repair actions | `docs/optimizer-constraint-ledger-spec.md` and future `data/optimizer-constraint-ledger.csv` / `data/optimizer-constraint-budget.csv` |
| Pavement and ride-quality floors | `docs/tier-pavement-standards.md`, `data/tier-pavement-standards.csv`, `data/tier-pavement-docket.csv`, `data/tier-pavement-source-gaps.csv`, `data/tier-pavement-debt-budget.csv`, `data/tier-pavement-acquisition-plan.csv`, and `data/tier-pavement-acquisition-docket.csv` |
| Source fetch cache preservation | `docs/source-fetch-cache-policy.md` and `data/source-fetch-policy.csv` |
| Beck topology truth | `docs/beck-renderer-contract.md` plus map diagnostics |
| T2 service doctrine | `docs/t2-regional-treatment.md` |
| T3/T4 access doctrine | `docs/t3-t4-access-optimization.md` |
| Release/publication status | `data/release-manifest.csv` and release docs |

The manifest should link to the artifact that owns the proof rather than
duplicating all of its fields.

## Gate Evolution

Near-term improvements:

- add manifest rows for spec/review artifacts when they become gates;
- include command hash or config id once optimizer configs are externalized;
- include source artifacts for generated rows;
- include expected row-count ranges for stable fixtures;
- include `next_artifact` for held-known rows.

These are improvements, not prerequisites for the current manifest to be useful.

## Non-Goals

- The manifest is not a source manifest like `data/manifest.json`.
- The manifest is not a release manifest.
- The manifest is not a mathematical proof of optimality.
- The manifest does not make held rows pass; it makes them visible.
