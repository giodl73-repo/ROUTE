---
name: Editorial Gate Review — route-rust-architecture
slug: R1-editorial-route-rust-architecture
type: review
artifact: specs/2026-05-06-route-rust-architecture.md
voices: [citation-auditor, scope-keeper, numeracy-checker]
round: 1
status: draft
rubric_version: v1.0
author: editorial
created: 2026-05-06
---

# Editorial Gate Review

Three editorial voices. Form gate only — not a substance review.

---

## Citation Auditor

**Verdict: citation-conditional — 3 blockers before `validated`**

| Claim | Source cited | Traceable | Verdict |
|---|---|---|---|
| NHS shapefile as data source | "FHWA National Highway System documentation" in frontmatter | Yes — specific enough | PASS |
| HPMS provides AADT, PCT_TRUCK, IRI | §4.1 table, source "FHWA HPMS summary" | Yes — matches `data/sources.md` | PASS |
| NBI provides SUFFICIENCY_RATING, YEAR_BUILT | §4.1 table, source "FHWA NBI" | Yes — matches `data/sources.md` | PASS |
| FAF5 provides TONS, VALUE by O-D pair | §4.1 table, source "FAF5 flows" | Yes — matches `data/sources.md` | PASS |
| ATRI bottleneck seed "hand-curated" | §4.1 — no specific ATRI report cited | **NO** — which ATRI report(s)? Which year? | **BLOCKER** |
| "0.01° tolerance ≈ 1.1km" | §8 — stated without citation | Implied from spherical geometry; acceptable for a spec | PASS (acceptable) |
| petgraph Brandes algorithm for centrality | §5 — crate named but not versioned | Needs crate version pinned in §3 | MINOR |
| FAF5 version not pinned | §4.1, §3 — "FAF5 flows" but no dataset version | **NO** — FAF5 has v4 and v5; v5 released 2022 | **BLOCKER** |
| resvg "0.42" in dependencies | §3 — versioned | Yes | PASS |
| FEMA SFHA shapefile | §4.1 — matches `data/sources.md` | Yes | PASS |

**Blockers for `validated`:**
1. ATRI bottleneck seed: cite specific ATRI annual report(s) and year(s) of data used for the hand-curated CSV.
2. FAF5 version: pin to FAF5 v5.6 (2022 data) or state which version.

---

## Scope Keeper

**Verdict: scope-pass with one flag**

| Check | Result |
|---|---|
| Artifact type is `spec` and content is a spec | PASS |
| Content stays within technical architecture boundary | PASS |
| §9 (What This Spec Does Not Cover) is explicit | PASS — strong scope discipline |
| No drift into corridor scoring content (belongs in corpus/ not spec) | PASS |
| No drift into design proposal content | PASS |
| METIS noted and explicitly deferred | PASS |

**One flag (not a blocker):** §5 scoring engine includes a Rust code example for `score_a2`. This is implementation detail that could belong in `route-score/src/` rather than a spec. However, since this spec explicitly states the structs are "the contract between crates" (§10), including implementation examples is defensible as contract illustration. Not a blocker; note for author consideration.

---

## Numeracy Checker

**Verdict: numeracy-pass with 2 clarifications needed**

| Claim | Value | Unit check | Order-of-magnitude | Arithmetic | Verdict |
|---|---|---|---|---|---|
| Bridge join tolerance | ≤0.01° | Degrees of lat/lon — correct unit for geographic coord | At 40°N: 0.01° lon ≈ 850m, 0.01° lat ≈ 1,100m. Traffic engineer says 850m; spec says "≤0.01° tolerance" — both defensible | N/A | PASS |
| A2 score cap | `.min(10.0)` | Dimensionless 0–10 | 10,000 trucks/day / 1,000 = 10.0 — arithmetic is correct | ✅ | PASS |
| Map resolution | 1600×900 | Pixels — standard 16:9 | Reasonable for corridor maps | N/A | PASS |
| `pct_pop_below_poverty` | Unspecified unit | **Is this 0–100 or 0.0–1.0?** | Ambiguous | N/A | **CLARIFY** |
| HPMS AADT range | Not stated | — | AADT on interstates: 3,000–350,000. Field name `mean_aadt: Option<u32>` — max u32 is ~4.3B, fine for AADT | N/A | PASS (unit type is fine) |
| `annual_freight_value_b` (not in spec yet) | — | If added: "B" for billions — needs unit comment in code | — | — | Flag for when added |
| `fema_sfha_miles` | `Option<f64>` | Miles — correct | Max interstate ~3,000 miles; SFHA miles ≤ route miles | N/A | PASS |

**Clarifications needed (not blockers):**
1. `pct_pop_below_poverty`: specify whether the field is a percentage (0.0–100.0) or a proportion (0.0–1.0). This must be consistent across all uses in `route-score`.
2. `pct_truck` in HPMS: similarly — is this 0–100 or 0.0–1.0? The A2 scoring function uses `pct as f64 / 100.0`, implying it expects 0–100. State this assumption explicitly in the struct doc comment.
