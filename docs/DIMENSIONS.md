# ROUTE Dimension Registry

This is the public registry for the v1.4 16-dimension scoring instrument. It is checked by `route-score` tests so the docs cannot silently drift from the Rust enum.

Status vocabulary follows `docs/SPEC_INDEX.md`:

| Status | Meaning |
|---|---|
| Implemented | Direct source path is wired and the scorer can use it |
| Heuristic | Source path is partial, estimated, or uses a proxy |
| Stub | Interface exists but no real data path is wired |

## Registry

| Code | Band | Name | Primary Evidence | Current Truth Label |
|---|---|---|---|---|
| A1 | Flow | Throughput Gap | HPMS AADT, scored from corridor p90 segment demand | Implemented |
| A2 | Flow | Freight Intensity | FAF5 target field or HPMS truck-volume cargo-value proxy | Heuristic |
| A3 | Flow | Speed Reliability | FPM/NPMRDS PTI when available; BPR PTI fallback; IRI last resort | Heuristic |
| A4 | Flow | International Trade Corridor | USMCA and border-corridor designation scoring | Heuristic |
| A5 | Flow | Safety Record | FARS fatal crash rate joined by route | Implemented |
| B1 | Network | Redundancy | Detour penalty, nearest parallel route, rail-parallel discount | Heuristic |
| B2 | Network | Network Centrality | Graph edge betweenness aggregated to corridor p90 | Implemented |
| B3 | Network | Port/Border Access | Port and border proximity joins | Implemented |
| B4 | Network | Military/Strategic | STRAHNET and installation-proximity designation scoring | Heuristic |
| C1 | People | Population Reach | ACS county population within corridor buffer | Implemented |
| C2 | People | Rural Connectivity | ACS/RUCC rural share plus rural interchange gap | Heuristic |
| C3 | People | Economic Opportunity Access | ACS income relative to national median; descriptive only | Implemented |
| C4 | People | Agricultural Export Access | Agricultural/export-corridor designation scoring | Heuristic |
| D1 | Future | Climate Resilience | FEMA SFHA route-edge join plus hazard-zone composite | Heuristic |
| D2 | Future | Multimodal Integration | DCFC density plus intermodal terminal count | Implemented |
| D3 | Future | Infrastructure Vintage | NBI bridge condition when cached; IRI proxy fallback | Heuristic |

## Milestone 1 Notes

- `route calibrate` writes `data/confidence-risks.csv` for corridor-level review targets.
- `route calibrate` writes `data/confidence-risk-summary.csv` for dimension-level total and tier-sensitive review risk.
- A2, A3, and D1 remain the highest-priority source-quality gaps for score confidence.
- C3 is explicitly descriptive, not causal. Use it to identify co-location of economic opportunity and corridor access, not to claim that a corridor caused the observed condition.
