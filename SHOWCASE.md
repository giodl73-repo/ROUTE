# ROUTE Showcase

**Who this is for:** someone you would hand the repo to for 15–40 minutes —
an **infrastructure planner / researcher** asking what Interstate 2.0 should
promise, or a **CLI implementer** who wants the already-modular command layout.

**Posture:** research and tooling lab. Not a finished national plan, not
construction-ready design, not agency endorsement, and not full T1–T4 map
publication. Flagship I-80 review is **hold and narrow**; `design/` is empty
until a design yield is promoted.

| Audience | Open this first | Time |
|---|---|---|
| Planner / researcher | [Planner brief](docs/show/planner-brief.md) | 15–25 min |
| CLI implementer | [Implementer brief](docs/show/implementer-brief.md) | 15–35 min |
| Either, hands-on | [Public proof](docs/how-to/public-proof.md) (`npm run proof:public`) | ~15 min |

## One-minute pitch

The US already chose highways. ROUTE asks what the **next national road system
should promise**, using an evidence-first pipeline:

```text
PUBLIC SOURCES → CORPUS → SCORE → SERVICE PROMISE → GAP MAP
                                                     ↓
                                      CONCEPT → REVIEW → DESIGN
```

Instead of one flat “interstate = interstate” category, ROUTE uses service
tiers (T1 timed freight spine through T4 terminal access) so coast-to-coast
spines, regional connectors, feeders, and port approaches are not scored as the
same job.

## Two doors

### A. Infrastructure planner / researcher path

**Question ROUTE answers well:** *What service promises and gaps can we defend
from public sources today—and what remains held?*

| Step | What to look at | Why |
|---|---|---|
| 1 | README “Current lab posture” | Honest status table before any map |
| 2 | [Planner brief](docs/show/planner-brief.md) | Tier story + claim boundaries |
| 3 | [Public proof](docs/how-to/public-proof.md) | 15-minute external honesty edge |
| 4 | [Open adoption](docs/adoption/README.md) | Safe local adaptation path |
| 5 | Optional media pack | [`docs/media/README.md`](docs/media/README.md) |

**Planner takeaways:**

- Scoring, corpus, maps tooling, gap diagnostics, and review process exist and are gated.
- I-80 flagship packet is a **hold-and-narrow** review artifact, not an approved design.
- Map publication claims are scoped/blocked — see [`docs/map-publication-scope.md`](docs/map-publication-scope.md).
- “Milepost complete” means internal command/artifact gates, not external validation.

**Do not say:** official Interstate 2.0 plan, ROI guarantee, construction
readiness, SLA commitment, or agency endorsement.

### B. CLI implementer path

**Question ROUTE answers well:** *Where does clap end, where do commands live,
and how thin is `main` supposed to stay?*

| Step | What to look at | Why |
|---|---|---|
| 1 | [Implementer brief](docs/show/implementer-brief.md) | Dispatcher contract |
| 2 | [`docs/dev/cli-layout.md`](docs/dev/cli-layout.md) | Target modular layout |
| 3 | `commands/<domain>/*` + `support/<domain>/*` | One command = one `run` |
| 4 | Data/FLETCH surfaces under `commands/data/` | Shared fetch ledger handoff |

**Implementer takeaways:**

- `main.rs` is bootstrap + thin match; business logic stays out of fat arms.
- Domains: core, data, map, stop, standards, analysis, governance, optimizer,
  network, pavement, t1–t4, game.
- New work lands in `commands/` or `support/`, never as a new encyclopedia arm.

## Fastest hands-on (both audiences)

```powershell
npm run proof:public
```

No Census key required for the packet check path documented in public proof.
Full clean-clone regeneration may still need credentials for some sources.

## Claim packet (this showcase)

| Field | Value |
|---|---|
| Claim text | ROUTE can be shown as an Interstate 2.0 research lab with separate planner and CLI-implementer entry paths. |
| Audience | Infrastructure planners/researchers; CLI/systems implementers. |
| Evidence | README lab posture; public-proof how-to; adoption guide; cli-layout; I-80 hold packet. |
| Validation | `npm run proof:public` and existing gates; not L2 external agency validation. |
| Limitations | Design folder empty; flagship held; map publication scoped; not a national build program. |
| Non-claims | Official plan, construction, funding eligibility, guaranteed service, numeric ROI. |
| Review lane | Applied Systems / movement lane; BOUNDARY for plan-language drift. |

## Where not to start

| Avoid leading with… | Why |
|---|---|
| Full national rebuild | Logistics bury the promise model |
| Uncaveated atlas screenshots | Easy to overclaim publication readiness |
| Game/tycoon surfaces first | Side quest; not the honesty edge |

## Related

- Family hub: [`../README.md`](../README.md)
- Show pack: [`docs/show/README.md`](docs/show/README.md)
- Architecture: [`docs/route-architecture.md`](docs/route-architecture.md)
