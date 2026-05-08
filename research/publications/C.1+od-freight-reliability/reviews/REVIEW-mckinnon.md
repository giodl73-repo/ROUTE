---
reviewer: Alan McKinnon
persona: Alan McKinnon, Professor of Logistics, Kühne Logistics University, Hamburg; Emeritus Professor, Heriot-Watt University
round: 1
date: 2026-05-07
score: 3/4
---

> **Note:** This is an AI-generated simulated review, written by Claude in the voice of a named-expert persona. It is not an actual review by the named person and does not reflect their views or endorsement.

## Overall

This paper makes a useful contribution by combining O-D corridor analysis with PTI-based shipper commitment modeling and the I2.0 managed lane concept. The $8.2B combined annual reliability cost figure is a strong motivating statistic, and the ATRI cost model is a credible foundation. My principal concerns are about the order-of-magnitude validation of that figure and whether the decomposition between the two corridors ($5.7B NY-LA, $2.5B HOU-CHI) is defensible at the stated precision. Score: 3/4.

## What Works

The use of ATRI's Operational Costs of Trucking as the unit cost foundation is the right choice — it is the most widely cited and regularly updated source for US trucking cost benchmarks, and using it as the basis for the reliability cost calculation is methodologically sound. The PTI-to-shipper-commitment conversion (SLA window = trip time × PTI) is a clean and interpretable translation that freight shippers will recognize from practice.

The corridor pairing — NY→LA and HOU→CHI — captures the two most important inter-regional freight movements in the US freight economy. The binding constraint identification (Donner Pass, Bay Area V/C, Dallas interchange) is consistent with FHWA bottleneck studies and ATRI's annual Top Truck Bottleneck report. The I-69 completion analysis is well-motivated: the 290-mile shortening and elimination of two high-V/C interchange nodes (Dallas, St. Louis) is a credible operational benefit.

## What Doesn't Work

The $8.2B annual reliability cost estimate requires more transparent construction. The paper should show: (a) annual freight VMT on each corridor segment, (b) loaded truck-hours per year experiencing PTI conditions above 1.0, (c) ATRI cost rate applied ($/hour), and (d) reliability premium multiplier. Without this decomposition, the reader cannot assess whether $5.7B for NY-LA is plausible relative to corridor volume.

As a rough check: I-80 Bay Area carries approximately 280,000 vpd with roughly 8–12% heavy truck share — approximately 22,000–34,000 trucks per day. At a reliability delay of, say, 45 minutes per truck due to PTI conditions and an ATRI marginal cost of ~$90/hour, the annual reliability cost for that segment alone approaches $600M–$900M for the Bay Area section. Scaling across the full 2,800-mile corridor plausibly reaches the stated range — but the paper should show this arithmetic explicitly, not present the $5.7B as a black-box output.

The HOU-CHI $2.5B figure deserves similar decomposition. I-45/I-35/I-55 are all high-volume corridors, but the Dallas V/C 1.9+ condition is concentrated on a relatively short urban segment; the paper should state what proportion of the annual reliability cost is attributable to the Dallas node versus the St. Louis node versus the corridor overall.

## The Question I'd Push On

What is the marginal shipper reliability benefit — in dollars per year — of moving the NY-LA PTI from 1.86 to 1.15 via I2.0 managed lanes, and how was that benefit calculated? The paper states the SLA window shrinks from 80 hours to 48 hours, which is a 40% improvement in shipper planning buffer. But not all freight is time-sensitive to the same degree: bulk commodities bear different time-cost than JIT manufacturing supply. If the $5.7B reliability cost is computed against a uniform value-of-reliability, that overstates the benefit for bulk and understates it for high-value freight. A commodity-weighted reliability cost would strengthen the paper materially.
