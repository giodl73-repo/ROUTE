---
wave: t2-game-publication-evidence-review
pulse: 02
status: done
---

# Pulse 02 - Game Publication Review Artifact

## Deliverable

Generate `data/t2-game-publication-evidence-review.csv`.

## Gates

- Every representative scenario hook has one review row.
- Rows preserve `game;publication;upgrade` blockers with
  `claim_blocker_delta = 0`.
- Rows route to `data/t2-game-publication-evidence-policy.csv`.

## Result

Done by `route t2-game-publication-evidence-review --gate`.

