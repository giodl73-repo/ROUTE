# T1/T1 Intersection Vulnerability Analysis

## Scoring criteria
- **SPF risk**: Single point of failure — how many independent paths connect the two T1s within 50 miles?
- **Terrain**: whether geography constrains alternate paths
- **Urban density**: whether urban road network provides incidental distributed connectivity
- **Diamond status**: whether the 50-mile zone already has distributed connections or needs them built

SPF risk: HIGH = 1 path only; MED = 2 paths; LOW = 3+ paths (distributed)

## The 15 T1/T1 Intersections

| # | Intersection | Location | SPF Risk | Terrain | Urban net | Diamond needed? |
|---|---|---|---|---|---|---|
| 1 | I-80 × I-90 | Indiana/Ohio (shared 158 mi) | **CRITICAL** | Flat | Yes | No — they ARE the same road; at the split they diverge safely |
| 2 | I-35 × I-80 | Des Moines IA | **RECHECK** | Flat | Medium | Re-evaluate — prior Omaha/I-680 description was a location error |
| 3 | I-35 × I-40 | Oklahoma City | **HIGH** | Flat | Limited | Yes — OKC inner loop inadequate for T1 load |
| 4 | I-40 × I-75 | Knoxville TN | **HIGH** | Mountains | No | Yes — terrain limits alternate paths severely |
| 5 | I-10 × I-35 | San Antonio TX | **HIGH** | Flat | Limited | Yes — single spaghetti interchange, no 50-mile zone |
| 6 | I-75 × I-80 | Toledo OH | **HIGH** | Flat | Limited | Yes — I-475 bypass exists but at capacity |
| 7 | I-90 × I-95 | Boston MA | **HIGH** | Urban | Dense | Yes — Central Artery is single path, world's most expensive fix |
| 8 | I-10 × I-95 | Jacksonville FL | **HIGH** | Flat | Limited | Yes — I-295 is the only ring, single point |
| 9 | I-5 × I-10 | Los Angeles | **MED** | Urban | Very dense | Partial — 5 connectors exist but all converge on same node cluster |
| 10 | I-5 × I-80 | Sacramento/Bay Area | **MED** | Bay water | Dense | Yes — Bay crossing is single point, all paths use Bay Bridge or tunnel |
| 11 | I-5 × I-90 | Seattle/Tacoma | **MED** | Puget Sound | Dense | Yes — water constrains paths; SR-520 + I-90 are only 2 crossing points |
| 12 | I-35 × I-90 | Minneapolis MN | **MED** | Flat | Dense | Partial — Twin Cities ring provides 3 paths |
| 13 | I-40 × I-95 | Wilmington NC | **MED** | Flat | Limited | Yes — I-40 terminus forces all traffic through single interchange |
| 14 | I-75 × I-90 | Detroit metro | **LOW** | Flat | Very dense | No — Detroit metro has 4+ paths between I-75 and I-90 |
| 15 | I-5 × I-40 | Barstow CA | **LOW** | Desert | None | Low priority — rural, low volume, easy to detour via US-395 |

## Priority for Diamond Investment

### TIER A — Build now (SPF HIGH, high volume)
1. I-35 × I-80 (Des Moines) — recheck required; prior Omaha/I-680 rationale was a location error
2. I-40 × I-75 (Knoxville) — terrain-constrained, no alternate
3. I-10 × I-35 (San Antonio) — busiest SPF node by freight volume
4. I-90 × I-95 (Boston) — highest population exposure
5. I-10 × I-95 (Jacksonville) — Southeast gateway SPF

### TIER B — Build within 10 years
6. I-35 × I-40 (Oklahoma City)
7. I-75 × I-80 (Toledo)
8. I-5 × I-80 (Bay crossing)
9. I-5 × I-90 (Seattle/Puget Sound)

