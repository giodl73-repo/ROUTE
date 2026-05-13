# R1 - Optimization Methodologist

## Verdict

No `BLOCK` finding. The wave has a deterministic pulse order and preserves
claim blockers instead of deleting unresolved rows.

## Findings

| Severity | Artifact | Consequence | Concrete fix |
|---|---|---|---|
| WARN | `waves/2026-05-13-t4-terminal-contact-evidence/plans/pulse-01.md` | If decision states are implicit, downstream ledger and scenario logic can promote rows by convention rather than rule. | Pulse 01 must enumerate allowed decisions and legal status transitions for source-needed, source-backed, demotion/local-only, held-known, and scenario-ready rows. |
| WARN | `waves/2026-05-13-t4-terminal-contact-evidence/plans/pulse-05.md` | A new contact queue could influence selectors before manifests expose it as a governed source artifact. | Pulse 05 must register the queue in optimizer/release manifests before any scenario candidate is release-facing. |
