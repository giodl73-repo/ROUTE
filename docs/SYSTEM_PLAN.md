# ROUTE System Plan

ROUTE is a research system, a Rust analysis toolkit, and a structured argument process for Interstate 2.0. This document is the living bridge between the original specs, the research module, the `.roles` review system, and the implementation plan.

The organizing theme is **Mileposts**. Each phase is a visible marker on the road from raw public data to defensible national investment proposals.

---

## The Thesis

The United States already chose highways. ROUTE asks which corridors matter most, where the existing network fails, what those failures cost, and which Interstate 2.0 upgrades should happen first.

The project deliberately separates four kinds of work:

| Layer | Purpose | Main artifacts |
|---|---|---|
| Measurement | Turn public data into repeatable corridor scores | `route` CLI, `config/scoring.toml`, `data/*.csv` |
| Diagnosis | Find missing links, bottlenecks, resilience holes, and access gaps | `research` tracks A-D, maps, gap reports |
| Judgment | Expose value conflicts instead of hiding them | `.roles/parliament`, `.roles/stakeholders`, reviews |
| Design | Convert evidence into sequenced Interstate 2.0 investments | `specs`, `research` tracks E-F, design proposals |

ROUTE is not an engineering drawing set, a construction promise, or a pure advocacy document. It is a quantified argument machine: score, compare, stress, argue, revise.

---

## Milepost Theme

| Milepost | Name | Question | Exit gate |
|---|---|---|---|
| 0 | Ground Survey | What do we already have? | Repo, specs, roles, data inventory, and CLI scaffold exist |
| 1 | Instrument | Can the scoring system measure consistently? | 16-dimension rubric tested, documented, and reproducible |
| 2 | Atlas | What does the national network look like under the instrument? | Existing corridors scored; tier map and calibration ledger generated |
| 3 | Fault Lines | Where does the system fail? | Missing-link, bottleneck, resilience, port, and coverage gaps identified |
| 4 | Pressure Test | Which standards still deliver under adversity? | Standards proof ledger, T1/T1 diamond stress tests, SLA/throughput scenarios, and bounded simulation outputs exist |
| 5 | The Forum | Which claims survive adversarial review? | Parliament, stakeholder, editorial, and panel-review records attached |
| 6 | Blueprint | What should Interstate 2.0 build first? | Feature packages, cost ranges, NPV cases, and phase sequencing written |
| 7 | Program | How does this become a repeatable public artifact? | Reproducible builds, CI, release docs, maps, papers, and public corpus |
| 8 | Evidence Campaign | Can a visible release hold survive source acquisition and review? | Target hold, source attempt, normalized evidence result, propagated claim update, and review decision exist |
| 9 | Evidence Operations | Can the evidence campaign become repeatable enough to support or reject promotion? | Repeat-window or archive path, freshness metadata, snapshot-history guard, review decision, and closeout exist |

The mileposts are sequential in logic but iterative in practice. A failed Pressure Test can send a corridor back to Instrument; a Parliament collision can create a new dimension; a panel review can rewrite a paper's quantification contract.

---

## What Got Us Here

The project has already moved through a messy but productive bootstrap:

1. The initial design specs established ROUTE as a TIGRIS-like corpus system for interstate corridors.
2. The Rust workspace split the system into data, network, scoring, mapping, reporting, simulation, and CLI crates.
3. The role system was created under `.roles`: parliament voices, stakeholder lenses, editorial gates, and transportation-domain panel reviewers.
4. The research module expanded from a handful of papers into Tracks A-F: scoring, gaps, freight, resilience, Interstate 2.0 design, and transit/relay.
5. The rubric evolved from a 12-dimension candidate pool to a 16-dimension v1.4 scorer with trade, safety, military/strategic, and agricultural export dimensions.
6. The implementation accumulated real functionality: graph build, HPMS joins, scoring, coverage, flow, investment, mapping, relay/SLA simulation.
7. The last cleanup pass aligned active docs with the 16-dimension scorer, hardened float handling, removed warning noise, and added the first focused test coverage.

