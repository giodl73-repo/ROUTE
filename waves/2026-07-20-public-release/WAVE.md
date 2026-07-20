# ROUTE Public Release

## Goal

Publish ROUTE as an inspectable research and design workbench while preserving
its evidence labels, held claims, and explicit distinction between repository
visibility and deployment readiness.

## Completed work

- Rewrote historical public commit metadata from the work email to
  `giodl73@gmail.com`.
- Confirmed the root MIT license, public README license section, `.roles`, and
  GitHub Actions gates.
- Added an explicit README boundary between public source visibility and claim
  readiness.
- Scanned the rewritten history for common credential and private-key patterns.

## Validation

```powershell
cargo fmt --all -- --check
cargo clippy --locked -p route-score -p route-report --all-targets --no-deps -- -D warnings
cargo test -q --locked -p route-score -p route-report
npm run check:i80:packet
npm run test:i80:sources
npm run test:i80:reproduction
npm run check:i80:sources
.\scripts\check-mileposts.ps1
git diff --check
```
