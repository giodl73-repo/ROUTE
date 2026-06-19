---
name: International Canada Fixture Replacement Role Review 001
slug: international-canada-fixture-replacement-role-review-001
type: review
status: draft
rubric_version: v1.0
author: codex
created: 2026-06-19
updated: 2026-06-19
sources:
  - .roles/ROLE.md
  - .roles/editorial/scope-keeper.md
  - .roles/editorial/citation-auditor.md
  - .roles/parliament/schematic-cartographer.md
  - .roles/parliament/traffic-engineer.md
  - .roles/stakeholders/state-dot.md
  - tools/build_canada_fixture_replacement_role_review.py
  - tools/check_canada_fixture_replacement_role_review.py
  - data/international-canada-fixture-replacement-role-review-001.csv
  - data/international-canada-source-row-validation-001.csv
  - data/international-canada-parser-extraction-candidates-001.csv
  - data/canada_source_link_candidates.csv
---

# International Canada Fixture Replacement Role Review 001

## Result

This review compares the current Canada internal link fixture with the
source-derived extraction candidates.

The extraction candidates are stronger than the dry-run placeholders because
they carry bounded source attributes: route IDs, route names where present,
source class, source owner, date, access note, evidence label, and blocked
claims. That is enough to keep parser extraction work moving.

This review originally blocked fixture replacement until role review,
source-row validation, geometry policy, and a replacement contract closed. The
later closeout now replaces `data/canada_source_link_candidates.csv` for the
narrow internal parser link fixture only. Map use, topology proof, parsed
adapter promotion, operational evidence, and jurisdictional authority or
delivery evidence remain held.

## Role Findings

| Role Lane | Decision | Finding | Required Next Step |
|---|---|---|---|
| Scope Keeper | pass_with_risk | Source-derived candidates now occupy the internal link fixture under closeout scope. | Keep non-link fixture replacement, map, adapter, and external uses blocked. |
| Citation Auditor | pass_with_risk | Source owner, date, access note, evidence label, blocked claims, and source-row validation are carried into the internal link fixture. | Preserve source custody and blocked claims in any future parser closeout. |
| Schematic Cartographer | hold_for_map_or_fixture_use | No-geometry rows cannot support map overlay or topology proof. | Define geometry policy and map-caption posture before any map-facing replacement. |
| Traffic Engineer | hold_for_operational_claims | Route number, class, and name fields do not prove capacity, reliability, geometry, safety, or throughput. | Require operational source fields or explicit no-operational-claim posture before promotion. |
| State DOT Planner | hold_for_authority_and_delivery_claims | Extracted rows do not establish Canadian authority review, funding eligibility, project delivery, or designation. | Keep approval, eligibility, delivery, and designation claims blocked unless jurisdiction-specific sources close. |

## Replacement Decision

Decision: **replacement_review_pass_with_holds; internal_link_fixture_replaced**

Allowed language:

- ROUTE has source-derived Canada link-candidate rows for internal parser
  extraction review.
- Those rows now occupy the internal parser link fixture under the replacement
  closeout contract.
- Internal link-fixture replacement is closed with holds.

Do not say:

- ROUTE has a parsed Canada adapter.
- The extracted rows prove an official Canadian network, route designation,
  geometry, topology, operating SLA, reliability, capacity, eligibility,
  compliance, approval, endorsement, construction readiness, ROI, validation,
  public readiness, or external readiness.

## Command Closeout

Run:

```powershell
npm run check:canada:replacement-review
```

Expected gate result:

```text
Canada fixture replacement role-review gate: PASS
  checked role coverage, comparison inputs, held replacement posture, and blocked claims
```

## Validation Closeout

| Check | Command / Inspection | Result | Evidence |
|---|---|---|---|
| Build role review | `python tools\build_canada_fixture_replacement_role_review.py` | pass | `data/international-canada-fixture-replacement-role-review-001.csv` written |
| Role-review gate | `python tools\check_canada_fixture_replacement_role_review.py` | pass | role coverage, comparison inputs, held replacement posture, and blocked claims checked |
| Package command | `npm run check:canada:replacement-review` | pass | build and gate run together |
| Python compile | `python -m py_compile tools\build_canada_fixture_replacement_role_review.py tools\check_canada_fixture_replacement_role_review.py` | pass | scripts compile |
| Claim-boundary scan | scan role-review artifacts and edited indexes | pass | hits are blocked, held, or do-not-infer contexts |
| Diff hygiene | `git diff --check` | pass | no whitespace errors |
| L0 | `npm run check:l0` | pass | workspace lib/bin tests passed |

## Gate

Decision: **replacement_review_pass_with_holds; internal_link_fixture_replaced**

Rationale: the extraction candidates now occupy the internal link fixture, but
role findings still block geometry/map use, non-link fixture replacement,
operational claims, authority/delivery claims, parsed-adapter promotion, and
external use.
