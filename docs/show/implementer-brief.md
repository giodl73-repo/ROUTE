# ROUTE — CLI implementer brief

**Time:** 15–35 minutes. **Goal:** keep `route-cli` thin and domain-modular.

## Target shape

Canonical write-up: [`../dev/cli-layout.md`](../dev/cli-layout.md).

| Layer | Owns |
|---|---|
| `cli.rs` | clap surface only |
| `main.rs` | parse, `Ctx`, one-line dispatch |
| `commands/<domain>/*` | `run(ctx, fields) -> Result<()>` |
| `support/<domain>/*` | printers, gates, row builders |
| `types/` | shared structs peeled from main |

## Domains

`core`, `data` (including FLETCH/fetch), `map`, `stop`, `standards`, `analysis`,
`governance`, `optimizer`, `network`, `pavement`, `t1`–`t4`, `game`.

## Done-enough bar

- Almost every `Commands::*` arm is a one-liner.
- New logic never lands as a fat `main` arm.
- Domain folders stay the navigation unit.

## Hands-on

```powershell
npm run proof:public
# explore help after local build conventions in README / package scripts
```

## Next docs

- [`../../SHOWCASE.md`](../../SHOWCASE.md)
- [`../dev/cli-layout.md`](../dev/cli-layout.md)
- [`../fletch-source-orchestration-spec.md`](../fletch-source-orchestration-spec.md)
- [`../route-architecture.md`](../route-architecture.md)
