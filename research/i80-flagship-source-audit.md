---
name: I-80 Flagship Source Audit
slug: i80-flagship-source-audit
type: report
status: reviewed
rubric_version: v1.4
author: copilot
created: 2026-07-11
updated: 2026-07-11
sources:
  - corpus/existing/i80.md
  - data/corridor-designations.csv
  - data/sources.md
  - config/scoring.toml
  - crates/route-network/src/aggregate.rs
  - crates/route-network/src/strategic.rs
  - crates/route-score/src/score.rs
  - crates/route-report/src/lib.rs
  - https://www.fhwa.dot.gov/planning/national_highway_system/
  - https://ops.fhwa.dot.gov/freight/infrastructure/nfn/
  - https://www.fhwa.dot.gov/bridge/britab.cfm
  - https://www.ams.usda.gov/sites/default/files/media/TransportationofUSGrainsModalShare.pdf
---

# I-80 Flagship Source Audit

## Research Question

Which claims in the current I-80 corpus record are defensible now, which require
recomputation or relabeling, and what must change before the corridor can move
from `draft` to `reviewed`?

## Decision Supported

This audit governs Pulse 03 of the I-80 flagship wave. The corpus record should
not be completed by prose-only editing. The generator, source custody, and
annotation boundary must be repaired first so regeneration does not restore
known defects or erase reviewed narrative.

## Local Baseline

- `corpus/existing/i80.md:1-31` records a v1.4 draft with a score of 89.8 / 160,
  Medium confidence, and five broad source labels.
- `corpus/existing/i80.md:43-101` contains useful generated measurements but
  also three human-annotation placeholders and several heuristic dimensions.
- `data/corridor-designations.csv:13` assigns I-80 A4=9.0, B4=8.5, and C4=9.0
  from one prose note rather than row-level source records.
- `crates/route-report/src/lib.rs:205-307` regenerates the placeholders and
  overwrites the whole corpus file.

## Findings

### ROUTE-I80-01 — Federal network framing is safe

**Sources**

- FHWA, National Highway System:
  https://www.fhwa.dot.gov/planning/national_highway_system/
- FHWA, National Highway Freight Network:
  https://ops.fhwa.dot.gov/freight/infrastructure/nfn/

**Observed constraint**

FHWA defines the NHS as roads important to national economy, defense, and
mobility. It separately defines STRAHNET as highways important to strategic
defense and the NHFN as the federally designated highway freight network. FHWA
also states that every Interstate route is part of the NHFN, either through the
PHFS or the non-PHFS Interstate component.

**Implication**

ROUTE can safely describe I-80 as an Interstate, NHS, STRAHNET, and NHFN
corridor. Those classifications do not prove that every I-80 segment is a
top-tier bottleneck, border corridor, or positive-return investment.

**Confidence:** High.

### ROUTE-I80-02 — A4/B4/C4 lack source custody

**Sources**

- `data/corridor-designations.csv:1-13`
- `crates/route-network/src/strategic.rs:1-60`
- `config/scoring.toml`, sections `a4`, `b4`, and `c4`

**Observed constraint**

The CSV assigns three numeric scores to I-80 from the note
`Transcontinental; Offutt AFB/STRATCOM; Corn Belt->Pacific`. It has no source
URL, source year, geographic scope, distance, commodity-flow measure, or
calculation. The Rust module calls the values sourced and not hard-coded, but
moving manually selected constants into CSV does not create source custody.

A4 is especially weak: the configured anchor describes direct or significant
US-Canada and US-Mexico crossing service, while I-80 has no international
border terminus.

**Implication**

A4=9.0, B4=8.5, and C4=9.0 must remain heuristic. Pulse 03 should either add
source-level designation rows and a documented calculation or preserve the
scores with an explicit `source-needed` label. A4 must not be described as
direct border service.

**Confidence:** High.

### ROUTE-I80-03 — Freight importance is supportable; the USMCA claim is not

**Sources**

- FHWA, National Highway Freight Network:
  https://ops.fhwa.dot.gov/freight/infrastructure/nfn/
- `data/corridor-designations.csv:13`

**Observed constraint**

Federal freight-network membership supports a national freight-role claim.
The current local note instead jumps from "transcontinental" to a 9.0 USMCA
score without identifying a crossing or measured border flow.

**Implication**

Adopt NHFN/PHFS language for the overview. Defer any top-tier international
trade score until a route-to-border or FAF flow method exists.

**Confidence:** High.

### ROUTE-I80-04 — The AADT summary is not length-weighted

**Sources**

- `crates/route-network/src/aggregate.rs:38-42`
- `crates/route-network/src/corridor.rs:28-31`
- `crates/route-report/src/lib.rs:217-223`

**Observed constraint**

The implementation computes a simple mean across matched graph edges. The
attribute comment calls the P90 length-weighted, but the aggregation shown in
`aggregate.rs` does not apply segment length weights. The report labels the
result only as `Mean AADT`, which invites interpretation as a corridor-wide
traffic average.

**Implication**

Rename the published metric to `mean across matched HPMS segments` and publish
coverage plus a distribution or state/segment breakdown. Do not use 11,344 as
a uniform I-80 traffic statement.

**Confidence:** High.

### ROUTE-I80-05 — The C3 GDP text emits a false-looking value

**Sources**

- `crates/route-score/src/score.rs:609-617`
- `corpus/existing/i80.md`, C3 row

**Observed constraint**

