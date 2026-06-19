---
name: International Network Inference Portability Report
slug: international-network-inference-portability-report
type: report
status: draft
rubric_version: v1.0
author: codex
created: 2026-06-18
updated: 2026-06-18
sources:
  - README.md
  - docs/reports/route-evidence-posture.md
  - docs/reports/interstate-2-0-doctrine-report.md
  - docs/reports/forty-eight-hour-freight-promise-report.md
  - docs/reports/t3-t4-access-evidence-appendix.md
  - docs/reports/source-operations-evidence-roadmap.md
  - docs/reports/optimizer-evidence-appendix.md
  - docs/reports/graph-scoring-measurement-appendix.md
  - docs/reports/maps-are-not-proof-report.md
  - docs/reports/release-publication-scope-appendix.md
  - docs/reviews/communications-crate-coverage-audit.md
  - docs/reviews/media-discovery-stress-test-001.md
  - docs/reviews/international-portability-pilot-map-run-001.md
  - docs/reviews/international-canada-source-adapter-preflight-001.md
  - data/international-portability-pilot-map-index.csv
  - data/international-canada-source-custody-preflight.csv
  - data/international-canada-adapter-coverage-preflight.csv
  - data/international-canada-adapter-field-map.csv
  - data/international-canada-source-adapter-readiness.csv
  - data/international-canada-source-adapter-gap-backlog.csv
  - docs/reviews/international-canada-source-adapter-readiness-001.md
  - data/international-canada-candidate-hierarchy.csv
  - docs/reviews/international-canada-candidate-hierarchy-map-001.md
  - maps/international/canada-candidate-hierarchy.svg
  - docs/reviews/international-canada-hierarchy-iteration-001.md
  - docs/how-to/international-hierarchy-iteration-playbook.md
  - maps/international/canada-candidate-hierarchy-v2.svg
  - docs/vtrace/COMMUNICATIONS_STRATEGY.md
  - docs/vtrace/VERIFICATION.md
---

# International Network Inference Portability Report

## Purpose

This report reframes ROUTE as a portable product idea: infer a service network
from existing roads, freight and passenger needs, service-level targets,
constraints, and evidence quality for any country or region.

It is not a claim that ROUTE has produced an official network for another
country or region. It does not create foreign agency review, international
validation, official-plan status, construction readiness, guaranteed service,
numeric ROI, eligibility, compliance, public readiness, approval, endorsement,
or external readiness.

## Product Hypothesis

ROUTE can become a country- or region-agnostic inference engine with a stable
kernel and replaceable jurisdiction adapters.

| Layer | Portable Question | U.S.-Specific Today | Portable Product Target |
|---|---|---|---|
| Service roles | Which roads should act as national spine, regional connector, feeder/access, and terminal/local access? | T1/T2/T3/T4 framed around Interstate 2.0. | Configurable role taxonomy for each country or region. |
| Service targets | What time, reliability, access, resilience, and freight promises should the network test? | 48h / 36h / 12h / 1h style planning targets. | Service-level target portfolio selected by market, geography, and policy needs. |
| Need surfaces | Which movements, communities, terminals, hazards, and economic nodes need visibility? | U.S. freight, rural, state DOT, port/border, and resilience lanes. | Need classes mapped from local freight, passenger, rural, industrial, port, border, climate, and emergency-management data. |
| Evidence posture | Which claims are supported, heuristic, source-needed, gated, or held? | Current VTRACE/evidence posture and role review. | Same evidence labels applied to every jurisdiction. |
| Inference output | What network should be inspected next? | U.S. maps, bundles, scores, fixtures, and reports. | Candidate service graph, role assignments, gaps, source needs, and refinement tasks. |

## Portable Kernel

The product should preserve these mechanics across countries:

| Kernel Mechanic | Why It Is Portable | Boundary |
|---|---|---|
| Role-based road hierarchy | Every region has roads that play different jobs even when official labels differ. | ROUTE role names are hypotheses until localized. |
| Service-level target portfolio | Time, reliability, access, resilience, and terminal promises make needs inspectable. | Targets are planning thresholds, not guaranteed SLAs. |
| Stop-first / node-first graph | Ports, border crossings, industrial zones, metros, rural production zones, and terminals can anchor service questions. | Node selection needs local source custody. |
| Bundle and stitch identity | Route labels alone are ambiguous; service corridors need stable identities. | Identity rules must adapt to local route numbering and geometry conventions. |
| Scoring with confidence labels | A score is useful only when source quality and missing data are explicit. | Scores are review indices, not funding or construction rankings. |
| Source-needed to source-backed workflow | Every jurisdiction needs a way to move from hypothesis to evidence. | Public sources and access permissions vary by region. |
| Map-with-holds publication posture | Structural maps help explain the network but do not prove readiness. | Maps remain non-proof surfaces unless separate evidence closes. |

