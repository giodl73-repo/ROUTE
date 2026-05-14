---
wave: t4-terminal-access-proof-source-capture
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

Register the source-capture artifact, close the wave, and commit.

## Scope Inventory

- `data/t4-terminal-access-proof-source-capture.csv`
- `data/tier-optimizer-runs.csv`
- `data/release-manifest.csv`
- `waves/2026-05-14-t4-terminal-access-proof-source-capture/`

## Deliverables

- [x] Register optimizer and release manifest rows.
- [x] Write `CLOSE.md`.
- [x] Write role review under `panels/source-capture/`.
- [x] Run final gates and commit.

## Expected Gates

- `cargo test -p route`
- `route t4-terminal-access-proof-source-capture --gate`
- `route t4-terminal-access-proof-intake --gate`
- `route tier-optimize --all-tiers --gate`
- `route optimizer-manifest --gate`
- `route release-manifest --gate`
- `scripts/check-mileposts.ps1 -SkipTests`

## Non-Goals

- Do not declare proof accepted.
