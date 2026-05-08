# Plan: The Last Interstate Mile — T1 Port Connections and the Highway-Maritime Interface

**Track**: B — Gap Analysis
**Venue**: Maritime Policy and Management
**Target**: 8,000–10,000 words

## The Question

I2.0 managed lanes on T1 corridors solve the 2,000-mile trunk haul. But freight entering or leaving major US ports travels the "last interstate mile" — a T1 or T2 connector from the port gate to the national T1 corridor. These segments are short (5–20 miles), politically fragmented (port authority, city, state, and FHWA jurisdictions overlap), and routinely operate at V/C > 1.2. They are the binding capacity constraint for US import and export freight, but they do not appear in standard T1 tier analysis because corridor-level scoring averages away short-segment bottlenecks.

The paper makes three arguments: (1) port connector segments are the systemic weak point in the US freight network, not the T1 trunk corridors; (2) a dedicated "port connector" classification is needed — something more demanding than T2 but purpose-built for port-interface geometry and drayage patterns; (3) targeted I2.0 investment in port connectors ($2–5B per major port, 10 ports nationally = $20–50B) would produce disproportionate gains in national import/export freight throughput.

The USMCA border crossing case adds a second tier of evidence: I-35 at Laredo handles $277B in annual US-Mexico trade volume. At peak, 16,000 trucks/day cross with 2–4 hour average wait times. At $225/hr truck operating cost, that is $10.8B/year in border delay cost alone — for a single connector segment that FHWA does not classify as a T1 bottleneck.

## The Method

1. **Port connector identification**: identify the primary interstate connector segment for the top 10 US container ports by TEU volume (LA/Long Beach, NY/NJ, Savannah, Houston, Seattle/Tacoma, Baltimore, Norfolk, Charleston, Miami, Oakland). For each, identify the first interstate segment from the port gate — not the nearest T1 corridor, but the actual approach road. Record classification (T1, T2, state highway, non-interstate) and length.

2. **V/C measurement**: retrieve HPMS AADT and lane count for each port connector segment. Compute V/C using HCM 6th Edition freeway capacity (2,200 pcphpl base). Flag segments where V/C > 1.0 (oversaturated). Cross-reference with ATRI freight bottleneck rankings for corroboration.

3. **Drayage pattern analysis**: port drayage differs from long-haul TL in two ways relevant to connector design: (a) extreme peak concentration (container gates operate 6am–6pm, creating a 12-hour demand window vs. 24-hour long-haul flows), and (b) empty container return trips that double vehicle-miles on the connector per loaded container. Model the peak-hour demand separately from AADT — the connector must handle peak load, not average load.

4. **Gap identification**: classify connectors by failure mode — (a) capacity failure (V/C > 1.2 at peak), (b) classification failure (non-interstate standard on final approach, e.g., SR-509 at Tacoma), (c) resilience failure (single chokepoint with no alternate, e.g., I-710), (d) jurisdictional fragmentation (multiple agency overlaps prevent investment). Report which ports face which failure modes.

5. **Investment case**: estimate upgrade cost per connector and projected throughput improvement. Use FHWA port connectivity studies and INFRA grant awards as cost benchmarks. Compute NPV using port TEU volume as the demand base and USDOT TIGER/INFRA cost-benefit methodology.

6. **USMCA border crossing analysis**: I-35 at Laredo as the canonical case. $277B annual trade volume, 16,000 trucks/day, 2–4 hour average wait → $10.8B/year delay cost. Model the connector from the bridge to I-35 junction. Compare to the cost of a dedicated truck-only lane on the connector ($200–400M).

## The Finding (hypothesis)

- 7 of the top 10 US port connector segments operate at V/C > 1.2 at peak — as congested as the worst-ranked T1 bottlenecks in the ROUTE corpus
- The binding constraint on US import/export freight capacity is the port connector, not the T1 trunk corridor, for at least 5 major ports
- Savannah's I-16/I-95 interchange becomes the binding East Coast freight constraint within 5 years if port growth continues at 2020–2024 CAGR (12% annually)
- LA/Long Beach I-710 upgrade is the single highest-NPV port investment in the US: it serves 40% of US container import volume on an 11-mile segment running at V/C 1.5+
- Combined port connector investment of $20–50B would increase national import/export freight throughput by ~15%, with a cost 40–60% lower per throughput-unit than equivalent T1 capacity expansion

## Key Claims

- B5.1: The top 10 US port connector segments have mean V/C > 1.2 at peak — equivalent to the worst T1 bottlenecks in the ROUTE corpus — but are excluded from T1-level analysis by their short length
- B5.2: A dedicated "port connector" classification standard is needed, defined by: drayage-pattern peak demand (not 24-hour AADT), port gate throughput as the design demand basis, and V/C ≤ 0.85 at the 95th-percentile peak hour
- B5.3: Targeted port connector investment of $20–50B (10 ports × $2–5B) increases national import/export freight throughput by approximately 15% — a higher throughput-per-dollar return than equivalent T1 managed-lane expansion
- B5.4: Savannah's I-16/I-95 connector will become the binding US East Coast freight constraint within 5 years given current port growth trajectory; investment lead time is 7–10 years, meaning the decision window is now

## Sections

1. Introduction — The last interstate mile problem; why short connectors are the binding constraint; scope of the paper
2. Background — US port freight volumes; drayage economics; prior port connectivity studies; IIJA port infrastructure investment history
3. Data and Methods — Port connector identification methodology; HPMS V/C analysis; drayage peak demand model; HCM capacity standards; USDOT NPV methodology
4. Port Connector Baseline — The top 10 US ports: connector segment, classification, length, V/C, failure mode taxonomy; comparative ranking
5. Case Studies — LA/Long Beach (I-710, V/C 1.5+, 40% of US container imports); Savannah (I-16/I-95, fastest-growing port, infrastructure lag); NY/NJ (I-278/I-95, chronic V/C 1.3+); Seattle/Tacoma (SR-509/SR-518, sub-interstate standard)
6. The USMCA Border Connector — I-35 at Laredo: $277B trade volume, $10.8B/year delay cost; truck-only lane investment case
7. The Port Connector Standard — Proposed classification criteria; design demand basis; investment prioritization framework; jurisdictional coordination model
8. Conclusion — Binding constraint diagnosis; investment sequencing; policy implications for I2.0 and IIJA port infrastructure programs

## Data Sources

- USDOT Bureau of Transportation Statistics, Port Performance Freight Statistics Program (2022 TEU volumes, gate hours, dwell times)
- FHWA HPMS 2021 (AADT by segment; lane count; functional classification)
- ATRI Top Freight Bottlenecks 2023 (corroboration for V/C rankings)
- POLA/POLB Gate Statistics 2023 (LA/Long Beach truck gate volumes by hour)
- Georgia Ports Authority Annual Report 2023 (Savannah TEU growth, connector traffic data)
- FHWA Freight Performance Measures: TTI/PTI for designated truck corridors
- CBP USMCA Trade Statistics 2023 ($277B Laredo trade volume, crossing counts)
- GSA/DOT INFRA Grant award database (port connector cost benchmarks)
- HCM 6th Edition (Highway Capacity Manual — freeway capacity standards)
- USDOT TIGER/INFRA benefit-cost guidance (NPV methodology)
- ROUTE corpus: existing I-710, I-10, I-95 corridor scores (freight intensity, resilience dimensions)
