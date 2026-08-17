# ROUTE reuse boundary

ROUTE is intentionally a specialist highway-planning research and tooling
system. Its public adoption path is method and evidence reuse, not an implied
stable Rust library for unrelated infrastructure domains.

## Safe reuse

Reuse these assets by reference or local adaptation:

- the source-to-corpus-to-score-to-promise-to-gap workflow;
- evidence labels and explicit claim holds;
- T1-T4 service-role questioning where those labels fit the local network;
- bounded public-proof and adversarial-review discipline;
- local adaptation worksheets, reviewer records, and null-result acceptance;
  and
- map-publication boundaries that separate rendered structure from validated
  service, ROI, construction, or approval claims.

## Specialist surfaces

Keep these surfaces owned by ROUTE:

- highway, corridor, stop, bundle, tier, SLA, relay, freight, and terminal
  semantics;
- US and jurisdiction-specific source interpretation;
- route scoring weights, threshold policy, recursive tiering, gap repair, and
  pressure propagation;
- highway graph, simulation, map, corpus, and report types in the `route-*`
  workspace crates; and
- construction, funding, safety, legal, operating, or official-plan claims.

The `route-*` crates are internal components of one coherent product. No
external repository currently consumes them as a stable package contract.
Similar crate topology in another infrastructure repository is not evidence
that the underlying units or policy are interchangeable.

## Extraction gate

Do not extract or advertise a shared cross-domain crate until a named external
adopter needs the same product-neutral contract and can prove compatibility
without importing ROUTE's highway semantics.

A proposal must identify:

1. the adopter and concrete call sites;
2. the minimal types or algorithms that remain neutral across both domains;
3. fixtures from both repositories;
4. versioning and migration ownership; and
5. the domain-specific code that will remain in ROUTE.

Until that gate is met, adapt the method and evidence discipline while keeping
implementation contracts local.
