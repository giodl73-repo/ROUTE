# Donner Weather Closure Amendment Log

This log is forward-only. Old playtests keep the rules they used; new playtests use the latest accepted amendment version.

Current scenario version: G0 v0.2  
Prior version: G0 v0.1  
Source evidence: `docs/game/donner-weather-closure-playtest-001.md`

## Amendment Protocol

| Status | Meaning |
|---|---|
| Candidate | A playtest finding worth considering |
| Accepted | Rule/copy/card change applies to future playtests |
| Held | Finding needs another occurrence before changing rules |
| Declined | Finding is understood but not adopted |

Two repeated findings create an amendment candidate. A high-severity single finding can be accepted when it blocks playability or evidence honesty.

## v0.2 Accepted Amendments

| ID | Source finding | Decision | Rule change |
|---|---|---|---|
| DW-A001 | Early egress was easy to skip because trapped queues were not explicit | Accepted | Whiteout closure adds a trapped-queue marker unless early egress or dynamic closure routing is ready |
| DW-A002 | Managed freight tunnel looked like an immediate answer despite long build time | Accepted | Tunnel copy and project notes frame it as a powerful long-term SLA project, not the first-storm fix |
| DW-A003 | Source request worked, but validated evidence rejection needed clearer copy | Accepted | Add "source requested is not source observed" language to evidence rules and publication-lock copy |
| DW-A004 | Players may ask why Donner uses an 8-hour recovery window instead of Des Moines' 4-hour window | Accepted | Add a scenario-specific recovery-window note explaining pass reopening and queue-drain timing |

## v0.2 Rule Changes

### Trapped Queue Marker

When `Whiteout closure` resolves, add a trapped-queue marker if neither of these is complete or active:

- Early egress spurs
- Dynamic closure routing

The marker means freight reached the closure zone before the system could meter or redirect it. It reduces throughput scoring until the player builds egress, routing, bypass, tunnel, or intermodal capacity.

### Long-Term Tunnel Framing

The managed freight tunnel remains a strong project, but it is not an instant response to the first forced storm.

Player-facing copy should say:

- "The tunnel protects priority freight after it opens."
- "It does not stop the first storm from creating a queue."
- "Pair it with egress, routing, operations, or intermodal capacity if the first winter matters."

### Source Requested Is Not Source Observed

`Source request` names the missing evidence path and raises evidence confidence. It does not unlock publication.

`Validated weather evidence` remains unavailable until an observed closure-history artifact and alternate-capacity validation exist. When rejected, the table should say: "source requested is not source observed."

### Recovery-Window Note

Donner uses an 8-hour tutorial recovery window because the scenario models a pass reopening plus queue-drain problem after a 48-hour weather closure. Des Moines uses a 4-hour transfer-recovery window because that scenario tests an interchange-zone disruption. Both are heuristic teaching thresholds until direct PTI/SLA evidence is attached.

## v0.2 Expected Effects

| Risk | Expected effect |
|---|---|
| Players skip early egress | Reduced; trapped queue is visible and scored |
| Tunnel becomes a disguised correct answer | Reduced; long-term timing is explicit |
| Evidence feels arbitrary | Reduced; source request and observed proof are separated |
| Recovery threshold feels inconsistent | Reduced; scenario-specific recovery window is named |

## Held Findings

None yet.

## Declined Findings

None yet.
