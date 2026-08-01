# ROUTE VERDICT capability assessment

## Decision and scored object

- Assessment: `route-program-capability:2026-07-31`
- Decision supported: identify the next evidence-producing product slice.
- Object class: `program_capability`
- Object: ROUTE at commit `db3c9a86ef9347bd3c0325681a2068847cb01700`
- Perimeter: public Interstate 2.0 research, analysis, review, and design-yield machinery.
- Not scored: the current US road system, I-80 as an approved candidate, or a
  fiscal package.

Scale: `0 missing; 1 designed/partial; 2 executable/bounded; 3 demonstrated`.

## Dimension evidence

| ID | Dimension | Score | Evidence | Strength | Principal hold |
|---|---|---:|---|---|---|
| V | Value | 1 | `docs/reports/route-roi-cost-framework.md`; `docs/reports/roi-without-fake-numbers-report.md` | Defines a reviewable cost/ROI evidence contract. | No numeric current cost, price year, lifecycle estimate, NPV, or competitively priced corridor alternative. |
| E | Effectiveness | 2 | `crates/`; `corpus/`; `gaps/`; `maps/`; `README.md` | Executable scoring, corpus, gap, map, and service-promise machinery. | No promoted corridor with an observed service outcome. |
| R | Resilience | 2 | `gaps/resilience.md`; recursive optimizer and scenario surfaces under `crates/route-*` | Resilience and lower-tier pressure can change analytical results. | No delivered corridor stress/recovery result. |
| D | Deliverability | 1 | `docs/vtrace/`; `waves/2026-07-11-i80-flagship-stabilization/` | Review and narrowing process is explicit. | `design/` contains no promoted corridor specification, delivery owner, procurement path, or schedule. |
| I | Iteration | 2 | adaptive proof tooling under `tools/`; review and wave records | Evidence and requirements can produce a revised analytical successor. | No operating institution has demonstrated response, outcome learning, or fiscal rebalancing. |
| C | Coverage and fair access | 2 | `docs/t3-t4-access-optimization.md`; terminal, rural, port, border, production, and regional evidence | Access is represented across T1–T4 rather than reduced to a national spine. | Candidate-specific incidence, burden, and observed access improvement remain missing. |
| T | Trust | 2 | `docs/reports/route-evidence-posture.md`; `.roles/`; source packs and held I-80 review | Sources, holds, review, and publication limits are inspectable. | Important corridor inputs and external validation remain incomplete. |

Total: **12/21**. This reproduces the TRACKER pilot; adoption creates no score
increase.

## Iteration evidence

| Loop | State | Evidence or hold |
|---|---|---|
| Analytical refresh | demonstrated for program capability | Corpus, gap, optimizer, and adaptive-proof artifacts can be regenerated and reviewed. |
| Operational response | held | No road owner has implemented a ROUTE-selected response. |
| Outcome learning | held | No postimplementation service and burden observations exist. |
| Fiscal rebalancing | held | No admitted candidate cost or Taxlane fiscal effect exists. |

## Hard floors and claims

Safety, service continuity/resilience, rural and terminal access, equity, and
source/claim integrity are applicable and unresolved for any candidate. They
block promotion regardless of the program total.

This assessment allows the claim that ROUTE has a bounded analytical program.
It does not authorize a corridor design, construction, procurement, savings,
allocation, rate, official-plan, or public-deployment claim.

## Next evidence-producing action

Create one real corridor package with a current service and cost baseline, one
bounded improvement, a compatible lifecycle-price alternative, delivery owner
and schedule, access and resilience floors, observation cadence, and rollback
or reopen triggers. I-80 remains hold-and-narrow until that evidence exists.

## `.roles` fixed point

The transportation, freight, rural-access, climate, citation, numeracy, and
scope lenses retain the 12/21 result. The score is reproducible, the empty
`design/` yield is not treated as failure or silently filled, and a strong
analytical program is not described as a delivered transportation outcome.
No critical or major actionable documentation finding remains.

## Validation

- Arithmetic: `1 + 2 + 2 + 1 + 2 + 2 + 2 = 12`.
- Repository whitespace: `git diff --check`.
- Full workspace testing was previously incomplete after exceeding the survey
  window during compilation; this documentation-only adoption makes no new
  implementation claim.
