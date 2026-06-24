---
wave: milestone-10-t2-blocker-relief-qualification-compatibility
pulse: 01
date: 2026-06-23
status: done
---

# Pulse 01 - Blocker Relief Accepts Qualification Effects

## Deliverables

- Add `qualification_effects` to `T2GameOpsBundleEvidenceBlockerReliefRow`.
- Copy `qualification_effects` from
  `T2GameOpsBundleEvidencePolicyAcceptanceRow` when relief rows are generated.
- Default the field during deserialization so existing blocker-relief CSVs remain
  readable.
- Extend blocker-relief gate compatibility so source acceptance rows with
  qualification effects require relief rows to preserve qualification semantics.

## Gates

- `cargo test -q -p route --bin route t2_game_ops_bundle_evidence_blocker_relief_reduces_accepted_blockers`
- `cargo test -q -p route --bin route optimizer_constraint_ledger_replays_t2_game_ops_bundle_relief`
- `npm run check:l2`

## Non-goals

- Do not regenerate `data/t2-game-ops-bundle-evidence-blocker-relief.csv`; the
  current selector budget has no active unrelieved intake/decision rows.
- Do not change blocker-relief decisions or optimizer replay routing.
- Do not resolve unrelated `data/t1-design-review.csv` edits.

## Result

Done. Future blocker-relief rows preserve qualification effects from evidence
acceptance, while existing relief CSVs remain compatible.
