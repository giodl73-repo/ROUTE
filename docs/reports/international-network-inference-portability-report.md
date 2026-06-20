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
  - docs/reviews/international-eu-rhine-alpine-hierarchy-iteration-001.md
  - data/international-eu-rhine-alpine-candidate-hierarchy-v2.csv
  - maps/international/eu-rhine-alpine-candidate-hierarchy-v2.svg
  - docs/reviews/international-india-hierarchy-iteration-001.md
  - data/international-india-candidate-hierarchy-v2.csv
  - maps/international/india-candidate-hierarchy-v2.svg
  - docs/reviews/international-japan-hierarchy-iteration-001.md
  - data/international-japan-candidate-hierarchy-v2.csv
  - maps/international/japan-candidate-hierarchy-v2.svg
  - docs/reviews/international-china-hierarchy-iteration-001.md
  - data/international-china-candidate-hierarchy-v2.csv
  - maps/international/china-candidate-hierarchy-v2.svg
  - docs/reviews/international-hierarchy-replication-closeout-001.md
  - data/international-hierarchy-replication-summary-001.csv
  - data/international-cross-region-scoring-rubric-001.csv
  - docs/templates/source-packs/international-adapter-source-pack-template.md
  - docs/reviews/international-canada-adapter-source-pack-001.md
  - data/international-canada-adapter-source-pack-001.csv
  - docs/reviews/international-canada-parser-preflight-001.md
  - data/international-canada-parser-preflight-001.csv
  - docs/reviews/international-canada-parser-output-contract-001.md
  - data/international-canada-parser-output-contract-001.csv
  - docs/reviews/international-canada-parser-dry-run-fixture-001.md
  - data/canada_source_link_candidates.csv
  - data/canada_source_need_candidates.csv
  - data/canada_source_node_candidates.csv
  - data/canada_service_target_candidates.csv
  - data/canada_adapter_evidence_labels.csv
  - data/canada_adapter_review_backlog.csv
  - docs/reviews/international-canada-adapter-promotion-preflight-001.md
  - data/international-canada-adapter-promotion-preflight-001.csv
  - docs/reviews/international-canada-node-source-selection-001.md
  - data/international-canada-node-source-selection-001.csv
  - docs/reviews/international-canada-node-source-probe-001.md
  - data/international-canada-node-source-probe-001.csv
  - docs/reviews/international-canada-node-fixture-contract-001.md
  - docs/reviews/international-canada-node-replacement-role-review-001.md
  - docs/reviews/international-canada-node-fixture-replacement-closeout-001.md
  - docs/reviews/international-canada-target-posture-001.md
  - docs/reviews/international-canada-internal-adapter-proof-001.md
  - data/international-canada-internal-adapter-proof-001.csv
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

## EU Rhine-Alpine Hierarchy Iteration 001

`docs/reviews/international-eu-rhine-alpine-hierarchy-iteration-001.md` applies
the same score-repair-render loop to a second region:

`maps/international/eu-rhine-alpine-candidate-hierarchy-v2.svg`

The iteration keeps the port-cluster and Rhine-Alpine spine visible while adding
alternate access candidates:

- Basel-Lyon is added as a T2 alternate regional connector.
- Cologne-Strasbourg is added as a T3 cross-border access feeder.
- Alpine T1 candidates remain visible with constraint risk held.
- Governance, TEN-T/source binding, member-state approval, and SLA claims remain
  held.

This confirms the process is portable beyond Canada. It does not validate an EU
corridor, member-state agreement, official designation, guaranteed SLA,
construction priority, ROI, eligibility, compliance, endorsement,
public-readiness, or external-readiness claim.

## India Hierarchy Iteration 001

`docs/reviews/international-india-hierarchy-iteration-001.md` applies the same
score-repair-render loop to a third region:

`maps/international/india-candidate-hierarchy-v2.svg`

The iteration converts fixture critique into concrete row changes:

- Ahmedabad-Mumbai is repaired from fixture T4 to T1.
- Mumbai-Pune is up-tiered to T1.
- Lucknow and Bengaluru are added as regional connectors.
- Port access is separated into T3 feeders.
- Hyderabad is added as an inland access candidate.

This strengthens the replicability story by showing the process can find and
repair hierarchy errors in another geography. It does not validate an Indian
corridor, national/state approval, official designation, guaranteed SLA,
construction priority, ROI, eligibility, compliance, endorsement,
public-readiness, external-readiness, or external validation claim.

## Japan Hierarchy Iteration 001

`docs/reviews/international-japan-hierarchy-iteration-001.md` applies the same
score-repair-render loop to a fourth region:

`maps/international/japan-candidate-hierarchy-v2.svg`

The iteration tests whether the portable hierarchy process can handle dense
megaregion continuity and resilience branches without turning them into proof:

- Osaka-Kobe is repaired from fixture T4 to T1.
- Kobe-Hiroshima is up-tiered to T1 to preserve Pacific Belt continuity.
- Tokyo-Niigata is added as an alternate-coast resilience connector.
- Kobe terminal access is separated into a T3 feeder.
- MLIT/source-bound, hazard, terminal, disaster-readiness, and SLA claims remain
  held.

