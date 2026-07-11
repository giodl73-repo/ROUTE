---
wave: i80-flagship-stabilization
pulse: 06
date: 2026-07-11
status: done
depends_on:
  - pulse-05
governing_roles:
  - scope-keeper
  - citation-auditor
  - freight-economist
  - traffic-engineer
  - foxx
---

# Pulse 06 - Flagship Packet And External Review Docket

## Mission

Compress the I-80 work into one deterministic review packet and three external
review lanes without promoting the held hypothesis into a recommendation.

## Deliverables

- [x] Add one deterministic packet builder with check mode.
- [x] Generate a compact ten-minute review packet.
- [x] Define DOT/MPO, freight, and transportation-research reviewer lanes.
- [x] Name the decision each reviewer is asked to make.
- [x] Keep official-plan, capital, ROI, SLA, and endorsement claims held.

## Gates

- `npm run build:i80:packet`
- `npm run check:i80:packet`
- The packet is assembled from canonical artifacts, not copied by hand.
- The packet states `hold and narrow`.
- External review questions can return rejection as a valid result.
- `git diff --check`

## Non-Goals

- Name or contact reviewers.
- Claim external review has occurred.
- Create a presentation deck.
- Approve a design or capital package.
- Resolve clean-clone source-cache reproducibility.
- Resolve unrelated `data/t1-design-review.csv` worktree state.

## Result

The flagship now has a deterministic, compact internal review packet and a
three-lane external-review docket. The packet makes the current null/hold
posture visible and asks reviewers whether the validation plan is credible,
not whether they endorse construction.
