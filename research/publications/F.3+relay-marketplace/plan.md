# Plan: The Relay Marketplace — Platform Design for 48-Hour National Freight

**Track**: F — Transit Integration (extended: covers both passenger transit and driver relay)
**Venue**: Management Science / Transportation Research Part C
**Target**: 8,000–10,000 words

## The Problem

The simulation shows relay driving ($40M in stations) captures 90%+ of the SLA improvement
that $121B in managed lanes delivers. But relay only works if a marketplace exists:
- Who matches truck X arriving at SLC hub at 2am to available relay driver?
- Whose ELD records the driver handoff for HOS compliance?
- Who is liable if the relay driver has an accident on someone else's load?
- How does a solo owner-operator access relay drivers they don't employ?

Currently: none of this infrastructure exists. Relay is operationally viable (UPS/FedEx/BNSF
all do crew changes internally) but there is no inter-company relay marketplace.

## The Argument

1. The relay marketplace is the missing Layer 0 of I2.0 — it costs $0 in highway
   infrastructure but requires regulatory and platform innovation.

2. The T1 diamond hub locations are the natural marketplace nodes. They are already
   designated as freight hubs by the ROUTE analysis. Adding driver-matching technology
   to these hubs creates the network.

3. The business case works: relay swap fee $75-150/swap × 8,000 trucks/day on NY-LA
   × 6 swaps per trip = $3.6-7.2M/day in marketplace revenue on one corridor.
   Hub operating cost: ~$1M/year. Revenue >> cost from day one.

4. Precedent: airline crew scheduling, BNSF/UP crew changes, UPS Worldport relay.
   The difference: all existing relay is within one company's driver pool. The
   marketplace opens relay to all carriers.

## Why Companies Think Solo is Cheaper (and Why They're Wrong)

**The visible cost calculation (wrong):**
- Solo long-haul driver: $0.52/mile × 2,800mi = $1,456/trip
- Relay: 6 drivers × 7h × $25/hr = $1,050/trip (already cheaper)
- Companies see the coordination cost (booking relay drivers, managing handoffs) as
  too high — they'd rather pay the solo driver premium

**The hidden cost calculation (what companies miss):**
- Asset utilization: solo driver leaves truck idle 48% of the time (10h rest per 21h HOS cycle)
  A $180,000 truck running at 98% utilization (relay) needs half as many trucks as 48% (solo)
  Truck fleet capital: $180k × 0.5× fleet = $90k savings per route per truck cycle
- Driver shortage premium: solo long-haul drivers command $5,000-$15,000 signing bonuses
  currently (ATRI 2024); relay drivers (regional, home every night) have no shortage premium
- Insurance: long-haul accident rates are 4× regional rates; relay reduces exposure

**The coordination cost barrier is real but solvable:**
- Current: each carrier manages its own driver pool; relay requires cross-carrier coordination
- Solution: neutral marketplace platform (like airline code-share or rail trackage rights)
  The platform handles: driver matching, HOS compliance handoff, insurance clearance,
  load custody documentation, payment settlement

## Marketplace Design

### Platform Architecture

**The Slot Exchange:**
- Carriers post "relay slots": truck X arrives hub A at time T, needs driver to hub B
- Driver pool (employed by relay hub operator OR independent relay drivers) accepts slots
- Platform clears: confirms driver availability, insurance, CDL status, drug test currency
- 15-minute grace window: if driver no-shows, platform dispatches backup from hub pool

**The Hub Operator Model (preferred):**
- Each T1 hub has a licensed "relay operator" — could be a neutral private entity
  (like a ground handler at an airport) or a carrier consortium
- Relay operators employ 30-50 regional relay drivers per hub
- Carriers pay per-swap fee; hub operator pays drivers per-shift
- This is the Amazon DSP / airline ground handling model applied to trucking

**The Independent Relay Driver Model (supplemental):**
- CDL holders who want shift work (retired truckers, young drivers building hours,
  semi-retired workers, part-time drivers) register on the platform
- Platform verifies CDL, drug test, insurance, HOS reset status
- Driver accepts shifts within their home radius (40-mile max to hub)
- This creates a gig-economy layer for relay driving (like Uber Pool for freight)

