# R1 - Eisenhower

Verdict: **hold**

## Findings

- **BLOCK** - `design/i80-des-moines-transfer-resilience.md` relied on a
  zero-demand scenario, so it could not demonstrate national freight or defense
  consequence. Fix: require a nonzero source-labeled demand fixture.
- **WARN** - `data/t1-intersection-failures.csv` carried an annual probability
  derived from a snapshot-only window. Fix: withhold annualization until archive
  or repeated-window evidence exists.
- **WARN** - k=0 on an operating interchange is not a credible national-network
  baseline. Fix: reconcile topology and physical connectivity first.
- **NOTE** - Chicago and Donner remain higher-consequence comparison cases, but
  the active wave should not expand until the Des Moines hypothesis is either
  validated or rejected.

## Disposition

The capital package was held. The remaining work is a validation plan with a
null-result gate.
