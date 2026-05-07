---
name: Parliament Summary — route-rust-architecture — Round 1
slug: R1-SUMMARY-route-rust-architecture
type: review
artifact: specs/2026-05-06-route-rust-architecture.md
round: 1
status: draft
rubric_version: v1.0
voices_seated: [eisenhower, moses, foxx, freight-economist, traffic-engineer, climate-engineer, rural-advocate, citation-auditor, scope-keeper, numeracy-checker]
created: 2026-05-06
---

# Parliament Summary — Round 1

**Artifact**: `specs/2026-05-06-route-rust-architecture.md`
**Headline**: Spec is architecturally sound and buildable for Phase 1; 8 specific gaps must be addressed before the scoring engine can produce defensible corpus entries.

---

## Earned stakes

Stakes that survived argument — the spec does these well and should not change them.

| Voice | Stake | Why earned |
|---|---|---|
| Moses | Anchor-first, manual-first | Phase 1 manual pipeline before automation is explicit and correct. All voices accepted this. |
| Moses | Crate separation | One job per crate. No voice challenged this. |
| Traffic Engineer | IRI for D3 pavement condition | Correct metric, correct source. Eisenhower and Freight Economist concurred. |
| Freight Economist | FAF5 as freight value source | Right source. Only voice that pushed on FAF5 also agreed it belongs; dispute is on attribution method, not inclusion. |
| Climate Engineer | FEMA SFHA as v1.0 D1 proxy with explicit scope boundary | Defensible for Phase 1; §9 is honest about what's missing. Rural Advocate and Foxx accepted the boundary. |
| Eisenhower | NHS shapefile as corpus foundation | Authoritative strategic network data. No voice disputed this. |
| Editorial / Scope Keeper | §9 scope discipline | Unusually strong scope boundary for a technical spec. All editorial voices noted this positively. |

---

## Contested stakes (argument live, not resolved)

| Voice | Stake | Contested by | State |
|---|---|---|---|
| Eisenhower | Undirected graph for B2 centrality | Traffic Engineer (directional flow matters operationally) | Unresolved — needs spec amendment |
| Moses | Scoring anchors as binary constants | Freight Economist (calibration requires runtime config) | Collision — see below |
| Foxx | Equity gap detection algorithm unspecified | Rural Advocate (same concern, different framing) | Unresolved — needs spec amendment |

---

## Collisions

### Collision 1: Scoring anchor mutability
**Moses** stakes: hardcoded scoring anchors are acceptable for Phase 1 — build what ships.
**Freight Economist** stakes: anchors compiled as constants block calibration — runtime config is required.
**Resolution**: Freight Economist wins this argument. The design spec (§4.3 of the design spec) explicitly states the rubric calibration pass may require rescaling. If rescaling requires a recompile, the calibration loop is broken. Amendment required: scoring anchors move to a runtime config file (`~/.route/config/scoring.toml` or committed `config/scoring.toml`). Moses accepts; this does not delay the anchor run.

### Collision 2: What does A1 score for a highly-variable corridor?
**Moses** stakes: AADT averaging is a reasonable first pass.
**Traffic Engineer** stakes: averaging AADT across an 1,800-mile corridor with 25:1 variance between rural and urban segments produces a meaningless score.
**Resolution**: contested, not resolved. Both voices have strong arguments. Proposed path: score A1 at the segment level (per 50-mile segment), then report corridor-level A1 as the 90th-percentile segment score (worst-performing segment drives the corridor score). This preserves the corridor as the unit of analysis while not hiding urban/rural variance. Needs spec amendment.

---

## Refuted stakes

| Voice | Stake | Refuted by | Why |
|---|---|---|---|
| (none) | — | — | No voice staked a position that collapsed under argument |

---

## Key gaps — required spec amendments before `validated`

The following are blockers. The spec cannot advance to `validated` without addressing them.

### G1 — B2 centrality on partial graph (Eisenhower)
During corpus build, the national graph is incomplete. B2 scores on a partial graph are misleading. **Fix**: mark B2 as `estimated: true` for all corridors until `route score-all` completes the national graph. Document the partial-graph limitation in the B2 scoring function.