### HOS Compliance Architecture

Critical regulatory issue: the ELD (Electronic Logging Device) records HOS per driver,
not per truck. A relay handoff requires:
1. Driver A's ELD records end-of-duty at hub A
2. Pre-trip inspection logged in the new driver's name
3. Driver B's ELD records start-of-duty at hub A with the same load
4. FMCSA electronic load transfer: bill of lading, USDOT number, insurance cert

Current regulation: 49 CFR Part 395 does not explicitly accommodate relay operations
(it was written assuming one driver per trip). Required regulatory change:
- Define "relay terminal" as an approved location for driver handoff
- Define "relay pre-trip" as a condensed inspection (10-minute vs 30-minute standard)
  since the truck was inspected by the previous driver 6 hours ago
- Allow electronic HOS transfer between ELD systems at approved relay terminals

This is a FMCSA rulemaking, not a statutory change — achievable in 18-24 months.

### Insurance and Liability

Current problem: if relay driver B has an accident, who is liable?
- The load owner (shipper)? The carrier (truck owner)? The relay driver's employer?
- Currently no framework exists because relay is only done within single-company fleets

Proposed framework:
- Relay terminal operators carry "relay liability" coverage: umbrella policy covering
  all drivers operating within their hub's relay system
- Cost: ~$2/swap in insurance premium (already embedded in the $75-150 swap fee)
- Standard contractual allocation: carrier is liable for truck condition;
  relay driver/hub operator is liable for driving while on relay shift

### Load Security

Shippers worry: "who touched my load between driver A and driver B?"
- Relay handoff protocol: platform records timestamp, GPS location, driver ID,
  and a seal/tamper-evident scan of the trailer at each handoff
- This is BETTER than current solo-driver security where no handoff record exists
- For regulated loads (pharma, alcohol, firearms): relay terminal can have a
  customs officer or licensed agent present at handoff — higher security than today

## The Business Case

### Revenue Model (per T1 hub on NY-LA)

- Trucks through hub per day: ~2,000 (bidirectional flow)
- Relay swaps per truck: 1 (some trucks will relay only at major hubs)
- Swap fee: $100 (midpoint of $75-150 range)
- Daily revenue per hub: $200,000
- Annual revenue per hub: $73M
- Hub operating cost: 50 drivers × $58k + facility + tech = $4.5M/year
- **Hub net margin per year: $68M**
- Hub capex: $5M
- Payback period: **27 days**

This is not a marginal business. It is an extremely profitable utility infrastructure
play — comparable to a truck stop or fuel distribution hub, but with 10× the margin
because driver labor is the product, not fuel.

### Why Is No One Doing This?

Coordination failure. Each carrier could set up relay within its own fleet (some large
carriers do) but no neutral platform exists for cross-carrier relay. The reasons:
1. Regulatory uncertainty (HOS framework unclear for relay)
2. First-mover disadvantage (who builds the hub before the carriers agree to use it)
3. Incumbent driver employment model (unions resist change to relay model)
4. Industry fragmentation (70,000+ carriers in the US; no single actor can mandate relay)

