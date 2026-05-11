# Milepost 7 Closeout — Program

Status: complete.

Milepost 7 turns the current ROUTE corpus into a reproducible release candidate. The release process now has a local gate bundle, CI-ready workflow, release manifest, release checklist, and explicit held-claim policy.

## Closure Decision

Program can close because another contributor can run one script and exercise the current release surface across tests, map atlas, pressure gates, Forum gates, Blueprint gates, and whitespace hygiene.

This closeout does not make held claims true. It makes the holds release-visible.

## Gate Bundle

The Milepost 7 gate bundle passed locally on 2026-05-10:

```powershell
powershell -ExecutionPolicy Bypass -File scripts/check-mileposts.ps1
```

The script ran:

- `cargo test --workspace`
- release manifest path check
- Forum docket path check, allowing explicitly held outputs to remain absent
- `route map-atlas --gate`
- `route standards-proof --gate-pressure`
- `route standards-inventory --gate --gate-planned`
- `route pressure-scenarios --gate-l2 --gate-readiness`
- `route pressure-scenarios --coverage --gate-coverage`
- `route throughput-proof --gate`
- `route t1-failures --gate-evidence`
- `route t1-failure-events --gate-observations`
- `route t1-snapshot-plan --gate-plan --script --priority A`
- `route game campaign --gate`
- `docs/game/browser/check-des-moines-browser.ps1`
- `route forum --gate`
- `route blueprint --gate`
- `route blueprint-evidence --gate`
- `route blueprint-costs --gate`
- `git diff --check`

Result: PASS.

`git diff --check` still reports LF/CRLF warnings on Windows-managed files, but no whitespace errors.

## Closed Artifacts

| Artifact | Role |
|---|---|
| `scripts/check-mileposts.ps1` | Local release gate bundle |
| `.github/workflows/ci.yml` | CI-ready workflow that runs the local gate bundle |
| `data/release-manifest.csv` | Release surface manifest with owner milepost, public status, and verification command |
| `docs/release/release-checklist.md` | Release policy for public, held, internal, and source-needed artifacts |
| `docs/milepost-7-plan.md` | Program tasklist and release surface |

## Release Policy

Publishable artifacts can include held claims only when the hold is visible in the artifact, closeout, docket, or Blueprint ledger.

The following remain held after Milepost 7:

- SLA/PTI and reliability-dollar claims.
- Managed-lane benefits.
- T1/T1 diamond recovery benefit claims.
- Donner, Atlanta, and other no-delta scenario benefit claims.
- Rural spur and T2 relief scope claims.
- Source-backed cost claims for planning, corridor-specific, or source-needed rows.
- Des Moines and Donner owner/human playtest acceptance.

## Handoff

The repo is now ready for a release-candidate pass: review the manifest, decide which artifacts should be included in a public bundle, and then either cut a release or start a new evidence slice targeting one held claim.
