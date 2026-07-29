# Current Goal: Public Edge Credibility - In Progress

## Outcome

Make ROUTE's **outside edge** match its honesty culture:

1. One loud public proof path that fails closed without secrets.
2. Explicit lab posture: design yield empty until review promotes.
3. Start dismantling the `route-cli` monolith without behavior change.

## Done this slice (2026-07-29)

- `npm run proof:public` + `docs/how-to/public-proof.md`
- README current lab posture table
- `BACKLOG.md` living queue (waves remain archive)
- Extract clap surface to `crates/route-cli/src/cli.rs`

## Still open

- Continue `run_cli` modularization (`BACKLOG` S1-S2)
- Keep I-80 packet hold-and-narrow; optional ACS reproduce only with user-supplied key
- No automatic design wave

## Credential hold (unchanged)

Set `CENSUS_API_KEY` in the environment only when deliberately running
`npm run reproduce:i80:report`. ROUTE never stores or logs the key.

## Closed prior wave

`waves/2026-07-11-i80-clean-clone-source-reproducibility/CLOSE.md`
