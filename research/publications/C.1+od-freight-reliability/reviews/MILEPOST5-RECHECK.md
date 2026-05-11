---
paper: C.1+od-freight-reliability
review_type: milepost-5-recheck
date: 2026-05-10
reviewers_rechecked: [lily-elefteriadou, alan-mckinnon, david-neumark]
forum_docket_id: F5-06
verdict: BLUEPRINT-HOLD
---

> **Note:** AI-generated simulated panel recheck for ROUTE quality control. It is not an actual review by the named people and does not reflect their views or endorsement.

# Milepost 5 Recheck — C.1 SLA/PTI Claims

## Question

Do C.1 SLA/PTI claims remain acceptable when direct NPMRDS/FPM validation is absent?

## Short Answer

Yes for research framing as a labeled heuristic. No for Blueprint promotion or publication-grade quantitative claims.

The paper's corridor logic and SLA framing remain useful: PTI is a sensible way to translate link reliability into shipper planning windows, and the NY-LA / HOU-CHI case studies still identify plausible binding constraints. But Milepost 4 now requires a stricter claim boundary. Any claim that a standard or feature package *achieves* a 48-hour SLA, a specific PTI target, or a monetized reliability benefit must remain held until direct observed reliability data or a validated oversaturation model is attached.

## Panel Positions

### Lily Elefteriadou — Traffic Engineering

Prior recheck accepted the paper's BPR limitation paragraph because it disclosed the calibration issue and reframed PTI as a conservative lower-bound heuristic. That remains acceptable for a methods paper.

It is not sufficient for Blueprint. A Blueprint claim needs observed PTI or a validated model at the bottleneck. The current route system correctly labels T1 PTI/SLA as Heuristic. The required next evidence is still:

- NPMRDS/FPM direct PTI extraction for the Bay Area I-80/Donner segments and comparable managed-lane baselines, or
- a queueing/oversaturation model validated against probe data.

Verdict: pass for Forum as a held heuristic; hold for Blueprint.

### Alan McKinnon — Freight Economics

The SLA window idea is operationally meaningful, but the benefit cannot be carried into Blueprint as a dollar claim without commodity and carrier segmentation. High-value JIT freight, refrigerated freight, parcel, bulk, and low-value truckload freight do not value reliability equally.

The $8.2B annual reliability framing can remain a motivating order-of-magnitude research result if caveated. It should not become a capital-allocation result until the model separates:

- carrier operating cost,
- shipper inventory/reliability cost,
- commodity value/time sensitivity,
- toll or managed-lane access cost,
- take-up rate.

Verdict: pass for research motivation; hold for investment sequence and NPV.

### David Neumark — Economics / Benefit-Cost

The paper still lacks a shipper-level benefit model. A PTI improvement does not automatically imply a welfare gain of the same magnitude. Some benefits may be transfers, some may be captured by tolls, and some freight may not use the managed facility.

Before Blueprint uses C.1, it needs at least one worked carrier/shipper example and a range of take-up assumptions. The benefit claim should be expressed as a sensitivity range, not a point estimate.

Verdict: hold for Blueprint economics; acceptable as a labeled analytic hypothesis.

## Earned Claims

| Claim | Forum status |
|---|---|
| PTI is an appropriate reliability metric for translating traffic operations into shipper planning windows | Earned |
| NY-LA and HOU-CHI are useful freight reliability case studies | Earned |
| BPR-derived PTI can be retained as a disclosed heuristic / lower-bound sensitivity | Earned |
| I-69 completion has a geometry-grounded HOU-CHI routing benefit independent of the PTI model | Earned |

## Held Claims

| Claim | Hold reason | Required evidence |
|---|---|---|
| T1 standards achieve PTI <= 1.15 | Direct observed PTI validation absent | NPMRDS/FPM extract or validated queueing model |
| I2.0 enables a 48-hour NY-LA SLA as a program claim | SLA depends on heuristic PTI and service/take-up assumptions | Corridor travel-time distribution, relay/driver operations validation, shipper window sensitivity |
| $8.2B annual reliability cost can drive investment ranking | Cost decomposition and commodity weighting remain incomplete | Segment truck-hours, commodity mix, reliability value, toll/take-up range |
| Managed freight lanes produce net shipper benefit | Toll and take-up assumptions are not modeled | Worked carrier/shipper example and sensitivity table |

## Required Claim Labels

| Artifact / command family | Required label |
|---|---|
| `route standards-proof` T1-OPS-PTI | Heuristic until NPMRDS/FPM validation |
| `route pressure-scenarios` SLA rows | Heuristic; direct PTI validation missing |
| Blueprint feature packages using C.1 | Held unless they use sensitivity ranges and source labels |
| Public/policy copy | Must not state 48-hour SLA as achieved; may state "target" or "modeled heuristic" |

## Decision

F5-06 completes with a Blueprint hold.

C.1 remains usable in the Forum as a reviewed research artifact and as a source of hypotheses for Blueprint. It does not clear the Blueprint gate for SLA/PTI or reliability-cost claims.

## Follow-Up Tasks

1. Keep `route standards-proof --gate-blueprint` failing for T1-OPS-PTI until observed PTI/source validation is attached.
2. Add an NPMRDS/FPM extraction task or partner-data task to the Blueprint evidence backlog.
3. When Blueprint packages are drafted, require any SLA claim to name PTI source, driver/relay operating mode, and take-up/toll assumptions.
4. Treat C.1 reliability dollars as sensitivity inputs, not final NPV, until commodity weighting and shipper/carrier examples are added.
