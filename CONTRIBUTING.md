# Contributing

Contributions should preserve ROUTE's evidence labels, source custody, and
distinction between implemented tooling and held planning claims.

## Useful public contributions

ROUTE is open for reference, review, and local adaptation. Good first
contributions include:

- source inventories for a state, corridor, port, terminal, or region;
- corrections to source interpretation, geography, map captions, or claim
  labels;
- role-review notes from freight, DOT, planning, community, resilience, transit,
  construction, finance, labor, safety, or environmental perspectives;
- bounded diagnostic fixtures that show service roles, failure modes, evidence
  holds, and next source asks;
- research-paper comments tied to a specific `research/publications/`
  directory;
- safer public language that prevents official-plan, endorsement, construction,
  SLA, ROI, funding, compliance, procurement, or validation drift.

If you are proposing a local adaptation, start with
[`docs/adoption/README.md`](docs/adoption/README.md).

## Before opening a pull request

Run the checks documented in [`docs/vtrace/VERIFICATION.md`](docs/vtrace/VERIFICATION.md).
At minimum, format and test the affected Rust crates and run the relevant
artifact gate.

Every changed number or public claim needs a traceable source. Do not commit raw
restricted datasets, credentials, local state, or artifacts that imply official
plan, agency endorsement, construction, funding, or deployment readiness.
