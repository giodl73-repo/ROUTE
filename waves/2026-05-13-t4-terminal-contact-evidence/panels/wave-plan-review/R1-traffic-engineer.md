# R1 - Traffic Engineer

## Verdict

No `BLOCK` finding. The wave appropriately refuses to use terminal proximity as
proof of operational access.

## Findings

| Severity | Artifact | Consequence | Concrete fix |
|---|---|---|---|
| WARN | `waves/2026-05-13-t4-terminal-contact-evidence/plans/pulse-01.md` | A route may be near a terminal district without a usable truck contact, interchange path, gate connection, or local connector. | Pulse 01 must gate `contact_basis` as an operational proof field, not a free-text proximity note. |
| NOTE | `waves/2026-05-13-t4-terminal-contact-evidence/plans/pulse-02.md` | The Great Lakes sample can validate the contact basis vocabulary against dense terminal geography. | Use Pulse 02 to test whether the schema distinguishes direct contact, connector-needed, and source-needed rows. |
