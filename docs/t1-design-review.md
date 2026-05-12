# T1 Design Review

The T1 design review joins the promise-horizon selector to Beck-map diagnostics.
It answers a narrower question than the selector:

```text
Which selected T1 routes are accepted design lines, and which still need policy?
```

Canonical rows live in `data/t1-design-review.csv`.
Policy action definitions live in `data/t1-design-policy-actions.csv`.
Score-backbone exception decisions live in `data/t1-score-exceptions.csv`.

Regenerate and gate with:

```powershell
cargo run -q -p route -- t1-design-review --gate
cargo run -q -p route -- t1-design-policy --gate
cargo run -q -p route -- t1-score-exceptions --gate
```

## Current Interpretation

- `I5`, `I10`, `I75`, `I70`, `I35`, and `I20` are accepted T1 design lines.
- `I95`, `I80`, `I90`, and `I40` are promise-spine lines with Beck overlap policy still to resolve.
- `I69` is cut by the score-exception ledger because it is not fully built/connected as the current T1 promise path and has no current T1 promise pair.
- `I84` rises into the 11-route set as a conditional built-network exception and needs Beck diagnostic integration.
- `I64`, `I94`, `I395`, `I55`, and `I45` are current cutline candidates held outside the 11-route budget.

## Design Rule

T1 should be nationally legible because it carries 48h/36h promises. A route
selected without promise pairs must have a named national relay, resilience, or
market-coverage justification, or it should be demoted/replaced in the next
selector pass.
