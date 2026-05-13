---
wave: maps-became-contracts
date_open: 2026-05-07
date_close: 2026-05-11
status: done
source: git-history
---

# Maps Became Contracts

## Mission

Turn ROUTE maps from persuasive images into gated artifacts with map ids,
render commands, release ownership, and game reuse contracts.

## Commit-Derived Pulses

| Pulse | Status | Evidence |
|---|---|---|
| 01 - Real renderer and metro-quality redesign | done | `e7a7378`, `b0ee411`, `13154ee` |
| 02 - Map atlas and game hook | done | `74d257f`, `484e330`, `fbebb88` |
| 03 - T2 service map joins atlas/release | done | `137adac`, `5285a0e`, `d4f6b30`, `5fee436`, `90fa940` |

## Close Evidence

Map artifacts now have ids and gates in `data/map-atlas.csv`; game/campaign
artifacts point to maps rather than screenshots.