The important lesson: the concept is ahead of the implementation, but the implementation is no longer just scaffolding. The next phase should make claims, code, and papers line up tightly.

---

## System Architecture

The Rust implementation should keep the following ownership boundaries.

The core architectural invariant is now segment identity: route labels, tier
names, map ids, and zone assignments are presentation or classification fields,
not primary keys. Segment-bearing artifacts should join through
`national_segment_id`, `segment_bundle_id`, or `stitch_group_id` as defined in
`docs/route-architecture.md` and `docs/national-segment-identity-spec.md`.

| Crate | Owns | Should not own |
|---|---|---|
| `route-data` | Fetching, parsing, manifests, source-specific records | Scoring policy |
| `route-network` | Graph construction, joins, stable segment identity, geometry state scope, corridor attributes, coverage, flow, centrality, investment primitives | CLI presentation |
| `route-score` | Dimensions, scoring anchors, ledgers, calibration statistics over segments and bundles | Data fetching or graph mutation |
| `route-map` | Geographic and schematic rendering from selected segments, bundles, stops, and stitch groups | Corridor scoring or topology invention from route labels |
| `route-sim` | Traffic assignment, incidents, relay, SLA, OD simulation attached to segments or bundles | File-system orchestration |
| `route-report` | Corpus/report generation | Analysis algorithms |
| `route-cli` | Command parsing, orchestration, terminal output, artifact gates during migration | Business logic or identity policy that cannot be unit-tested elsewhere |

The desired direction is to keep moving logic out of `route-cli` and into library crates. The CLI should become a thin conductor.

---

## Truth Labels

Every command, score, paper, and design claim should carry one of these labels:

| Label | Meaning |
|---|---|
| Implemented | Works end-to-end in Rust and is covered by at least one test or reproducible command |
| Heuristic | Works, but uses a proxy, default, partial data source, or simplified algorithm |
| Stub | Exists as interface or placeholder but does not perform the claimed analysis |
| Planned | Described in specs or papers but not yet implemented |
| Deprecated | Kept for history but not part of the current system claim |

This is the key discipline for ROUTE now. The project can be ambitious, but it must not let a planned claim masquerade as an implemented result.

---

## Role System

The `.roles` directory is part of the system, not decoration.

### Parliament

The seven parliament voices review corridor and design proposals:

| Voice | Tension protected |
|---|---|
| Eisenhower | National defense and unity versus local/regional interest |
| Moses | Throughput and construction scale versus community impact |
| Foxx | Equity and access versus efficiency-only framing |
| Freight Economist | NPV and commodity flow versus social/equity cost |
| Traffic Engineer | Capacity and safety versus cost/right-of-way |
| Climate Engineer | Long-horizon risk versus near-term delivery |
| Rural Advocate | Agricultural/rural access versus metro-centric priorities |

Parliament is not a voting body. It is an argument generator. Its output is a record of collisions, objections, earned claims, refuted claims, and rubric amendments.

### Stakeholders

Stakeholder roles are lenses applied during scoring, gap analysis, and feature selection. They keep the project from reducing highway value to only vehicle throughput.

Stakeholders include long-haul truckers, regional shippers, rural farmers, rural residents, local officials, state DOT planners, intercity travelers, transit-dependent travelers, freight industry, and environmental communities.

### Editorial Gates

Editorial roles are quality gates before validation:

| Role | Gate |
|---|---|
| Citation Auditor | Every material number has a traceable source |
| Numeracy Checker | Units, magnitudes, arithmetic, and comparisons are sane |
| Scope Keeper | Artifact stays in its declared lane |

### Panel Reviewers

Panel reviewers are domain specialists used for research papers. They protect research credibility: transport geography, rural economics, policy, network science, traffic engineering, freight logistics, resilience, transit planning, and equity.

---

## Research Tracks

The current research module has six tracks:

