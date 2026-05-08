# Plan: Empty Miles and Load Matching — The Relay Marketplace as National Freight Optimizer

**Track**: C — Freight & Throughput
**Venue**: Transportation Research Part E (Logistics and Transportation Review)
**Target**: 8,000–10,000 words

## The Question

35% of US truck miles run empty — called "deadhead" or empty backhaul. A truck delivers LA→NYC, then either returns empty (the dominant choice when the driver wants to go home), waits at origin for a return load (adds dwell time and HOS waste), or finds a return load via broker (30–60 minute overhead, 15–20% commission). The question: does the I2.0 relay hub architecture provide a structural solution to the empty backhaul problem, and if so, how large is the efficiency gain?

The relay hub has scheduling information that spot brokers lack: it knows what trucks are arriving in the next 4–8 hours and what loads are available within 400 miles. That advance knowledge is the input to a pre-matched load assignment that eliminates the 30–60 minute search cost and the broker commission, and which — at scale — drives down the national empty-mile rate from 35% toward the 8–12% rate achieved by UPS and FedEx on their optimized closed networks.

The precise mechanism: a relay hub scheduler runs a bipartite matching (trucks arriving ↔ loads departing) with a 5–10 minute computation window. A driver who drops a trailer at the hub can be assigned a return load before the trailer is unhooked. The constraint that drove 35% empty — "driver wants to go home" — becomes tractable because the relay hub is a planned home location on a relay route, not an ad hoc destination.

## The Method

1. **Empty-mile baseline**: ATRI Operational Costs of Trucking (2023) for national empty-mile rate. Disaggregate by corridor type: long-haul truckload (TL), regional TL, LTL, and dedicated private fleet. Report empty-mile rate variance across O-D pairs using FMCSA MCMIS commodity flow data.

2. **Relay hub load-matching model**: Queueing theory formulation — the relay hub as an M/G/k queue where trucks are the customers and load-matching slots are the servers. Key parameters: truck arrival rate λ (from ROUTE corridor AADT × truck fraction), load availability μ (from commodity flow density around the hub catchment area, 400-mile radius), matching latency (5–10 min target vs. 30–60 min baseline). Model the match rate as a function of hub throughput and load density.

3. **Empty-mile reduction curve**: parameterize the model over hub throughput (500–5,000 trucks/day) and catchment load density. Report projected empty-mile rate at each node. Calibrate against UPS hub performance (Memphis Worldport: ~8% empty) as the upper-bound efficiency benchmark.

4. **Economic valuation**: empty miles carry full variable costs (fuel, driver hours, depreciation) with zero revenue. At 35% empty: $900B × 35% = $315B/year in cost producing zero revenue. Estimate savings at 20% empty vs. 35% baseline: 15 percentage points × $900B = $135B/year in efficiency gain. Separately compute carbon reduction: 45M fewer empty truck-miles per day nationally at the 15pp improvement level.

5. **Corridor-level ranking**: which I2.0 T1 corridors have the highest empty backhaul rates and therefore the highest relay hub value? Hypothesis: corridors serving unidirectional commodity flows (coal/grain outbound from rural interior, manufactured goods inbound from ports) will have structural imbalance that drives high empty rates. Rank T1 corridors by estimated empty-mile rate differential (outbound vs. inbound commodity flow ratio).

6. **Algorithm design**: specify what data the relay hub scheduler requires (truck arrival schedule, load tender inventory, driver HOS clock, trailer type), what the matching objective function is (maximize loaded miles, minimize driver detention, minimize carbon), and what the latency requirement is (match must be offered before driver completes unloading — approximately 20 minutes). Compare to Uber Freight, Convoy, and DAT load board architectures.

## The Finding (hypothesis)

- National empty-mile rate drops from 35% to 18–22% when relay hub pre-matching is deployed on the top 20 T1/T2 corridors
- The reduction is larger on unidirectional corridors (I-29 grain corridor, I-10 port connector segments) where structural flow imbalance is high
- Economic gain: $113B–$135B/year in freight efficiency, with zero additional infrastructure cost beyond the relay hub
- Carbon reduction: 35% fewer empty truck-miles ≈ 15% reduction in national trucking emissions per revenue ton-mile
- The load-matching function contributes more value per relay hub dollar than the driver-matching function (relay driver handoff)
- At scale, the relay hub marketplace becomes a national freight clearing platform — more analogous to an exchange than a dispatch system

## Key Claims

- C4.1: Relay hubs reduce national empty backhaul from 35% to approximately 20% via scheduled pre-matched load assignment, benchmarked against UPS/FedEx closed-network performance (~8%)
- C4.2: The empty-mile reduction captures $113B–$135B/year in freight efficiency — larger than the $121B managed-lane capital investment — with no additional infrastructure cost
- C4.3: Carbon reduction: 15pp improvement in empty-mile rate yields approximately 45M fewer empty truck-miles per day nationally, reducing trucking sector emissions by ~12–15% per revenue ton-mile
- C4.4: Corridors with structural commodity flow imbalance (outbound/inbound ratio > 1.5) generate 2–3× more empty-mile savings per relay hub than balanced corridors; I-29 (grain), I-10 (port), and I-95 (manufactured goods southbound) are primary targets

## Sections

1. Introduction — The deadhead problem; why 35% of US truck miles earn zero revenue; the relay hub as structural solution
2. Background — Empty backhaul economics; broker market structure; UPS/FedEx benchmark; existing load-matching platforms and their limitations
3. Data and Methods — ATRI empty-mile data; FMCSA commodity flow data; queueing model formulation; hub catchment parameterization
4. Empty-Mile Baseline by Corridor Type — Long-haul TL, regional TL, LTL, private fleet; which corridors are worst; structural flow imbalance analysis
5. The Load-Matching Model — Bipartite matching formulation; latency requirements; match rate as function of hub throughput and load density
6. Economic Valuation — Per-hub savings; national aggregate; carbon reduction; comparison to managed-lane capital cost
7. Algorithm and Data Architecture — What data the relay hub scheduler needs; matching objective function; comparison to Uber Freight / Convoy / DAT
8. Conclusion — The relay hub as national freight exchange; policy implications for I2.0 hub siting decisions

## Data Sources

- ATRI Operational Costs of Trucking 2023 (empty-mile rates, operating cost structure)
- FMCSA MCMIS 2022 (carrier-level mileage, loaded vs. empty splits)
- BTS Commodity Flow Survey 2022 (O-D flow imbalance by corridor)
- FHWA HPMS 2021 (truck AADT by segment, used to parameterize hub arrival rates)
- UPS 2023 Sustainability Report (Memphis Worldport throughput, empty-mile benchmark)
- DAT Freight Intelligence (load-to-truck ratios by lane, spot rate data)
- EPA MOVES3 (emissions per truck-mile, empty vs. loaded)
- ROUTE corpus: existing T1 corridor scores (commodity flow dimension, relay hub dimension)
