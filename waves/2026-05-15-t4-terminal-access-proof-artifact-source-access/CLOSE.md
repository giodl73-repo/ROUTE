---
wave: t4-terminal-access-proof-artifact-source-access
date_closed: 2026-05-15
status: done
---

# T4 Terminal Access Proof Artifact Source Access Close

## Decision

The 69 T4 terminal-access proof artifact acquisition targets now have explicit
source-access policy rows. Each row requires manual or cached non-seed proof;
live fetch is not enabled, evidence remains source-needed, and no map,
publication, or upgrade blocker is reduced.

## Evidence

| Rows | Access mode | Cache status | Evidence | Proof status | Blocked claims | Delta |
|---:|---|---|---|---|---|---:|
| 69 | manual-or-cached-source-needed | not-cached | source-needed | not-accepted | map;publication;upgrade | 0 |

## Gate Bundle

- `cargo fmt -p route`
- `cargo test -p route`
- `cargo run -q -p route -- t4-terminal-access-proof-artifact-acquisition-targets --gate`
- `cargo run -q -p route -- t4-terminal-access-proof-artifact-source-access --gate`
- `cargo run -q -p route -- tier-optimize --all-tiers --gate`
- `cargo run -q -p route -- optimizer-constraint-ledger --gate`
- `cargo run -q -p route -- optimizer-constraint-budget --gate`
- `cargo run -q -p route -- optimizer-manifest --gate`
- `cargo run -q -p route -- release-manifest --gate`
- `C:\src\target\debug\proof check docs\SPEC_INDEX.md docs\optimizer-constraint-ledger-spec.md waves\PHASES.md waves\2026-05-15-t4-terminal-access-proof-artifact-source-access`
- `powershell -ExecutionPolicy Bypass -File scripts\check-mileposts.ps1 -SkipTests`

## Residual Holds

- The map-validity blocker remains the 69-row T4 terminal-access evidence gap.
- The next effective map-validity step is to capture and attach non-seed
  route-to-terminal proof artifacts, then review, accept, and replay those rows.