This extends the method beyond continental freight corridors into an
island/geohazard context. It does not validate a Japanese corridor, ministry
approval, disaster-readiness, official designation, guaranteed SLA,
construction priority, ROI, eligibility, compliance, endorsement,
public-readiness, external-readiness, or external validation claim.

## China Hierarchy Iteration 001

`docs/reviews/international-china-hierarchy-iteration-001.md` applies the same
score-repair-render loop to a fifth region:

`maps/international/china-candidate-hierarchy-v2.svg`

The iteration tests whether the portable hierarchy process can handle scale,
coastal manufacturing, Yangtze inland movement, and port/export gateways
without converting them into official or policy claims:

- Beijing-Tianjin is repaired from fixture T4 to T1.
- Guangzhou-Shenzhen is repaired from fixture T4 to T2.
- Zhengzhou is added as an inland distribution connector.
- Tianjin Port and Yantian Port are separated as T3 terminal feeders.
- Policy alignment, source-bound road graph, logistics node, terminal,
  performance, and SLA claims remain held.

This completes the first five-fixture replication ladder from Canada through
EU Rhine-Alpine, India, Japan, and China. It does not validate a Chinese
corridor, policy alignment, official designation, guaranteed SLA, construction
priority, ROI, eligibility, compliance, endorsement, public-readiness,
external-readiness, or external validation claim.

## Replication Closeout 001

`docs/reviews/international-hierarchy-replication-closeout-001.md` packages the
five hierarchy iterations into a single review surface. The companion summary
ledger is `data/international-hierarchy-replication-summary-001.csv`.

The closeout shows that the reusable loop found a recurring hierarchy defect:
port, terminal, border, or local-access proximity can downgrade trunk continuity
unless the product separates trunk roles from access feeders. Canada, EU
Rhine-Alpine, India, Japan, and China each repair that pressure in different
local terms while preserving source-bound, official-network, SLA, construction,
ROI, compliance, endorsement, public-readiness, external-readiness, and
validation holds.

## Source-Bound Next Step 001

The replication ladder now has two source-bound follow-through artifacts:

| Artifact | Path | Use | Boundary |
|---|---|---|---|
| Cross-region scoring rubric | `data/international-cross-region-scoring-rubric-001.csv` | Compare coverage, tier fit, freight value, resilience, regional access, source readiness, SLA plausibility, and claim discipline across regions. | Scores are review indices, not rankings, funding priorities, official readiness, or ROI. |
| International adapter source-pack template | `docs/templates/source-packs/international-adapter-source-pack-template.md` | Collect road graph, node, need, terminal, hazard, governance, and service-target source custody before adapter promotion. | Source rows do not create official network, approval, policy alignment, SLA, construction, ROI, compliance, endorsement, public-readiness, or external validation claims. |

## Canada Adapter Source Pack 001

`docs/reviews/international-canada-adapter-source-pack-001.md` fills the
international adapter source-pack template for Canada using the existing Canada
source-custody preflight and readiness ledgers. The machine-readable source
family declaration is `data/international-canada-adapter-source-pack-001.csv`.

The result is a source-pack declaration, not a parsed adapter. Road graph and
need vocabulary rows are parse-ready candidates but not promoted. Port/terminal,
constraint, and service-target rows remain source-needed or held. No Canadian
network, route designation, Transport Canada/provincial/port approval,
guaranteed SLA, construction, ROI, eligibility, compliance, endorsement,
public-readiness, external-readiness, or external validation claim is created.

## Canada Parser Preflight 001

`docs/reviews/international-canada-parser-preflight-001.md` converts the Canada
source-pack declaration into an auditable parser job:

`data/international-canada-parser-preflight-001.csv`

The preflight names candidate output tables for road links, need/context rows,
node gaps, service-target holds, evidence labels, and role-review backlog. It
does not implement the parser, download sources, replace fixture links, or
promote source-bound rows. Parsed-adapter, official-network, route designation,
Transport Canada/provincial/port approval, guaranteed SLA, construction, ROI,
eligibility, compliance, endorsement, public-readiness, external-readiness, and
external validation claims remain held.

## Canada Parser Output Contract 001

`docs/reviews/international-canada-parser-output-contract-001.md` defines the
mechanical output contract for a future Canada parser dry run:

`data/international-canada-parser-output-contract-001.csv`

The contract names required columns, allowed evidence labels, acceptance rules,
and blocked claim values for candidate links, candidate needs, node gaps,
service-target holds, evidence-label carry-forward, and role-review backlog. It
does not implement the parser, download/cache sources, replace fixture rows, or
promote parsed-adapter, official-network, SLA, construction, ROI, compliance,
endorsement, public-readiness, external-readiness, or validation claims.

## Canada Parser Dry Run Fixture 001

