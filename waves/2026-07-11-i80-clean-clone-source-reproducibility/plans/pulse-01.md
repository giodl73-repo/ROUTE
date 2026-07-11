---
wave: i80-clean-clone-source-reproducibility
pulse: 01
date: 2026-07-11
status: done
depends_on: []
governing_roles:
  - citation-auditor
  - scope-keeper
  - numeracy-checker
---

# Pulse 01 - Source Acquisition Inventory

## Mission

Classify every source required by the reviewed I-80 report before implementing
an orchestrator.

## Deliverables

- [x] Inventory current manifest, commands, loaders, and cache contracts.
- [x] Verify official external source surfaces where needed.
- [x] Separate automated, partial, credential-gated, manual, and adapter-missing
      sources.
- [x] Write a cited research decision.
- [x] Write a machine-readable source contract.

## Gates

- Every actionable claim has a local path, command, or official URL.
- Current and candidate source years are distinguished.
- Credential requirements are explicit.
- Download support and parser support are separate fields.
- `git diff --check`

## Non-Goals

- Change source years during the inventory pulse.
- Add credentials to the repository.
- Implement new fetchers.
- Regenerate the I-80 corpus.
- Resolve unrelated `data/t1-design-review.csv` worktree state.

## Result

Four source families have usable local acquisition paths, two are currently
credential-blocked, four need adapters or conversion policy, and two are
unconditional source citations without a wired clean-clone join. The next pulse
can implement orchestration without guessing source readiness.
