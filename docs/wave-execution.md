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

The wave tree is explicitly historical for work completed before the wave system
existed. Backfilled waves cite commit hashes or commit ranges in each pulse
instead of pretending the pulse plan existed before the work.

Current backfill coverage:

| Wave | Commit-history role |
|---|---|
| `2026-05-06-ground-survey` | Project/spec/workspace/source/scoring/research bootstrap. |
| `2026-05-07-research-module-sprint` | Papers, reviews, tier candidates, relay/SLA/intervention models. |
| `2026-05-08-instrument-calibration` | v1.4 scorer, live/proxy dimensions, confidence ledgers. |
| `2026-05-09-milepost-gates` | Atlas/Fault Lines/Pressure Test gates, source health, primitive tests. |
| `2026-05-10-system-became-playable` | Interstate Tycoon paper, CLI, browser, and campaign spine. |
| `2026-05-10-maps-became-contracts` | Map atlas, Beck schematics, game map reuse. |
| `2026-05-11-stops-bend-the-map` | Stop-first Beck grammar and T2 schematic diagnostics. |
| `2026-05-12-promise-horizon` | T1 promise selector, design review, METIS stop/topology gates. |
| `2026-05-12-optimizer-got-a-constitution` | Optimizer doctrine, T2/T3/T4 closure, bundles, manifests, source policy. |
| `2026-05-13-constraint-ledger-spine` | Closed wave: constraint ledger/budget migration across all blocker families. |
| `2026-05-13-constraint-ledger-blocker-burndown` | Active wave: turn normalized constraint-budget blockers into explicit decisions. |

The next planned pulse is `Constraint Ledger Blocker Burn-Down` pulse 01:
resolve or deliberately carry the I-84 T1 promise hard blocker.
