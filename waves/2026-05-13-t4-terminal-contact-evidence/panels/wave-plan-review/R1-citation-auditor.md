# R1 - Citation Auditor

## Verdict

No `BLOCK` finding. The wave may start because it does not validate terminal
claims yet; it creates the source contract that will decide them.

## Findings

| Severity | Artifact | Consequence | Concrete fix |
|---|---|---|---|
| WARN | `waves/2026-05-13-t4-terminal-contact-evidence/plans/pulse-01.md` | `data/intermodal_terminals.csv` could be misread as route-contact evidence instead of a terminal-district seed. | Pulse 01 must split district seed source fields from route-to-terminal contact-proof source fields and gate the difference. |
| NOTE | `waves/2026-05-13-t4-terminal-contact-evidence/plans/pulse-04.md` | Scenario candidates will need traceable source rows before any claim can leave held status. | Require each scenario-ready row to name the contact proof source, not just the terminal district. |
