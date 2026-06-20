# International India Source Row Validation 001

Status: draft; bounded internal row check only.

## Result

This checks the India dry-run rows against the field inventory and preserves the
current evidence posture:

- source-candidate rows are bounded metadata matches, not validated rows;
- heuristic-held rows remain fixture-gap tracking only;
- held target rows remain assumptions only.

This does not validate source rows, replace fixtures, accept geometry, promote
an official Indian corridor, claim national or state approval, prove SLA or ROI,
or claim public/external readiness.

## Gate

Decision: **india_row_validation_ready_with_holds; fixture_replacement_held**

Run:

```powershell
npm run check:india:source-row-validation
```
