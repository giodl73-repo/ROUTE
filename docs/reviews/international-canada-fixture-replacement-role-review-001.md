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

This review compares the current Canada dry-run link fixture with the
source-derived extraction candidates.

The extraction candidates are stronger than the dry-run placeholders because
they carry bounded source attributes: route IDs, route names where present,
source class, source owner, date, access note, evidence label, and blocked
claims. That is enough to keep parser extraction work moving.

It is not enough to replace `data/canada_source_link_candidates.csv`.
Replacement remains held because the candidate rows still have no accepted
geometry, no parsed adapter promotion, no operational evidence, and no
jurisdictional authority or delivery evidence. A later candidate-only
source-row validation gate now closes row matching for this bounded extraction
table, but that does not change this replacement decision.

## Role Findings

| Role Lane | Decision | Finding | Required Next Step |
|---|---|---|---|
| Scope Keeper | pass_with_risk | Extraction candidates are correctly separate from the dry-run fixture and stay inside review scope. | Keep the dry-run fixture unchanged until replacement closeout explicitly changes output ownership. |
| Citation Auditor | pass_with_risk | Source owner, date, access note, evidence label, and blocked claims are carried forward on extracted rows. | Add source-row validation evidence before promotion beyond candidate rows. |
| Schematic Cartographer | hold_for_map_or_fixture_use | No-geometry rows cannot support map overlay or topology proof. | Define geometry policy and map-caption posture before any map-facing replacement. |
| Traffic Engineer | hold_for_operational_claims | Route number, class, and name fields do not prove capacity, reliability, geometry, safety, or throughput. | Require operational source fields or explicit no-operational-claim posture before promotion. |
| State DOT Planner | hold_for_authority_and_delivery_claims | Extracted rows do not establish Canadian authority review, funding eligibility, project delivery, or designation. | Keep approval, eligibility, delivery, and designation claims blocked unless jurisdiction-specific sources close. |

## Replacement Decision

Decision: **replacement_review_pass_with_holds; dry_run_fixture_unchanged**

Allowed language:

- ROUTE has source-derived Canada link-candidate rows for internal parser
  extraction review.
- Those rows can be compared with the dry-run fixture and role-reviewed before
  replacement.
- Fixture replacement is still held.

Do not say:

- The Canada fixture has been replaced.
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

Decision: **replacement_review_pass_with_holds; dry_run_fixture_unchanged**

Rationale: the extraction candidates are ready for internal parser review, but
role findings still block fixture replacement, geometry/map use, operational
claims, authority/delivery claims, parsed-adapter promotion, and external use.
