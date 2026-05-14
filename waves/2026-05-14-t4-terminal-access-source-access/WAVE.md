---
wave: t4-terminal-access-source-access
date_open: 2026-05-14
status: done
source: waves/2026-05-14-t4-terminal-access-proof-review/CLOSE.md
---

# T4 Terminal Access Source Access

## Mission

Classify source-access policy for the 69 unresolved T4 terminal-access proof
review rows before any source artifact can be attached or accepted.

## Opening Rule

This wave may record source-access requirements and cache-policy blockers. It
may not fetch sources, attach source artifacts, accept proof, mark scenario
readiness, or reduce `map;publication;upgrade` blockers.

## Inputs Inherited

| Input | Source |
|---|---|
| Terminal-access proof review rows | `data/t4-terminal-access-proof-review.csv` |

## Pulse Status

| Pulse | Status | Evidence |
|---|---|---|
| 01 - Wave card and scope | done | this wave card and pulse plans |
| 02 - Source-access policy surface | done | `data/t4-terminal-access-source-access.csv`; CLI gate |
| 03 - Review and close | done | `CLOSE.md`; `panels/source-access/review.md`; final gates |

## Done Criteria

- Every held terminal-access proof review row has one source-access policy row.
- Rows preserve `map;publication;upgrade` and `claim_blocker_delta = 0`.
- Rows keep proof `not-accepted` and evidence artifact `source-needed`.
- Rows explicitly block live fetch unless a policy-compliant fetcher exists.
- Optimizer and release manifests register the source-access artifact.
- Final gates pass before close.

## Non-Goals

- Do not fetch or attach live source artifacts.
- Do not accept proof.
- Do not reduce terminal-access blockers.
