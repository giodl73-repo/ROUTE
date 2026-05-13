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
route tier-pavement-docket --gate
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

HPMS source/cache values may arrive in inches per mile. The pavement docket
normalizes graph IRI values above `20` as inches-per-mile source values into
meters per kilometer before applying tier floors, while leaving already-normal
meters-per-kilometer values unchanged.

`data/tier-pavement-docket.csv` joins `data/tier-segment-candidates.csv` to the
graph edge IRI evidence and emits a segment-level status:

| Status | Meaning |
|---|---|
| `pavement-floor-pass` | The member segment has IRI evidence at or below its tier floor. |
| `pavement-repair-required` | The member segment has IRI evidence above its tier floor and must be repaired before SLA/transit readiness is claimed. |
| `pavement-source-needed` | The member segment is selected, but pavement evidence is not joined yet. |
| `missing-tier-standard` | The selected tier has no pavement threshold row. |
| `missing-graph-edge` | The candidate references an edge id that is no longer present in the graph. |

The docket gate requires complete row contracts and known statuses. It does not
fail merely because a row is a repair/source blocker; those rows are the point of
the docket and must remain visible to the optimizer.
