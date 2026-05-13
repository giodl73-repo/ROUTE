---
name: route-wave
description: "Operate ROUTE waves: inspect active wave, create wave cards, advance pulses, and close waves using waves/PHASES.md."
tags: [route, wave, milestone, execution, planning]
---

# route-wave

Use this skill when the user asks for a wave, next stage, roadmap, milestone
execution rail, or to continue from the active ROUTE wave.

## Procedure

1. Read `waves/PHASES.md`; the first `active` row is the active wave.
2. Read `waves/{active}/WAVE.md`.
3. Report current wave, completed pulses, first planned pulse, and gates.
4. If asked to continue, execute the first planned pulse using `/route-pulse`.
5. If asked to close, ensure all pulses are done, write `CLOSE.md`, update
   `waves/PHASES.md`, run gates, and commit.

## Wave Card Minimum

- frontmatter: `wave`, `date_open`, `status`
- mission
- opening rule
- inputs inherited
- pulse status table
- done criteria
- non-goals

## ROUTE Gates

- `cargo fmt -p route`
- `cargo test -p route`
- relevant `route ... --gate` commands
- `route optimizer-manifest --gate`
- `route release-manifest --gate`
- `powershell -ExecutionPolicy Bypass -File scripts/check-mileposts.ps1 -SkipTests`
