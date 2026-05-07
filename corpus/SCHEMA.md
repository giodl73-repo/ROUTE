---
name: Corridor Schema v1.0
slug: corridor-schema
type: spec
status: draft
rubric_version: v1.0
author: human
created: 2026-05-06
updated: 2026-05-06
sources: []
---

# Corridor Schema v1.0

Every file in `corpus/existing/` and `corpus/proposed/` follows this schema.
Read this before authoring any corridor entry. Don't invent fields.
If a new field is needed, amend this file first and document the change.

---

## Frontmatter

```yaml
---
name: "Interstate 80 — New York to San Francisco"
slug: i-80
type: existing-corridor          # existing-corridor | proposed-corridor
status: draft                    # draft | reviewed | validated | deprecated
rubric_version: v1.0
author: human
created: YYYY-MM-DD
updated: YYYY-MM-DD
sources:
  - "FHWA Highway Statistics 2023"
  - "ATRI Top Truck Bottlenecks 2024"
corridor:
  termini: ["Teaneck, NJ", "San Francisco, CA"]
  states: [NJ, PA, OH, IN, IL, IA, NE, CO, UT, NV, CA]
  approx_miles: 2909
  designation: "I-80"
  classification: trunk            # trunk | connector | spur | proposed
  # proposed-corridor only:
  proposed_by: ""                  # FHWA | AASHTO | state-DOT | user-mapped
  study_reference: ""
---
```

---

## Body Structure

### 1. Overview (2–4 sentences)
What this corridor is, where it runs, and its primary economic/social function. No pre-judgment — just what it does.

### 2. Key Facts Table

| Fact | Value | Source |
|---|---|---|
| Total miles | | |
| Average AADT | | |
| Average truck % | | |
| Number of bridges | | |
| % bridges rated poor | | |
| States traversed | | |
| Major metros served | | |
| Major ports/borders accessed | | |

### 3. Dimension Scores

Score each dimension 0–10. One-sentence justification with source citation required for every score.
Proposed-corridor scores are estimates — mark with `†`.

| Band | Dim | Name | Score | Justification |
|---|---|---|---|---|
| A | A1 | Throughput Gap | | |
| A | A2 | Freight Intensity | | |
| A | A3 | Speed Reliability | | |
| B | B1 | Redundancy | | |
| B | B2 | Network Centrality | | |
| B | B3 | Port/Border Access | | |
| C | C1 | Population Reach | | |
| C | C2 | Rural Connectivity | | |
| C | C3 | Equity Access | | |
| D | D1 | Climate Resilience | | |
| D | D2 | Multimodal Integration | | |
| D | D3 | Infrastructure Vintage | | |

**Band totals**: A: _/30 · B: _/30 · C: _/30 · D: _/30 · **Total: _/120**

### 4. Notable Segments
Key segments that differ significantly from corridor average — major bottlenecks, major high-performers, or anomalies worth noting. 2–5 bullet points.

### 5. Interstate 2.0 Fit
Which Interstate 2.0 features (from spec §5) would apply to this corridor and why. One sentence per applicable feature. Proposed corridors: which features are required to justify the designation.

### 6. Open Questions
Unresolved issues, data gaps, or contested claims. Honest accounting of what's uncertain.

### 7. Sources
Full citations for every number in the entry. Format: Author/Organization, Title, Year, URL or access note.

---

## Scoring Anchors

Use these to calibrate scores across the corpus.

### A1 — Throughput Gap
- 0: AADT well below capacity; free-flow conditions throughout
- 5: Moderate congestion in urban segments; rural segments free-flow
- 10: Chronic LOS E/F on majority of route miles; major bottleneck corridor

### A2 — Freight Intensity
- 0: <500 trucks/day average across route
- 5: 2,000–5,000 trucks/day; regional freight significance
- 10: >10,000 trucks/day average; top-tier national freight corridor

### A3 — Speed Reliability
- 0: Average speed consistently at or above design speed
- 5: Moderate speed variance; 10–15 mph below design in peak periods
- 10: Chronically 20+ mph below design speed; high day-to-day variance

### B1 — Redundancy
- 0: 3+ parallel interstate-quality alternatives within 50 miles
- 5: 1 alternative route; adds significant distance
- 10: No viable alternative; single point of failure for the region

### B2 — Network Centrality
- 0: Peripheral route; failure has minimal cascade effects
- 5: Regional connector; failure reroutes significant but manageable traffic
- 10: Spine route; failure cascades across national freight and passenger network

### B3 — Port/Border Access
- 0: No port or border crossing within 100 miles of termini
- 5: Access to mid-tier port or border crossing
- 10: Direct terminus at top-5 US port or major US–Canada/Mexico crossing

### C1 — Population Reach
- 0: <500k people within 50 miles of corridor
- 5: 2–5M people within 50 miles
- 10: >20M people within 50 miles

### C2 — Rural Connectivity
- 0: Primarily urban; <10% of route through agricultural or rural land
- 5: Mixed; serves both urban and significant rural/agricultural areas
- 10: Primary access route for large agricultural region; rural communities have no close alternative

### C3 — Equity Access
- 0: Primarily serves high-income metro areas with abundant transportation alternatives
- 5: Mixed; serves some lower-income areas; some alternatives exist
- 10: Primary access for low-income, tribal, or rural communities with no viable alternatives

### D1 — Climate Resilience (higher = more at risk)
- 0: Low exposure; inland, low flood/heat/fire risk through 2050
- 5: Moderate exposure; some segments at risk from one climate hazard
- 10: High exposure; coastal flood zone, wildfire corridor, or extreme heat region through majority of route

### D2 — Multimodal Integration
- 0: No adjacent freight rail; no intermodal facilities; no transit connection
- 5: Some rail proximity; 1–2 intermodal hubs on route
- 10: Parallel freight rail throughout; multiple intermodal hubs; transit corridor potential

### D3 — Infrastructure Vintage (higher = older/worse condition)
- 0: Recent construction (post-1990); good pavement and bridge condition; low deferred maintenance
- 5: Mixed vintage; some pre-1970 sections; moderate deferred maintenance
- 10: Predominantly pre-1970 construction; significant deferred maintenance backlog; poor bridge condition ratings
