---
name: International EU Rhine-Alpine Parser Preflight 001
slug: international-eu-rhine-alpine-parser-preflight-001
type: review
status: draft
rubric_version: v1.0
author: codex
created: 2026-06-20
updated: 2026-06-20
sources:
  - tools/build_eu_rhine_alpine_parser_preflight.py
  - tools/check_eu_rhine_alpine_parser_preflight.py
  - data/international-eu-rhine-alpine-parser-preflight-001.csv
  - data/international-eu-rhine-alpine-parser-output-contract-001.csv
  - docs/reviews/international-eu-rhine-alpine-adapter-source-pack-001.md
---

# International EU Rhine-Alpine Parser Preflight 001

## Result

This defines what an EU Rhine-Alpine parser dry run may inspect and emit before
any source-bound adapter promotion.

It is not a parser implementation, not source download/cache evidence, not a
parsed EU adapter, and not EU internal proof. It does not replace hierarchy
fixtures or create official-corridor, member-state approval, route designation,
geometry/topology proof, terminal performance, guaranteed SLA, ROI,
construction, eligibility, compliance, endorsement, validation, public
readiness, external readiness, or external-validation claims.

## Parser Surfaces

| Surface | Allowed Label | Current Posture |
|---|---|---|
| Link candidates | source-candidate | metadata/no-geometry only |
| Need candidates | source-candidate | bounded corridor vocabulary only |
| Node candidates | source-needed | node/terminal custody missing |
| Service targets | held | assumptions only |
| Evidence labels | carry-forward | required for every emitted row |
| Review backlog | carry-forward | pending role rows only |

## Gate

Decision: **eu_parser_preflight_ready; implementation_held**

Run:

```powershell
npm run check:eu:parser-preflight
```

Rationale: EU can now move from source-pack preflight to a contract-shaped dry
run. Parser implementation, source-row validation, fixture replacement, internal
proof, media proof, external review, and all stronger claims remain held.
