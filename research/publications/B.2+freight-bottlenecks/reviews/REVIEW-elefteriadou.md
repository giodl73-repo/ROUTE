---
reviewer: elefteriadou
persona: Lily Elefteriadou — traffic engineer, University of Florida Transportation Institute, HCM technical committee
round: 1
date: 2026-05-07
score: 3/4
---

> **Note:** This is an AI-generated simulated review, written by Claude in the voice of a named-expert persona. It is not an actual review by the named person and does not reflect their views or endorsement.

## Overall

The bottleneck identification methodology is sound and the ATRI integration is technically appropriate. The paper's capacity analysis is where I have concerns: the V/C thresholds used to define bottlenecks and the PTI targets used in the implications section are not consistently anchored to HCM Level of Service standards, making it difficult to compare results against the established traffic engineering literature. The I-95 Northeast Corridor PTI of 1.8-2.2 is cited without a source, and PTI targets (≤1.15 for T1) need HCM-based justification.

## What Works

**ATRI data integration is technically competent.** The GPS-derived truck travel time methodology (compare actual vs. free-flow, apply $150/hr operational cost) is correctly understood. The paper correctly characterizes ATRI's data as location-specific rather than corridor-average — an important distinction when interpreting why I-95's corridor-average A1 score of 5.2 understates the extreme congestion in the New York-to-Baltimore segment.

**Bottleneck density normalization is the right metric.** Normalizing by corridor length (ATRI locations per 100 route miles) is correct: a 3,000-mile corridor with 5 bottlenecks is different from a 100-mile corridor with 3. The paper applies this consistently and the interpretation is sound.

**I-40 validation is clean.** From a traffic engineering standpoint, V/C = 0.84 is consistent with LOS D/E — operating at or near capacity but without the severe breakdown seen at V/C > 0.95. The prediction (no ATRI top-50 bottlenecks at this V/C level) and the ATRI confirmation are methodologically meaningful. This is the paper's best empirical result.

**Pattern 1 (Atlanta concentration) is correctly attributed.** The paper correctly identifies that I-285 congestion is primarily an interchange weaving problem — the density of T1/T2 crossings (I-75, I-85, I-20) within a short beltway section creates multiple merge-weave-diverge sequences that operate as coupled bottlenecks. This is a geometric observation consistent with HCM weaving segment analysis.

## What Doesn't Work

**PTI targets (≤1.15 for T1) lack HCM grounding.** Section 8.3 proposes PTI ≤ 1.15 as the T1 standard. PTI = 1.15 means the 95th-percentile travel time is 15% above free-flow — a fairly stringent target. Where does this threshold come from? The FHWA Freight Performance Measures program uses a 1.50 threshold as the "significant delay" threshold; NCHRP Report 618 discusses TTI thresholds for reliability assessment. A PTI of 1.15 would correspond to roughly LOS C reliability conditions. The paper should cite the standard or program from which 1.15 is derived, or explain how the ROUTE program set this target. Without this, readers cannot benchmark the investment implications against industry practice.

**V/C computation method is not specified for the bottleneck density analysis.** Section 3.3 (Bottleneck Density) and Section 4.2 compare ROUTE A1 scores (based on 90th-percentile AADT proxy for V/C) against ATRI bottleneck locations, finding Spearman ρ = 0.67. But the specific V/C computation used for A1 scoring is not described here — is it based on HPMS AADT divided by HCM-computed capacity (using lane count, facility type, and free-flow speed), or a simpler AADT/design-capacity ratio? For a TRR methods section, the V/C computation formula and its HCM basis (if any) must be specified.

**I-95 PTI 1.8-2.2 is uncited.** The implications section (8.3) states the I-95 Northeast Corridor has "current PTI 1.8-2.2" as the basis for the managed lane investment priority. This is a specific quantitative claim that requires a source. FHWA Freight Performance Measures publishes PTI by NHS segment; if the 1.8-2.2 range is from that dataset, cite it. If it is from the ROUTE analysis, explain the estimation method. An uncited PTI range is the kind of claim that returns from TRR peer review with a mandatory revision.

## The Question I'd Push On

The paper recommends managed freight lanes as the primary T1 investment. The HCM and TRB literature on managed lanes (HOT lanes, truck-only lanes) consistently shows that bottleneck relief depends critically on ramp spacing and weaving length — geometric factors that are not analyzed here. For I-95 in the Baltimore-to-New York segment, where the bottlenecks (Fort Lee, Baltimore I-695, Philadelphia I-476) are associated with interchange clusters rather than midblock segments, managed freight lanes require either new interchange geometry or restrictions on freight lane access points. Has the paper considered whether the physical geometry of the top-9 I-95 bottleneck locations is compatible with through-managed-lane operation, or whether the interchange weaving configuration requires a different intervention geometry?
