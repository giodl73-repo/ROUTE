---
wave: t2-stitched-member-evidence-contract
pulse: 03
date: 2026-05-13
status: done
depends_on: [pulse-02]
governing_roles:
  - optimization-methodologist
  - scope-keeper
  - traffic-engineer
---

# Pulse 03 - Review and Close

## Mission

Register the evidence contract artifact, close the wave, and commit.

## Scope Inventory

- `data/t2-stitched-member-evidence-contract.csv`
- `data/tier-optimizer-runs.csv`
- `data/release-manifest.csv`
- `waves/2026-05-13-t2-stitched-member-evidence-contract/`

## Deliverables

- [x] Register optimizer and release manifest rows.
- [x] Write `CLOSE.md`.
- [x] Write role review under `panels/evidence-contract/`.
- [x] Run final gates and commit.

## Expected Gates

- `cargo test -p route`
- `route t2-stitched-member-evidence-contract --gate`
- `route t2-stitched-member-selection-docket --gate`
- `route tier-optimize --all-tiers --gate`
- `route optimizer-manifest --gate`
- `route release-manifest --gate`
- `scripts/check-mileposts.ps1 -SkipTests`

## Non-Goals

- Do not declare stitched-member repair complete from evidence contracts alone.
