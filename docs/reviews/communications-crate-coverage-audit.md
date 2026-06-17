---
name: ROUTE Communications Crate Coverage Audit
slug: route-communications-crate-coverage-audit
type: review
status: draft
rubric_version: v1.0
author: codex
created: 2026-06-17
updated: 2026-06-17
sources:
  - Cargo.toml
  - crates/route-data/Cargo.toml
  - crates/route-network/Cargo.toml
  - crates/route-score/Cargo.toml
  - crates/route-map/Cargo.toml
  - crates/route-report/Cargo.toml
  - crates/route-sim/Cargo.toml
  - crates/route-cli/Cargo.toml
  - crates/route-cli/src/main.rs
  - README.md
  - docs/SYSTEM_PLAN.md
  - docs/SPEC_INDEX.md
  - docs/vtrace/COMMUNICATIONS_STRATEGY.md
  - docs/reports/route-evidence-posture.md
  - docs/reviews/communications-pressure-test-run-002.md
  - docs/reviews/communications-external-rehearsal-readiness.md
---

# ROUTE Communications Crate Coverage Audit

## Scope

This audit compares the current communications package against the implemented
Rust workspace and command families. It looks for story gaps where ROUTE has
designed or implemented features that are not yet legible in the Interstate 2.0
communications package.

This audit does not promote any feature to official-plan, construction-ready,
guaranteed-SLA, numeric ROI, eligibility, compliance, endorsement, or public
readiness status.

## Overall Finding

Decision: **partial_coverage**

The communications package is strong for the service-network pitch, evidence
posture, stop/SLA demo, map-proof discipline, stakeholder fixture workflow, and
pressure-test review ladder.

It does not yet fully represent the implementation breadth in ROUTE. The biggest
story gap is not a missing feature list; it is the missing bridge between the
public promise and the deeper machinery that makes ROUTE defensible:

- source acquisition and source-health operations;
- standards proof and Blueprint downgrade gates;
- recursive tier optimizer and constraint ledger;
- bundle-first identity and segment stitching;
- T2/T3/T4 access, terminal, and game/ops readiness surfaces;
- pavement, bridge, and asset-condition evidence gates;
- T1/T1 failure evidence windows and live snapshot limitations;
- Interstate Tycoon/game publication layer;
- release/readiness manifests.

The package should not try to pitch all of these at once. It should add a
technical appendix and evidence-roadmap layer so reviewers can see the full
machine without confusing internal gates for public proof.

## Crate Coverage Matrix

| Crate | Owns | Current Communications Coverage | Gap |
|---|---|---|---|
| `route-data` | Source fetching, parsing, manifests, FLETCH handoff, ACS/HPMS/FEMA/NBI/FAF5/source policy. | Mentioned indirectly through source-pack templates and evidence posture. | Source operations are under-storied. The audience does not yet see why source custody, cache policy, source health, and snapshot windows are first-class ROUTE features. |
| `route-network` | Graph build, joins, bundles, stable segment identity, coverage, flow, centrality, investment primitives, tier regions. | Bundle-first identity is mentioned in the tech deck; coverage/flow/investment are mostly absent from current communications. | The story underplays the graph and identity engine. Bundle registry, national segment ids, stitching, tier regions, flow, and investment primitives need a technical appendix before technical reviewers can understand the architecture. |
| `route-score` | 16-dimension scoring, anchors, confidence labels, score ledgers. | Service hierarchy and evidence labels are communicated; scoring instrument is not foregrounded. | The package does not yet explain the 16-dimension instrument as the measurement layer behind Interstate 2.0. This weakens the "why these priorities?" story. |
| `route-map` | Geographic maps, Beck schematics, T1/T2 diagnostics, T3 zone boards, map publication surfaces. | Strong map-proof guardrails and stop/SLA demo coverage. | T1 diagnostics, T3 zone render boards, and map-publication readiness are underrepresented compared with T2/stop surfaces. |
| `route-report` | Corpus/report generation over bundle identities. | Communications reports exist, but generated corpus/report capability is barely named. | The package should show that ROUTE can write reviewable corpus entries and ledgers, not only decks and prose reports. |
| `route-sim` | Wardrop assignment, incidents, chaos testing, OD simulation, relay hubs, EV/rest outage, passenger/SLA matrices. | Relay hubs and 48-hour freight reports cover the story; command-level simulation evidence is only lightly represented. | Simulation is one of the strongest hidden gems, but current package avoids showing it except through the stop/SLA demo and reports. Need a bounded simulation appendix with clear heuristic labels. |
| `route-cli` | Command orchestration and artifact gates. | Demo runbook and pressure-test gates show a subset. | CLI family breadth is not visible. Reviewers cannot yet see how many gates exist across standards, Blueprint, optimizer, tier access, game, release, source, and evidence operations. |

