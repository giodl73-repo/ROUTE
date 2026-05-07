---
name: Interstate 2.0 — Tier Standards v1.0
slug: tier-standards
type: spec
status: draft
rubric_version: v1.0
author: human
created: 2026-05-06
updated: 2026-05-06
sources:
  - "Interstate 2.0 design spec (specs/2026-05-06-interstate-2-design.md)"
  - "FHWA Highway Statistics 2023"
  - "FHWA Rest Area Guidelines"
  - "FHWA Alternative Fuels Corridors Program"
  - "ATRI Truck Parking Study 2023"
---

# Interstate 2.0 — Tier Standards

## §1. Purpose

Every interstate tier shall be an improvement over today's standards. The degree of improvement scales with the corridor's strategic tier. Tier 1 receives the full feature set at the highest specification. Tiers 2 and 3 receive scaled-down but still modernized standards. Tier 4 receives maintenance and safety only — no expansion, no new features.

**Design principle**: no corridor regresses. I2.0 sets a minimum floor for every tier that is better than the current system's actual performance. The floor rises as the tier rises.

---

## §2. Coverage Standards

The most fundamental standard: how close is anyone to the interstate system?

| Standard | Current (implied) | T1 target | T2+T3 combined target |
|---|---|---|---|
| % of US population within 30 miles of any interstate on-ramp | ~85% (estimated) | — | **99%** |
| % of US population within 30 miles of T1 or T2 | ~70% (estimated) | — | **95%** |
| % of agricultural land within 25 miles of T2+ on-ramp | ~60% (estimated) | — | **90%** |
| % of rural population within 20 miles of T2+ transit stop | ~20% (estimated) | — | **70%** |

**The 30-mile standard** is the anchor. It corresponds to:
- Healthcare: ~35-minute drive to an interstate on-ramp → ≤35 min from hospital corridor
- Evacuation: viable egress within 1 hour of emergency declaration
- Agricultural freight: within harvest-window truck range
- Employment: within viable daily commute range for rural workers

Communities currently >30 miles from any interstate on-ramp are the primary targets for Tier 3 rural connectivity spurs.

---

## §3. Tier 1 Standards (Primary Arteries — 8 corridors)

### 3.1 Throughput and Operations
| Standard | Specification |
|---|---|
| **PTI target** | ≤ 1.15 on express freight lanes; ≤ 1.30 on GP lanes |
| **Express freight lanes** | 2 dedicated lanes per direction, physically separated from GP |
| **Speed design** | 65 mph sustained for trucks; 70 mph GP design speed |
| **Lane configuration** | 4 express freight + 4 GP = 8 total maximum within existing ROW |
| **Platooning** | V2X communication at all interchanges; PCAs every 100 miles |
| **PrePass/WIM** | All T1 corridor weigh stations converted to WIM; PrePass mandatory |
| **Access control (freight)** | Express freight lane access at freight terminals and intermodal hubs only |
| **Pricing (freight lanes)** | Dynamic tolling; funds operations + rural cross-subsidy |

### 3.2 EV Charging
| Standard | Specification |
|---|---|
| **Passenger EV** | DC fast (≥150 kW), every **50 miles**, minimum 8 chargers per location |
| **Commercial truck EV** | ≥350 kW, at all express freight terminals; every 150 miles on express freight |
| **Reliability** | 98% uptime required; redundant connection required |
| **Coverage** | 100% of T1 corridor miles within 50 miles of qualifying station by 2030 |

### 3.3 Rest Areas
| Standard | Specification |
|---|---|
| **Spacing** | Every **100 miles** maximum on T1 corridors |
| **Truck parking** | Minimum **50 spaces** per facility (current average: 18) |
| **Services** | Full: food, showers, fuel, health clinic, wi-fi, EV charging |
| **Hours** | 24/7 staffing at all T1 facilities |
| **Reservation system** | Real-time truck parking availability broadcast via fleet management API |
| **Safety** | Lighting, security, emergency call stations |

