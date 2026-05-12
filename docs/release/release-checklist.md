# ROUTE Release Checklist

Status: Milepost 7 working checklist.

## Release Classes

| Class | Meaning |
|---|---|
| `public` | Safe to publish as a current ROUTE artifact with the evidence labels shown |
| `held_public` | Safe to publish only if the hold is visible and no benefit claim is promoted |
| `internal` | Not part of the release surface |
| `source_needed` | Named source path exists but the artifact cannot support a public claim |

## Required Local Gate

Run:

```powershell
powershell -ExecutionPolicy Bypass -File scripts/check-mileposts.ps1
```

For a faster documentation-only check while iterating:

```powershell
powershell -ExecutionPolicy Bypass -File scripts/check-mileposts.ps1 -SkipTests
```

The full gate is the one that counts for closeout.

## Publishable Now

- Current specs and milepost closeouts.
- Scoring, map-atlas, Beck T2 service standards, T2 game overlays, gap, pressure, Forum, and Blueprint ledgers with their evidence labels.
- Game campaign spine, T2 service overlay contract, and checked-in simulated/CLI playtest records.
- Static browser prototype as a demonstration artifact, not as proof of claims.

## Must Stay Held

- SLA/PTI and reliability-dollar claims.
- Managed-lane benefits.
- T1/T1 diamond recovery benefit claims.
- Donner, Atlanta, or other no-delta scenario benefit claims.
- Rural spur and T2 relief scope claims.
- Source-backed cost claims for rows that are planning, corridor-specific, or source-needed.
- Des Moines and Donner owner/human playtest acceptance.

## Release Steps

1. Run `scripts/check-mileposts.ps1`.
2. Confirm `data/release-manifest.csv` lists any new public artifact.
3. Confirm `docs/SPEC_INDEX.md` names the ownership home for any new claim.
4. Confirm every held claim appears in a closeout, docket, or Blueprint ledger.
5. Run `git diff --check`.
6. Write or update the Milepost closeout.
