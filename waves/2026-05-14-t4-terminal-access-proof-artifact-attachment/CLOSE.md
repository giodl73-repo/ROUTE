---
wave: t4-terminal-access-proof-artifact-attachment
date_closed: 2026-05-14
status: done
---

# Close - T4 Terminal Access Proof Artifact Attachment

## Decision

All 69 T4 terminal-access source-capture rows now have artifact-attachment
placeholders. Each row remains source-needed, not-reviewed, and not-accepted;
no manual or cached non-seed artifact has been attached or accepted.

## Evidence

- `data/t4-terminal-access-proof-artifact-attachment.csv` has 69 rows.
- Every row preserves `map;publication;upgrade`.
- Every row has `claim_blocker_delta = 0`, `source_artifact_reference =
  source-needed`, `attachment_status = source-needed`,
  `evidence_review_status = not-reviewed`, and `proof_acceptance_status =
  not-accepted`.
- The next valid artifact is proof review only after a real attachment exists.

## Gate Record

- `cargo fmt -p route`
- `cargo test -p route`
- `route t4-terminal-access-proof-artifact-attachment --gate`
- `route t4-terminal-access-proof-source-capture --gate`
- `route tier-optimize --all-tiers --gate`
- `route optimizer-manifest --gate`
- `route release-manifest --gate`
- `proof check docs\SPEC_INDEX.md waves\PHASES.md waves\2026-05-14-t4-terminal-access-proof-artifact-attachment`
- `scripts\check-mileposts.ps1 -SkipTests`

## Next Action

Review attached terminal-access proof artifacts only after non-seed manual or
cached evidence is actually attached. No terminal-access blocker should be
reduced until proof is attached, reviewed, and accepted.