## Command Family Coverage

| Family | Examples | Coverage | Story Gap |
|---|---|---|---|
| Source acquisition / source health | `fetch`, `fetch-hpms`, `fetch-acs`, `fetch-fema`, `source-fetch-policy`, `fletch-sources`, `t1-fetch-*`, `t1-import-*`, `t1-source-health` | partial | Current story says "source pack first" but not "ROUTE has a source operations layer." |
| Scoring / corpus / reports | `score`, `score-all`, `coverage`, `report` | partial | Measurement layer is not yet a first-class communications surface. |
| Graph / flow / investment | `build`, `flow`, `invest`, `connectivity`, `diamond`, `tier-connectivity` | weak | The "argument machine" story needs a small technical map from graph facts to claims. |
| Map and schematic publication | `map`, `map-atlas`, `map-publication-readiness`, `beck-t1-diagnostics`, `beck-t2-diagnostics`, `t3-zone-map-diagnostics`, `t3-zone-render-board` | strong for guardrails, partial for breadth | Maps are well bounded, but diagnostics and render-board richness are underused. |
| Standards / pressure scenarios | `standards-proof`, `standards-inventory`, `standards-pavement`, `standards-bridges`, `pressure-scenarios`, `throughput-proof` | partial | Pressure-test language exists, but not the standards proof ledger as a product feature. |
| Blueprint / cost / evidence downgrade | `blueprint`, `blueprint-evidence`, `blueprint-costs` | weak | Current ROI story blocks fake numbers, but does not show the Blueprint downgrade machinery that prevents premature investment claims. |
| Tier optimizer / constraint ledger | `tier-optimize`, `optimizer-manifest`, `optimizer-constraint-ledger`, `optimizer-constraint-budget`, `optimizer-claim-review`, `optimizer-map-hooks` | weak | Recursive optimizer is in the tech deck, but the command and artifact chain is not visible enough for analysts. |
| T1/T2/T3/T4 access and terminal proof | `t1-sla-candidate-pairs`, `t1-stop-selector`, `t2-*`, `t3-zone-*`, `t4-terminal-*`, `t3-t4-access-gaps` | partial | The public story names T1-T4, but the lower-tier access machinery is far richer than the decks show. |
| Pavement / bridge / asset evidence | `tier-pavement-*`, `standards-bridges` | weak | Asset-condition gates are mostly held in map-proof language; they need an evidence-roadmap slot. |
| Failure evidence windows / evidence operations | `t1-failures`, `t1-failure-events`, `t1-evidence-windows`, `t1-snapshot-plan`, `t1-accumulate-events` | weak | The current package does not explain the snapshot-history guard, which is a strong proof-discipline story. |
| Game / Interstate Tycoon | `game scenarios`, `game inspect`, `game run-season`, `game score`, T2 game publication evidence commands | partial outside communications package | Game layer is named in README, but the current communications package does not say how game mechanics preserve evidence labels. |
| Release / readiness | `release-manifest`, L0/L1/L2 scripts | partial | External-readiness gate exists, but release manifest and public-readiness mechanics are not integrated into the story. |

## Highest-Value Story Gaps

