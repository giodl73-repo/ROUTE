---
wave: i80-flagship-stabilization
pulse: 02
date: 2026-07-11
status: done
depends_on:
  - pulse-01
governing_roles:
  - citation-auditor
  - numeracy-checker
  - scope-keeper
---

# Pulse 02 - I-80 Source Audit

## Mission

Audit the generated I-80 record before writing narrative prose so unsupported
scores, misleading summaries, and regeneration hazards are corrected or held
rather than repeated.

## Scope Inventory

- `corpus/existing/i80.md`
- `data/corridor-designations.csv`
- `data/sources.md`
- `config/scoring.toml`
- `crates/route-network/src/aggregate.rs`
- `crates/route-network/src/strategic.rs`
- `crates/route-score/src/score.rs`
- `crates/route-report/src/lib.rs`
- authoritative FHWA, USDA, FEMA, NBI, and defense-source surfaces

## Deliverables

- [x] Record source-backed facts that can be adopted now.
- [x] Identify generated claims that must be relabeled or recomputed.
- [x] Separate adopt-now, prototype-boundary, and defer recommendations.
- [x] Name owners, validation, and non-goals for corpus completion.

## Gates

- Every actionable finding has a local path, measured command, or public URL.
- No score is promoted because a plausible narrative exists.
- The research output names the decision it supports.
- `git diff --check`

## Non-Goals

- Rewrite the generated I-80 corpus record.
- Change rubric scores in isolation.
- Select an investment treatment.
- Regenerate the national corpus.
- Resolve unrelated `data/t1-design-review.csv` worktree state.

## Result

The audit found a usable federal framing for I-80 through the NHS, STRAHNET,
and NHFN, but it also found that A4/B4/C4 designation scores lack row-level
source custody, the corridor AADT summary is unweighted, the C3 report prints a
false-looking zero-dollar GDP value when total GDP is absent, and regeneration
would overwrite human narrative. Pulse 03 must repair those boundaries before
the corridor can be marked reviewed.
