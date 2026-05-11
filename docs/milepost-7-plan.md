# Milepost 7 Plan — Program

Status: active.

Milepost 7 turns the Milepost 4-6 work into a repeatable release process. The goal is not to prove new highway claims. The goal is to make the current claims, holds, gates, and artifacts reproducible by someone other than the author.

## Done Criteria

Milepost 7 is done when ROUTE has:

- A scripted gate bundle for the current release surface.
- A release manifest naming the public artifacts and their verification commands.
- A checklist for what is publishable, held, internal, or source-needed.
- CI or CI-ready workflow coverage for Rust tests and release gates.
- A closeout record that reports the gate bundle result.

## Tasklist

| Slice | Task | Status | Exit Gate / Artifact |
|---|---|---|---|
| B7-A | Add Milepost 7 plan and release tasklist | ✅ done | `docs/milepost-7-plan.md` |
| B7-B | Add scripted gate bundle for Mileposts 4-6 and Program release checks | ✅ done | `scripts/check-mileposts.ps1` |
| B7-C | Add release manifest with artifact ownership and verification commands | ✅ done | `data/release-manifest.csv` |
| B7-D | Add release checklist and public/held claim policy | ✅ done | `docs/release/release-checklist.md` |
| B7-E | Add CI-ready workflow for tests and release gate script | ✅ done | `.github/workflows/ci.yml` |
| B7-F | Run release gate bundle and update tracker/index | ✅ done | Local script run plus `git diff --check` |
| B7-G | Write Milepost 7 closeout | ✅ done | `docs/milepost-7-closeout.md` |

## Release Surface

The first release surface is the current internal-public corpus:

- Scoring, atlas, gap, pressure-test, Forum, and Blueprint ledgers.
- Map atlas and game campaign fixtures.
- Closeout records for Mileposts 4, 5, and 6.
- Specs with explicit evidence labels and held-claim caveats.

The first release does not publish SLA/PTI, reliability-dollar, managed-lane, Donner, T1/T1 diamond, rural spur, or T2 relief claims as proven. Those remain release-visible holds.
