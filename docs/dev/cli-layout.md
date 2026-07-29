# route-cli layout (taste / target shape)

## Goal

`main.rs` is a **bootstrap + thin dispatcher**, not a product encyclopedia.

| Layer | Owns | Does not own |
|-------|------|----------------|
| `cli.rs` | clap surface (`Cli`, `Commands`, value enums) | business logic |
| `main.rs` | parse, `Ctx`, `match` → `commands::*::run` | multi-hundred-line arms |
| `commands/*` | one command = `run(ctx, fields) -> Result<()>` | clap derive |
| `support/*` | row builders, gates, printers, shared pure-ish helpers | CLI parsing |
| `game.rs` / `optimizer_*.rs` | existing focused subsystems | new catch-all dumps |

## Command contract (exemplar: `commands/build.rs`)

```rust
pub(crate) fn run(ctx: &ctx::Ctx<'_>, /* clap fields */) -> Result<()> {
    // use ctx.manifest_path / scoring_cfg
    // do the work
    Ok(())
}
```

`run_cli` only:

```rust
Commands::Build { .. } => commands::build::run(&cmd_ctx, ..)?,
```

## Support grouping

Prefer **domain folders** under `support/` (`tier/`, `pavement/`, `print/`, `gates/`)
over hundreds of sibling files at `src/` root named after a single function.

## What "done enough" looks like

- Almost every `Commands::*` arm is a one-liner to `commands::`.
- New logic lands in `commands/` or `support/<domain>/`, never as a new 200-line arm.
- Tests stay out of the hot path (`tests_inline.rs` / eventual `tests/`).
- Empty `design/` stays honest until something is promoted on purpose.

## Non-goals

- Perfect domain taxonomy on day one
- Moving clap into each command module
- Big-bang rewrite of remaining helper soup in one PR
