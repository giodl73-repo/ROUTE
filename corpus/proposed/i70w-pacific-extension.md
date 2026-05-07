---
name: "I-70 West — Pacific Extension"
slug: i70w-pacific-extension
type: proposed-corridor
status: draft
rubric_version: v1.2
author: route-score
created: 2026-05-07
updated: 2026-05-07
sources:
  - "ROUTE v1.2 scoring (US-50 proxy)"
corridor:
  termini: ["Cove Fort UT (current I-70 western terminus, junction I-15)", "San Francisco/Oakland CA Bay Area"]
  states: [UT, NV, CA]
  approx_miles: 700
  designation: "I-70 West (extension of existing I-70)"
  classification: proposed
  proposed_by: ROUTE-analysis
  alignment_basis: US-50 — Ely NV → Fallon NV → Sacramento CA
---

# I-70 West — Pacific Extension

## Overview

I-70 runs 2,151 miles from Baltimore MD to Cove Fort UT — making it the only major east-west transcontinental interstate that does NOT reach the Pacific Ocean. Every other E-W transcontinental (I-10, I-40, I-80, I-90) is coast-to-coast. I-70 ends at a T-intersection with I-15 in rural Utah, relying on I-15 south to I-10 or north to I-84/I-90 to reach the coast.

The US-50 alignment (Cove Fort UT → Ely NV → Fallon NV → Sacramento CA) provides a direct connection. At ~700 miles, this is the shortest possible Pacific extension. Designated as **I-70 West** (or simply extending I-70's mileage westward), it would complete I-70 as a true Baltimore-to-Bay-Area coast-to-coast interstate.

**Why no new number**: I-70 is already the corridor. Extending it west doesn't need a new identity — it completes an existing concept. Like I-40 absorbed US-66, I-70W would absorb US-50 on its western extension.

## The Gap This Fills

Between I-80 (northern, SLC → Reno → Bay Area) and I-40 (southern, Barstow → Flagstaff → Albuquerque), there is a 500-mile band with no east-west transcontinental. Denver-to-Bay-Area freight currently requires:
- North route: I-70 to SLC → I-80 west → Bay Area (adds 150 miles via SLC)
- South route: I-70 to SLC → I-15 south → I-40 west → LA (adds 400 miles)
- I-70 West direct: Cove Fort UT → Ely NV → Sacramento CA (700 miles, direct)

## The Central Nevada Concern

The US-50 corridor through Nevada is famously called "the Loneliest Road in America." Ely NV has ~4,000 people; Fallon NV ~10,000. This is not a high-traffic corridor — it's a geographic necessity.

However: the B1 score of 8.3 for US-50 confirms the strategic logic. The corridor has no viable parallel. There's 500 miles of desert with no east-west alternative between I-80 (north) and I-40 (south). Building I-70W isn't about serving current demand — it's about having a backup transcontinental route when I-80 or I-40 has a major closure.

## Dimension Scores (estimated, US-50 alignment)

| Band | Dim | Score | Note |
|---|---|---|---|
| A | A1 | 0.5 | Currently very low volume — "Loneliest Road" |
| A | A2 | 1.5 | Minimal current freight |
| A | A3 | 4.0 | No congestion; weather-limited in winter |
| A | A4 | 2.0 | No border crossings; California fruit possible |
| B | B1 | **8.3** | No parallel E-W interstate within 250 miles (North: I-80; South: I-40) |
| B | B2 | 5.5 | Would add transcontinental redundancy centrality |
| B | B3 | 5.0 | Port of Oakland/Sacramento access (western terminus) |
| B | B4 | 4.0 | Sierra Army Depot (Herlong CA), Hawthorne Army Depot (NV) |
| C | C1 | 1.5 | Very sparse Nevada desert population |
| C | C2 | 6.0 | Rural Nevada/Utah communities with no interstate access |
| C | C3 | 7.0 | Below-national-average GDP in rural NV |
| C | C4 | 5.0 | California Central Valley agriculture (western end) |
| D | D1 | 2.0 | Low flood/wildfire; high heat; winter mountain passes |
| D | D2 | 2.0 | No rail parallel through central Nevada |
| D | D3 | 0.0 | New construction |
| **Total** | | **~54/150** | **Borderline T1/T2 — strategic value is B1 resilience** |

## Investment Priority

I-70W scores lower than I-31/I-29S/I-92/I-11 because it serves sparse population and carries minimal current traffic. The investment case is **pure resilience** — a backup transcontinental when I-80 or I-40 is closed. At ~$21B (700 miles × $30M/mi upgrade), it's the cheapest transcontinental completion. The NPV case depends heavily on how frequently I-80 or I-40 would be the "diverted route" in a major incident scenario.

**Route sim scenario**: `route sim --scenario i80-multi-day-closure --intervention i70w` would quantify this.
