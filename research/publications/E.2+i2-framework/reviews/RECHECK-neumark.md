---
reviewer: David Neumark
paper: E.2+i2-framework
review_type: recheck
round: 1
date: 2026-05-07
pp_items_rechecked: [PP1.1]
verdict: PASS-WITH-NOTE
score: 3/4
---

> **Note:** AI-generated simulated recheck review.

## Items Rechecked

### PP1.1 — NPV Reconciliation: Does the $298B claim now close arithmetically?

My blocking concern in the Round 1 review was direct: the $298B aggregate NPV was stated without a reconciliation table showing how component-level benefits sum to that figure. At a 7% social discount rate over 30 years, the implied annuity factor is 12.41, and any claimed aggregate NPV must be arithmetically traceable from annual benefit flows through that factor to a gross PV, and then to an NPV after subtracting the present value of capital. That chain of reasoning was absent.

The revision adds Section~\ref{sec:npv-reconcile} (Table 2: "I2.0 Portfolio Benefit Reconciliation at 7\%/30 Years") which provides precisely this chain. Six components are listed with capital outlay, annual benefit, gross PV (= annual benefit × 12.41), and NPV (= gross PV − capital). The column arithmetic is transparent and checks out: $31.2B/yr × 12.41 = $387.2B gross PV; $387.2B − $251.5B capital = $135.7B NPV under the most conservative treatment (capital undiscounted). The note below the table then explains the $246B–$298B range by introducing capital timing: when capital is discounted at Phase midpoints (Year 5 for Phase 1, Year 20 for Phase 2), the PV of capital falls from $251.5B to approximately $141B (conservative, multi-phase discount) or $89B (if treated as fully productive from Year 1), yielding NPVs of $246B and $298B respectively.

This is satisfactory. The arithmetic now closes. The range is properly labeled — $246B as conservative, $298B as upper bound — and both are reported rather than only the favorable figure. The sensitivity analysis at 5% and 10% discount rates ($390B and $180B respectively) is appropriately included and shows the investment is positive-NPV across the full span of standard public discount rates. My PP1.1 concern is resolved.

### PP1.1 (residual) — Capital Timing Assumption as Primary Uncertainty Driver

The resolution of PP1.1 surfaces a secondary issue that I want to flag as a note rather than a blocking item. The $246B–$298B range is entirely explained by the capital timing assumption: whether capital is discounted at Phase midpoints or treated as Year 1 productive. This is a $52B spread driven by a single modeling choice, and the paper treats it in a footnote-equivalent note paragraph rather than as the primary sensitivity driver it actually is.

At 7% discount, the difference between Year 1 and Year 5 capitalization for an $89B Phase 1 tranche is $89B × (1 − 1/1.07^5) = approximately $28B in present value. That is a meaningful sum, and the paper's reporting of "both are positive by a substantial margin" understates how much work this assumption is doing. The appropriate treatment would be to label the capital timing assumption as Sensitivity S1, present the $246B–$298B range as the primary result with equal billing to both endpoints, and defer to the reader's judgment about which timing assumption is more appropriate for a program of this structure. As written, the note reads as though $298B is the headline and $246B is the conservative hedge; the arithmetic suggests they should be co-equal.

I will not block promotion on this point. The reconciliation table is present, the arithmetic is transparent, and the sensitivity analysis is included. But the revision plan for the next draft should address the framing of capital timing as the primary uncertainty driver, not a secondary qualification.

## Verdict: PASS-WITH-NOTE

The arithmetic now closes and the reconciliation table satisfies my PP1.1 requirement. Score remains 3/4 — no score change because the underlying methodology was always credible; the issue was presentation, not substance. The capital timing framing note is a P3 item for the next revision cycle and does not block promotion.
