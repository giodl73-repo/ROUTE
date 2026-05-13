# R1 Consolidated - Wave Plan Review

## Decision

Proceed to Pulse 01. No reviewer raised a `BLOCK` finding against starting the
T4 Terminal Contact Evidence wave.

## Consolidated Findings

| Severity | Artifact | Consequence | Concrete fix |
|---|---|---|---|
| WARN | `waves/2026-05-13-t4-terminal-contact-evidence/plans/pulse-01.md` | Terminal district seed sources could be mistaken for route-to-terminal contact proof. | Pulse 01 now requires separate district seed and contact-proof source fields. |
| WARN | `waves/2026-05-13-t4-terminal-contact-evidence/plans/pulse-01.md` | Implicit decision states could allow unsupported scenario promotion. | Pulse 01 now requires enumerated decision states, legal status transitions, and tests for proximity-only held rows. |
| WARN | `waves/2026-05-13-t4-terminal-contact-evidence/plans/pulse-04.md` | Scenario readiness could drift into scenario implementation or economic claims. | Pulse 04 now requires contact proof, operational contact basis, higher-tier attachment, and freight/access rationale while keeping implementation out of scope. |
| WARN | `waves/2026-05-13-t4-terminal-contact-evidence/plans/pulse-05.md` | A new contact queue could affect release surfaces without visible manifest ownership. | Pulse 05 now requires manifest propagation before any scenario candidate is release-facing. |

## Start Condition

Pulse 01 may start after this review lands. The top gate is the terminal contact
evidence schema gate: create the queue contract, prove source-needed versus
scenario-ready behavior, and preserve unresolved terminal claims in the ledger.
