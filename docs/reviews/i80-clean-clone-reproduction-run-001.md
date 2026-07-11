---
name: I-80 Clean-Clone Reproduction Run 001
slug: i80-clean-clone-reproduction-run-001
type: review
status: reviewed
rubric_version: v1.4
author: copilot
created: 2026-07-11
updated: 2026-07-11
sources:
  - tools/reproduce_i80_report.py
  - tools/prepare_i80_report_sources.py
  - data/cache/i80-report-source-readiness.csv
  - data/cache/i80-reproduction-status.csv
---

# I-80 Clean-Clone Reproduction Run 001

## Command

```powershell
npm run reproduce:i80:report
```

## Result

Status: **blocked as designed**

| Source | Status | Required action |
|---|---|---|
| ACS county population 2022 | blocked | Provide `CENSUS_API_KEY` in the environment |
| ACS county income 2022 | blocked | Provide `CENSUS_API_KEY` in the environment |

All no-credential sources passed. Six unwired source families remained
explicitly excluded and did not fail the contract.

## Safety Result

- `corpus/existing/i80.md` remained byte-identical.
- No generated comparison was written because the full contract did not pass.
- The status file names both blockers and no credential value.
- A console-encoding failure found during the first attempt was repaired; the
  rerun completed the blocker path cleanly.

## Next Step

Run the same command with `CENSUS_API_KEY` available. Review the generated
comparison before any canonical replacement.