### 3.4 Transit (T1/T1 Hubs)
| Standard | Specification |
|---|---|
| **Hub locations** | 9 T1/T1 diamond intersection cores |
| **Bus platforms** | Minimum 8 intercity bus platforms per hub |
| **Parking** | Minimum 2,000 spaces at T1/T1 hubs |
| **Rail connection** | Amtrak/commuter rail spur where rail alignment within 2 miles |
| **Amenities** | Full rest area + bus terminal facilities (food, restrooms, wi-fi, EV) |
| **Frequency** | Bus service minimum every 2 hours in each direction on T1 corridor |

### 3.5 Resilience
| Standard | Specification |
|---|---|
| **Diamond k-connectivity** | k ≥ 3 at all T1/T1 intersections (3 independent paths within 50-mile zone) |
| **Express freight flyovers** | At each T1/T1 diamond: **dedicated express freight flyover connections** (physically separated from GP lanes, no intermediate exits, access-controlled). These are distinct from the diamond connector roads. Connector roads handle general traffic redundancy; flyovers provide freight-only through-capacity that cannot be captured by local traffic growth. Connector roads fill with local traffic over time — this is expected and acceptable; flyovers must remain freight-dedicated permanently. |
| **Resilience spurs** | Emergency egress every **50 miles** on rural segments (connection to US highway network) |
| **Climate hardening** | All T1 segments in FEMA SFHA must achieve 500-year flood protection or elevated roadbed |
| **Recovery target** | T1 corridor closure: 80% throughput restored within 4 hours via alternates |
| **Intermodal spur** | One intermodal freight spur to Class I terminal per state traversed |

### 3.6 Connectivity
| Standard | Specification |
|---|---|
| **On-ramp spacing (urban)** | 2–5 miles |
| **On-ramp spacing (rural)** | 20–30 miles (resilience spurs fill the gap) |
| **C-D roads** | Required through all metros >500k population on T1 alignment |
| **Weight/clearance** | No load restrictions on T1 mainline; minimum 16' clearance |

---

## §4. Tier 2 Standards (Major Connectors — 25 corridors)

Tier 2 is the relief valve for Tier 1 and the backbone of regional connectivity. Standards are meaningful improvements over today — not the full T1 specification, but no longer treated as ordinary roads.

### 4.1 Throughput and Operations
| Standard | Specification |
|---|---|
| **PTI target** | ≤ 1.30 (reliable enough for regional freight scheduling) |
| **Freight accommodation** | No dedicated freight lanes, but truck-friendly design: consistent 12-ft lanes, no tight curves, grade ≤4% |
| **Speed design** | 65 mph design speed |
| **PrePass/WIM** | WIM at high-volume weigh stations; PrePass honored |
| **Capacity expansion** | Add GP lanes only where V/C > 0.90 at peak |

### 4.2 EV Charging
| Standard | Specification |
|---|---|
| **Passenger EV** | DC fast (≥100 kW), every **75 miles**, minimum 4 chargers |
| **Commercial truck EV** | ≥150 kW at fuel stops; no dedicated terminals required |
| **Coverage** | 100% of T2 corridor miles within 75 miles of qualifying station by 2032 |

### 4.3 Rest Areas
| Standard | Specification |
|---|---|
| **Spacing** | Every **150 miles** maximum |
| **Truck parking** | Minimum **20 spaces** per facility |
| **Services** | Enhanced: fuel, basic food, showers, EV charging, wi-fi |
| **Hours** | 18/7 minimum staffing (6am–midnight) |

### 4.4 Transit (T1/T2 Stops)
| Standard | Specification |
|---|---|
| **Stop locations** | All T1/T2 interchanges in communities >10,000 population |
| **Bus platforms** | Minimum 4 platforms |
| **Parking** | Minimum 500 spaces |
| **Service frequency** | Bus minimum every 4 hours per direction |
| **Regional connections** | County bus or demand-responsive connection to nearest community |

