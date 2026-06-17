---
name: Source Operations Evidence Roadmap
slug: source-operations-evidence-roadmap
type: report
status: draft
rubric_version: v1.0
author: codex
created: 2026-06-17
updated: 2026-06-17
sources:
  - docs/reviews/communications-crate-coverage-audit.md
  - docs/fletch-source-orchestration-spec.md
  - docs/source-fetch-cache-policy.md
  - docs/evidence-campaigns/milepost-9-snapshot-history-guard.md
  - docs/reviews/fletch-source-orchestration-role-review.md
  - docs/reviews/milepost-8-t1-failure-evidence-review.md
  - docs/reviews/milepost-9-evidence-operations-review.md
  - docs/vtrace/COMMUNICATIONS_STRATEGY.md
  - docs/reports/route-evidence-posture.md
---

# Source Operations Evidence Roadmap

## Purpose

This report explains source operations as part of the ROUTE communications
story.

The point is simple: `source-needed` is not a shrug. In ROUTE, a missing source
can become a cacheline, source-health row, access docket, proof artifact,
snapshot window, role review, and claim decision.

This report does not promote any source, corridor, map, route, hub, terminal,
standard, service window, cost, ROI, eligibility, compliance, construction, or
endorsement claim.

## Communications Thesis

ROUTE is strongest when it treats evidence acquisition as a visible workflow,
not a footnote.

The communications package already says that claims need sources. The deeper
implementation story is that ROUTE has machinery to protect those sources:

- cache mutation policy;
- FLETCH handoff and registry rows;
- source-fetch policy gates;
- source health and access dockets;
- manual/cached proof artifacts;
- live snapshot preservation rules;
- snapshot-history guards;
- evidence-window promotion blocks.

That machinery turns evidence discipline from "we do not know yet" into "here
is the exact path from missing source to allowed claim."

## Source Operations Stack

| Layer | ROUTE Surface | What It Does | Claim Boundary |
|---|---|---|---|
| Source family | `route fetch`, `route fetch-hpms`, `route fetch-acs`, `route fetch-fema`, `route t1-fetch-*` | Acquires public or configured source payloads. | Downloaded does not mean validated. |
| Cache policy | `docs/source-fetch-cache-policy.md`; `route source-fetch-policy --gate` | Preserves last usable cache, scoped merges, temp-then-replace writes, and live snapshot semantics. | Cache safety is not proof that a transportation claim is true. |
| FLETCH handoff | `docs/fletch-source-orchestration-spec.md`; `route fletch-sources --gate`; `data/fletch-source-handoff.csv` | Maps ROUTE source families to cachelines and acquisition handoff rows. | FLETCH orchestrates acquisition; ROUTE owns admissibility and promotion. |
| Source health | `route t1-source-health`, source-access dockets, source plans | Separates live, blocked, key-gated, endpoint, archive, and request-needed sources. | Source availability does not close annual, recovery, ROI, or readiness claims. |
| Proof artifacts | T4 terminal proof contracts, stitched-member proof intake, stakeholder source packs | Lets manual or cached source evidence enter review without live fetcher support. | Proof artifact must be accepted before any row can become source-backed. |
| Snapshot windows | `route t1-evidence-windows --gate-windows` | Prevents snapshot-only rows from becoming promotion eligible. | Current-event snapshots are samples, not historical annual evidence. |
| Claim review | claim trace, evidence posture, `.roles` review | Decides whether a source changes a label, hold, blocker, or next step. | Review can hold or downgrade even when the source exists. |

## Safe Story

| Safe Message | Do Not Say |
|---|---|
| ROUTE has a source operations layer for turning missing evidence into reviewable source tasks. | ROUTE has all sources needed for publication-grade claims. |
| A cached source is available for ROUTE to consume after parser and policy gates. | A cached source proves a corridor, terminal, SLA, or ROI claim. |
| FLETCH helps ROUTE orchestrate fetch/cache handoff. | FLETCH validates ROUTE transportation claims. |
| Snapshot feeds can support observation samples and source operations. | Snapshot feeds prove annual reliability or recovery performance. |
| Manual/cached proof artifacts can support review when live fetch is unavailable. | Manual proof placeholders are source-backed by default. |

## Source Family Posture

