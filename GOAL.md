# Current Goal: I-80 Clean-Clone Source Reproducibility - Complete

## Outcome

Every source required or previously claimed by the reviewed I-80 report now has
an explicit source contract.

- 4 sources are ready.
- 6 sources are explicitly excluded pending reviewed adapters.
- 2 ACS sources are blocked on environment-provided credentials.

`npm run reproduce:i80:report` acquires available inputs, emits complete
blockers, preserves the canonical report, and generates a separate comparison
only after the full source gate passes.

## Credential Hold

Set `CENSUS_API_KEY` in the environment and rerun only when the user chooses to
provide the credential. ROUTE never stores or logs the key.

## Closed Wave

`waves/2026-07-11-i80-clean-clone-source-reproducibility/CLOSE.md`

## Next Trigger

Do not open another wave automatically. Wait for a credential-backed
reproduction request or a new bounded objective.