### Already adequate (LOW SPF)
- I-75 × I-90 (Detroit) — dense urban network, 4 paths
- I-5 × I-40 (Barstow) — low volume, easy desert detour
- I-80 × I-90 (Indiana/Ohio shared) — shared corridor, not an intersection

## The Diamond Design

### What a 50-mile diamond looks like: I-35 × I-80 at Des Moines

```
         [NW connector]                [NE connector]
              |                              |
  I-35 ------+------ I-35 ----------------  |
             25mi N of                       |
             Des Moines                      |
                                            |
              I-80 ============================= I-80
                        [DES MOINES CORE]
                        (current single interchange)
                                            |
  I-35 ------+------ I-35 ----------------  |
             25mi S of                       |
             Des Moines                      |
         [SW connector]                [SE connector]
```

**Northern connector** (~25 miles north):
- A new 10-mile connector from I-80 near Fremont NE westward to meet I-35 near Blair NE
- Allows I-80 eastbound traffic to reach I-35 northbound WITHOUT entering the Des Moines core
- Cost: ~$150M (new interchange + 10 miles connector)

**Southern connector** (~25 miles south):
- A new connector from I-80 near Greenwood NE southward to I-35 near Nebraska City/Plattsmouth
- Allows I-80 westbound traffic to reach I-35 southbound WITHOUT entering the Des Moines core
- Cost: ~$200M (two new interchanges + connector)

**Target result**: the Des Moines core interchange can close completely for construction or incident
and 80% of freight flow is maintained via the northern and southern connectors.

**Data correction (2026-05-09)**: earlier drafts incorrectly located the I-35 × I-80 T1/T1 junction at Omaha and cited I-680 as the relevant bypass. `route sim bind` confirmed that Omaha-area I-80/I-680/I-29 edges are present, but no I-35 edges exist near Omaha. The active T1/T1 fixture is now `des-moines-interchange`; the k-class and diamond design for this site require manual revalidation before publication use.

### What a 50-mile diamond looks like: I-40 × I-75 at Knoxville

This is the hard case — the Appalachian Mountains constrain alternate paths.

```
  I-75 (north)
      |
      | [existing interchange]
      |
  I-40 ========= [1 mile gap] ========= I-40
      |
  I-75 (south)
```

**Northern connector**: I-640 provides the existing Knoxville bypass but still rejoins the same metro corridor
**Southern/western connector**: I-140/Pellissippi Parkway and regional arterials provide partial relief, not a full T1-grade freight bypass
**What's needed**: validate a West Knoxville diamond concept using I-640, I-140, and truck-capable regional connectors before proposing any new build
- Creates a second or third operational transfer path between I-75 and I-40 without forcing all flow through the same West Knoxville merge
- Specifically valuable for I-75 to I-40 freight when the West Knoxville concurrency or split is constrained

## The Diamond Metric

For each T1/T1 intersection, define:
- **k-connectivity**: minimum number of edge-disjoint paths between the two corridors within 50-mile zone
- **Current**: most intersections have k=1 (single point of failure)
- **I2.0 target**: k ≥ 3 for all T1/T1 intersections
- **Intermediate target**: k ≥ 2 for all intersections within 10 years

## New Paper: B.4 — T1/T1 Intersection Resilience

This analysis warrants its own paper in the B track (Gap Analysis):
- **B.3** focuses on resilience holes in individual corridors (flood exposure, climate)
- **B.4** focuses on resilience at intersection nodes — the k-connectivity analysis

Key claims:
- C1: 9 of 15 T1/T1 intersections currently have k=1 (single point of failure)
- C2: Average annual cost of T1/T1 intersection incidents: $X billion (ATRI bottleneck data)
- C3: Diamond investment ($1.5–3B per priority intersection × 9 intersections = $15–27B total) has a lower NPV than full corridor managed-lane build while delivering equal or greater resilience
- C4: k=3 is achievable for all T1/T1 intersections within current highway footprint for $25B total