`docs/reviews/international-canada-parser-dry-run-fixture-001.md` emits the
contract-shaped Canada parser dry-run tables:

- `data/canada_source_link_candidates.csv`
- `data/canada_source_need_candidates.csv`
- `data/canada_source_node_candidates.csv`
- `data/canada_service_target_candidates.csv`
- `data/canada_adapter_evidence_labels.csv`
- `data/canada_adapter_review_backlog.csv`

These rows demonstrate the parser output contract can be represented with
candidate, gap, held, evidence-label, and review-backlog tables. They are still
fixture rows, not parser output from downloaded sources, not a parsed adapter,
and not a replacement for the Canada hierarchy fixture.

`tools/build_canada_parser_dry_run.py` now regenerates the dry-run tables from
the Canada source pack, parser preflight ledger, and output contract. This makes
the Canada dry-run surface reproducible, but it still does not fetch source
payloads, parse Canadian network data, promote a source-bound adapter, or prove
official-network, SLA, construction, ROI, compliance, endorsement,
public-readiness, external-readiness, or validation claims.

`tools/check_canada_parser_dry_run.py` gates that dry-run surface by checking
contract columns, evidence labels, source-ID limits, minimum gap rows,
evidence-label coverage, and pending review backlog. A pass means the Canada
dry-run contract is internally runnable; it is not a source-payload validation
or service-performance claim.

`data/international-canada-source-payload-access-001.csv` and
`tools/check_canada_source_payload_access.py` add the next source-payload access
gate. The gate confirms which Canada source-pack rows are URL cache candidates,
source-needed, or held before any payload fetch, field inventory, or parser
extraction claim.

`data/international-canada-source-payload-resolution-001.csv` resolves the
Geo.ca road-graph landing page to ESRI REST service/layer metadata and FGDB
download candidates. `data/international-canada-source-payload-probe-001.csv`
records a bounded live probe: the resolved road-graph metadata endpoint and the
other URL candidates return HTTP 200 samples, and all rows remain not accepted
as evidence. The next Canada parser step is source field inventory, not adapter
promotion.

`data/international-canada-source-field-inventory-001.csv` now records that
field inventory: 49 road-graph fields from the resolved ESRI REST layer plus
held rows for road-context, base-road, trade-need, port/terminal, and service
target sources. The inventory identifies candidate route, class, and geometry
fields but does not query features, replace fixtures, or validate source rows.

`data/international-canada-road-graph-feature-sample-001.csv` then performs a
bounded no-geometry source query against the resolved road-graph layer. It
records five feature-attribute rows for parser intake while keeping feature
validation, fixture replacement, official-network, SLA, construction, ROI,
compliance, endorsement, public-readiness, and external-readiness claims held.

`data/international-canada-parser-mapping-ledger-001.csv` maps the resolved
road-graph fields into the dry-run output contract: class fields map cleanly,
route-number/name fields are present but sampled as `None` in the bounded first
rows, and geometry remains a reference candidate only. The next Canada parser
step is broader filtered extraction, not fixture replacement.

`data/international-canada-road-graph-filtered-route-sample-001.csv` performs
that first filtered extraction step with a bounded no-geometry object-ID window.
It records usable route numbers and a route name for parser intake, while source
validation, geometry acceptance, fixture replacement, and official-network
claims remain held.

`data/international-canada-parser-extraction-candidates-001.csv` converts the
filtered sample into separate link-candidate extraction rows. They follow the
dry-run link-candidate shape but remain outside the fixture table until role
review, geometry policy, source-row validation, and replacement closeout pass.

`data/international-canada-fixture-replacement-role-review-001.csv` runs that
role review against the current dry-run link fixture. It confirms the extraction
candidates are stronger than placeholders for internal parser review, but it
keeps fixture replacement, map/topology use, operational claims, authority
claims, parsed-adapter promotion, and external use held.

`data/international-canada-source-row-validation-001.csv` validates each
extraction candidate against its bounded filtered source row. Candidate
source-row matching now passes for the five-row extraction table, while
geometry acceptance, fixture replacement, parsed-adapter promotion, operational
claims, authority claims, and external validation remain held.

`data/international-canada-geometry-policy-001.csv` makes the geometry boundary
explicit: the current Canada candidates remain no-geometry rows. Geometry,
topology, map overlay, fixture replacement, and adapter promotion require a
separate geometry intake fixture and role review.

`data/international-canada-fixture-replacement-contract-001.csv` narrows the
replacement path: no-geometry source-derived rows may be considered for an
internal parser link-candidate fixture closeout only. Map, topology, adapter,
official, operational, approval, ROI, public-readiness, and external-readiness
uses remain blocked.

`data/international-canada-link-fixture-replacement-closeout-001.csv` closes
that narrow replacement. `data/canada_source_link_candidates.csv` now
regenerates from the validated source-derived no-geometry extraction candidates,
but it remains an internal parser fixture and does not promote map, topology,
adapter, official-network, operational, approval, ROI, public-readiness, or
external-readiness claims.

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
