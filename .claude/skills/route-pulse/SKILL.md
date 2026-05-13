---
name: route-pulse
description: "Execute the first planned pulse in the active ROUTE wave, update checkboxes, run gates, and commit."
tags: [route, pulse, execute, gates, commit]
---

# route-pulse

Use this skill when the user says continue/go/next slice and an active wave
exists.

## Procedure

1. Read `waves/PHASES.md` and active `WAVE.md`.
2. Pick the first `planned` pulse in the active wave unless the user names one.
3. Read the pulse plan completely.
4. Inspect relevant code/docs/data before editing.
5. Implement the deliverables.
6. Regenerate artifacts named in the plan.
7. Run the expected gates plus `cargo test -p route` when Rust changes.
8. Update pulse checkboxes and active wave status.
9. Commit with message `{Wave} pulse {NN}: {short outcome}` or a concise
   project-style equivalent.

## Never

- Do not skip gates because a row is held; held rows must be explicit.
- Do not erase a blocker just to make a gate pass.
- Do not leave generated artifacts stale after changing their producers.