| Priority | Gap | Why It Matters | Recommended Surface |
|---|---|---|---|
| P1 | Source operations as product feature | External reviewers will ask where evidence comes from. ROUTE has more than templates: cache policy, FLETCH handoff, source health, snapshot guards, and source-access dockets. | `docs/reports/source-operations-evidence-roadmap.md` or technical appendix. |
| P1 | Standards proof and Blueprint downgrade gates | The pitch says "evidence-bounded"; the proof ledger and Blueprint gates are the strongest mechanism behind that claim. | Add a "How claims are downgraded before investment" appendix. |
| P1 | Recursive optimizer artifact chain | The technology deck says requirements change artifacts, but the actual optimizer chain is much richer than the 225-mile demo. | Add an optimizer evidence appendix with tier regions, constraint ledger, manifest, map hooks, and held blockers. |
| P1 | Bundle-first identity story | Stable bundle/member/stitch identity is a core architecture advantage, but it is easy for audiences to miss. | Add a one-page "Why route labels are not enough" technical brief. |
| P1 | Lower-tier access machinery | Rural/access story is persuasive, but the T3/T4 command surfaces show a serious access system behind it. | Add a T3/T4 access appendix tied to rural and terminal reports. |
| P2 | Simulation engine proof posture | Wardrop, incident, relay, EV/rest outage, passenger/SLA matrix features are designed but not visible in the main comms package. | Add bounded simulation appendix with implemented/heuristic labels. |
| P2 | Game layer evidence discipline | Interstate Tycoon is a useful public-facing bridge, but needs communications guardrails. | Create game-facing public-positioning surface before using game material in external rehearsal. |
| P2 | Asset condition / pavement / bridge evidence | Map and SLA claims are held partly because pavement and bridge evidence is unresolved. That should be legible, not buried. | Add asset evidence rows to evidence posture and source-pack roadmap. |
| P2 | Release/readiness manifests | External/public readiness should point to release manifest mechanics, not only L0/L1/L2 commands. | Link release manifest to external rehearsal gate and evidence posture. |

## Current Package Strengths

| Strength | Evidence |
|---|---|
| Evidence boundaries are strong. | Red lines, evidence posture, role review, prohibited-claim scans, and external-readiness gates are consistent. |
| Service-network story is coherent. | Split decks, doctrine report, 48-hour promise, relay hubs, rural access, resilience, and maps report align. |
| Internal pressure ladder is useful. | Simulation plus Run 001 and Run 002 now define how to pass each round internally. |
| Stakeholder fixture workflow is mature. | Template, runbook, external gate, and external packet template exist without fabricating evidence. |
| Map-proof discipline is strong. | Maps Are Not Proof, caption pattern, map-publication scope, and evidence posture reinforce each other. |

## Risks If Gaps Remain

| Risk | Likely Failure Mode |
|---|---|
| Reviewer sees ROUTE as a deck package, not a machine. | Technical sponsor misses source ops, optimizer, standards proof, release, and game mechanics. |
| Story over-indexes on stop/SLA demo. | The demo becomes the apparent whole system, undercutting the richer crate design. |
| Evidence discipline sounds defensive instead of powerful. | Source-health, snapshot guards, proof downgrades, and release manifests are hidden, so holds look like weakness rather than governance. |
| Rural/access story sounds aspirational. | T3/T4 access ledgers and terminal proof dockets are not visible enough to prove the system actually carries lower-tier pressure. |
| Game layer drifts into entertainment-only framing. | Interstate Tycoon could be misunderstood as separate from ROUTE evidence labels and pressure-test scenarios. |

## Recommended Next Work Packages

1. **Source Operations Evidence Roadmap**
   Explain source fetching, cache policy, FLETCH handoff, source health, snapshot
   windows, and source-access dockets as the evidence supply chain.

2. **Technical Appendix: From Requirement To Optimizer Artifact**
   Expand the current 225-mile fixture into a map of the real optimizer chain:
   tier regions, candidate columns, constraint ledger, manifest, map hooks, held
   blockers, and claim review.

3. **Bundle Identity One-Pager**
   Make the route-label problem legible for non-engineers: stable segment ids,
   bundle ids, stitch groups, aliases, state scope, and why this prevents claim
   drift.

4. **T3/T4 Access Appendix**
   Tie rural access, terminal access, zone obligations, selected feeder routes,
   held gaps, terminal proof, and T3 zone render boards to the rural-access
   report.

5. **Simulation And Game Evidence Boundary**
   Show how route-sim and Interstate Tycoon use scenarios without promoting
   proof-grade claims, and what would be required before game/public readiness.

## Gate

Decision: **partial_coverage; expand technical evidence appendices before
external rehearsal**

Rationale: The current communications package is not gratuitously thin; it is
appropriately focused for the pitch and review ladder. But ROUTE has designed
and implemented enough deeper machinery that the package now needs selected
technical appendices. Those appendices should reveal the evidence engine without
promoting construction, ROI, SLA, eligibility, compliance, endorsement, or
public-readiness claims.