### 4.5 Resilience
| Standard | Specification |
|---|---|
| **Resilience spurs** | Emergency egress every **75 miles** on rural segments |
| **Climate hardening** | T2 segments in FEMA SFHA: elevated signage and flood-resistant barriers at minimum |
| **Recovery target** | T2 closure: alternate routing within 60 miles available; no recovery time guarantee |
| **Bridge condition** | All T2 bridges rated fair or better by 2035 |

### 4.6 Connectivity
| Standard | Specification |
|---|---|
| **On-ramp spacing (urban)** | 3–5 miles |
| **On-ramp spacing (rural)** | 15–25 miles |
| **Coverage role** | T2 + T3 combined: all communities within 30 miles of an on-ramp |
| **Weight/clearance** | No load restrictions on T2 mainline; minimum 14'6" clearance |

---

## §5. Tier 3 Standards (Regional Feeders — 61 corridors)

Tier 3 fills coverage gaps and connects rural communities to the national network. Standards focus on access, basic freight accommodation, and the minimum transit connection.

### 5.1 Throughput and Operations
| Standard | Specification |
|---|---|
| **PTI target** | ≤ 1.50 (functional reliability; no guarantee) |
| **Freight accommodation** | Standard lanes; no load restrictions on mainline unless bridge-specific |
| **Speed design** | 65 mph design speed; 55 mph acceptable on rural mountainous segments |
| **Capacity** | No expansion except safety-critical widening; maintain existing configuration |

### 5.2 EV Charging
| Standard | Specification |
|---|---|
| **Passenger EV** | DC fast (≥50 kW), every **100 miles**, minimum 2 chargers |
| **Truck EV** | Level 2 (≥19 kW) at rest areas; no truck-specific DC fast required |
| **Coverage** | 100% of T3 corridor miles within 100 miles of any qualifying station by 2035 |
| **Gap fill** | Where no station possible within 100 miles: state highway EV corridor program |

### 5.3 Rest Areas
| Standard | Specification |
|---|---|
| **Spacing** | Every **200 miles** maximum |
| **Truck parking** | Minimum **10 spaces** |
| **Services** | Basic: restrooms, vending, parking, basic EV (Level 2), wi-fi |
| **Hours** | Unstaffed with security lighting acceptable |

### 5.4 Transit (T2/T3 Access Nodes)
| Standard | Specification |
|---|---|
| **Node locations** | All T2/T3 interchanges; all rural communities >5,000 population within 5 miles of T3 |
| **Facility** | Covered shelter with lighting, real-time bus info display, bike parking |
| **Parking** | Minimum 50–100 spaces |
| **Transit connection** | Demand-responsive transit connection to nearest T2 stop; minimum 2 round trips/day |

### 5.5 Resilience
| Standard | Specification |
|---|---|
| **Resilience spurs** | Emergency egress every **100 miles** or connection to US highway network |
| **Climate hardening** | Safety-critical bridges only; drainage improvements in SFHA zones |
| **Recovery** | No throughput guarantee; safe alternative routing within 100 miles |
| **Bridge condition** | All T3 bridges rated fair or better by 2040 |

### 5.6 Connectivity — Rural Access Spurs
| Standard | Specification |
|---|---|
| **Coverage trigger** | Any rural community >5,000 population that is >30 miles from any T1/T2/T3 on-ramp |
| **Spur spec** | ≤10 miles, 2-lane limited access, connects to T3 mainline |
| **Target** | All communities >5,000 meeting trigger condition: spur by 2035 |
| **On-ramp spacing (rural)** | 10–20 miles on T3 in high-density agricultural zones |
| **Weight** | Weight restrictions only on posted bridges; not corridor-wide |

---

## §6. Tier 4 Standards (Local Access — 133 corridors)

Tier 4 is maintenance and safety. No new features. No expansion. The goal is to prevent degradation, not to add capability.

