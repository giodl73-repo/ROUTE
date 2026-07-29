# ROUTE living backlog

Single active queue. Historical waves stay under `waves/` as archive.
Do not open a new wave unless it burns a row here or ships an external artifact.

Last updated: 2026-07-29

## Now (edge credibility)

| ID | Item | Status | Notes |
|----|------|--------|-------|
| B1 | Public proof on-ramp (`npm run proof:public`) | done 2026-07-29 | `docs/how-to/public-proof.md` |
| B2 | README lab posture (design empty, holds, milepost meaning) | done 2026-07-29 | Top-of-README table |
| B3 | First `route-cli` split (`cli.rs` clap surface) | done 2026-07-29 | Superseded by S1/S2 structural split |
| B4 | External I-80 / state packet that needs no insider dialect | open | Packet exists; keep hold posture; improve first-run UX |
| B5 | ACS credential path documented only as optional | open | Never block public proof on Census key |

## Next (structure)

| ID | Item | Status | Notes |
|----|------|--------|-------|
| S1 | Split `run_cli` match arms into command modules | done 2026-07-29 | ~240 cmds in 15 domains under commands/*; thin dispatch; cli-layout.md |
| S2 | Shrink/replace god-helpers still in `main.rs` | done 2026-07-29 | main ~1.6k (main+run_cli+tests); types/ 243 items; support/* ~1.2k helpers by domain |
| S3 | Freeze micro-wave creation; use this backlog | open | Process gravity control |

## Later (product yield)

| ID | Item | Status | Notes |
|----|------|--------|-------|
| Y1 | First `design/` entry only if parliament clears | held | Empty design is honest |
| Y2 | Shared gap-detector kernel with GAUGE/PACKET/HARBOR | open | Portfolio method sync |
| Y3 | Map publication unblock only with residual ledger relief | held | See map-publication-scope |

## Explicit non-goals (for now)

- Full-library international hierarchy expansion
- Treating milepost complete as external validation
- Storing API keys in-repo
- Promoting Tycoon features that do not prove a network claim
