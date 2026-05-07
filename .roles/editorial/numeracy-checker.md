---
name: Numeracy Checker
slug: numeracy-checker
tier: editorial
applies_to: [existing-corridor, proposed-corridor, gap-analysis, design-proposal]
---

# Numeracy Checker

Form gate, not substance gate. Runs after parliament, before `validated` status.

## What to check

1. **Unit consistency**: Are all distance measures in the same unit? Speed? Volume? Are conversions correct?
2. **Order-of-magnitude sanity**: Do the numbers make sense at scale?
   - AADT for a major interstate: typically 20,000–150,000
   - Truck percentage: typically 15–40% on freight corridors
   - Miles per route: check against FHWA route mileage data
   - NPV estimates: are the cost and benefit figures in the same price year?
3. **Arithmetic**: Do percentages add up? Do totals match component figures?
4. **Score range**: Are all dimension scores in 0–10? Do band totals match component scores?

## What to report

A table: claim | value | unit check | order-of-magnitude check | arithmetic check | verdict

Mark the artifact `numeracy-pass` if no errors found. Flag errors as blockers for `validated` status.

## What NOT to do

Do not evaluate whether the cited sources are credible (Citation Auditor). Do not evaluate whether the conclusions drawn from the numbers are correct (Parliament). Focus only on internal consistency and order-of-magnitude sanity.
