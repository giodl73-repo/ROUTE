# route-cli layout (taste / target shape)

## Goal

`main.rs` is a **bootstrap + thin dispatcher**, not a product encyclopedia.

| Layer | Owns | Does not own |
|-------|------|----------------|
| `cli.rs` | clap surface (`Cli`, `Commands`, value enums) | business logic |
| `main.rs` | parse, `Ctx`, thin `match` → `commands::<domain>::<cmd>::run` | helpers, types, fat arms |
| `types/` | shared structs/enums peeled from main | command orchestration |
| `commands/<domain>/*` | one command = `run(ctx, fields) -> Result<()>` | clap derive |
| `support/<domain>/*` | row builders, gates, printers, shared helpers | CLI parsing |
| `game.rs` | game subsystem | new catch-all dumps |

## Command domains

```
commands/
  ctx.rs                 # shared Ctx (manifest + scoring paths)
  core/                  # build, score, score_all, report, coverage, sim, …
  data/                  # fetch*, fletch*, source_fetch_policy
  map/                   # map, map_atlas, publication*
  stop/                  # stop_* SLA / coverage
  standards/             # standards*
  analysis/              # matrices, EV, hubs, OD, interventions
  governance/            # forum, blueprint*, release, moments
  optimizer/             # optimizer_*
  network/               # national segments, cross-tier columns/regions
  pavement/              # tier_pavement_*
  t1/ t2/ t3/ t4/        # tier-specific command surfaces
  game/                  # game_cmd → game.rs
```

Support mirrors the same idea: `support/{tier,pavement,print,gates,optimizer,network,misc}/`.

## Command contract (exemplar: `commands/core/build.rs`)

```rust
use crate::*;
use crate::commands::ctx;

pub(crate) fn run(ctx: &ctx::Ctx<'_>, /* clap fields */) -> Result<()> {
    // use ctx.manifest_path / scoring_cfg
    Ok(())
}
```

`run_cli` only:

```rust
Commands::Build { .. } => commands::core::build::run(&cmd_ctx, ..)?,
```

## What "done enough" looks like

- Almost every `Commands::*` arm is a one-liner to `commands::<domain>::…`.
- New logic lands in `commands/<domain>/` or `support/<domain>/`, never as a new fat arm.
- Domain folders stay the navigation unit — avoid re-flattening to crate root.
- Tests stay out of the hot path (`tests_inline.rs` / eventual `tests/`).
- Empty `design/` stays honest until something is promoted on purpose.

## Snapshot (2026-07-29, organized)

- ~240 command modules in **15 domains**
- `support/{tier,pavement,print,gates,optimizer,network,misc}/` — ~1.2k helper modules
- `types/` — 243 shared structs/enums
- `main.rs` ~1.6k: `main` + thin `run_cli` + `tests` include only
- Crate root: `cli.rs`, `game.rs`, `main.rs`, `tests_inline.rs`, `commands/`, `support/`, `types/`

## Non-goals

- Perfect taxonomy forever (rename when a better seam appears)
- Moving clap into each command module
- Splitting `support/misc` until real seams appear
