---
name: Parliament Review — Moses — route-rust-architecture
slug: R1-moses-route-rust-architecture
type: review
artifact: specs/2026-05-06-route-rust-architecture.md
voice: moses
round: 1
status: draft
rubric_version: v1.0
author: moses
created: 2026-05-06
---

> **Note:** This is an AI-generated simulated review, written by Claude in
> the voice of a named-expert persona. It is not an actual review by the
> named person and does not reflect their views or endorsement.

# Review — Robert Moses

## Overall

This is a buildable spec. I have built larger things from worse plans. The pipeline is linear, the crate separation is clean, and the anchor-first principle avoids the fatal mistake of most infrastructure planning: building the whole blueprint before proving you can build anything. My concerns are about join failure handling — every join in this pipeline will fail on some percentage of records, and the spec is silent on what that means for scores — and about the ATRI data story, which is a hand-curated CSV pretending to be a data pipeline.

## What works

**Anchor-first, manual-first (§8)**: "Phase 1 is manual-first. Run the anchor (I-80) by hand before scripting anything." This is how you build things. The Triborough Bridge was not designed as a system on day one; it was built as three bridges that happened to connect. Prove the pipeline works for I-80, then automate for the rest. Anyone who wants to automate before proving the manual case is building a plan, not a road.

**Crate separation (§2)**: `route-data` has one job. `route-network` has one job. `route-score` has one job. I built the West Side Highway this way: one contractor per section, clear scope, no overlap. When something fails you know exactly where the failure is.

**`route build` serializes to cache (§6)**: Parse once, analyze many times. This is correct. You don't repave a road every time a truck crosses it. Build the graph once, score from it.

**`route score-all` with Rayon parallelism (§6)**: If you have 70 corridors to score, score them in parallel. Time is money. Rayon is the right tool.

**§9 (What This Spec Does Not Cover)**: A spec that knows what it isn't is more useful than one that claims to be everything. Web interface, real-time traffic, climate rasters — all correctly deferred. I have seen a thousand infrastructure projects fail because they tried to solve everything at once.

## What doesn't work

**Join failure handling is absent (§4.1, §8)**: Every join in this pipeline will produce failures. NHS route IDs do not always match HPMS route IDs. NBI bridge coordinates don't always snap to the NHS centerline within 0.01°. FAF5 regions don't align with NHS route segments. The spec says "N join failures" appears in the `route build` report and nothing else. What happens to scores when attributes fail to join? Does A2 (Freight Intensity) score 0? Does the corridor get dropped from the corpus? Does it get marked as `estimated`? This is not a minor implementation detail — it determines whether the corpus is complete or full of silent holes. A score of 0 on A2 because the join failed is indistinguishable from a score of 0 because there's genuinely no freight traffic. That is a data quality problem, not an analysis result.

**ATRI data is a liability (§4.1)**: "ATRI bottleneck seed — CSV (hand-curated) — committed to `data/atri-bottlenecks.csv`." Hand-curated data committed to the repo becomes stale immediately and silently. ATRI publishes an annual report; the hand-curated CSV will not update with it. This is acceptable for Phase 1 but must be flagged prominently as a maintenance liability, not buried in a table row. The A1 (Throughput Gap) and A3 (Speed Reliability) scores that depend on ATRI data carry this limitation.

**Scoring functions as binary constants (§5)**: "Scoring anchor maps are compiled into the binary as constants." This means changing a scoring anchor — which the calibration pass will almost certainly require — needs a recompile and a new binary. For a scoring system that is explicitly designed to evolve via calibration, this is the wrong architecture. Anchors should be in a config file (`~/.route/config/scoring.toml`) loaded at runtime. Change the config, re-score without rebuilding.

## The question I'd push on

A1 (Throughput Gap) is my primary stake. The spec defines it as "current volume vs. designed capacity; congestion severity across route miles." But the data source for this is HPMS AADT — annual average daily traffic. I-80 through Nevada averages 10,000 vehicles/day. I-80 through the Bay Area averages 250,000. Averaging these together for "I-80" produces a number that describes neither corridor accurately. How does `route score` handle corridors with extreme within-route variance in AADT? Does it average, take the maximum, or report by segment? If it averages, the A1 score for I-80 will be meaningless. I want the answer to this before we score the anchor.