### G2 — A1 scoring for high-variance corridors (Moses + Traffic Engineer collision)
AADT averaging across a multi-state corridor loses the information needed for A1. **Fix**: amend spec §5 to define A1 as the 90th-percentile segment score (worst 10% of route miles drives the score). Add segment-level AADT computation to `route-network`.

### G3 — FAF5 attribution method unspecified (Freight Economist)
How commodity flows from FAF5 O-D pairs are attributed to specific corridors is the most analytically complex join in the pipeline and is completely unspecified. **Fix**: add §4.4 "FAF5 Attribution" to the spec. For v1.0: attribute flows to corridors by FAF5 zone traversal (which FAF5 zones does the corridor pass through; sum flows between those zones). Flag this as an approximation; note that routing-based attribution is a v2.0 enhancement.

### G4 — B3 scoring function and data field absent (Freight Economist)
B3 (Port/Border Access) has no implementation path in the current spec. **Fix**: add `port_terminus_flag: bool` and `nearest_top10_port_miles: Option<f32>` to `CorridorAttributes`. Define B3 scoring function in §5. Data source: BTS port data + FHWA border crossing data, both in `data/sources.md`.

### G5 — Rural/urban population decomposition missing (Rural Advocate + Foxx)
`pop_within_50mi` without rural/urban split makes C2 and C3 scoring coarse. **Fix**: add `rural_pop_within_50mi: Option<u64>` and `pct_rural_in_buffer: Option<f32>` to `CorridorAttributes`. Source: USDA ERS rural codes joined to Census ACS population. Add `tribal_land_miles: Option<f32>` for C3 equity scoring.

### G6 — `parallel_interstate_count` too coarse for B1 (Traffic Engineer)
A count of parallel interstates doesn't capture detour distance. **Fix**: replace `parallel_interstate_count: u8` with `nearest_parallel_miles: f64` (distance in miles to nearest parallel interstate-quality route) and `detour_penalty_miles: f64` (additional miles via best alternate route). Both computable from the graph.

### G7 — Equity gap algorithm unspecified (Foxx + Rural Advocate)
`route gap --type equity` has no defined detection threshold. **Fix**: add §4.x "Gap Detection Algorithms" to the spec. Equity gap definition: C3 > 6.5 AND C1 < 4.0 (high need, low political visibility). Rural connectivity gap: C2 > 7.0 AND B1 > 6.0 (primary rural access with no redundancy).

### G8 — ATRI citation and FAF5 version unspecified (Editorial / Citation Auditor)
**Fix**: cite specific ATRI annual bottleneck report(s) in the committed CSV header. Pin FAF5 to v5.6 (2022 data) in §4.1.

---

## Improvements that are not blockers (do in Round 2)

- `max_consecutive_sfha_miles` alongside `fema_sfha_miles` for better D1 geographic clustering (Climate Engineer)
- `wildfire_hazard_miles` using USFS polygon data — polygon overlay, not raster, fits current stack (Climate Engineer)
- `dcfc_per_100mi` for EV charging density, sourced from DOE AFDC (Climate Engineer)
- `intercity_bus_stops` and `park_and_ride_count` for D2 transit equity (Foxx)
- `max_rural_interchange_gap_miles` for rural access granularity (Rural Advocate)
- `pct_no_vehicle_households` for C3 transit-dependent population (Foxx)
- Scoring anchor units: clarify `pct_pop_below_poverty` and `pct_truck` as 0–100 or 0.0–1.0 (Editorial / Numeracy)

---

## Recommended next steps

1. **Author addresses G1–G8** — spec amendment pass. Most are additive (new fields, new sections). G2 (A1 variance) and G3 (FAF5 attribution) require new §§.
2. **Round 2 review** — abbreviated; only the amended sections need re-review. Eisenhower, Freight Economist, and Traffic Engineer have the live stakes; re-seat those three plus Editorial.
3. **Advance to `reviewed` status** after G1–G8 are addressed and Round 2 passes.
4. **Begin Cargo workspace scaffold** — parallel to Round 2 review. The crate structure (§2) and dependency list (§3) are stable and do not depend on the contested sections.