| Standard | Specification |
|---|---|
| **Pavement** | IRI ≤ 170 (poor → fair threshold); all T4 segments at fair or better by 2040 |
| **Bridge** | All T4 bridges rated fair or better by 2045 |
| **Safety** | Standard signing, guardrails, lighting at interchanges; rumble strips |
| **EV** | No new requirement; preserve existing rest area sites for future charging |
| **Transit** | No new requirement; rural spur program may extend to T4 where coverage gaps exist |
| **Weight** | Posted restrictions where applicable; no corridor-wide restrictions |
| **Capacity** | No expansion; safety widening only where LOS F creates documented hazard |

---

## §7. Feature Matrix Summary

| Feature | T1 | T2 | T3 | T4 |
|---|---|---|---|---|
| Express freight lanes | ✓ (2 per dir, separated) | ✗ | ✗ | ✗ |
| PTI target | ≤ 1.15 | ≤ 1.30 | ≤ 1.50 | None |
| EV charging spacing | 50 miles (≥150kW) | 75 miles (≥100kW) | 100 miles (≥50kW) | None required |
| Truck EV | ≥350kW at terminals | ≥150kW at fuel stops | Level 2 at rest areas | None |
| Rest area spacing | 100 miles (full service) | 150 miles (enhanced) | 200 miles (basic) | Preserve existing |
| Truck parking (min) | 50 spaces | 20 spaces | 10 spaces | Existing |
| Transit facility | Full hub (T1/T1) | Regional stop | Access node | None |
| Bus frequency | ≤ 2 hr headways | ≤ 4 hr headways | Demand-responsive | None |
| Resilience spur | Every 50 mi | Every 75 mi | Every 100 mi | None |
| Diamond k-connectivity | k ≥ 3 | k ≥ 2 (T2/T2 nodes) | None | None |
| Climate hardening | SFHA full protection | SFHA signage + barriers | Safety-critical bridges | None |
| Intermodal spur | 1 per state traversed | Within 30 mi of rail | None required | None |
| C-D roads | Metros >500k | None required | None required | None |
| Bridge condition target | Fair by 2030 | Fair by 2035 | Fair by 2040 | Fair by 2045 |
| Coverage role | Not primary | Not primary | Gap fill + access | Local only |

---

## §8. Investment Phasing by Feature

Not all standards are achievable simultaneously. Phasing by feature across all tiers:

| Phase | Years | Features | Cost estimate |
|---|---|---|---|
| **Phase 1** | 1–5 | EV charging all tiers (fastest deployment), bridge condition assessments, rest area EV retrofit, transit hub planning | $15B |
| **Phase 2** | 5–10 | T1 managed freight lanes (highest-urgency segments), T2 rest area upgrades, T2 transit stops, diamond intersections, resilience spurs all T1 | $60B |
| **Phase 3** | 10–20 | Remaining T1 managed lanes, T2 EV full coverage, T3 EV full coverage, rural access spurs for coverage gaps | $80B |
| **Phase 4** | 20–30 | New T1 corridors (Northern Tier, I-69), remaining T2 capacity improvements, T3/T4 bridge condition completion | $54B |
| **Total** | | | **~$209B** |

---

## §9. The 30-Mile Coverage Standard — Measurement

The 30-mile coverage standard is the binding constraint that determines where T3 and rural access spurs are needed.

**Computation** (implemented in `route coverage`):
1. Grid the continental US at 10-mile resolution
2. For each grid cell, find nearest interchange node in the HighwayGraph
3. Compute Haversine distance to nearest on-ramp
4. Population-weight by Census tract overlap
5. Report: % within 20/30/50 miles by tier, by state, by rural/urban classification

**Current estimate** (pre-computation): approximately 8–12% of rural US population lives >30 miles from any interstate on-ramp. In absolute terms: 3–5 million rural Americans. These are the primary beneficiaries of the T3 rural access spur program.

---

## §10. Amendment Protocol

Changes to tier standards that affect CLI computation (coverage thresholds, EV spacing, rest area spacing) require updating both this spec and the `config/tier-standards.toml` runtime configuration.

| Date | Amendment | Reason |
|---|---|---|
| 2026-05-06 | Initial tier standards | First comprehensive cross-tier specification |
