# Milepost 5-7 Source And Release Review

Date: 2026-05-10

Review scope: Milepost 5 Forum, Milepost 6 Blueprint, and Milepost 7 Program artifacts, with emphasis on source traceability, held-claim visibility, gate coverage, and release readiness.

## Findings

### Fixed During Review: Program Gate Did Not Yet Cover Several Manifested Source Gates

Severity: medium.

`data/release-manifest.csv` listed source-sensitive artifacts whose verification commands were not included in the first `scripts/check-mileposts.ps1` bundle: standards inventory, T1/T1 failure evidence, T1/T1 event observations, T1/T1 snapshot plan, browser fixture check, release manifest path check, and Forum docket path check.

Resolution: `scripts/check-mileposts.ps1` now runs those gates. The hardened script passed with `-SkipTests` after the change.

### Residual: Held Owner Acceptance Outputs Are Intentionally Missing

Severity: low, release-visible hold.

`data/forum-docket.csv` has two held owner rows whose `output_artifact` paths do not exist:

- `docs/game/des-moines-diamond-owner-acceptance.md`
- `docs/game/donner-weather-closure-owner-acceptance.md`

This is acceptable because F5-02 and F5-03 are explicitly `held`. The release gate now checks that completed docket outputs exist while allowing held outputs to remain absent. Release notes should continue to say these are blocked on owner/human playtest acceptance.

### Residual: Manual-Review Rows Are Not Machine-Verified Beyond Path Existence

Severity: low.

Several release manifest rows use `manual review` as their verification command. That is reasonable for prose closeouts and specs, but it means the release gate confirms only that the files exist, not that their claims are internally consistent.

Recommended next hardening: add a small `route release-manifest --gate` command or script check that enforces allowed `release_status`, `public_status`, owner milepost vocabulary, and no missing ownership entries for new closeout artifacts.

## Source Traceability Checks

| Check | Result |
|---|---|
| Release manifest artifact paths exist | pass |
| Forum docket input artifacts exist | pass |
| Forum completed outputs exist | pass |
| Forum held outputs may be absent | pass |
| Blueprint evidence `proof_artifact` paths or command labels resolve | pass |
| Blueprint package, evidence, and cost gates pass | pass |
| Hardened release gate passes with `-SkipTests` | pass |

## Review Verdict

The last three milestones are release-credible with visible holds. The strongest remaining risk is not hidden overclaiming in the Blueprint ledgers; those are well-labeled. The practical release risk is that some prose/manual-review artifacts can drift because they are not yet parsed by a structured release-manifest gate.
