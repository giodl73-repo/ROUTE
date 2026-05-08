---
reviewer: David Neumark
persona: David Neumark — Distinguished Professor of Economics, UC Irvine; Director, Economic Self-Sufficiency Policy Research Institute; specialist in labor and regional economics
round: 1
date: 2026-05-07
score: 3/4
---

> **Note:** This is an AI-generated simulated review, written by Claude in the voice of a named-expert persona. It is not an actual review by the named person and does not reflect their views or endorsement.

## Overall

The NPV calculation ($115B aggregate, B/C 2.3:1) is a credible first-order estimate, and the paper's use of a 30-year horizon and 7% social discount rate is consistent with federal infrastructure appraisal practice (OMB Circular A-94). My main concern is the induced demand assumption: the paper's central NPV claim rests on the proposition that freight demand grows at 1.8%/yr while passenger demand grows at 2.4%/yr, and that managed freight lanes successfully contain the freight-induced demand response while the GP lanes absorb the passenger growth. This is not demonstrated — it is assumed — and the NPV is sensitive to it.

Score: 3/4 — the NPV methodology is sound in structure; the induced demand treatment needs to be either demonstrated or clearly labeled as an assumption with sensitivity analysis.

## What Works

The 30-year horizon and 7% discount rate are appropriate for federal infrastructure investment appraisal and consistent with FHWA TIGER/INFRA grant evaluation methodology. Using OMB-standard parameters makes the results directly comparable to the benefit-cost analyses submitted in FHWA competitive grant applications, which is the paper's natural policy audience.

The B/C ratio range (I-95 at 3.1:1 to I-90 at 1.6:1) is methodologically important. A program with corridor-level variation in B/C ratios is being honest about where the economics work and where they don't. The I-90 finding (1.6:1, rural-dominant) is particularly valuable — it identifies the lower bound of the program's economic case and provides a threshold for corridor selection.

The transponder tolling mechanism ($0.05/mile) provides a plausible capital recovery pathway. The $2.3B/yr revenue figure creates a 30-year discounted revenue stream that offsets a meaningful fraction of the $121B capital cost, and the paper is right to include it in the NPV rather than treating the program as purely grant-funded.

## What Doesn't Work

The core NPV claim depends on the freight demand growth differential (1.8%/yr freight vs. 2.4%/yr passenger). This differential drives the result that managed freight lanes provide permanent throughput benefits rather than temporary ones that are eroded by induced demand. But the paper does not demonstrate this differential — it cites it as an assumption without source attribution. Two questions:

First, what is the source for the 1.8%/yr freight demand growth rate and the 2.4%/yr passenger growth rate? FAF4 (Freight Analysis Framework) and FHWA vehicle miles traveled projections provide this data; the paper should cite them specifically and report the time horizon and scenario (reference case, high, low) these figures represent.

Second, the managed lane concept prevents induced passenger demand by design (freight-only access), but does it prevent induced freight demand? If freight shippers respond to lower transit times on managed freight lanes by shifting freight from rail to road (modal shift) or by increasing shipment frequency (just-in-time intensification), the managed lanes will experience induced freight demand growth above the 1.8%/yr baseline. The paper does not model this possibility, and it is not implausible — lower road freight cost is the standard inducement mechanism for modal shift from rail to truck.

If induced freight demand grows at 2.4%/yr rather than 1.8%/yr (the passenger growth rate, as a bounding scenario), how much does this reduce the aggregate NPV? This is a named sensitivity that should be reported.

The aggregate NPV ($115B) and B/C (2.3:1) are presented as point estimates without confidence intervals. Given the sensitivity to the capacity assumption (flagged by Elefteriadou), the platooning penetration assumption (flagged by McKinnon), and the induced demand growth rate, a Monte Carlo or structured two-way sensitivity analysis is appropriate. At minimum: NPV as a function of (managed lane capacity per direction) × (freight demand growth rate) would establish the robustness of the B/C>2.0 finding.

## The Question I'd Push On

The paper's NPV calculation implicitly models the no-build counterfactual as a status quo with freight demand growing at 1.8%/yr on congested GP lanes. But the relevant counterfactual for a federal investment appraisal is not "do nothing" — it is "best alternative use of $121B." At $121B, the alternative investment universe includes: (1) full rehabilitation of 35,000 structurally deficient bridges on the NHS; (2) a national intermodal freight infrastructure program; (3) continued GP lane expansion on the same 8 corridors.

If the B/C of GP lane expansion on the same corridors is 1.8:1 (because it induces both freight and passenger demand, reducing the net freight throughput gain), then the managed freight lane program's 2.3:1 ratio represents a 28% improvement over the best alternative — a more honest and compelling framing than a comparison against a do-nothing baseline. Has the paper computed the B/C for the most plausible counterfactual investment?
