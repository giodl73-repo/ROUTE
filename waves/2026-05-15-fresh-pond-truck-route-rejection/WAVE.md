---
wave: fresh-pond-truck-route-rejection
date_open: 2026-05-15
status: done
---

# Fresh Pond Truck Route Rejection

## Mission

Resolve the six remaining New York Fresh Pond T4 terminal-access upgrade holds
only if an official public truck-route source supports rejecting the held routes
as terminal access proof.

## Opening Rule

Do not infer Fresh Pond terminal proof from the seed terminal district or from
general proximity. Rejection requires a public source that names the legal truck
route network serving the Fresh Pond area and contradicts the held route list.

## Inputs Inherited

- `data/t4-terminal-contact-evidence.csv`
- `data/t4-terminal-contact-rejected-proof-sources.csv`
- `data/optimizer-constraint-ledger.csv`
- `data/optimizer-residual-blocker-backlog.csv`
- [NYC DOT Truck Routing Network](https://www.nyc.gov/html/dot/html/motorist/truckrouting.shtml)
- [NYC DOT New York City Truck Routes dataset](https://data.cityofnewyork.us/resource/jjja-shxy.json)

## Pulse Status

| Pulse | Status | Output |
|---|---|---|
| Fresh Pond truck-route rejection | done | `data/t4-terminal-contact-rejected-proof-sources.csv`; optimizer replay |

## Done Criteria

- NYC DOT source rows identify Fresh Pond Road and Metropolitan Avenue as
  Queens local truck routes and Long Island Expressway / Brooklyn Queens
  Expressway as through truck routes.
- The six held Fresh Pond routes are rejected as unsupported by that terminal
  access source.
- T4 terminal-access upgrade blockers fall from six to zero.
- Optimizer, publication, manifest, proof, and milepost gates pass.

## Non-goals

- Do not promote any Fresh Pond route as accepted positive terminal-contact
  proof.
- Do not change T2 pavement repair debt or T1 snapshot evidence holds.
