# R1 - Numeracy Checker

Verdict: **hold, canonical contradiction repaired**

## Findings

- **BLOCK** - Zero demand made the stored 96.2% and 100% throughput-retention
  values mathematically unusable.
- **BLOCK** - k=0 on an operating interchange cannot support connector
  arithmetic.
- **WARN** - The annual probability was calculated from stale, non-annualizable
  snapshot counts.
- **NOTE** - The 80% within four hours gate cannot be evaluated until demand and
  recovery are loaded.

## Disposition

`data/t1-intersection-failures.csv` now withholds annual probability,
throughput retention, and reroute metrics. Connector count is held until a
physically correct baseline exists.
