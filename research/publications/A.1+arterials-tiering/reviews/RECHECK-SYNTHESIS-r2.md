---
paper: A.1+arterials-tiering
round: 2
recheck_date: 2026-05-08
recheck_reviewers: [hanson, adamic]
recheck_verdicts: [PASS, PASS]
scores_before: {hanson: 3, adamic: 3, transport-policy: 3, traffic-engineer: 3, freight-economist: 3}
scores_after: {hanson: 3, adamic: 3, transport-policy: 3, traffic-engineer: 3, freight-economist: 3}
avg_before: 3.0
avg_after: 3.0
min_before: 3
min_after: 3
stage: ready
---

# Recheck Synthesis — A.1+arterials-tiering Round 2

## Result

Both recheck reviewers pass. **Paper remains at `ready`. Stage: ready.**

| Reviewer | After R1 Recheck | R2 Verdict | Change |
|---|---|---|---|
| Susan Hanson (Transport Geographer) | 3/4 | PASS — 3/4 | held |
| Lada Adamic (Network Scientist) | 3/4 | PASS — 3/4 | held |
| Transport Policy | 3/4 | not rechecked | held |
| Traffic Engineer | 3/4 | not rechecked | held |
| Freight Economist | 3/4 | not rechecked | held |
| **Mean** | **3.0/4** | **3.0/4** | **held** |
| **Min** | **3/4** | **3/4** | **held** |

## What This Recheck Covers

Round 2 rechecks are post-write factual corrections only — not a new methodological review. Both P1 items (α circularity, B2 reliability) were resolved in the Round 1 recheck. The corrections reviewed here were caught by cross-checking against `scores-all.csv` after writing was complete.

**Tier label corrections (Hanson recheck).**
- I-110: "drops to T4" corrected to "drops to T2." Cross-checked against scores-all.csv: I-110's centrality-adjusted composite falls in the T2 band. The T4 label was inconsistent with Table 2's tier counts in the same section.
- I-880: "drops to T2" corrected to "drops to T3." Cross-checked against scores-all.csv: I-880 is regionally but not nationally central, placing it in T3. Both labels now match the corpus data.

**Abstract tier count correction (Adamic recheck).**
- Abstract T3/T4 counts corrected from 80/114 to 61/133. The original abstract mixed aggregate-score T3 (80) with centrality-adjusted T4 (133), conflating two different classification tables. The corrected counts match Table 2 exactly. Sum check: 8 + 25 + 61 + 133 = 227. Internal consistency restored.

## No New Issues Introduced

The corrections are factual: prose labels now match scores-all.csv; abstract counts now match Table 2. No methodological revision was required. No new P1 or P2 items emerged from this recheck.

## Stage

Paper remains `ready` for venue submission. No further review required.
