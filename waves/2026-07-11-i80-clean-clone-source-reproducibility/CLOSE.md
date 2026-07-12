# I-80 Clean-Clone Source Reproducibility Closeout

Date closed: 2026-07-11

## Outcome

ROUTE no longer relies on invisible source-cache assumptions for the reviewed
I-80 report. Every source has an access, parser, year, readiness, exclusion, or
blocker contract.

## Final Source Posture

| Posture | Count | Sources |
|---|---:|---|
| Ready | 4 | TIGER 2023, Gazetteer 2023, HPMS 2018, RUCC 2023 |
| Excluded | 6 | FEMA, AFDC DCFC, NBI, FARS, FAF5, BEA |
| Blocked | 2 | ACS population 2022, ACS income 2022 |

Excluded sources do not contribute ambient cache values to I-80 regeneration.
Unavailable components are labeled and are not described as observed zero need.

## Reproduction Contract

```powershell
$env:CENSUS_API_KEY = "<user-provided>"
npm run reproduce:i80:report
```

The command:

1. acquires available sources;
2. writes source readiness;
3. blocks on missing required sources;
4. generates `data/cache/i80-regenerated.md` only after the full gate passes;
5. writes `data/cache/i80-report-comparison.csv`;
6. never replaces `corpus/existing/i80.md`.

## Current Hold

The current environment has no `CENSUS_API_KEY`. Reproduction therefore stops
on:

- `SRC-I80-ACS-POP`
- `SRC-I80-ACS-INCOME`

This is the intended credential boundary. No secret is requested, logged, or
committed.

## Wave Commits

- `041b72e` - source inventory and contract
- `7c20e0d` - no-credential orchestration
- `1cf3c9d` - Indiana HPMS repair and FEMA disposition
- `7c54853` - ACS/RUCC support and adapter exclusions
- `8332c76` - non-destructive reproduction and comparison
- Pulse 06 - closing commit containing this file

## Remaining Claim Holds

- A5 FARS, D1 FEMA, D2 DCFC, and D3 NBI historical values are not clean-clone
  reproducible and remain held.
- FAF5 and BEA are not cited as active report sources without wired joins.
- No capital, SLA, ROI, construction, funding, compliance, or endorsement claim
  is promoted.

## Next Trigger

Do not open another ROUTE wave automatically. Resume when:

1. the user supplies `CENSUS_API_KEY` in the environment and requests the
   reproduction run; or
2. the user selects another bounded ROUTE objective.
