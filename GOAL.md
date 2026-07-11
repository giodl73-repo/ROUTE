# Current Goal: I-80 Clean-Clone Source Reproducibility

## Mission

Make every source required by the reviewed I-80 report reproducible from a
clean clone or explicitly machine-blocked with an authoritative source, access
mode, and next action.

## Why This Matters

ROUTE now protects the reviewed corpus from incomplete overwrites, but a clean
checkout cannot yet acquire every source used by the current I-80 measurements.
The next step is to replace hidden local cache assumptions with explicit
acquisition, credential, parse, coverage, and blocker contracts.

## Active Wave

`waves/2026-07-11-i80-clean-clone-source-reproducibility/WAVE.md`

## Success Criteria

- Every guarded I-80 report input has a source-contract row.
- One command acquires available inputs and emits blockers for the rest.
- Credentials are environment-provided and never logged or committed.
- Source year, parser status, coverage, and claim readiness remain distinct.
- The reviewed corpus is regenerated only when the complete source contract
  passes.

## Opening Rule

Download success is not evidence acceptance. No missing source may be replaced
with a success-shaped default.

## Immediate Work

1. Inventory required sources and access modes.
2. Orchestrate no-credential sources with parse and coverage gates.
3. Add credential support and adapter decisions.
4. Prove clean-clone regeneration or emit a complete blocker record.

## Prior Wave

I-80 Flagship Stabilization closed on 2026-07-11 with a Parliament
`hold and narrow` decision and no capital treatment approval. See
`waves/2026-07-11-i80-flagship-stabilization/CLOSE.md`.
