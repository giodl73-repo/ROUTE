# Plan: Freight Bottlenecks — Where the Interstate System Exceeds Capacity

**Track**: B — Gap Analysis
**Venue**: Transportation Research Part B: Methodological
**Target**: 9,000–11,000 words

## The Question

Which interstate corridors are operating above design capacity? What does that cost the freight economy annually? And are the worst bottlenecks on T1 corridors (structurally important) or T2 beltways (locally stressed)? The congestion-stress paradox from A.1 predicts that the ATRI bottleneck list will be dominated by T2 urban connectors — but the economic cost of T1 congestion may be higher per incident because T1 failures cascade nationally.

## The Method

1. **Corpus scoring**: use v1.1 ROUTE scores for all 227 corridors. Identify corridors with A1 (Throughput Gap) ≥ 6.0 as candidate bottlenecks.
2. **ATRI validation**: join `data/atri-bottlenecks.csv` (50 locations, annual congestion cost $M) to corridor scores. Verify that ATRI locations cluster on corridors with high A1 scores.
3. **Bottleneck economics**: for each ATRI location, compute:
   - Annual freight cost: ANNUAL_COST_M (from CSV)
   - Truck-hours lost: ANNUAL_COST_M × $1M ÷ ($150/hr per truck)
   - Freight value at risk: truck-hours × average freight value per truck-hour
4. **Tier distribution**: which tiers account for what share of ATRI bottleneck cost?
   - Hypothesis: T2 connectors account for >60% of ATRI bottleneck count but T1 corridors account for >50% of total economic cost (because T1 cascades nationally)
5. **Bottleneck-to-corridor attribution**: map each ATRI location to its corridor and compute the corridor-level bottleneck density (ATRI locations per 100 miles).

## Primary Finding (hypothesis)

- M corridors score A1 ≥ 6.0 (V/C > 0.85 equivalent) from HPMS data
- Top 10 ATRI bottlenecks annual cost: $X billion total ($916M–$197M range)
- T2 connectors dominate count (60%+): I-285, I-95 segments, I-75 Atlanta
- T1 corridors dominate total economic cost ($X vs $Y for T2) — cascade multiplier
- Donner Pass (I-80) is the highest-impact single bottleneck NOT in ATRI top 50 (weather-driven, not congestion-driven — a different bottleneck type)

## Quantification Contract

- Primary: M corridors at A1 ≥ 6.0; top-10 ATRI annual cost $X billion total
- Secondary: tier distribution of ATRI bottleneck cost (T1 vs T2 share)
- Tertiary: bottleneck density (ATRI locations per 100 miles by tier)

## Sections

1. Introduction — bottleneck as a distinct problem from coverage gaps (B.1)
2. Background — ATRI methodology; HCM capacity; freight economics of delay
3. Data & Methods — ROUTE scores + ATRI CSV join; tier attribution
4. Bottleneck Identification — A1 ≥ 6.0 corridors; distribution by tier
5. Economic Cost Analysis — ATRI annual costs; tier distribution
6. The Congestion-Stress Paradox in Bottleneck Data — T2 count vs T1 cost
7. Donner and Weather Bottlenecks — a different bottleneck type
8. Implications for I2.0 — managed lane priority; T2 relief valve targeting
9. Conclusion

## Data Sources

- data/scores-all.csv (v1.1 ROUTE scores, 227 corridors)
- data/atri-bottlenecks.csv (ATRI Top 50, 2024)
- HPMS 2018 AADT data (partial — 28/50 states)
- ROUTE HighwayGraph (TIGER 2023)
