---
reviewer: Elefteriadou
paper: E.1+managed-lanes
review_type: recheck
round: 1
date: 2026-05-07
pp_items_rechecked:
  - PP1.1
verdict: PASS
score: 3/4
---
> **Note:** AI-generated simulated recheck review.

## Items Rechecked

### PP1.1 — 2,400 pcphpl capacity claim does not account for access-point geometry or mountain-corridor grade corrections

**Concern**: The paper used a uniform 2,400 pcphpl capacity figure for all managed-lane corridors. This figure is not defensible as a system-wide assumption: urban corridors with frequent access points operate well below this threshold, and mountain corridors (I-70, I-80) require PCE corrections for heavy vehicles on sustained grades. The result was an NPV ($115B) and B/C (2.3:1) inflated by an optimistic capacity assumption.

**Revision**: Corridor-by-corridor HCM7 capacity table added. Urban corridors (I-95, I-85) receive access-point corrections that reduce effective capacity. Mountain corridors (I-70, I-80) apply PCE factors for sustained grades. The weighted average across the system is 2,108 pcphpl — not 2,400. NPV revised to $101B; B/C revised to 2.0:1.

**Verdict**: PASS, enthusiastically. The HCM7-grounded, corridor-by-corridor approach is exactly what I asked for. The access-point corrections for urban corridors and PCE application for mountain grades are done correctly; the resulting weighted average of 2,108 pcphpl is credible. Critically, the revised $101B NPV at a 2.0:1 B/C ratio is actually more defensible than the original claim — a slightly lower number with a sound methodology is stronger evidence than a higher number from an unsupported assumption. Score rises from 2/4 to 3/4.

**P3 note**: The I-70 PCE of 3.8 on mountain grades is correct for 4%+ grade but the paper should specify which HCM7 exhibit (Exhibit 26-9) was used. Citing the exhibit prevents the next reviewer from having to reconstruct the lookup. This is a note, not a blocking condition.

## Verdict

The corridor-by-corridor HCM7 capacity table resolves my blocking objection completely; the revised NPV and B/C are more defensible than the originals. Score rises from 2/4 to 3/4; paper is ready to advance.
