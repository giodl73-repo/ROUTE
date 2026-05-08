---
name: "Session Handoff: Simulation Pipeline Session"
slug: simulation-pipeline-session
type: plan
status: draft
author: human
created: 2026-05-07
updated: 2026-05-07
sources: []
---

# Session Handoff: Simulation Pipeline Session

**Date:** 2026-05-07
**Rubric version at session end:** v1.3
**Most recent corridor scored:** (see od-corridors.toml — all 10 corridors in file)
**Open tasks:** 17 remaining across 7 tracks

---

## Major Accomplishments This Session

### 1. Seasonal Simulation

Implemented and ran `route od --month 1..12` across all corridor configurations. Key finding: winter conditions double the Donner Pass closure probability. January p95 travel times:

- Solo/GP: 97 hours (Donner Pass closures drive the tail)
- Relay/I-2.0: 46 hours (Donner tunnel bypass eliminates the closure exposure)

The relay/I-2.0 advantage in January is not marginal — it is structural. The tunnel bypass removes the stochastic closure event entirely from the relay path, compressing the p95 by 51 hours relative to solo/GP. This is the clearest quantified argument for the Donner tunnel as a Phase 1 I-2.0 design target.

### 2. Rubric v1.3

ECH100 normalization applied to the D1 composite dimension. `scoring.toml` updated. The normalization corrects for the edge-case inflation that appeared when scoring high-volume urban corridors against rural freight corridors on the same raw ECH index. Rubric v1.3 is the active calibration version; all subsequent corridor scores should reference v1.3.

### 3. Mega-Map

Visualization updated with:

- T1 corridors: bold 4px line weight
- T2 corridors: 2.5px line weight
- Relay hub markers at Phase 1 diamond hub locations
- Upgrade candidates: dashed gold overlay

Maps #24 and #25 are the next visual priority (see open tasks).

### 4. Paper C.3: The 48-Hour Economy

Complete draft written. Target venue: Journal of Economic Perspectives. Estimated length: 8,500 to 9,500 words. The paper argues that compressing cross-continental freight transit from 4 to 6 days (current solo driver) to under 48 hours (relay/I-2.0) restructures inventory economics at the national level — enabling just-in-time logistics for corridors currently too long for JIT viability. Key empirical anchor: January NY-LA p95 of 46 hours under relay/I-2.0 vs. 97 hours solo/GP.

### 5. Paper F.3: Relay Marketplace

Complete draft written. Target venue: Management Science. The paper models the relay marketplace using the container shipping analogy: McLean's 1956 interface standardization as the structural template for the Relay-Compatible Cab Standard. Insurance framework (Mode 1 W-2, Mode 2 contractor), hub slot system (airport gate model), and AV runway (RDU as Phase 2 driver-AV interface) are the three substantive sections.

### 6. od-corridors.toml

All 10 corridors entered and validated. File is the authoritative corridor definition for OD simulation runs. Corridor list:

1. NY-LA (I-80/I-70)
2. Chicago-Dallas (I-55/I-35)
3. Atlanta-Memphis (I-22/I-78)
4. Seattle-Portland-SF (I-5)
5. Miami-Jacksonville (I-95)
6. Houston-El Paso (I-10)
7. Denver-Kansas City (I-70)
8. Minneapolis-Chicago (I-94)
9. Phoenix-Las Vegas (I-11 proposed)
10. Detroit-Cleveland-Pittsburgh (I-80/I-90)

### 7. RELAY Project Scaffolded

Project initialized at `C:\src\relay`. Directory structure, CLAUDE.md, and README.md in place. Three specification documents written this session: relay-cab-standard.md, insurance-framework.md, hub-slot-system.md. The RELAY project is the operational design complement to ROUTE's infrastructure analysis.

### 8. Hub Pipeline Model

Driver relay vs. trailer relay vs. cab pool operational flows defined. Key metric established: freight-ready to rolling in 35 minutes. This is the hub pipeline equivalent of airport gate-to-wheels-up time — the operational SLA that determines hub throughput capacity.

Current model pipeline steps (driver relay):
- Driver arrives at hub: 0 min
- Driver profile RFID load + RDU sign-on: 2 min
- ELD relay handoff: 3 min
- Simplified pre-trip (5 physical items, OBD pre-validated): 5 min
- Dock clearance + departure: 10 min (includes staging yard transit)
- Total: 35 minutes from freight-ready to rolling

Compare to current solo-driver model: 4 to 6 hours for rest-compliant handoff when the inbound driver has exhausted HOS.

### 9. Mail Service Connection

USPS spends approximately $3B per year on air contracts for overnight mail service — the primary mechanism for serving rural delivery points on tight schedules. An overnight relay network operating on I-2.0 corridors offers a surface alternative at lower cost per ton-mile, with comparable transit time on most corridors under 1,500 miles. This is not a displacement argument; it is a load consolidation argument: USPS mail volume on relay trucks reduces air contract spend and increases relay truck load factor simultaneously.

### 10. Task List

22 tasks created across 7 tracks. 5 tasks completed this session. 17 remain open.

---

## Key Simulation Numbers (Cite These)

All numbers from `route od --month 1..12` simulation against od-corridors.toml, rubric v1.3:

| Metric | Value | Source |
|---|---|---|
| January NY-LA solo/GP p95 | 97 hours | Seasonal simulation, Donner Pass closure model |
| January NY-LA relay/I-2.0 p95 | 46 hours | Seasonal simulation, tunnel bypass path |
| Donner Pass winter closure probability | 24.7% in January (vs. 10.3% annual) | Seasonal simulation, CalTrans closure history |
| Empty backhaul share (current) | 35% of truck miles at $0 revenue | FHWA VIUS, 2021 |
| Empty backhaul share (relay model) | ~20% of truck miles | Model projection, relay marketplace load matching |
| Empty backhaul efficiency gain | $135B/year at fleet scale | Derived from ATA revenue-per-mile data |
| Port Laredo annual delay cost | $10.8B/year | 16,000 trucks/day × 3 hours × $225/hour × 365 days |
| Hub pipeline: freight-ready to rolling | 35 minutes | Hub operations model |
| Current solo-driver handoff (HOS-constrained) | 4 to 6 hours | FMCSA HOS regulations, 49 CFR 395 |

---

## Open Tasks (17 Remaining)

**Most urgent — HPMS fetch for 22 states.** Rubric calibration for D1 (freight intensity) requires state-level HPMS data for 22 states not yet in the corpus. This is the single highest-leverage data task: without it, D1 scores for non-priority corridors are estimated, not measured.

**Next visual priority — Maps #24 and #25.** Map #24 is the seasonal simulation overlay (winter closure probability by corridor segment). Map #25 is the relay hub catchment model (which highway segments feed each Phase 1 hub within 100 miles).

**Papers in progress.** C.4 (inventory economics at the firm level, extends C.3 argument) and B.5 (hub siting optimization, covers the Atlanta capacity math in detail) are outlined but not drafted.

**Remaining 14 tasks** span rubric calibration (4 tasks), corpus entries (3 tasks), simulation validation (3 tasks), and design specification (4 tasks). Full task list in the relay marketplace project tracker.

---

## Next Session Starting Point

1. Pull HPMS data for the 22 remaining states (data pipeline task — approximately 4 hours of data work)
2. Score 3 additional existing corridors against rubric v1.3 to advance toward the 20-corridor calibration threshold
3. Draft map #24 (seasonal closure overlay)
4. Begin C.4 paper outline

Or: `go` / `1` to start with HPMS fetch.
