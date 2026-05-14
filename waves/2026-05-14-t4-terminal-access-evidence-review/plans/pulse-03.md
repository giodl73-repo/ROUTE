---
wave: t4-terminal-access-evidence-review
pulse: 03
date: 2026-05-14
status: done
depends_on: [pulse-02]
governing_roles:
  - optimization-methodologist
  - citation-auditor
  - scope-keeper
---

# Pulse 03 - Review and Close

## Mission

Register the terminal-access evidence review artifact, close the wave, and
commit.

## Scope Inventory

- `data/t4-terminal-access-evidence-review.csv`
- `data/tier-optimizer-runs.csv`
- `data/release-manifest.csv`
- `waves/2026-05-14-t4-terminal-access-evidence-review/`

## Deliverables

- [x] Register optimizer and release manifest rows.
- [x] Write `CLOSE.md`.
- [x] Write role review under `panels/terminal-access-evidence/`.
- [x] Run final gates and commit.

## Expected Gates

- `cargo test -p route`
- `route t4-terminal-access-evidence-review --gate`
- `route t4-terminal-contact-evidence --gate`
- `route tier-optimize --all-tiers --gate`
- `route optimizer-manifest --gate`
- `route release-manifest --gate`
- `scripts/check-mileposts.ps1 -SkipTests`

## Non-Goals

- Do not declare terminal-access evidence repaired from review rows alone.
