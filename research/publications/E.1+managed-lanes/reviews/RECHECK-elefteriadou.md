---
reviewer: Elefteriadou
paper: E.1+managed-lanes
review_type: recheck
round: 1
date: 2026-05-08
pp_items_rechecked:
  - PP1.1
verdict: PASS
score: 3/4
---
> AI-generated simulated recheck. Not an actual review.

## Items Rechecked

### PP1.1 — 2,400 pcphpl capacity claim does not account for access-point geometry or mountain-corridor grade corrections

**Concern**: The paper used a uniform 2,400 pcphpl capacity figure for all managed-lane corridors. This figure is not defensible as a system-wide assumption: urban corridors with frequent access points operate well below this threshold, and mountain corridors (I-70, I-80) require PCE corrections for heavy vehicles on sustained grades. The result was an NPV ($115B) and B/C ratio (2.3:1) inflated by an optimistic capacity assumption. I scored this 2/4 and flagged it as the blocking item.

**What was done**: A corridor-by-corridor HCM7 capacity table was added in §04. Urban corridors (I-95, I-85) receive access-point corrections; mountain corridors (I-70, I-80) apply PCE factors for sustained grades. The weighted average across the system is 2,108 pcphpl — not 2,400. The NPV was revised from $115B to $101B; the B/C ratio from 2.3:1 to 2.0:1; the annual portfolio benefit from $12.7B/yr to $11.2B/yr.

**Is it satisfactory?** Yes — completely. I read the abstract and the §05 portfolio table. All three corrected numbers (2.0:1 B/C, $101B NPV, $11.2B/yr annual benefit) are now consistent across the abstract and Table 1. The HCM7-grounded, corridor-by-corridor approach is exactly what I asked for. The access-point corrections for urban corridors and the PCE application for mountain grades are done correctly; the resulting weighted average of 2,108 pcphpl is credible. Critically, the revised $101B NPV at 2.0:1 B/C is more defensible than the original claim — a slightly lower number with a sound methodology is stronger evidence than a higher number from an unsupported assumption.

**P3 note**: The I-70 PCE of 3.8 on mountain grades is correct for 4%+ grade but the paper should specify which HCM7 exhibit was used. Citing HCM7 Exhibit 26-9 explicitly prevents the next reviewer from having to reconstruct the lookup. This is a note, not a blocking condition.

## Verdict

All three corrected numbers (2.0:1, $101B, $11.2B/yr) are now consistent across abstract and §05 table. The corridor-by-corridor HCM7 capacity table resolves my blocking objection completely; the revised NPV and B/C are more defensible than the originals. Score rises from 2/4 to 3/4; paper is ready to advance.