## Jurisdiction Adapter Contract

Each country or region needs an adapter before ROUTE can infer a bounded
candidate network.

| Adapter Field | Required Input | Claim Boundary |
|---|---|---|
| Jurisdiction scope | Country, cross-border region, province/state group, corridor basin, or economic region. | Does not imply official jurisdiction approval. |
| Road graph | Authoritative or declared road geometry and classification source. | Classification is source posture, not service role proof. |
| Node catalog | Ports, border crossings, logistics hubs, industrial zones, major metros, rural production zones, passenger terminals, emergency nodes. | Nodes require source owner, date/year, and access note. |
| Demand / need signals | Freight flow, passenger flow, production, rural access, hazard, resilience, or policy need sources. | Need signals do not prove project priority. |
| Service target set | Time, reliability, access, resilience, terminal, or emergency-response planning targets. | Targets are planning assumptions until adopted elsewhere. |
| Constraint ledger | Geography, weather, border inspection, tolling, capacity, maintenance, safety, environmental, legal, funding, and governance constraints. | Constraint rows hold construction and compliance claims. |
| Evidence labels | Story-ready, implemented, heuristic, source-needed, gated, held. | Labels travel with outputs. |
| Review roles | Local transport planner, freight/logistics reviewer, rural/access reviewer, environmental/community reviewer, numeracy reviewer, map reviewer, technical reviewer. | Simulated roles do not replace external validation. |

## International Pilot Ladder

| Pilot | Why It Helps | Minimum Evidence Needed | Do Not Claim |
|---|---|---|---|
| U.S.-Canada / U.S.-Mexico border gateway pilot | ROUTE already has border and USMCA-style gateway concepts, so it tests cross-border adapter seams first. | Selected gateway nodes, road graph, inspection/terminal constraints, freight need source, held-claim map. | Binational approval, border performance proof, guaranteed travel time, customs capacity, or agency validation. |
| Canada national-service pilot | Similar federal/provincial split and large-distance freight/access questions make it a clean first non-U.S. national test. | National/provincial road graph, freight corridors, ports/borders, northern/rural access needs, weather/resilience constraints. | Transport Canada/provincial approval, official network designation, SLA, ROI, eligibility, or construction readiness. |
| EU corridor-region pilot | Tests multi-country governance, borderless freight, ports, and regional cohesion without assuming one national planner. | Selected corridor region, TEN-style corridor/source references if used, ports, terminals, freight/passenger flows, member-state constraints. | EU adoption, member-state agreement, funding priority, compliance, or cross-border delivery commitment. |
| India national/logistics pilot | Tests dense population, freight corridors, ports, rural access, and state/national governance. | National/state road graph, logistics nodes, production zones, rural access needs, monsoon/resilience constraints, freight source posture. | National highway approval, corridor build priority, guaranteed travel time, ROI, or state/federal endorsement. |
| Japan resilience/access pilot | Tests island geography, ports, seismic/weather resilience, dense metros, rural access, and high-reliability expectations. | Road graph, port/terminal nodes, hazard/resilience sources, rural/island access needs, passenger/freight target set. | Disaster-readiness proof, ministry approval, capacity proof, construction program, or public-readiness claim. |
| China national/logistics pilot | Tests scale, high-speed logistics, industrial nodes, ports, inland access, and provincial coordination. | Declared road graph, logistics/industrial node sources, ports/borders, regional development needs, constraints, evidence-access notes. | Official approval, policy alignment, construction priority, performance proof, ROI, or compliance. |

## Inference Workflow