When relative GDP is present but total corridor GDP is absent, the formatter
uses `unwrap_or(0.0)` and prints `$0.0B total buffer GDP`.

**Implication**

This is a reporting defect, not a real economic result. Pulse 03 should print
`total buffer GDP unavailable` unless a value exists and add a regression test.

**Confidence:** High.

### ROUTE-I80-06 — Bridge terminology is defensible, route totals still need a gate

**Sources**

- FHWA, Frequently Requested NBI Information:
  https://www.fhwa.dot.gov/bridge/britab.cfm
- `data/sources.md`, Infrastructure condition
- `corpus/existing/i80.md`, Key Facts

**Observed constraint**

FHWA defines bridge condition from the lowest deck, superstructure,
substructure, or culvert rating and defines ratings of 4 or lower as Poor. The
repo has an NBI cache path, but the corpus provides no route-level coverage
count or command evidence next to the exact 3,327 and 2% values.

**Implication**

Keep the NBI-derived values as implemented measurements, but add route-match
coverage and the exact cache year before review promotion.

**Confidence:** Medium-high.

### ROUTE-I80-07 — Agricultural production access is safer than export dominance

**Sources**

- USDA AMS, *Transportation of U.S. Grains: A Modal Share Analysis,
  1978-2020*:
  https://www.ams.usda.gov/sites/default/files/media/TransportationofUSGrainsModalShare.pdf
- `data/corridor-designations.csv:13`

**Observed constraint**

I-80 crosses major agricultural production regions, but agricultural export
movement is multimodal. A route-level export score requires commodity,
origin-destination, and terminal evidence; a Corn Belt label alone does not
prove that long-haul truck exports use I-80.

**Implication**

Use `agricultural production access` in the narrative. Keep `export corridor`
and C4=9.0 held until FAF/USDA flows are joined to the route.

**Confidence:** High for the boundary; medium for the eventual score.

### ROUTE-I80-08 — Human narrative and generated measurement need separate ownership

**Sources**

- `crates/route-report/src/lib.rs:40-51`
- `crates/route-report/src/lib.rs:205-307`
- `corpus/existing/i80.md`

**Observed constraint**

`route report I80` overwrites the complete markdown file and regenerates
placeholder sections. Directly completing the prose would make the next
regeneration destructive.

**Implication**

Prototype a compatibility boundary before narrative completion. Preferred
options are a checked annotation sidecar consumed by `route-report` or a
generated measurement include embedded by a curated corridor file. Pulse 03
must choose one and add a regeneration test.

**Confidence:** High.

### ROUTE-I80-09 — Climate and reliability claims remain bounded

**Sources**

- `corpus/existing/i80.md`, A3 and D1 rows
- `docs/game/donner-weather-closure-g0.md`
- `config/scoring.toml`, sections `a3` and `d1`

**Observed constraint**

The current A3 value is a BPR estimate rather than observed NPMRDS reliability.
The Donner scenario explicitly remains heuristic pending observed closure
history and alternate-capacity evidence. Zero mapped SFHA miles cannot be
promoted into a general zero-flood-risk claim.

**Implication**

Keep observed reliability, annual closure frequency, and corridor-wide flood
exposure held. The narrative may identify Donner and central-plains weather as
evidence targets, not quantified benefits.

**Confidence:** High.

### ROUTE-I80-10 — The flagship lacks a corridor-specific decision chain

**Sources**

- `docs/anchors/i80-flagship-baseline.md`
- `gaps/`
- `reviews/`

**Observed constraint**

ROUTE has national gap reports, I-80-related scenarios, and architecture
reviews, but no single artifact traces an I-80 measured gap to a compared
treatment, Parliament decision, and compact packet.

**Implication**

Corpus completion should improve the measurement record without selecting a
treatment. Treatment selection belongs after the corridor-specific gap pulse.

**Confidence:** High.

## Recommendations

### Adopt Now

| Action | Owner | Validation |
|---|---|---|
| Use NHS, STRAHNET, and NHFN language for I-80's federal network role | ROUTE docs/reporting | Citation audit against FHWA pages |
| Relabel mean AADT as an unweighted mean across matched HPMS segments | `route-network`, `route-report` | Unit test and regenerated I-80 record |
| Suppress `$0.0B` when total GDP is unavailable | `route-score` | Focused formatter/scoring regression test |
| Keep A3, A4, B4, C4, ROI, SLA, and construction claims visibly heuristic or held | ROUTE corpus | Editorial gate |

### Prototype Behind A Compatibility Boundary

| Action | Owner | Validation |
|---|---|---|
| Add an annotation sidecar or generated-measurement include boundary | `route-report` | Regeneration preserves reviewed prose |
| Add source-custody fields for strategic designation rows | `route-network`, `data/` | CSV schema gate and I-80 source audit |
| Add AADT coverage/distribution output | `route-network`, `route-report` | Known weighted/unweighted fixture |

### Reject Or Defer

| Action | Reason |
|---|---|
| Publish I-80 as a top-tier USMCA border corridor | No direct crossing or measured border-flow method is attached |
| Describe C4 as dominant agricultural export movement | Production access does not establish export mode or route share |
| Treat `$0.0B` corridor GDP as evidence | It is formatter fallback behavior |
| Select a managed-lane, Donner, interchange, or new-corridor treatment now | The corridor-specific gap comparison has not been run |

## Non-Goals

- Re-score the national corpus during the audit.
- Claim official-plan, construction, SLA, or ROI readiness.
- Replace source acquisition with narrative confidence.
- Expand the wave beyond I-80.
