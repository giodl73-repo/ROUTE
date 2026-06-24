---
wave: milestone-10-t2-evidence-policy-qualification-compatibility
pulse: 01
date: 2026-06-23
status: done
---

# Pulse 01 - Evidence Policy Accepts Qualification Effects

## Deliverables

- Add `qualification_effects` to `T2GameOpsBundleEvidencePolicyRow`.
- Copy `qualification_effects` from `T2GameOpsBundleEvidenceReviewRow` when
  policy rows are generated.
- Default the field during deserialization so existing policy CSVs remain
  readable.
- Add gate coverage so future policy rows with qualification effects cannot drop
  their qualification gate/game-use contract.

## Gates

- `cargo test -q -p route --bin route t2_game_ops_bundle_evidence_policy_preserves_review_blockers`
- `cargo test -q -p route --bin route t2_game_ops_bundle_evidence_policy_acceptance_preserves_policy_blockers`
- `npm run check:l2`

## Non-goals

- Do not regenerate `data/t2-game-ops-bundle-evidence-policy.csv`; the current
  selector budget has no active unrelieved intake/decision rows.
- Do not change evidence policy decisions or acceptance routing.
- Do not resolve unrelated `data/t1-design-review.csv` edits.

## Result

Done. Future evidence policy rows preserve qualification effects from evidence
review, while existing policy CSVs remain compatible.
