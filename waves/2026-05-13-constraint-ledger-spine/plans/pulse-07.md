---
wave: constraint-ledger-spine
pulse: 07
date: 2026-05-13
status: done
depends_on: [pulse-06]
governing_roles:
  - optimization-methodologist
  - scope-keeper
---

# Pulse 07 - Wave Close and Doctrine Cleanup

## Mission

Close the Constraint Ledger Spine wave by reconciling specs, generated
artifacts, release manifests, wave status, and significant-moment history.

## Deliverables

- [x] Write `waves/2026-05-13-constraint-ledger-spine/CLOSE.md`.
- [x] Update `waves/PHASES.md` status and identify the next active wave.
- [x] Update `data/significant-moments.csv` if the source/game migration creates
  a durable doctrine shift.
- [x] Ensure `docs/SPEC_INDEX.md` points to the wave system and active wave
  artifacts.
- [x] Run final gates and commit.

## Expected Gates

- `route significant-moments --gate`
- `route optimizer-manifest --gate`
- `route release-manifest --gate`
- `cargo test -p route`
- `scripts/check-mileposts.ps1 -SkipTests`