| Track | Purpose | Feeds |
|---|---|---|
| A — Corpus & Scoring | Validate the measurement instrument and tier structure | All downstream tracks |
| B — Gap Analysis | Identify missing links, bottlenecks, resilience holes, T1/T1 intersections, port connectors | Design and investment |
| C — Freight & Throughput | Quantify OD reliability, max-flow, 48-hour freight, relay marketplace | Managed lane and relay design |
| D — Resilience | Price climate and incident exposure | Hardening and redundancy priorities |
| E — Interstate 2.0 Design | Synthesize managed lanes, hubs, hardening, investment sequencing | Public framework |
| F — Transit + Relay | Show how T1/T1 hubs support passenger transit and relay markets | Phase 0 and shared facilities |

Each paper needs a quantification contract: primary number, experiment, decision it informs, and falsifier.

---

## Forward Plan

### Milepost 1 — Instrument

Goal: make the 16-dimension scorer boringly reliable.

Deliverables:
- Unit tests for every dimension's missing-data behavior and anchor extremes.
- Calibration ledgers that emit all 16 dimensions consistently, including score confidence, weakest risk-driving corridor dimensions, and dimension-level total/review risk.
- A dimension registry table in docs generated or checked against code.
- Clear labeling for estimated, proxy, and unavailable dimensions.

Primary risks:
- Scores look precise where source data is partial.
- Dimension docs drift from `config/scoring.toml` and `route-score`.

### Milepost 2 — Atlas

Goal: make the existing network corpus reproducible.

Deliverables:
- Fresh `route score-all` output using the current rubric.
- Tier table and map that can be regenerated from commands.
- Corpus files that record command, rubric version, data version, and estimation flags.
- Basemap implementation or explicit downgrade of map claims.

Primary risks:
- Graph build joins fail silently or over-match in dense interchange areas.
- Maps persuade visually without enough data provenance.

### Milepost 3 — Fault Lines

Goal: separate true gaps from artifacts.

Deliverables:
- Missing-link report with county-centroid artifact sensitivity.
- Bottleneck report using HPMS/ATRI/FPM where available.
- Resilience-hole report with explicit hazard data limitations.
- Port-connector gap report that handles short but critical links.

Primary risks:
- Large western counties exaggerate access gaps.
- Bottleneck and gap concepts blur into one another.

### Milepost 4 — Pressure Test

Goal: stress the network under movement, incidents, investment, and service obligations.

This milepost converts tier standards from design intent into proof obligations. Every standard must name the outcome it protects, the mechanism by which it protects it, the adverse condition it is tested against, and the evidence level that currently supports it. Standards that cannot pass this pressure test remain useful design ideas, but they cannot be treated as proven Blueprint claims.

The first binding issue is T1/T1 interchange resilience. T1 corridors carry the national freight backbone; T1/T1 nodes are where two backbone corridors share the same physical failure point. A diamond zone or express freight flyover standard earns its place only if it demonstrates that a single interchange or connector failure does not collapse T1/T1 transfer capacity and that the T1 recovery target remains plausible.

Deliverables:
- A standards proof ledger that maps each standard to outcome, mechanism, stressor, acceptance gate, artifact, evidence level, and owner.
- T1/T1 diamond scenario outputs for the 50-mile zone: baseline k-connectivity, post-intervention k-connectivity, single-failure transfer retention, and 4-hour recovery throughput.
- Tests for max-flow, incident, relay, and SLA simulation primitives.
- Scenario outputs for NY-LA, Houston-Chicago, and at least one port corridor.
- Sensitivity tables for PTI, closure duration, relay hub spacing, and managed-lane assumptions.
- A shared SLA proof table that separates freight-lane PTI, GP PTI, relay buffer, incident buffer, throughput restoration, and confidence label.
- L0/L1/L2 coverage: primitive invariants, reproducible generated artifacts, and bounded representative scenarios.

