## Summary

Describe the change and the ROUTE surface it affects.

## Claim Boundary

- [ ] This change preserves ROUTE's evidence labels and source custody.
- [ ] No official-plan, agency-endorsement, construction-readiness, guaranteed-SLA, numeric-ROI, funding-eligibility, compliance, procurement, or external-validation claim is introduced unless explicitly source-backed and reviewed.
- [ ] Any changed public claim has a traceable source path or is marked held/source-needed/confidence-limited.
- [ ] Maps or visuals are described as structural unless a stronger evidence row supports them.

## Validation

List the checks run. For docs-only changes, include:

```powershell
git diff --check -- README.md docs .github CONTRIBUTING.md
```

For code or generated-artifact changes, use `docs/vtrace/VERIFICATION.md`.
