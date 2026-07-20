# Contributing

Contributions should preserve ROUTE's evidence labels, source custody, and
distinction between implemented tooling and held planning claims.

## Before opening a pull request

Run the checks documented in [`docs/vtrace/VERIFICATION.md`](docs/vtrace/VERIFICATION.md).
At minimum, format and test the affected Rust crates and run the relevant
artifact gate.

Every changed number or public claim needs a traceable source. Do not commit raw
restricted datasets, credentials, local state, or artifacts that imply official
plan, agency endorsement, construction, funding, or deployment readiness.