Primary risks:
- Simulation assumptions become headline claims without uncertainty bounds.
- Relay marketplace claims outrun regulatory and operational evidence.
- T1/T1 diamond claims prove graph connectivity but not usable freight throughput.
- Standards accumulate because they sound desirable rather than because they improve a tested SLA, throughput, resilience, safety, access, or equity outcome.

### Milepost 5 — The Forum

Goal: make disagreement productive and traceable.

Deliverables:
- At least three parliament reviews on high-stakes corridor/design proposals.
- Stakeholder pass for each design proposal.
- Editorial gate before any `validated` status.
- Panel review/revision records attached to research papers.

Primary risks:
- Reviews become prose theater instead of changing scores, specs, or claims.
- Consensus is overvalued; useful conflict is the actual output.

### Milepost 6 — Blueprint

Goal: produce a defensible Interstate 2.0 sequence.

Deliverables:
- Feature package taxonomy: managed freight lanes, diamonds, relay hubs, hardening, EV, transit, rest areas, rural spurs.
- Cost and NPV ranges with source labels and uncertainty.
- Phase 0 relay plan, Phase 1 T1 managed-lane plan, Phase 2 resilience/intermodal plan.
- A public design spec that says what is implemented, heuristic, and planned.

Primary risks:
- National plan becomes too broad to defend.
- Cost ranges look cleaner than their underlying evidence.

### Milepost 7 — Program

Goal: make ROUTE repeatable outside this one working session.

Deliverables:
- CI gates for Rust tests, formatting, docs link checks, and generated artifact checks.
- Release checklist for data, corpus, maps, papers, and specs.
- Public README that tells users exactly what works today.
- Versioned handoffs and tracker updates after each major pass.

Primary risks:
- Generated data and prose drift out of sync.
- The repo becomes impressive but not reproducible.

### Milepost 8 — Evidence Campaign

Goal: take one visible release hold and run it through source acquisition, ingestion, validation, review, and claim update.

Deliverables:
- A selected target hold with a written rationale.
- Source-access plan, cache policy, and ingestion script or documented blocker.
- Normalized evidence ledger with source labels, confidence, and limitations.
- Updated claim status in the relevant pressure, Blueprint, release, and spec artifacts.
- Review record that decides whether the claim is promoted, remains held, or is downgraded.

Primary risks:
- Chasing too many holds at once.
- Treating source acquisition as proof before validation.
- Updating a headline claim without propagating the downgrade/promotion through every ledger that references it.

### Milepost 9 — Evidence Operations

Goal: turn the first evidence campaign into a repeatable source operation with freshness metadata, archive/repeated-window paths, and gates that prevent snapshot-only rows from supporting annual or recovery claims.

Deliverables:
- Repeated-window or archive-access path for the selected T1/T1 failure evidence target.
- Source-window metadata that distinguishes snapshot-only rows from historical evidence.
- INDOT/OHGO enrichment or blocker record for the I-80/I-90 shared-corridor target.
- Snapshot-history guard in a gate or review checklist.
- Updated pressure, Blueprint, release, and closeout artifacts after the review decision.

Primary risks:
- Confusing more snapshots with stable annual evidence.
- Building polling before naming the claim it can actually support.
- Letting a source blocker disappear from the release surface.

---

## Current Highest-Value Work

1. Add truth labels to CLI commands and README status tables.
2. Finish L0/L1 tests for all `route-score` dimensions.
3. Move high-value analysis helpers out of `route-cli` into library crates.
4. Decide whether to implement a real basemap now or amend the map spec.
5. Re-run score-all with the current 16-dimension rubric and refresh stale corpus claims.
6. Update `TRACKER.md` to use the Milepost plan as the live board.

---

## Definition of Done

A ROUTE claim is done when:

1. The command or artifact can be regenerated.
2. The data sources and rubric version are named.
3. Missing/proxy data is labeled.
4. At least one test protects the behavior or calculation.
5. A relevant role pass has either challenged it or explicitly declined to challenge it.
6. The public-facing doc says no more than the implementation can support.
