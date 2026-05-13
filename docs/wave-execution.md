# ROUTE Wave Execution

ROUTE uses Mileposts for public phase meaning and waves for execution control.
Waves are borrowed from the Craft artifacts pattern: an active wave has a
mission, pulse plans, optional review panels, generated evidence, and close
notes. Pulses are the committable units.

## Layout

```text
waves/
  PHASES.md
  YYYY-MM-DD-wave-slug/
    WAVE.md
    plans/
      pulse-01.md
      pulse-02.md
    panels/
    forks/
    CLOSE.md
```

## Skills

Local skills live under `.claude/skills/`:

| Skill | Purpose |
|---|---|
| `/route-wave` | Find active wave, report status, create/advance/close waves. |
| `/route-plan` | Draft a wave or pulse plan with gates, non-goals, and roles. |
| `/route-pulse` | Execute the next planned pulse from the active wave. |
| `/route-review` | Run `.roles` persona review over a wave, pulse, spec, or implementation. |
| `/route-fork` | Materialize one pulse plus role context into a single fork file for agent execution. |

## Operating Contract

1. Start by reading `waves/PHASES.md` and the active wave `WAVE.md`.
2. Pick the first `planned` pulse unless the user names a pulse.
3. Read the pulse plan completely. The pulse file is the execution contract.
4. Load named governing roles from `.roles/` when the pulse calls for review or
   role-specific judgment.
5. Implement, regenerate artifacts, run gates, update checkboxes, and commit.
6. Close the wave only when `WAVE.md`, pulse files, generated artifacts, specs,
   gates, and commit history agree.

## How This Retrofits Existing Work

The first active wave, `Constraint Ledger Spine`, backfills the optimizer work
already done in recent commits. It records pavement debt, bundle/T2 blockers,
ledger/budget creation, selector adoption, and Beck diagnostic migration as
completed pulses. The next planned pulse is game/source migration into the same
ledger.