| Step | Product Action | Output | Evidence Gate |
|---|---|---|---|
| 1. Scope jurisdiction | Select country/region and audience lane. | Jurisdiction packet. | Scope Keeper confirms no official-plan claim. |
| 2. Load road graph | Normalize routes, links, nodes, and classifications. | Source-labeled graph. | Citation Auditor records owner/date/access note. |
| 3. Load need surfaces | Add freight, passenger, rural, industrial, port, border, hazard, and resilience signals. | Need layers with labels. | Missing sources become source-needed rows. |
| 4. Select service targets | Define planning targets by role and geography. | Service target portfolio. | Numeracy Checker marks assumptions and units. |
| 5. Infer candidate roles | Assign tentative spine, connector, feeder/access, and terminal/local roles. | Candidate service graph. | Scores are review indices only. |
| 6. Generate gaps and holds | Identify where targets are unsupported or blocked by source gaps. | Gap ledger and held-claim map captions. | Held rows block promotion. |
| 7. Run role review | Apply local transport, freight, rural/access, environmental/community, numeracy, map, and technical roles. | Review record. | Simulated review remains internal. |
| 8. Create refinement packet | Convert objections into source packs, constraints, and artifact changes. | Requirement-to-refinement backlog. | No external validation unless named venue/source closeout exists. |

## Product Test Criteria

| Test | Pass Condition | Fail Condition |
|---|---|---|
| Adapter completeness | Road graph, node catalog, need surfaces, target set, constraints, evidence labels, and roles are declared. | Product infers a network without naming required sources and assumptions. |
| Claim discipline | Every output separates candidate role, evidence label, and blocked claims. | Map, score, or SLA target becomes proof language. |
| Local vocabulary | Role names and need surfaces can be localized without losing the kernel. | U.S. Interstate/FHWA/state DOT terms leak into foreign outputs as if universal. |
| Source portability | Missing local sources become source-needed rows, not silent defaults. | U.S. sources or assumptions fill foreign gaps without disclosure. |
| Review portability | Local role lanes can challenge freight, rural/access, environmental/community, finance, map, and technical claims. | The product treats code output as validation. |
| Refinement loop | Objections become source packs, constraints, or artifact changes. | Objections are treated as narrative feedback only. |

## Pilot Map Run 001

`docs/reviews/international-portability-pilot-map-run-001.md` implements the
first compact replicability fixture. It uses the same node/link input shape and
role inference rule to generate five held-claim schematic maps:

| Pilot | Map | Current Posture |
|---|---|---|
| Canada service network | `maps/international/canada-service-network.svg` | replicability fixture generated; validation held |
| EU Rhine-Alpine region | `maps/international/eu-rhine-alpine-region.svg` | replicability fixture generated; validation held |
| India logistics spine | `maps/international/india-logistics-spine.svg` | replicability fixture generated; validation held |
| Japan Pacific Belt | `maps/international/japan-pacific-belt.svg` | replicability fixture generated; validation held |
| China logistics spine | `maps/international/china-logistics-spine.svg` | replicability fixture generated; validation held |

This is stronger than a concept memo because it produces comparable map
artifacts from comparable adapter rows. It still does not prove any official
network, guaranteed SLA, construction priority, ROI, compliance, endorsement,
or country/region validation.

## Canada Source Adapter Preflight 001

`docs/reviews/international-canada-source-adapter-preflight-001.md` promotes
the Canada pilot one step beyond the compact map fixture. It adds source
custody candidates, adapter coverage rows, and a source-custody overlay map:

| Artifact | Path | Current Posture |
|---|---|---|
| Source custody rows | `data/international-canada-source-custody-preflight.csv` | candidate sources and source-needed rows declared |
| Adapter coverage matrix | `data/international-canada-adapter-coverage-preflight.csv` | preflight-ready, source-candidate, source-needed, and held fields separated |
| Source-custody overlay map | `maps/international/canada-source-custody-preflight.svg` | review surface; validation held |

This is the first proof-of-process step after map replication: local source
candidates and source gaps are now visible adapter inputs. It still does not
bind source fields into graph inference, validate the Canada service network,
prove service targets, or create any official-plan, construction, ROI,
compliance, endorsement, public-readiness, or external-readiness claim.

## Canada Adapter Readiness 001

`docs/reviews/international-canada-source-adapter-readiness-001.md` adds the
machine-readable promotion step after preflight. The command
`python tools\build_canada_adapter_readiness.py` reads source custody,
coverage, and field-map rows, then writes:

