---
wave: t4-terminal-access-proof-source-capture
date_closed: 2026-05-14
status: done
---

# Close - T4 Terminal Access Proof Source Capture

## Decision

All 69 T4 terminal-access proof-intake rows now have source-capture
placeholders. Each row remains source-needed and not-reviewed; no manual or
cached non-seed artifact has been attached or accepted.

## Evidence

- `data/t4-terminal-access-proof-source-capture.csv` has 69 rows.
- Every row preserves `map;publication;upgrade`.
- Every row has `claim_blocker_delta = 0`, `source_artifact_reference =
  source-needed`, `capture_status = source-needed`, and
  `evidence_acceptance_status = not-reviewed`.
- Source artifact type is `manual-or-cached-terminal-access-proof`.

## Gate Record

- `cargo fmt -p route`
- `cargo test -p route`
- `route t4-terminal-access-proof-source-capture --gate`
- `route t4-terminal-access-proof-intake --gate`
- `route tier-optimize --all-tiers --gate`
- `route optimizer-manifest --gate`
- `route release-manifest --gate`
- `proof check docs\SPEC_INDEX.md waves\PHASES.md waves\2026-05-14-t4-terminal-access-proof-source-capture`
- `scripts\check-mileposts.ps1 -SkipTests`

## Next Action

Attach manual or cached non-seed terminal-access proof artifacts. No
terminal-access blocker should be reduced until proof is attached, reviewed, and
accepted.
