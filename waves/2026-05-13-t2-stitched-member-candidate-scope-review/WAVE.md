---
wave: t2-stitched-member-candidate-scope-review
date_open: 2026-05-13
status: closed
source: waves/2026-05-13-t2-stitched-member-registry-handoff/CLOSE.md
---

# T2 Stitched Member Candidate Scope Review

## Mission

Separate route-level stitched candidate evidence from the blocked I295/I664
bundle ids before any membership repair can reduce game, incident, publication,
or upgrade blockers.

## Opening Rule

Candidate evidence may show many route-level members, but a blocked one-member
bundle remains blocked until a later repair explicitly scopes, splits, merges,
or expands the bundle id.

## Inputs Inherited

| Input | Source |
|---|---|
| Stitched handoff | `data/t2-stitched-member-registry-handoff.csv` |
| Tier segment candidates | `data/tier-segment-candidates.csv` |
| National registry | `data/national-segment-registry.csv` |
| National bundles | `data/national-segment-bundles.csv` |

## Pulse Status

| Pulse | Status | Evidence |
|---|---|---|
| 01 - Wave card and scope | done | this wave card and pulse plans |
| 02 - Candidate scope review | done | `data/t2-stitched-member-candidate-scope-review.csv` has two held rows |
| 03 - Review and close | done | closeout and review preserve candidate-scope handoff |

## Done Criteria

- Every stitched-member handoff row has a candidate scope review row.
- Review rows distinguish blocked bundle scope from route-level candidate scope.
- Review rows preserve claim blockers and remain out of pass/bound status.
- Optimizer and release manifests register the scope artifact.
- Final gates pass before close.

## Non-Goals

- Do not edit tier segment candidates.
- Do not edit registry or bundle membership.
- Do not promote game/ops binding decisions.