| Source Family | Current Communications Use | Evidence Operation | Current Claim Posture |
|---|---|---|---|
| Manifest downloads | Shows ROUTE can acquire baseline public data. | `route fetch` delegates manifest-backed HTTP downloads through FLETCH and preserves legacy cache paths. | implemented acquisition; claim validation remains separate |
| HPMS pavement / roadway data | Supports future pavement, throughput, and asset-condition source tasks. | `route fetch-hpms`, scoped merge, pavement acquisition dockets. | source operations implemented / asset claims gated |
| ACS population / income | Supports reach, equity, rural/access, and distributional fields. | `route fetch-acs`, `route fetch-acs-income`, county joins. | source operations implemented / claim-specific use gated |
| FEMA flood / SFHA counts | Supports flood exposure and resilience source rows. | `route fetch-fema`, `route fetch-fema-d1`, corridor/tile cache behavior. | source operations implemented / hazard claims gated |
| T1 live event feeds | Supports evidence campaigns for T1/T1 failure observations. | Iowa 511, INDOT, TDOT, MDOT fetch/import paths; source health and snapshot plans. | observation/sample posture; annual/recovery claims gated |
| Terminal/contact proof | Supports T4 and terminal access source-needed rows. | manual/cached proof artifact contract, proof intake, source registry, attachment review. | proof workflow exists; source-backed rows still require accepted artifacts |
| Stakeholder fixture sources | Supports external rehearsal readiness and requirement-to-refinement fixtures. | source-pack template and fixture closeout runbook. | template/runbook exists; populated fixture held |

## Why This Matters In Review

| Reviewer Pressure | Source Operations Answer |
|---|---|
| "Where did this evidence come from?" | Source custody requires owner, title, date/year, path or access note, units, and reviewer. |
| "What happens if a fetch fails?" | Cache policy preserves previous usable artifacts; failed or partial fetches cannot silently reduce evidence scope. |
| "Can a live feed prove annual performance?" | No. Snapshot-only rows are blocked from promotion unless repeated-window or historical-archive evidence exists. |
| "Can you use manual evidence?" | Yes, but only through proof-intake fields, artifact attachment, review, and accepted source-backed status. |
| "Who decides whether a source promotes a claim?" | ROUTE evidence posture and `.roles` review, not the fetcher. |

## Roadmap From Source-Needed To Source-Backed

| Step | Artifact | Pass Condition | Failure / Hold |
|---|---|---|---|
| 1. Name the source need | Source pack, source plan, access docket, or proof contract | Need names source family, claim, field, and owner. | Generic "need data" remains held. |
| 2. Acquire or register source | Cacheline, manual artifact, URL, access note, or source registry row | Source has owner/title/date/path/access note. | Access blocked, key-gated, endpoint broken, or no source found. |
| 3. Preserve cache safely | Source-fetch policy and temp-then-replace/scoped merge behavior | Existing usable evidence is not destroyed by failed fetch. | Fetch cannot activate or must preserve last-good cache. |
| 4. Parse / normalize | ROUTE parser or proof-intake schema | Fields, units, and scope are explicit. | Empty, partial, malformed, or out-of-scope rows stay held. |
| 5. Attach to claim | Claim trace, evidence posture, or generated ledger | Claim label changes only with source and reviewer. | Source exists but does not support claim. |
| 6. Review | Citation Auditor, Numeracy Checker, Scope Keeper, and affected domain roles | Review records pass, pass_with_risk, hold, or downgrade. | Any role can preserve hold or block promotion. |
| 7. Propagate status | Evidence posture, pressure/Blueprint/release docs, external packet | All referenced surfaces share the same label. | Drift creates readiness hold. |

## Current External-Rehearsal Impact

The external rehearsal gate remains `hold_external_rehearsal` until a populated
source-backed stakeholder fixture or equivalent source-backed concrete example
exists.

Source operations help prepare that fixture, but they do not replace it. A
venue-specific packet still needs:

- named venue and audience lane;
- selected materials;
- source custody;
- before/after artifact or label change;
- affected role review;
- prohibited-claim scan;
- L0 closeout.

## Priority Additions To The Communications Package

| Priority | Addition | Reason |
|---|---|---|
| P1 | Add source operations row to presenter technical appendix. | Technical reviewers should see why evidence holds are operational, not vague. |
| P1 | Add source operations pointer to external rehearsal packet template. | Any concrete external example needs source custody before use. |
| P1 | Add source operations to route evidence posture. | Current posture should show this as a communications surface. |
| P2 | Add one populated source-backed fixture when real input exists. | This is the true external-readiness blocker. |
| P2 | Add command transcript for source gates when a technical rehearsal needs it. | Some reviewers may ask for reproducibility beyond row/status summaries. |

## Gate

Decision: **story_ready_as_evidence_roadmap; claim_promotion_held**

Rationale: ROUTE can safely tell the source-operations story now because it is a
workflow and governance story, not a transportation outcome claim. Concrete
source-backed claims remain held until source custody, parser/proof acceptance,
role review, and status propagation close for the specific claim.
