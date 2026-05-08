---
reviewer: David Neumark
paper: E.2+i2-framework
review_type: recheck
round: 1
date: 2026-05-08
pp_items_rechecked:
  - PP1.1
verdict: PASS-WITH-NOTE
score: 3/4
---
> AI-generated simulated recheck. Not an actual review.

## Items Rechecked

### PP1.1 — NPV Reconciliation: Does the $298B claim now close arithmetically?

**Concern**: The $298B aggregate NPV was stated without a reconciliation table showing how component-level benefits sum to that figure. At a 7% social discount rate over 30 years, the implied annuity factor is 12.41, and the claimed aggregate NPV must be arithmetically traceable from annual benefit flows through that factor to a gross PV, and then to an NPV after subtracting the present value of capital. That chain was absent. I scored this 2/4 and flagged it as the blocking PP1.1 item.

**What was done**: Section 3 now includes Table 2 ("I2.0 Portfolio Benefit Reconciliation at 7%/30 Years") providing the full arithmetic chain: capital outlay, annual benefit, gross PV (= annual benefit × 12.41), and NPV (= gross PV − capital) for each component. A note below the table explains the $246B–$298B NPV range by introducing capital timing — when capital is discounted at Phase midpoints (Year 5 for Phase 1, Year 20 for Phase 2), the PV of capital falls from $251.5B to approximately $141B (conservative) or $89B (if treated as fully productive from Year 1), yielding NPVs of $246B and $298B respectively. Sensitivity analysis at 5% and 10% discount rates is included.

**What was also corrected**: Component 1 figures in §03 were updated to match the E.1 revisions — $12.7B/yr → $11.2B/yr annual benefit; $86.4B → $74.8B NPV; 2.3:1 → 2.0:1 B/C. The NPV reconciliation table Component 1 row reflects these corrected figures. E.2 now cites E.1 figures consistently (2.0:1, $11.2B/yr) throughout.

**Is it satisfactory?** Yes — the arithmetic now closes. The range is properly labeled: $246B as conservative, $298B as upper bound, both reported rather than only the favorable figure. The sensitivity analysis at 5% and 10% is appropriately included.

**P3 note**: The portfolio total table (Table 1) now shows $228.9B NPV (ex-transit) rather than a figure in the $298B range — the $298B headline is explained in the reconciliation note as the upper end of a capital-timing range. The abstract of E.2 should be updated to reflect this revised portfolio figure and to clarify that $298B is the upper bound rather than the central estimate. This framing issue is a P3 item for the next draft; it does not block promotion.

## Verdict

NPV reconciliation is present and arithmetic closes. The Component 1 correction is internally consistent with the E.1 revisions. Score remains 3/4 — no score change because the underlying methodology was always credible; the issues were presentation and cross-paper consistency. The abstract framing note is a P3 item for the next revision cycle and does not block promotion.
