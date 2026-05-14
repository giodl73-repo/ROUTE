---
wave: t2-beck-transfer-complexity-blocker-relief
pulse: 02
status: done
---

# Pulse 02 - Blocker-Relief Surface

## Deliverable

Add `route t2-beck-transfer-complexity-blocker-relief` and write relief rows
for all accepted transfer-complexity policies.

## Gates

- Regression test proves accepted blockers reduce to zero inside the relief
  artifact.
- `route t2-beck-transfer-complexity-blocker-relief --gate`

## Result

Done in `crates/route-cli/src/main.rs` and
`data/t2-beck-transfer-complexity-blocker-relief.csv`.
