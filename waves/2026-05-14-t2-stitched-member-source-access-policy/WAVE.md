---
wave: t2-stitched-member-source-access-policy
date_open: 2026-05-14
status: closed
source: waves/2026-05-14-t2-stitched-member-evidence-acquisition/CLOSE.md
---

# T2 Stitched Member Source Access Policy

## Mission

Classify I295 and I664 DOT route geometry source targets as manual, cacheable,
or policy-covered before any stitched-member evidence collection begins.

## Opening Rule

The policy may define access mode and cache requirements, but it may not fetch,
cache, or accept source evidence and may not change candidate membership.

## Inputs Inherited

| Input | Source |
|---|---|
| Acquisition docket | `data/t2-stitched-member-evidence-acquisition.csv` |
| Evidence contract | `data/t2-stitched-member-evidence-contract.csv` |

## Pulse Status

| Pulse | Status | Evidence |
|---|---|---|
| 01 - Wave card and scope | done | this wave card and pulse plans |
| 02 - Source access policy surface | done | `data/t2-stitched-member-source-access-policy.csv` has 11 manual/cached source-needed rows |
| 03 - Review and close | done | manifests registered, role review written, final gates passed |

## Done Criteria

- Every source-needed acquisition row has a source-access policy row.
- Policy rows name access mode, live-fetch status, cache policy, and metadata.
- Policy rows remain source-needed and review.
- Optimizer and release manifests register the source-access policy.
- Final gates pass before close.

## Non-Goals

- Do not fetch or cache source evidence.
- Do not classify candidates in scope or rejected.
- Do not edit registry or bundle membership.
