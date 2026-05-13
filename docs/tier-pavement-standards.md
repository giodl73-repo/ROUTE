# Tier Pavement Standards

## Purpose

ROUTE service promises cannot rely on rough roads. A T1 or T2 line that meets a
map or SLA topology test but forces trucks, buses, coaches, emergency vehicles,
or freight-sensitive cargo to slow down is not actually meeting the service
standard.

`data/tier-pavement-standards.csv` defines the pavement and ride-quality floor
for each tier. The command contract is:

```text
route standards-pavement --gate
```

## Rule

Pavement is a service constraint, not a decoration:

1. Segment candidates carry physical edge identity.
2. Pavement evidence attaches to those segment ids.
3. A bundle cannot claim an SLA-ready or transit-ready service if member
   segments violate the tier pavement floor.
4. Bad pavement creates a repair task before promotion, upgrade, or publication
   proof.

## Tier Floors

The current planning thresholds use International Roughness Index in meters per
kilometer:

| Tier | Max IRI | Meaning |
|---|---:|---|
| T1 | 1.50 | National promise spine needs good ride quality for timed freight and intercity coach service. |
| T2 | 1.90 | Regional connectors must not add padding to 24h/12h promise windows. |
| T3 | 2.40 | Regional feeders must remain fair for ordinary truck and transit access. |
| T4 | 2.70 | Terminal and local access cannot remain poor where freight or passenger access depends on it. |

These are initial planning gates. Corridor-specific engineering can tighten
them, but it cannot loosen them silently.

## Evidence Path

The first evidence source is HPMS IRI where joined to graph edges. State pavement
condition feeds should replace or supplement HPMS when they provide better
segment resolution.

The next implementation step is to join pavement evidence to
`data/tier-segment-candidates.csv`, then emit a pavement repair docket for
candidate bundle members that violate their tier floor.