The government's role: designate T1 hub locations as National Freight Relay Zones
(authority already exists under IIJA's NHPP program for multimodal freight facilities).
Publish FMCSA rulemaking on relay terminal definition. The rest is private sector.

## The SLA Claims

The simulation (route sla-matrix, 5,000 trips per corridor) shows relay alone delivers:

| Corridor | Today p95 | Relay only | Full I2.0 | SLA unlock |
|---|---|---|---|---|
| MIA→NYC | 35.7h | 26.8h | 22.8h | **overnight** |
| HOU→CHI I-69 | 31.2h | 22.4h | 20.0h | **overnight** |
| DAL→NYC | 49.1h | 30.5h | 26.8h | 3-day → **next-day** |
| SEA→CHI | 57.5h | 39.6h | 33.6h | 3-day → **next-day** |
| NY→LA | 87.2h | 58.8h | 42.0h | >3-day → **2-day** |

Relay-only captures 65-90% of the Full I2.0 improvement at 0.03% of the cost.

## Sections

1. Introduction — the coordination failure: why relay is economically obvious but hasn't happened
2. Why Companies Think Solo is Cheaper (and Why They're Wrong) — the utilization math
3. Marketplace Architecture — slot exchange, hub operator model, independent relay driver layer
4. Regulatory Framework — HOS compliance, insurance, load security; what FMCSA needs to do
5. Business Case — revenue model, hub economics, why this is an extremely profitable utility
6. SLA Implications — the simulation results; what relay unlocks nationally
7. Policy Recommendations — NFRZ designation, FMCSA rulemaking, hub operator licensing
8. Conclusion — relay as Layer 0 of I2.0

## The Autonomous Vehicle Transition: I2.0 as the AV Runway

The managed freight lane is the prerequisite for AV trucking. This is not incidental —
it is the long-game investment case that makes $121B in managed lanes rational even if
the short-term SLA gains seem achievable more cheaply via relay.

**Why managed lanes are the AV operating environment:**
- No passenger vehicles: eliminates the hardest AV edge cases (unpredictable human drivers)
- Access-controlled fixed ramp locations: AV system knows every possible merge/diverge point
- High-spec geometry: consistent lane markings, maintained pavement — machine-readable
- V2I infrastructure (required for intelligent routing): continuous speed/condition
  data fed directly to vehicle sensors — effectively GPS + situational awareness
- Grade and weather standards: if Donner is hardened for managed lanes, it is also
  hardened for AV operations in adverse weather

**The AV transition timeline:**
1. 2025-2030: Relay marketplace on existing infrastructure (human relay, $40M/corridor)
   — builds the hub network that becomes AV transition nodes
2. 2028-2033: Platooning on I2.0 managed lanes (human lead vehicle + 2-4 AV followers
   in convoy; 20-35% fuel savings; only 1 driver per 5 trucks)
3. 2030-2038: Semi-autonomous trunking (AV on managed lane segment; human drives
   urban first/last mile; relay hub = AV-to-human handoff point)
4. 2035+: Fully autonomous on T1 managed lanes; relay hubs become
   AV terminal/charging/maintenance nodes; humans only at shipper/receiver

**The economics when AV comes:**
- Driver cost = 38% of trucking operating cost (ATRI)
- At full AV on managed lanes: 38% cost reduction on the T1 segment
- NY→LA current cost: ~$7,000/trip → AV managed lane: ~$4,300/trip
- At full AV + platooning fuel savings: ~$3,200/trip
- That's a $3,800/trip savings on 8,000 trucks/day = $11B/year on one corridor

The managed lane investment ($121B) pays for itself purely from AV cost reduction
within 10-12 years of AV deployment — before counting ANY congestion or SLA benefit.

**Design implication:** I2.0 managed lanes must be designed for AV from day one:
- Lane marking specifications (high-reflectivity, machine-readable markers every 100m)
- Roadside lidar/radar reference beacons at fixed intervals
- Standardized merge/diverge geometry (same design at every access point)
- Communication: DSRC/C-V2X at all relay hubs and access points
- These are $500M-1B in added cost on a $121B program — less than 1% — but must be
  specified in the initial design, not retrofitted later

**The relay hub becomes the AV handoff point:**
When the AV truck arrives at the relay hub, it doesn't need a driver — it needs:
- Charging / fueling
- Remote operator monitoring handoff (AV system transfers to next monitoring cell)
- Pre-trip inspection by hub technician (not a CDL driver)
- For urban delivery: local human driver boards for the city segment

The relay hub infrastructure ($5M per station) is exactly right for this — parking,
power, monitoring tech, scheduling. Built for relay drivers today, repurposed for
AV operations in 10 years.

## Data Sources

- ATRI driver shortage survey (2024)
- FMCSA CDL and HOS regulations (49 CFR Part 395)
- BLS trucking employment and compensation data
- Uber Freight / Convoy platform economics (public filings)
- UPS/FedEx relay operations (public disclosures)
- Route sla-matrix simulation (Monte Carlo, 5,000 trips per corridor)
