# ROUTE Principles

## ROUTE-P-01: Null results are valid outputs

**Statement:** ROUTE reports hold, downgrade, blocked, or no-design findings
when the evidence does not support promotion.

**Rationale:** Interstate 2.0 is a research and tooling lab, not a plan factory.
Forcing every analysis into a positive corridor or construction claim would make
the evidence model untrustworthy.

**Decision rule:** If a score, source gate, role review, or VTRACE row does not
support the stronger claim, keep the claim held or downgraded and name the next
evidence step.

**Consequence:** A rigorous null result can close a package; it does not require
scope expansion to rescue the hypothesis.

**Evidence:** `README.md`, `CLAUDE.md`, `docs/vtrace/REQUIREMENTS.md` REQ-002,
REQ-003, and REQ-010.

**Status:** ACTIVE

## ROUTE-P-02: Service identity beats route labels

**Statement:** ROUTE treats bundles as the service/corridor object and treats
route labels as presentation attributes.

**Rationale:** Route labels, tiers, zones, and map names can change as evidence
improves. Bundle and segment identity must stay stable enough for maps, SLA
surfaces, simulations, and reports to compare artifacts safely.

**Decision rule:** A segment-bearing artifact may display route labels, but the
join or primary identity must be `segment_bundle_id`,
`national_segment_id`, or an explicitly held identity-producing artifact.

**Consequence:** Convenience map labels cannot become hidden primary keys.

**Evidence:** `docs/route-architecture.md`, `docs/vtrace/REQUIREMENTS.md`
REQ-004 and REQ-005, and `docs/vtrace/TRACE.md`.

**Status:** ACTIVE

## ROUTE-P-03: Review tension is evidence

**Statement:** ROUTE preserves parliament, stakeholder, editorial, and panel
review tensions instead of averaging them into a single neutral voice.

**Rationale:** Transportation claims involve incompatible stakes: throughput,
access, environment, delivery feasibility, freight economics, schematic truth,
and public scope. Losing dissent hides the reason a claim was held or promoted.

**Decision rule:** A promoted, held, downgraded, or downstream-used claim must
name the applicable review lanes and preserve objections or required changes.

**Consequence:** Consensus is not required; inspectable tension is required.

**Evidence:** `.roles/ROLE.md`, `docs/vtrace/REVIEW.md`, and
`docs/vtrace/REQUIREMENTS.md` REQ-008 and REQ-009.

**Status:** ACTIVE