| Artifact | Path | Current Posture |
|---|---|---|
| Field map | `data/international-canada-adapter-field-map.csv` | target adapter columns declared |
| Readiness ledger | `data/international-canada-source-adapter-readiness.csv` | parse-ready, carry-forward, source-needed, and held decisions generated |
| Gap backlog | `data/international-canada-source-adapter-gap-backlog.csv` | next evidence actions generated |

The result is a promotion ledger, not a parsed source adapter. Road graph and
need-surface rows are ready for a future parser but are not promoted. Node,
constraint, and service-target rows remain source-needed or held. No Canadian
network, official review, SLA, construction, ROI, compliance, endorsement,
public-readiness, or external-readiness claim is created.

## Canada Candidate Hierarchy Map 001

`docs/reviews/international-canada-candidate-hierarchy-map-001.md` produces the
first visible Canada T1/T2/T3 candidate hierarchy result:

| Tier | Candidate Role | Count |
|---|---|---|
| T1 | national spine | 4 |
| T2 | regional connector | 3 |
| T3 | access feeder | 5 |

The result map is `maps/international/canada-candidate-hierarchy.svg`. It is a
candidate result from held rows, not an official Canadian network or parsed
source adapter. It does not create route-designation, agency/provincial/port
review, guaranteed-SLA, construction, ROI, eligibility, compliance,
endorsement, public-readiness, or external-readiness claims.

## Canada Hierarchy Iteration 001

`docs/reviews/international-canada-hierarchy-iteration-001.md` scores the first
Canada hierarchy and produces a v2 map:

`maps/international/canada-candidate-hierarchy-v2.svg`

The iteration changes are concrete:

- Toronto-Windsor is moved from T3 to T2.
- Winnipeg-Thompson is moved from T2 to T3 pending stronger evidence.
- Regina, Saskatoon, Ottawa, and Quebec City are added.
- Fort McMurray access is rerouted through Edmonton.

`docs/how-to/international-hierarchy-iteration-playbook.md` captures the
repeatable loop: score, repair rows, rerender, record holds. This process is
portable to the EU, India, Japan, China, or other regional pilots, but it still
does not produce official networks, guaranteed SLAs, construction priorities,
ROI claims, compliance findings, endorsements, public-readiness, or
external-readiness.

## Product Language

Use:

- "ROUTE can test a candidate service network for a country or region."
- "Service-level targets are planning assumptions until local evidence and
  authority close."
- "The product infers role candidates, gaps, source needs, and refinement tasks."
- "Maps and scores are review surfaces, not proof."
- "The same kernel can be adapted to Canada, EU regions, India, Japan, China, or
  other jurisdictions when local source custody exists."

Avoid:

- "ROUTE proves the national road network for every country."
- "ROUTE guarantees SLAs."
- "ROUTE replaces national transport ministries, road agencies, engineering
  review, environmental review, funding processes, or public engagement."
- "The EU, China, India, Canada, Japan, or any agency has validated this."
- "International maps are ready for publication or policy use."

## Next Work

1. Promote the Canada readiness ledger into a parsed source adapter by
   downloading or caching selected source artifacts, mapping fields, and
   replacing fixture node/link rows with source-bound rows.
2. Run a bounded border-gateway source-backed pilot next, because ROUTE already
   has port/border concepts and can test adapter seams without claiming a
   foreign national network.
3. Keep each non-U.S. national or regional pilot internal until
   source custody, local roles, prohibited-claim scan, L0, and any required L1/L2
   evidence close.

## Validation Closeout

| Check | Command / Inspection | Result | Evidence |
|---|---|---|---|
| Portability scope inspection | compare evidence posture, doctrine, freight promise, source operations, optimizer, graph/scoring, map, release, and media discovery surfaces | pass | report wired into strategy, verification, evidence posture, and media source index |
| Prohibited-claim scan | scan report and linked edited surfaces for promoted prohibited claims | pass | hits are guardrail, held, do-not-infer, or non-approved contexts |
| L0 | `npm run check:l0` | pass | workspace lib/bin tests passed |

## Gate

Decision: **international_portability_hypothesis_ready; validation_held**

Rationale: ROUTE can be framed as a portable network-inference product if the
kernel is separated from U.S.-specific adapters and every international output
keeps source custody, local role review, held-claim captions, and validation
gates. No country or region is validated by this report.
