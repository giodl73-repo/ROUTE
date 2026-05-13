# Pulse 02 - Great Lakes Contact Sample Review

## Decision

The 33 Great Lakes / Ohio Valley terminal-contact rows are classified but not
promoted. Candidate terminal districts are named from
`data/intermodal_terminals.csv`; no row has route-to-terminal contact proof.

## Findings

| Severity | Artifact | Consequence | Concrete fix |
|---|---|---|---|
| WARN | `data/t4-terminal-contact-evidence.csv` | Candidate districts are useful triage seeds, but they are not operational access proof. | Keep all Great Lakes rows `source-needed` until a separate contact proof source names route, terminal district, contact basis, and higher-tier attachment. |
| WARN | `data/t4-terminal-contact-evidence.csv` | No source-backed row exists, so a scenario would be unsupported. | Do not create a scenario artifact in Pulse 02; carry scenario readiness to Pulse 04 only if later rows earn source-backed status. |
| NOTE | `waves/2026-05-13-t4-terminal-contact-evidence/plans/pulse-03.md` | Remaining zones should use the same seed-vs-proof distinction. | Apply the Great Lakes classification pattern to Southeast, Mid-South, Mountain West, and Texas Border rows without widening scope. |

## Great Lakes Classification Summary

| Candidate district | Rows |
|---|---:|
| Chicago Intermodal Complex | 4 |
| Columbus South | 8 |
| Detroit Livernois | 5 |
| Indianapolis Avon | 3 |
| Minneapolis Twin Cities | 1 |
| New York Fresh Pond | 6 |
| Philadelphia Frankford | 3 |
| St. Louis Gateway | 3 |
