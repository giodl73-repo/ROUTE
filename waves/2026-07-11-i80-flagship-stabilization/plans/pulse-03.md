---
wave: i80-flagship-stabilization
pulse: 03
date: 2026-07-11
status: done
depends_on:
  - pulse-02
governing_roles:
  - citation-auditor
  - numeracy-checker
  - scope-keeper
---

# Pulse 03 - I-80 Corpus Completion

## Mission

Complete the I-80 corridor narrative without allowing regeneration to erase
reviewed prose or repeat known measurement-reporting defects.

## Scope Inventory

- `corpus/existing/i80.md`
- `corpus/annotations/i80.toml`
- `crates/route-report`
- `crates/route-score`
- `research/i80-flagship-source-audit.md`

## Deliverables

- [x] Add a checked annotation sidecar consumed by `route-report`.
- [x] Preserve reviewed overview, notable segments, fit, holds, open questions,
      and citations during regeneration.
- [x] Relabel mean AADT as an unweighted mean across matched HPMS segments.
- [x] Stop printing `$0.0B` when total buffer GDP is unavailable.
- [x] Complete the bounded I-80 narrative and claim holds.
- [x] Record the clean-clone regeneration hold exposed by the command gate.
- [x] Add focused regression coverage.

## Gates

- `cargo fmt --check`
- `cargo test -q -p route-score -p route-report`
- `cargo run -q -p route -- report I80`
- Regenerated I-80 contains no human-annotation placeholders.
- Regenerated I-80 preserves the annotation sidecar content.
- `git diff --check`

## Non-Goals

- Change the I-80 numeric score.
- Select an investment treatment.
- Regenerate the full national corpus.
- Promote official-plan, construction, SLA, or ROI claims.
- Resolve unrelated `data/t1-design-review.csv` worktree state.

## Result

The I-80 record is now `reviewed`, contains no annotation placeholders, and
keeps its narrative in `corpus/annotations/i80.toml`. `route-report` loads that
checked sidecar, validates its lifecycle status, preserves held-dimension
markers, and has regression coverage for regeneration.

The real `route report I80` gate was attempted and exposed a reproducibility
hold rather than a code failure: the clean checkout lacked the gitignored TIGER
cache. `route fetch` successfully acquired TIGER and the Census gazetteer, but
the report still cannot reproduce the existing HPMS, NBI, ACS, income, FEMA,
FARS, and related joined measurements because those caches are absent or manual.
The canonical file was therefore not overwritten with a degraded partial
report. Full clean-clone regeneration is carried into Pulse 07.
