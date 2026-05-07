# Plan: Interstate Arterials — Tiering the National Highway Network

**Track**: A — Corpus & Scoring
**Venue**: Transportation Research Part A
**Target**: 8,000–10,000 words

## The Question

The US interstate system has no official tier classification. FHWA designates routes as NHS or not, STRAHNET or not — but there is no "primary artery" vs "secondary connector" distinction analogous to transit systems. Metro maps show trunk lines in bold; arterials in lighter weight; local lines barely visible. Can we derive an empirically grounded tier classification for the national interstate network?

## The Method

Score all 227 interstate corridors in the ROUTE corpus against 12 dimensions. Use network centrality (B2 — Brandes betweenness) and freight intensity (A2 — FAF5 commodity flow) as the primary tiering signals. Cluster corridors into 3–4 tiers using the natural breaks in the joint B2/A2 distribution.

Validate: do the tiers match the intuitive "trunk" routes that practitioners name when asked? Do they align with STRAHNET designation? Do they predict ATRI bottleneck frequency?

## The Finding (hypothesis)

The national network has approximately:
- **Tier 1 — Primary Arteries** (~8 routes): The coast-to-coast and major N-S spines that carry >40% of national freight ton-miles: I-80, I-90, I-10, I-40, I-5, I-95, I-35, I-75
- **Tier 2 — Major Connectors** (~25 routes): Regional corridors linking Tier 1 nodes to secondary metros
- **Tier 3 — Regional Feeders** (~80 routes): State-level connectors
- **Tier 4 — Local Access** (~114 routes): Primarily serving urban distribution

The visual output is a schematic "metro map" of the US highway system — thick lines for Tier 1, thinner for Tier 2, etc.

## Sections

1. **Introduction** — The missing tier classification; why it matters for investment prioritization
2. **Background** — Existing NHS/STRAHNET designations; transit analogy; prior network science work on road networks
3. **Data & Scoring** — ROUTE corpus (227 corridors); 12-dimension pool; HPMS AADT; FAF5 commodity flows; Brandes centrality
4. **Tier Classification** — Joint B2/A2 clustering; natural break analysis; tier assignment
5. **Validation** — STRAHNET alignment; ATRI bottleneck correlation; practitioner survey comparison
6. **The Arterial Map** — Schematic visualization; what it shows that current FHWA maps don't
7. **Implications for I2.0** — Investment prioritization; which tiers need which I2.0 features
8. **Conclusion**

## Key Claims

- C1: The national interstate network has 3–4 natural tier clusters identifiable from betweenness centrality and freight intensity alone
- C2: Tier 1 arterials (≤10% of route miles) carry >50% of national truck freight ton-miles
- C3: The Tier 1/Tier 2 boundary aligns well with STRAHNET designation but captures freight intensity that STRAHNET misses
- C4: The schematic "metro map" is a more actionable planning tool than the current functional classification system

## Data Sources

- ROUTE corpus: 227 corridors, HPMS 2018, TIGER 2023, FAF5 v5.6
- ATRI Top 100 Bottleneck Report 2024
- FHWA STRAHNET designation
- `route score-all` output (to be generated)
